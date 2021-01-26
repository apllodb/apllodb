use crate::apllodb_ast::{
    Action, AddColumn, Alias, AlterTableCommand, ColumnConstraint, ColumnDefinition, ColumnName,
    ColumnReference, Condition, Constant, Correlation, CreateTableCommand, DataType, DeleteCommand,
    DropColumn, DropTableCommand, Expression, Identifier, InsertCommand, IntegerConstant,
    IntegerType, NonEmptyVec, NumericConstant, TableConstraint, TableElement, TableName,
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

impl InsertCommand {
    pub fn factory(
        table_name: &str,
        alias: Option<&str>,
        column_names: Vec<&str>,
        expressions: Vec<Expression>,
    ) -> Self {
        Self {
            table_name: TableName::factory(table_name),
            alias: alias.map(Alias::factory),
            column_names: NonEmptyVec::new(
                column_names.into_iter().map(ColumnName::factory).collect(),
            ),
            expressions: NonEmptyVec::new(expressions),
        }
    }
}

impl DropTableCommand {
    pub fn factory(table_name: &str) -> Self {
        Self {
            table_name: TableName::factory(table_name),
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

impl DeleteCommand {
    pub fn factory(
        table_name: &str,
        alias: Option<&str>,
        where_condition: Option<Condition>,
    ) -> Self {
        Self {
            table_name: TableName::factory(table_name),
            alias: alias.map(Alias::factory),
            where_condition,
        }
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

impl Expression {
    pub fn factory_integer(integer: &str) -> Self {
        Self::ConstantVariant(Constant::NumericConstantVariant(
            NumericConstant::IntegerConstantVariant(IntegerConstant(integer.to_string())),
        ))
    }

    pub fn factory_colref(column_reference: ColumnReference) -> Self {
        Self::ColumnReferenceVariant(column_reference)
    }
}

impl TableName {
    pub fn factory(column_name: &str) -> Self {
        Self(Identifier(column_name.to_string()))
    }
}

impl Alias {
    pub fn factory(name: &str) -> Self {
        Self(Identifier(name.to_string()))
    }
}

impl ColumnName {
    pub fn factory(column_name: &str) -> Self {
        Self(Identifier(column_name.to_string()))
    }
}

impl ColumnReference {
    pub fn factory(correlation: Option<Correlation>, column_name: &str) -> Self {
        Self {
            correlation,
            column_name: ColumnName::factory(column_name),
        }
    }
}

impl Correlation {
    pub fn factory_tn(table_name: &str) -> Self {
        Self::TableNameVariant(TableName::factory(table_name))
    }

    pub fn factory_alias(alias: &str) -> Self {
        Self::AliasVariant(Alias::factory(alias))
    }
}

impl DataType {
    pub fn integer() -> Self {
        DataType::IntegerTypeVariant(IntegerType::IntegerVariant)
    }
}
