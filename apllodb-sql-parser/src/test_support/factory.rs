use crate::apllodb_ast::{
    Action, AddColumn, AlterTableCommand, ColumnConstraint, ColumnDefinition, ColumnName, DataType,
    DropColumn, Identifier, IntegerType, NonEmptyVec, TableName,
};

impl AlterTableCommand {
    pub fn factory(table_name: &str, actions: Vec<Action>) -> Self {
        Self {
            table_name: TableName::factory(table_name),
            actions: NonEmptyVec::new(actions),
        }
    }
}

impl Action {
    pub fn factory_add_column(column_definition: ColumnDefinition) -> Self {
        Self::AddColumnVariant(AddColumn { column_definition })
    }

    pub fn factory_drop_column(column_name: &str) -> Self {
        Self::DropColumnVariant(DropColumn {
            column_name: ColumnName::factory(column_name),
        })
    }
}

impl ColumnDefinition {
    pub fn factory(
        column_name: &str,
        data_type: DataType,
        column_constraints: Vec<ColumnConstraint>,
    ) -> Self {
        Self {
            column_name: ColumnName::factory(column_name),
            data_type,
            column_constraints,
        }
    }
}

impl TableName {
    pub fn factory(column_name: &str) -> Self {
        Self(Identifier(column_name.to_string()))
    }
}

impl ColumnName {
    pub fn factory(column_name: &str) -> Self {
        Self(Identifier(column_name.to_string()))
    }
}

impl DataType {
    pub fn integer() -> Self {
        DataType::IntegerTypeVariant(IntegerType::IntegerVariant)
    }
}
