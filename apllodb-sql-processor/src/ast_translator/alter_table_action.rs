use apllodb_sql_parser::apllodb_ast;

use apllodb_shared_components::{AlterTableAction, ApllodbResult};

use super::AstTranslator;

impl AstTranslator {
    pub fn alter_table_action(
        ast_alter_table_action: apllodb_ast::Action,
    ) -> ApllodbResult<AlterTableAction> {
        match ast_alter_table_action {
            apllodb_ast::Action::AddColumnVariant(ac) => {
                let column_definition = Self::column_definition(ac.column_definition)?;
                Ok(AlterTableAction::AddColumn { column_definition })
            }
            apllodb_ast::Action::DropColumnVariant(dc) => {
                let column_name = Self::column_name(dc.column_name)?;
                Ok(AlterTableAction::DropColumn { column_name })
            }
        }
    }
}
