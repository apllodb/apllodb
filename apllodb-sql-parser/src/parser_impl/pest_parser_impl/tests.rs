mod alter_table;
mod create_table;
mod delete;
mod drop_table;
mod insert;
mod select;
mod update;

use crate::apllodb_ast::{
    ColumnConstraint, ColumnDefinition, ColumnName, CreateTableCommand, DataType, Identifier,
    IntegerType, NonEmptyVec, TableConstraint, TableElement, TableName,
};

fn create_table(table_name: &str, table_elements: Vec<TableElement>) -> CreateTableCommand {
    CreateTableCommand {
        table_name: TableName(Identifier(table_name.to_string())),
        table_elements: NonEmptyVec::new(table_elements),
    }
}

fn te_coldef(
    column_name: &str,
    data_type: DataType,
    column_constraints: Vec<ColumnConstraint>,
) -> TableElement {
    TableElement::ColumnDefinitionVariant(ColumnDefinition {
        column_name: ColumnName(Identifier(column_name.to_string())),
        data_type,
        column_constraints,
    })
}
fn te_pk(column_names: Vec<&str>) -> TableElement {
    TableElement::TableConstraintVariant(TableConstraint::PrimaryKeyVariant(NonEmptyVec::new(
        column_names
            .into_iter()
            .map(|s| ColumnName(Identifier(s.to_string())))
            .collect(),
    )))
}

impl DataType {
    fn integer() -> Self {
        DataType::IntegerTypeVariant(IntegerType::IntegerVariant)
    }
}
