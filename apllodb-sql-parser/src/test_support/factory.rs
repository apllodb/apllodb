use crate::apllodb_ast::{
    Action, AddColumn, AlterTableCommand, ColumnConstraint, ColumnDefinition, ColumnName,
    CreateTableCommand, DataType, DropColumn, Identifier, IntegerType, NonEmptyVec,
    TableConstraint, TableElement, TableName,
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
    pub fn factory_add_col(column_definition: ColumnDefinition) -> Self {
        Self::AddColumnVariant(AddColumn { column_definition })
    }

    pub fn factory_drop_col(column_name: &str) -> Self {
        Self::DropColumnVariant(DropColumn {
            column_name: ColumnName::factory(column_name),
        })
    }
}

impl CreateTableCommand {
    pub fn factory(table_name: &str, table_elements: Vec<TableElement>) -> Self {
        Self {
            table_name: TableName::factory(table_name),
            table_elements: NonEmptyVec::new(table_elements),
        }
    }
}

impl TableElement {
    pub fn factory_coldef(column_definition: ColumnDefinition) -> Self {
        Self::ColumnDefinitionVariant(column_definition)
    }

    pub fn factory_tc(table_constraint: TableConstraint) -> Self {
        Self::TableConstraintVariant(table_constraint)
    }
}

impl TableConstraint {
    pub fn factory_pk(column_names: Vec<&str>) -> Self {
        Self::PrimaryKeyVariant(NonEmptyVec::new(
            column_names
                .into_iter()
                .map(|s| ColumnName::factory(s))
                .collect(),
        ))
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
