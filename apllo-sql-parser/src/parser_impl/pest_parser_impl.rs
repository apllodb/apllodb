mod generated_parser;

use crate::{
    apllo_ast::{
        DropTableStatement, Identifier, SqlExecutableStatement, SqlSchemaManipulationStatement,
        SqlSchemaStatement,
    },
    apllo_sql_parser::{AplloSqlParserError, AplloSqlParserResult},
    parser_interface::ParserLike,
    AplloAst,
};
use generated_parser::{GeneratedParser, Rule};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

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

        let mut pairs: Pairs<Rule> =
            GeneratedParser::parse(Rule::sql_executable_statement, &apllo_sql).map_err(|e| {
                let reason = format!("{}", e);
                AplloSqlParserError::new(&apllo_sql, reason)
            })?;
        let pair: Pair<Rule> = pairs
            .next()
            .ok_or(AplloSqlParserError::new(&apllo_sql, "Unknown"))?;

        let ast_sql_executable_statement = self.parse_sql_executable_statement(pair, &apllo_sql)?;

        Ok(AplloAst(ast_sql_executable_statement))
    }
}

impl PestParserImpl {
    fn parse_sql_executable_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlExecutableStatement> {
        match pair.as_rule() {
            Rule::sql_executable_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_sql_schema_statement(inner_pair, apllo_sql)?;

                Ok(SqlExecutableStatement::SqlSchemaStatementVariant(inner_ast))
            }
            _ => unreachable!(),
        }
    }

    fn parse_sql_schema_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlSchemaStatement> {
        match pair.as_rule() {
            Rule::sql_schema_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast =
                    self.parse_sql_schema_manipulation_statement(inner_pair, apllo_sql)?;

                Ok(SqlSchemaStatement::SqlSchemaManipulationStatementVariant(
                    inner_ast,
                ))
            }
            _ => unreachable!(),
        }
    }

    fn parse_sql_schema_manipulation_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlSchemaManipulationStatement> {
        match pair.as_rule() {
            Rule::sql_schema_manipulation_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_drop_table_statement(inner_pair, apllo_sql)?;

                Ok(SqlSchemaManipulationStatement::DropTableStatementVariant(
                    inner_ast,
                ))
            }
            _ => unreachable!(),
        }
    }

    fn parse_drop_table_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<DropTableStatement> {
        match pair.as_rule() {
            Rule::drop_table_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();

                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_identifier(inner_pair, apllo_sql)?;

                Ok(DropTableStatement {
                    table_name: inner_ast,
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_identifier(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<Identifier> {
        match pair.as_rule() {
            Rule::identifier => {
                let s = pair.as_str().to_string();
                Ok(Identifier(s))
            }
            _ => unreachable!(),
        }
    }
}
