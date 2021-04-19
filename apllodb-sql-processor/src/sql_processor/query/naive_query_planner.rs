mod select_command_analyzer;

use std::collections::HashSet;

use apllodb_shared_components::{ApllodbResult, SchemaIndex};
use apllodb_sql_parser::apllodb_ast::{self};
use apllodb_storage_engine_interface::RowProjectionQuery;

use super::query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository;
use crate::{
    correlation::correlation_name::CorrelationName,
    sql_processor::query::query_plan::query_plan_tree::{
        query_plan_node::{
            node_kind::{QueryPlanNodeKind, QueryPlanNodeLeaf, QueryPlanNodeUnary},
            operation::{LeafPlanOperation, UnaryPlanOperation},
        },
        QueryPlanTree,
    },
};
use select_command_analyzer::SelectCommandAnalyzer;

/// Translates [SelectCommand](apllodb_sql_parser::apllodb_ast::SelectCommand) into [QueryPlanTree](crate::sql_processor::query::query_plan::query_plan_tree::QueryPlanTree).
///
/// Output tree has the following form:
///
/// ```text
/// proj
///  |
/// aggregation
///  |
/// sort
///  |
/// selection
///  |
/// join
///  |------+
/// CORR   CORR
/// ```
///
/// Nodes are created from bottom to top.
#[derive(Clone, Debug)]
pub(crate) struct NaiveQueryPlanner<'r> {
    node_repo: &'r QueryPlanNodeRepository,

    analyzer: SelectCommandAnalyzer,
}

impl<'r> NaiveQueryPlanner<'r> {
    pub(crate) fn new(
        node_repo: &'r QueryPlanNodeRepository,
        select_command: apllodb_ast::SelectCommand,
    ) -> Self {
        Self {
            node_repo,
            analyzer: SelectCommandAnalyzer::new(select_command),
        }
    }

    pub(crate) fn run(&self) -> ApllodbResult<QueryPlanTree> {
        self.create_correlation_nodes()?;
        self.create_join_nodes()?;
        self.create_selection_node()?;
        self.create_sort_node()?;
        // aggregation
        self.create_projection_node()?;

        Ok(QueryPlanTree::new(self.node_repo.latest_node_id()?))
    }

    fn create_correlation_nodes(&self) -> ApllodbResult<()> {
        let from_item_correlations = self.analyzer.from_item_correlations()?;
        let widest_schema = self.analyzer.widest_schema()?;

        for aliased_correlation_name in &from_item_correlations {
            match &aliased_correlation_name.correlation_name {
                CorrelationName::TableNameVariant(table_name) => {
                    let prj_idxs: HashSet<SchemaIndex> = widest_schema
                        .filter_by_correlations(&[aliased_correlation_name.clone()])
                        .to_aliased_field_names()
                        .iter()
                        .map(SchemaIndex::from)
                        .collect();

                    self.node_repo
                        .create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                            op: LeafPlanOperation::SeqScan {
                                table_name: table_name.clone(),
                                projection: RowProjectionQuery::ColumnIndexes(prj_idxs),
                                aliaser: self.analyzer.aliaser()?,
                            },
                        }));
                }
            }
        }

        Ok(())
    }

    /// # Limitations
    ///
    /// - Leaf correlations are supposed to be from SeqScan.
    fn create_join_nodes(&self) -> ApllodbResult<()> {
        self.analyzer.create_join_nodes(&self.node_repo)
    }

    fn create_selection_node(&self) -> ApllodbResult<()> {
        if let Some(condition) = self.analyzer.selection_condition()? {
            let selection_op = UnaryPlanOperation::Selection { condition };
            let child_id = self.node_repo.latest_node_id()?;

            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: selection_op,
                    left: child_id,
                }));
        }
        Ok(())
    }

    fn create_sort_node(&self) -> ApllodbResult<()> {
        let index_orderings = self.analyzer.sort_index_orderings()?;
        if index_orderings.is_empty() {
            Ok(())
        } else {
            let sort_op = UnaryPlanOperation::Sort { index_orderings };
            let child_id = self.node_repo.latest_node_id()?;

            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: sort_op,
                    left: child_id,
                }));

            Ok(())
        }
    }

    fn create_projection_node(&self) -> ApllodbResult<()> {
        let afns = self.analyzer.aliased_field_names_in_projection()?;

        let projection_op = UnaryPlanOperation::Projection {
            fields: afns.iter().map(SchemaIndex::from).collect(),
        };
        let child_id = self.node_repo.latest_node_id()?;

        let _ = self
            .node_repo
            .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                op: projection_op,
                left: child_id,
            }));

        Ok(())
    }
}
