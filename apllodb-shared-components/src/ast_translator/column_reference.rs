use crate::{ApllodbError, ApllodbErrorKind, ApllodbResult, FullFieldReference, data_structure::reference::{
        correlation_reference::CorrelationReference, field_reference::FieldReference,
    }};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// TODO may need Catalog value when:
    /// - ast_column_reference does not have correlation and
    /// - ast_from_items are more than 1
    /// because this function has to determine which of `from1` or `from2` `field1` is from.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `ast_from_items` is empty.
    ///   - none of `ast_from_item` has field named `ast_column_reference.column_name`
    /// - [UndefinedObject](apllodb_shared_components::ApllodbErrorKind::UndefinedObject) when:
    ///   - `ast_column_reference` has a correlation but it is not any of `ast_from_items`.
    pub fn column_reference_with_table_name(
        ast_column_reference: apllodb_ast::ColumnReference,
        ast_from_items: Vec<apllodb_ast::FromItem>,
    ) -> ApllodbResult<FullFieldReference> {
        match ast_from_items.len() {
            0 => {
                return Err(ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("no FROM item. cannot detect where `{:?}` field is from", ast_column_reference),
                    None
                ))
            }
            1 => {
                let correlation_reference: CorrelationReference = match ast_column_reference.correlation {
                    Some(apllodb_ast::Correlation::TableNameVariant(ast_table_name)) => {
                        
                    }
                    Some(apllodb_ast::Correlation::AliasVariant(ast_alias)) => {

                    }
                    None => {
        
                    }
                }
        
            }
            _ => unimplemented!()
        }



        let table_name = Self::table_name(ast_table_name)?;
        let column_name = Self::column_name(ast_column_name)?;

        let correlation_reference = CorrelationReference::TableNameVariant(table_name);
        let field_reference = FieldReference::ColumnNameVariant(column_name);

        Ok(FullFieldReference::new(
            correlation_reference,
            field_reference,
        ))
    }

    // TODO column_reference_with_table_alias, ...
}
