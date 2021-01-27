mod generated_parser;
mod helper;

use crate::{
    apllodb_ast::{
        types::NonEmptyVec, Action, AddColumn, Alias, AlterTableCommand, ColumnConstraint,
        ColumnDefinition, ColumnName, ColumnReference, Command, Condition, Constant, Correlation,
        CreateDatabaseCommand, CreateTableCommand, DataType, DatabaseName, DeleteCommand,
        DropColumn, DropTableCommand, Expression, FromItem, Identifier, InsertCommand,
        IntegerConstant, IntegerType, NumericConstant, SelectCommand, SelectField, TableConstraint,
        TableElement, TableName, UpdateCommand, UseDatabaseCommand,
    },
    apllodb_sql_parser::error::{ApllodbSqlParserError, ApllodbSqlParserResult},
    ApllodbAst,
};
use generated_parser::{GeneratedParser, Rule};
use helper::{parse_child, parse_child_seq, self_as_str, try_parse_child, FnParseParams};
use pest::{iterators::Pairs, Parser};
use std::convert::identity;

#[derive(Clone, Hash, Debug, Default)]
pub(crate) struct PestParserImpl;

impl PestParserImpl {
    pub(crate) fn parse<S: Into<String>>(
        &self,
        apllodb_sql: S,
    ) -> ApllodbSqlParserResult<ApllodbAst> {
        let apllodb_sql = apllodb_sql.into();

        let pairs: Pairs<Rule> =
            GeneratedParser::parse(Rule::command, &apllodb_sql).map_err(|e| {
                let reason = format!("{}", e);
                ApllodbSqlParserError::new(&apllodb_sql, reason)
            })?;

        let mut params = FnParseParams {
            apllodb_sql: &apllodb_sql,
            children_pairs: pairs.collect(),
            self_string: apllodb_sql.clone(),
        };

        parse_child(&mut params, Rule::command, Self::parse_command, ApllodbAst)
    }
}

impl PestParserImpl {
    /*
     * ================================================================================================
     * Lexical Structure:
     * ================================================================================================
     */

    /*
     * ----------------------------------------------------------------------------
     * Constants
     * ----------------------------------------------------------------------------
     */

    fn parse_constant(mut params: FnParseParams) -> ApllodbSqlParserResult<Constant> {
        parse_child(
            &mut params,
            Rule::numeric_constant,
            Self::parse_numeric_constant,
            Constant::NumericConstantVariant,
        )
    }

