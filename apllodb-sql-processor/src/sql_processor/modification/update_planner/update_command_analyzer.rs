use std::collections::HashMap;

use apllodb_shared_components::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::{ColumnName, TableName};

use crate::{ast_translator::AstTranslator, condition::Condition};

#[derive(Clone, Debug, new)]
pub(crate) struct UpdateCommandAnalyzer {
    command: apllodb_ast::UpdateCommand,
}

impl UpdateCommandAnalyzer {
    pub(super) fn table_name_to_update(&self) -> ApllodbResult<TableName> {
        AstTranslator::table_name(self.command.table_name.clone())
    }

    pub(super) fn update_column_values(&self) -> ApllodbResult<HashMap<ColumnName, Expression>> {
        let column_name = AstTranslator::column_name(self.command.column_name.clone())?;
        let expression = AstTranslator::expression_in_non_select(
            self.command.expression.clone(),
            vec![self.table_name_to_update()?],
        )?;

        let mut r = HashMap::<ColumnName, Expression>::new();
        r.insert(column_name, expression);
        Ok(r)
    }

    pub(super) fn where_condition(&self) -> ApllodbResult<Option<Condition>> {
        let opt_expression = if let Some(ast_condition) = &self.command.where_condition {
            let expr = AstTranslator::expression_in_non_select(
                ast_condition.clone().expression,
                vec![self.table_name_to_update()?],
            )?;
            Some(Condition::new(expr))
        } else {
            None
        };
        Ok(opt_expression)
    }
}
