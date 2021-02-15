use crate::apllodb_ast::{
    Action, AddColumn, Alias, AlterTableCommand, BinaryOperator, CharacterType, ColumnConstraint,
    ColumnDefinition, ColumnName, ColumnReference, Condition, Constant, Correlation,
    CreateDatabaseCommand, CreateTableCommand, DataType, DatabaseName, DeleteCommand, DropColumn,
    DropTableCommand, Expression, FromItem, GroupingElement, Identifier, InsertCommand,
    IntegerConstant, IntegerType, NonEmptyVec, NumericConstant, OrderBy, Ordering, SelectCommand,
    SelectField, StringConstant, TableConstraint, TableElement, TableName, UnaryOperator,
    UpdateCommand, UseDatabaseCommand,
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

impl CreateDatabaseCommand {
    pub fn factory(database_name: &str) -> Self {
        Self {
            database_name: DatabaseName::factory(database_name),
        }
    }
}

impl UseDatabaseCommand {
    pub fn factory(database_name: &str) -> Self {
        Self {
            database_name: DatabaseName::factory(database_name),
        }
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

impl SelectCommand {
    pub fn factory(
        select_fields: Vec<SelectField>,
        from_items: Option<Vec<FromItem>>,
        where_condition: Option<Condition>,
        grouping_elements: Option<Vec<GroupingElement>>,
        having_conditions: Option<Vec<Condition>>,
        order_bys: Option<Vec<OrderBy>>,
    ) -> Self {
        Self {
            select_fields: NonEmptyVec::new(select_fields),
            from_items: from_items.map(NonEmptyVec::new),
            where_condition,
            grouping_elements: grouping_elements.map(NonEmptyVec::new),
            having_conditions: having_conditions.map(NonEmptyVec::new),
            order_bys: order_bys.map(NonEmptyVec::new),
        }
    }
}

impl SelectField {
    pub fn factory(expression: Expression, alias: Option<&str>) -> Self {
        Self {
            expression,
            alias: alias.map(Alias::factory),
        }
    }
}

impl FromItem {
    pub fn factory(table_name: &str, alias: Option<&str>) -> Self {
        Self {
            table_name: TableName::factory(table_name),
            alias: alias.map(Alias::factory),
        }
    }
}

impl OrderBy {
    pub fn factory_expr(expression: Expression, ordering: Option<Ordering>) -> Self {
        Self {
            expression,
            ordering,
        }
    }

    pub fn factory_colref(column_reference: ColumnReference, ordering: Option<Ordering>) -> Self {
        Self {
            expression: Expression::factory_colref(column_reference),
            ordering,
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

impl UpdateCommand {
    pub fn factory(
        table_name: &str,
        alias: Option<&str>,
        column_name: &str,
        expression: Expression,
        where_condition: Option<Condition>,
    ) -> Self {
        Self {
            table_name: TableName::factory(table_name),
            alias: alias.map(Alias::factory),
            column_name: ColumnName::factory(column_name),
            expression,
            where_condition,
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
    pub fn factory_null() -> Self {
        Self::ConstantVariant(Constant::factory_null())
    }

    pub fn factory_integer(integer: &str) -> Self {
        Self::ConstantVariant(Constant::factory_integer(integer))
    }

    pub fn factory_text(text: &str) -> Self {
        Self::ConstantVariant(Constant::factory_text(text))
    }

    pub fn factory_colref(column_reference: ColumnReference) -> Self {
        Self::ColumnReferenceVariant(column_reference)
    }

    pub fn factory_uni_op(unary_operator: UnaryOperator, expression: Expression) -> Self {
        Self::UnaryOperatorVariant(unary_operator, Box::new(expression))
    }

    pub fn factory_eq(left_expression: Expression, right_expression: Expression) -> Self {
        Self::BinaryOperatorVariant(
            BinaryOperator::Equal,
            Box::new(left_expression),
            Box::new(right_expression),
        )
    }
}

impl Constant {
    pub fn factory_null() -> Self {
        Self::NullVariant
    }

    pub fn factory_integer(integer: &str) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(IntegerConstant(
            integer.to_string(),
        )))
    }

    pub fn factory_text(text: &str) -> Self {
        Self::StringConstantVariant(StringConstant(text.to_string()))
    }
}

impl DatabaseName {
    pub fn factory(database_name: &str) -> Self {
        Self(Identifier(database_name.to_string()))
    }
}

impl TableName {
    pub fn factory(table_name: &str) -> Self {
        Self(Identifier(table_name.to_string()))
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
    pub fn factory(correlation_name: &str) -> Self {
        Self(Identifier(correlation_name.to_string()))
    }
}

impl DataType {
    pub fn integer() -> Self {
        DataType::IntegerTypeVariant(IntegerType::IntegerVariant)
    }

    pub fn text() -> Self {
        DataType::CharacterTypeVariant(CharacterType::TextVariant)
    }
}
