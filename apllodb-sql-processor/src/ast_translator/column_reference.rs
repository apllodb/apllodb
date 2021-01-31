use apllodb_shared_components::{ApllodbResult, ColumnReference};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
    ) -> ApllodbResult<ColumnReference> {
        let column_name = Self::column_name(ast_column_reference.column_name)?;

        let table_name = match ast_column_reference.correlation {
            Some(apllodb_ast::Correlation::TableNameVariant(table_name)) => {
                Self::table_name(table_name)?
            }
            Some(apllodb_ast::Correlation::AliasVariant(_)) => {
                unimplemented!()
            }
            None => {
                unimplemented!()
            }
        };

        Ok(ColumnReference::new(table_name, column_name))
    }
}
