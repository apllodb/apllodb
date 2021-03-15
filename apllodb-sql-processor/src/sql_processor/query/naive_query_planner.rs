mod select_command_analyzer;

use apllodb_shared_components::{ApllodbResult, CorrelationIndex, FieldIndex};
use apllodb_sql_parser::apllodb_ast::{self};
use apllodb_storage_engine_interface::ProjectionQuery;

use super::query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository;
use crate::sql_processor::query::query_plan::query_plan_tree::{
    query_plan_node::{
        node_kind::{QueryPlanNodeKind, QueryPlanNodeLeaf, QueryPlanNodeUnary},
        operation::{LeafPlanOperation, UnaryPlanOperation},
    },
    QueryPlanTree,
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

        // join

        self.create_selection_node()?;

        self.create_sort_node()?;

        // aggregation

        self.create_projection_node()?;

        Ok(QueryPlanTree::new(self.node_repo.latest_node_id()?))
    }

    fn create_correlation_nodes(&self) -> ApllodbResult<()> {
        let from_correlations = self.analyzer.from_item_correlation_references()?;
        let widest_schema = self.analyzer.widest_schema()?;

        for corref in from_correlations {
            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: corref.as_table_name().clone(),
                        projection: ProjectionQuery::Schema(
                            widest_schema.filter_by_correlation(&CorrelationIndex::from(corref)),
                        ),
                    },
                }));
        }

        Ok(())
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
        let ffr_orderings = self.analyzer.sort_ffr_orderings()?;
        if ffr_orderings.is_empty() {
            Ok(())
        } else {
            let field_orderings = ffr_orderings
                .into_iter()
                .map(|(ffr, ordering)| (FieldIndex::from(ffr), ordering))
                .collect();

            let sort_op = UnaryPlanOperation::Sort { field_orderings };
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
        let ffrs = self.analyzer.projection_ffrs()?;

        let projection_op = UnaryPlanOperation::Projection {
            fields: ffrs.into_iter().map(FieldIndex::from).collect(),
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
