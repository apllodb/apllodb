use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::{
    ColumnConstraintKind, ColumnConstraints, ColumnDataType, ColumnDefinition,
};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn column_definition(
        ast_column_definition: apllodb_ast::ColumnDefinition,
    ) -> ApllodbResult<ColumnDefinition> {
        let column_name = Self::column_name(ast_column_definition.column_name)?;

        let nullable = Self::nullable(&ast_column_definition.column_constraints);

        let sql_type = Self::data_type(ast_column_definition.data_type);

        let column_constraint_kinds: Vec<ColumnConstraintKind> = ast_column_definition
            .column_constraints
            .into_iter()
            .flat_map(Self::column_constraint)
            .collect();
        let column_constraints = ColumnConstraints::new(column_constraint_kinds)?;

        let column_data_type = ColumnDataType::new(column_name, sql_type, nullable);

        Ok(ColumnDefinition::new(column_data_type, column_constraints))
    }
}
