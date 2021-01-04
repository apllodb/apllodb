use apllodb_shared_components::{ApllodbResult, TableConstraintKind};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn table_constraint(
        ast_table_constraint: apllodb_ast::TableConstraint,
    ) -> ApllodbResult<TableConstraintKind> {
        match ast_table_constraint {
            apllodb_ast::TableConstraint::PrimaryKeyVariant(column_names) => {
                Ok(TableConstraintKind::PrimaryKey {
                    column_names: column_names
                        .into_vec()
                        .into_iter()
                        .map(Self::column_name)
                        .collect::<ApllodbResult<_>>()?,
                })
            }
        }
    }
}
