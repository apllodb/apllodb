use crate::{
    ApllodbResult, ColumnConstraintKind, ColumnConstraints, ColumnDataType, ColumnDefinition,
    ColumnReference, TableName,
};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn column_definition(
        ast_column_definition: apllodb_ast::ColumnDefinition,
        table_name: TableName,
    ) -> ApllodbResult<ColumnDefinition> {
        let column_name = Self::column_name(ast_column_definition.column_name)?;
        let column_reference = ColumnReference::new(table_name, column_name);

        let nullable = Self::nullable(&ast_column_definition.column_constraints);

        let sql_type = Self::data_type(ast_column_definition.data_type);

        let column_constraint_kinds: Vec<ColumnConstraintKind> = ast_column_definition
            .column_constraints
            .into_iter()
            .flat_map(Self::column_constraint)
            .collect();
        let column_constraints = ColumnConstraints::new(column_constraint_kinds)?;

        let column_data_type = ColumnDataType::new(column_reference, sql_type, nullable);

        Ok(ColumnDefinition::new(column_data_type, column_constraints))
    }
}
