mod update_command_analyzer;

use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast;

use self::update_command_analyzer::UpdateCommandAnalyzer;

use super::modification_plan::modification_plan_tree::{
    modification_plan_node::{ModificationPlanNode, UpdateNode},
    ModificationPlanTree,
};

/// Translates [UpdateCommand](apllodb_sql_parser::apllodb_ast::UpdateCommandCommand) into [ModificationPlanTree](crate::sql_processor::modification::modification_plan::ModificationPlanTree).
#[derive(Clone, Debug)]
pub(crate) struct UpdatePlanner {
    analyzer: UpdateCommandAnalyzer,
}

impl UpdatePlanner {
    pub(crate) fn new(update_command: apllodb_ast::UpdateCommand) -> Self {
        Self {
            analyzer: UpdateCommandAnalyzer::new(update_command),
        }
    }

    pub(crate) fn run(&self) -> ApllodbResult<ModificationPlanTree> {
        let table_name = self.analyzer.table_name_to_update()?;
        let column_values = self.analyzer.update_column_values()?;

        let plan_node = ModificationPlanNode::Update(UpdateNode {
            table_name,
            column_values,
        });

        Ok(ModificationPlanTree::new(plan_node))
    }
}
