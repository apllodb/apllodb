use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast;

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
    node_kind::{QueryPlanNodeKind, QueryPlanNodeLeaf},
    node_repo::QueryPlanNodeRepository,
    operation::LeafPlanOperation,
};

use self::insert_command_analyzer::InsertCommandAnalyzer;

use super::modification_plan::modification_plan_tree::{
    modification_plan_node::{InsertNode, ModificationPlanNode},
    ModificationPlanTree,
};

mod insert_command_analyzer;

/// Translates [InsertCommand](apllodb_sql_parser::apllodb_ast::InsertCommand) into [ModificationPlanTree](crate::sql_processor::modification::modification_plan::ModificationPlanTree).
#[derive(Clone, Debug)]
pub(crate) struct InsertPlanner<'r> {
    node_repo: &'r QueryPlanNodeRepository,

    analyzer: InsertCommandAnalyzer,
}

impl<'r> InsertPlanner<'r> {
    pub(crate) fn new(
        node_repo: &'r QueryPlanNodeRepository,
        insert_command: apllodb_ast::InsertCommand,
    ) -> Self {
        Self {
            node_repo,
            analyzer: InsertCommandAnalyzer::new(insert_command),
        }
    }

    pub(crate) fn run(&self) -> ApllodbResult<ModificationPlanTree> {
        let table_name = self.analyzer.table_name_to_insert()?;
        let records = self.analyzer.records_to_insert()?;

        let records_query_node_id =
            self.node_repo
                .create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::Values { records },
                }));

        let plan_node = ModificationPlanNode::Insert(InsertNode {
            table_name,
            child: records_query_node_id,
        });

        Ok(ModificationPlanTree::new(plan_node))
    }
}
