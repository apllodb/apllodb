mod generated_parser;
mod helper;

#[cfg(test)]
mod tests;

use crate::{
    apllo_ast::{
        types::NonEmptyVec, Action, AddColumn, AlterTableCommand, ColumnConstraint, ColumnName,
        Command, CreateTableColumnDefinition, CreateTableCommand, DataType, DropColumn,
        DropTableCommand, Identifier, IntegerType, TableName,
    },
    apllo_sql_parser::{AplloSqlParserError, AplloSqlParserResult},
    parser_interface::ParserLike,
    AplloAst,
};
use generated_parser::{GeneratedParser, Rule};
use helper::{parse_child, parse_child_seq, self_as_str, try_parse_child, FnParseParams};
use pest::{iterators::Pairs, Parser};
use std::convert::identity;

#[derive(Clone, Hash, Debug)]
pub(crate) struct PestParserImpl;

impl PestParserImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl ParserLike for PestParserImpl {
    fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloSqlParserResult<AplloAst> {
        let apllo_sql = apllo_sql.into();

        let pairs: Pairs<Rule> =
            GeneratedParser::parse(Rule::command, &apllo_sql).map_err(|e| {
                let reason = format!("{}", e);
                AplloSqlParserError::new(&apllo_sql, reason)
            })?;

        let mut params = FnParseParams {
            apllo_sql: &apllo_sql,
            children_pairs: pairs.collect(),
            self_string: apllo_sql.clone(),
        };

        parse_child(
            &mut params,
            Rule::command,
            Self::parse_command,
            |inner_ast| AplloAst(inner_ast),
        )
    }
}

impl PestParserImpl {
    /*
     * ================================================================================================
     * Identifier:
     * ================================================================================================
     */

    fn parse_identifier(mut params: FnParseParams) -> AplloSqlParserResult<Identifier> {
        let s = self_as_str(&mut params);
        Ok(Identifier(s.into()))
    }

    /*
     * ================================================================================================
     * Value Expressions:
     * ================================================================================================
     */

    /*
     * ================================================================================================
     * Data Types:
     * ================================================================================================
     */

    fn parse_data_type(mut params: FnParseParams) -> AplloSqlParserResult<DataType> {
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

    fn parse_integer_type(mut params: FnParseParams) -> AplloSqlParserResult<IntegerType> {
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

    fn parse_command(mut params: FnParseParams) -> AplloSqlParserResult<Command> {
        try_parse_child(
            &mut params,
            Rule::alter_table_command,
            Self::parse_alter_table_command,
            Command::AlterTableCommandVariant,
        )?
        .or(try_parse_child(
            &mut params,
            Rule::create_table_command,
            Self::parse_create_table_command,
            Command::CreateTableCommandVariant,
        )?)
        .or(try_parse_child(
            &mut params,
            Rule::drop_table_command,
            Self::parse_drop_table_command,
            Command::DropTableCommandVariant,
        )?)
        .ok_or(AplloSqlParserError::new(
            params.apllo_sql,
            "Does not match any child rule of command.",
        ))
    }

    /*
     * ----------------------------------------------------------------------------
     * ALTER TABLE
     * ----------------------------------------------------------------------------
     */

    fn parse_alter_table_command(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<AlterTableCommand> {
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

    fn parse_action(mut params: FnParseParams) -> AplloSqlParserResult<Action> {
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
        .ok_or(AplloSqlParserError::new(
            params.apllo_sql,
            "Does not match any child rule of action.",
        ))
    }

    fn parse_add_column(mut params: FnParseParams) -> AplloSqlParserResult<AddColumn> {
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
        Ok(AddColumn {
            column_name,
            data_type,
            column_constraints,
        })
    }

    fn parse_drop_column(mut params: FnParseParams) -> AplloSqlParserResult<DropColumn> {
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
     * CREATE TABLE
     * ----------------------------------------------------------------------------
     */

    fn parse_create_table_command(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<CreateTableCommand> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            identity,
        )?;
        let create_table_column_definitions = parse_child_seq(
            &mut params,
            Rule::create_table_column_definition,
            &Self::parse_create_table_column_definition,
            &identity,
        )?;
        Ok(CreateTableCommand {
            table_name,
            create_table_column_definitions: NonEmptyVec::new(create_table_column_definitions),
        })
    }

    fn parse_create_table_column_definition(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<CreateTableColumnDefinition> {
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
        Ok(CreateTableColumnDefinition {
            column_name,
            data_type,
            column_constraints,
        })
    }

    /*
     * ----------------------------------------------------------------------------
     * DROP TABLE
     * ----------------------------------------------------------------------------
     */

    fn parse_drop_table_command(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<DropTableCommand> {
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
     * ================================================================================================
     * Misc:
     * ================================================================================================
     */

    /*
     * ----------------------------------------------------------------------------
     * Names
     * ----------------------------------------------------------------------------
     */

    fn parse_table_name(mut params: FnParseParams) -> AplloSqlParserResult<TableName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            TableName,
        )
    }

    fn parse_column_name(mut params: FnParseParams) -> AplloSqlParserResult<ColumnName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            ColumnName,
        )
    }

    /*
     * ----------------------------------------------------------------------------
     * Constraints
     * ----------------------------------------------------------------------------
     */

    fn parse_column_constraint(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<ColumnConstraint> {
        let s = self_as_str(&mut params);
        match s {
            "NOT NULL" => Ok(ColumnConstraint::NotNullVariant),
            x => {
                eprintln!("Unexpected constraint parsed: {}", x);
                unreachable!();
            }
        }
    }
}