    fn parse_numeric_constant(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<NumericConstant> {
        parse_child(
            &mut params,
            Rule::integer_constant,
            Self::parse_integer_constant,
            NumericConstant::IntegerConstantVariant,
        )
    }

    fn parse_integer_constant(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<IntegerConstant> {
        let s = self_as_str(&mut params);
        Ok(IntegerConstant(s.into()))
    }

    /*
     * ================================================================================================
     * Identifier:
     * ================================================================================================
     */

    fn parse_identifier(mut params: FnParseParams) -> ApllodbSqlParserResult<Identifier> {
        let s = self_as_str(&mut params);
        Ok(Identifier(s.into()))
    }

    /*
     * ================================================================================================
     * Value Expressions:
     * ================================================================================================
     */

    fn parse_condition(mut params: FnParseParams) -> ApllodbSqlParserResult<Condition> {
        parse_child(
            &mut params,
            Rule::expression,
            Self::parse_expression,
            |inner_ast| Condition {
                expression: inner_ast,
            },
        )
    }

    fn parse_expression(mut params: FnParseParams) -> ApllodbSqlParserResult<Expression> {
        try_parse_child(
            &mut params,
            Rule::constant,
            Self::parse_constant,
            Expression::ConstantVariant,
        )?
        .or(try_parse_child(
            &mut params,
            Rule::column_reference,
            Self::parse_column_reference,
            Expression::ColumnReferenceVariant,
        )?)
        .ok_or_else(|| {
            ApllodbSqlParserError::new(
                params.apllodb_sql,
                "Does not match any child rule of expression.",
            )
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * Column References
     * ----------------------------------------------------------------------------
     */

    fn parse_column_reference(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<ColumnReference> {
        let correlation = try_parse_child(
            &mut params,
            Rule::correlation,
            Self::parse_correlation,
            identity,
        )?;
        let column_name = parse_child(
            &mut params,
            Rule::column_name,
            Self::parse_column_name,
            identity,
        )?;
        Ok(ColumnReference {
            correlation,
            column_name,
        })
    }

    /*
     * ================================================================================================
     * Data Types:
     * ================================================================================================
     */

    fn parse_data_type(mut params: FnParseParams) -> ApllodbSqlParserResult<DataType> {
        parse_child(
            &mut params,
            Rule::integer_type,
            Self::parse_integer_type,
            DataType::IntegerTypeVariant,
        )
    }

    /*
     * ----------------------------------------------------------------------------
     * Integer Types
     * ----------------------------------------------------------------------------
     */

    fn parse_integer_type(mut params: FnParseParams) -> ApllodbSqlParserResult<IntegerType> {
        let s = self_as_str(&mut params);
        match s {
            "SMALLINT" => Ok(IntegerType::SmallIntVariant),
            "INTEGER" => Ok(IntegerType::IntegerVariant),
            "BIGINT" => Ok(IntegerType::BigIntVariant),
            x => {
                eprintln!("Unexpected data type parsed: {}", x);
                unreachable!();
            }
        }
    }

    /*
     * ================================================================================================
     * Commands:
     * ================================================================================================
     */

    fn parse_command(mut params: FnParseParams) -> ApllodbSqlParserResult<Command> {
        try_parse_child(
            &mut params,
            Rule::create_database_command,
            Self::parse_create_database_command,
            Command::CreateDatabaseCommandVariant,
        )?
        .or(try_parse_child(
            &mut params,
            Rule::use_database_command,
            Self::parse_use_database_command,
            Command::UseDatabaseCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::alter_table_command,
            Self::parse_alter_table_command,
            Command::AlterTableCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::create_table_command,
            Self::parse_create_table_command,
            Command::CreateTableCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::delete_command,
            Self::parse_delete_command,
            Command::DeleteCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::drop_table_command,
            Self::parse_drop_table_command,
            Command::DropTableCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::insert_command,
            Self::parse_insert_command,
            Command::InsertCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::select_command,
            Self::parse_select_command,
            Command::SelectCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::update_command,
            Self::parse_update_command,
            Command::UpdateCommandVariant,
        )?)
        .ok_or_else(|| {
            ApllodbSqlParserError::new(
                params.apllodb_sql,
                "Does not match any child rule of command.",
            )
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * ALTER TABLE
     * ----------------------------------------------------------------------------
     */

    fn parse_alter_table_command(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<AlterTableCommand> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            identity,
        )?;
        let actions = parse_child_seq(&mut params, Rule::action, &Self::parse_action, &identity)?;
        Ok(AlterTableCommand {
            table_name,
            actions: NonEmptyVec::new(actions),
        })
    }

    fn parse_action(mut params: FnParseParams) -> ApllodbSqlParserResult<Action> {
        try_parse_child(
            &mut params,
            Rule::add_column,
            Self::parse_add_column,
            Action::AddColumnVariant,
        )?
        .or(try_parse_child(
            &mut params,
            Rule::drop_column,
            Self::parse_drop_column,
            Action::DropColumnVariant,
        )?)
        .ok_or_else(|| {
            ApllodbSqlParserError::new(
                params.apllodb_sql,
                "Does not match any child rule of action.",
            )
        })
    }

    fn parse_add_column(mut params: FnParseParams) -> ApllodbSqlParserResult<AddColumn> {
        let column_definition = parse_child(
            &mut params,
            Rule::column_definition,
            Self::parse_column_definition,
            identity,
        )?;
        Ok(AddColumn { column_definition })
    }

    fn parse_drop_column(mut params: FnParseParams) -> ApllodbSqlParserResult<DropColumn> {
        parse_child(
            &mut params,
            Rule::column_name,
            Self::parse_column_name,
            |inner_ast| DropColumn {
                column_name: inner_ast,
            },
        )
    }

    /*
     * ----------------------------------------------------------------------------
     * CREATE DATABASE
     * ----------------------------------------------------------------------------
     */

    fn parse_create_database_command(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<CreateDatabaseCommand> {
        let database_name = parse_child(
            &mut params,
            Rule::database_name,
            Self::parse_database_name,
            identity,
        )?;
        Ok(CreateDatabaseCommand { database_name })
    }

    /*
     * ----------------------------------------------------------------------------
     * Use DATABASE
     * ----------------------------------------------------------------------------
     */

    fn parse_use_database_command(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<UseDatabaseCommand> {
        let database_name = parse_child(
            &mut params,
            Rule::database_name,
            Self::parse_database_name,
            identity,
        )?;
        Ok(UseDatabaseCommand { database_name })
    }

    /*
     * ----------------------------------------------------------------------------
     * CREATE TABLE
     * ----------------------------------------------------------------------------
     */

    fn parse_create_table_command(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<CreateTableCommand> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            identity,
        )?;
        let table_elements = parse_child_seq(
            &mut params,
            Rule::table_element,
            &Self::parse_table_element,
            &identity,
        )?;
        Ok(CreateTableCommand {
            table_name,
            table_elements: NonEmptyVec::new(table_elements),
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * DELETE
     * ----------------------------------------------------------------------------
     */

    fn parse_delete_command(mut params: FnParseParams) -> ApllodbSqlParserResult<DeleteCommand> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            &Self::parse_table_name,
            &identity,
        )?;
        let alias = try_parse_child(&mut params, Rule::alias, Self::parse_alias, identity)?;
        let where_condition = try_parse_child(
            &mut params,
            Rule::condition,
            Self::parse_condition,
            identity,
        )?;
        Ok(DeleteCommand {
            table_name,
            alias,
            where_condition,
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * DROP TABLE
     * ----------------------------------------------------------------------------
     */

    fn parse_drop_table_command(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<DropTableCommand> {
        parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            |inner_ast| DropTableCommand {
                table_name: inner_ast,
            },
        )
    }

    /*
     * ----------------------------------------------------------------------------
     * INSERT
     * ----------------------------------------------------------------------------
     */

    fn parse_insert_command(mut params: FnParseParams) -> ApllodbSqlParserResult<InsertCommand> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            &Self::parse_table_name,
            &identity,
        )?;
        let alias = try_parse_child(&mut params, Rule::alias, Self::parse_alias, identity)?;
        let column_names = parse_child_seq(
            &mut params,
            Rule::column_name,
            &Self::parse_column_name,
            &identity,
        )?;
        let expressions = parse_child_seq(
            &mut params,
            Rule::expression,
            &Self::parse_expression,
            &identity,
        )?;
        Ok(InsertCommand {
            table_name,
            alias,
            column_names: NonEmptyVec::new(column_names),
            expressions: NonEmptyVec::new(expressions),
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * SELECT
     * ----------------------------------------------------------------------------
     */

    fn parse_select_command(mut params: FnParseParams) -> ApllodbSqlParserResult<SelectCommand> {
        let select_fields = parse_child_seq(
            &mut params,
            Rule::select_field,
            &Self::parse_select_field,
            &identity,
        )?;
        let from_items = parse_child_seq(
            &mut params,
            Rule::from_item,
            &Self::parse_from_item,
            &identity,
        )?;
        let where_condition = try_parse_child(
            &mut params,
            Rule::condition,
            Self::parse_condition,
            identity,
        )?;
        Ok(SelectCommand {
            select_fields: NonEmptyVec::new(select_fields),
            from_items: NonEmptyVec::new(from_items),
            where_condition,
            // TODO: grouping_elements, having_conditions, order_bys
            grouping_elements: None,
            having_conditions: None,
            order_bys: None,
        })
    }

    fn parse_select_field(mut params: FnParseParams) -> ApllodbSqlParserResult<SelectField> {
        let expression = parse_child(
            &mut params,
            Rule::expression,
            Self::parse_expression,
            identity,
        )?;
        let alias = try_parse_child(&mut params, Rule::alias, Self::parse_alias, identity)?;
        Ok(SelectField { expression, alias })
    }

    fn parse_from_item(mut params: FnParseParams) -> ApllodbSqlParserResult<FromItem> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            identity,
        )?;
        let alias = try_parse_child(&mut params, Rule::alias, Self::parse_alias, identity)?;
        Ok(FromItem { table_name, alias })
    }

    /*
     * ----------------------------------------------------------------------------
     * UPDATE
     * ----------------------------------------------------------------------------
     */

    fn parse_update_command(mut params: FnParseParams) -> ApllodbSqlParserResult<UpdateCommand> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            &Self::parse_table_name,
            &identity,
        )?;
        let alias = try_parse_child(&mut params, Rule::alias, Self::parse_alias, identity)?;
        let column_name = parse_child(
            &mut params,
            Rule::column_name,
            &Self::parse_column_name,
            &identity,
        )?;
        let expression = parse_child(
            &mut params,
            Rule::expression,
            &Self::parse_expression,
            &identity,
        )?;
        let where_condition = try_parse_child(
            &mut params,
            Rule::condition,
            Self::parse_condition,
            identity,
        )?;
        Ok(UpdateCommand {
            table_name,
            alias,
            column_name,
            expression,
            where_condition,
        })
    }

    /*
     * ================================================================================================
     * Misc:
     * ================================================================================================
     */

    /*
     * ----------------------------------------------------------------------------
     * Names
     * ----------------------------------------------------------------------------
     */

    fn parse_database_name(mut params: FnParseParams) -> ApllodbSqlParserResult<DatabaseName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            DatabaseName,
        )
    }

    fn parse_table_name(mut params: FnParseParams) -> ApllodbSqlParserResult<TableName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            TableName,
        )
    }

    fn parse_column_name(mut params: FnParseParams) -> ApllodbSqlParserResult<ColumnName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            ColumnName,
        )
    }

    fn parse_alias(mut params: FnParseParams) -> ApllodbSqlParserResult<Alias> {
        parse_child(&mut params, Rule::identifier, Self::parse_identifier, Alias)
    }

    fn parse_correlation(mut params: FnParseParams) -> ApllodbSqlParserResult<Correlation> {
        try_parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            Correlation::TableNameVariant,
        )?
        .or(try_parse_child(
            &mut params,
            Rule::alias,
            Self::parse_alias,
            Correlation::AliasVariant,
        )?)
        .ok_or_else(|| {
            ApllodbSqlParserError::new(
                params.apllodb_sql,
                "Does not match any child rule of correlation.",
            )
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * Table Elements
     * ----------------------------------------------------------------------------
     */

    fn parse_table_element(mut params: FnParseParams) -> ApllodbSqlParserResult<TableElement> {
        try_parse_child(
            &mut params,
            Rule::column_definition,
            Self::parse_column_definition,
            TableElement::ColumnDefinitionVariant,
        )?
        .or(try_parse_child(
            &mut params,
            Rule::table_constraint,
            Self::parse_table_constraint,
            TableElement::TableConstraintVariant,
        )?)
        .ok_or_else(|| {
            ApllodbSqlParserError::new(
                params.apllodb_sql,
                "Does not match any child rule of table_element",
            )
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * Constraints
     * ----------------------------------------------------------------------------
     */

    fn parse_column_constraint(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<ColumnConstraint> {
        let s = self_as_str(&mut params);
        match s {
            "NOT NULL" => Ok(ColumnConstraint::NotNullVariant),
            x => {
                eprintln!("Unexpected constraint parsed: {}", x);
                unreachable!();
            }
        }
    }

    fn parse_table_constraint(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<TableConstraint> {
        let primary_key = parse_child_seq(
            &mut params,
            Rule::column_name,
            &Self::parse_column_name,
            &identity,
        )?;
        Ok(TableConstraint::PrimaryKeyVariant(NonEmptyVec::new(
            primary_key,
        )))
    }

    /*
     * ----------------------------------------------------------------------------
     * Column Definitions
     * ----------------------------------------------------------------------------
     */

    fn parse_column_definition(
        mut params: FnParseParams,
    ) -> ApllodbSqlParserResult<ColumnDefinition> {
        let column_name = parse_child(
            &mut params,
            Rule::column_name,
            Self::parse_column_name,
            identity,
        )?;
        let data_type = parse_child(
            &mut params,
            Rule::data_type,
            Self::parse_data_type,
            identity,
        )?;
        let column_constraints = parse_child_seq(
            &mut params,
            Rule::column_constraint,
            &Self::parse_column_constraint,
            &identity,
        )?;
        Ok(ColumnDefinition {
            column_name,
            data_type,
            column_constraints,
        })
    }
}
