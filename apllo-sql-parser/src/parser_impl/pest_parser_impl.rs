mod generated_parser;
mod helper;

#[cfg(test)]
mod tests;

use crate::{
    apllo_ast::{
        ColumnConstraintDefinition, ColumnDefinition, ColumnName, DataType, DropTableStatement,
        EmbeddedSqlStatement, Identifier, SqlExecutableStatement, SqlSchemaDefinitionStatement,
        SqlSchemaManipulationStatement, SqlSchemaStatement, StatementOrDeclaration,
        TableContentsSource, TableDefinition, TableElement, TableElementList, TableName,
    },
    apllo_sql_parser::{AplloSqlParserError, AplloSqlParserResult},
    parser_interface::ParserLike,
    AplloAst,
};
use generated_parser::{GeneratedParser, Rule};
use helper::{parse_child, parse_child_seq, self_as_str, try_parse_child, FnParseParams};
use pest::{iterators::Pairs, Parser};

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

        let pairs: Pairs<Rule> = GeneratedParser::parse(Rule::embedded_sql_statement, &apllo_sql)
            .map_err(|e| {
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
            Rule::embedded_sql_statement,
            Self::parse_embedded_sql_statement,
            |inner_ast| AplloAst(inner_ast),
        )
    }
}

impl PestParserImpl {
    fn parse_identifier(mut params: FnParseParams) -> AplloSqlParserResult<Identifier> {
        let s = self_as_str(&mut params);
        Ok(Identifier(s.into()))
    }

    fn parse_table_name(mut params: FnParseParams) -> AplloSqlParserResult<TableName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            |inner_ast| TableName(inner_ast),
        )
    }

    fn parse_column_name(mut params: FnParseParams) -> AplloSqlParserResult<ColumnName> {
        parse_child(
            &mut params,
            Rule::identifier,
            Self::parse_identifier,
            |inner_ast| ColumnName(inner_ast),
        )
    }

    fn parse_data_type(mut params: FnParseParams) -> AplloSqlParserResult<DataType> {
        let s = self_as_str(&mut params);
        match s {
            "INT" => Ok(DataType::IntVariant),
            x => {
                eprintln!("Unexpected data type parsed: {}", x);
                unreachable!();
            }
        }
    }

    fn parse_table_definition(mut params: FnParseParams) -> AplloSqlParserResult<TableDefinition> {
        let table_name = parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            |inner_ast| inner_ast,
        )?;
        let table_contents_source = parse_child(
            &mut params,
            Rule::table_contents_source,
            Self::parse_table_contents_source,
            |inner_ast| inner_ast,
        )?;
        Ok(TableDefinition {
            table_name,
            table_contents_source,
        })
    }

    fn parse_table_contents_source(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<TableContentsSource> {
        parse_child(
            &mut params,
            Rule::table_element_list,
            Self::parse_table_element_list,
            |inner_ast| TableContentsSource::TableElementListVariant(inner_ast),
        )
    }

    fn parse_table_element_list(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<TableElementList> {
        let head_table_element = parse_child(
            &mut params,
            Rule::table_element,
            Self::parse_table_element,
            |inner_ast| inner_ast,
        )?;
        let tail_table_elements = parse_child_seq(
            &mut params,
            Rule::table_element,
            &Self::parse_table_element,
            &|inner_ast| inner_ast,
        )?;

        Ok(TableElementList {
            head_table_element,
            tail_table_elements,
        })
    }

    fn parse_table_element(mut params: FnParseParams) -> AplloSqlParserResult<TableElement> {
        parse_child(
            &mut params,
            Rule::column_definition,
            Self::parse_column_definition,
            |inner_ast| TableElement::ColumnDefinitionVariant(inner_ast),
        )
    }

    fn parse_column_definition(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<ColumnDefinition> {
        let column_name = parse_child(
            &mut params,
            Rule::column_name,
            Self::parse_column_name,
            |inner_ast| inner_ast,
        )?;
        let data_type = parse_child(
            &mut params,
            Rule::data_type,
            Self::parse_data_type,
            |inner_ast| inner_ast,
        )?;
        let column_constraint_definitions = parse_child_seq(
            &mut params,
            Rule::column_constraint_definition,
            &Self::parse_column_constraint_definition,
            &|inner_ast| inner_ast,
        )?;
        Ok(ColumnDefinition {
            column_name,
            data_type,
            column_constraint_definitions,
        })
    }

    fn parse_column_constraint_definition(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<ColumnConstraintDefinition> {
        let s = self_as_str(&mut params);
        match s {
            "NOT NULL" => Ok(ColumnConstraintDefinition::NotNullVariant),
            x => {
                eprintln!("Unexpected constraint parsed: {}", x);
                unreachable!();
            }
        }
    }

    fn parse_drop_table_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<DropTableStatement> {
        parse_child(
            &mut params,
            Rule::table_name,
            Self::parse_table_name,
            |inner_ast| DropTableStatement {
                table_name: inner_ast,
            },
        )
    }

    fn parse_embedded_sql_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<EmbeddedSqlStatement> {
        parse_child(
            &mut params,
            Rule::statement_or_declaration,
            Self::parse_statement_or_declaration,
            |inner_ast| EmbeddedSqlStatement {
                statement_or_declaration: inner_ast,
            },
        )
    }

    fn parse_statement_or_declaration(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<StatementOrDeclaration> {
        parse_child(
            &mut params,
            Rule::sql_executable_statement,
            Self::parse_sql_executable_statement,
            |inner_ast| StatementOrDeclaration::SqlExecutableStatementVariant(inner_ast),
        )
    }

    fn parse_sql_executable_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<SqlExecutableStatement> {
        parse_child(
            &mut params,
            Rule::sql_schema_statement,
            Self::parse_sql_schema_statement,
            |inner_ast| SqlExecutableStatement::SqlSchemaStatementVariant(inner_ast),
        )
    }

    fn parse_sql_schema_definition_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaDefinitionStatement> {
        parse_child(
            &mut params,
            Rule::table_definition,
            Self::parse_table_definition,
            |inner_ast| SqlSchemaDefinitionStatement::TableDefinitionVariant(inner_ast),
        )
    }

    fn parse_sql_schema_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaStatement> {
        try_parse_child(
            &mut params,
            Rule::sql_schema_definition_statement,
            Self::parse_sql_schema_definition_statement,
            |inner_ast| SqlSchemaStatement::SqlSchemaDefinitionStatementVariant(inner_ast),
        )?
        .or(try_parse_child(
            &mut params,
            Rule::sql_schema_manipulation_statement,
            Self::parse_sql_schema_manipulation_statement,
            |inner_ast| SqlSchemaStatement::SqlSchemaManipulationStatementVariant(inner_ast),
        )?)
        .ok_or(AplloSqlParserError::new(
            params.apllo_sql,
            "Does not match any child rule of sql_schema_statement.",
        ))
    }

    fn parse_sql_schema_manipulation_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaManipulationStatement> {
        parse_child(
            &mut params,
            Rule::drop_table_statement,
            Self::parse_drop_table_statement,
            |inner_ast| SqlSchemaManipulationStatement::DropTableStatementVariant(inner_ast),
        )
    }
}
