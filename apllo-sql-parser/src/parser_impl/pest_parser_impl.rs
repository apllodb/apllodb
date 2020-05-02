mod generated_parser;

#[cfg(test)]
mod tests;

use crate::{
    apllo_ast::{
        DropTableStatement, EmbeddedSqlStatement, Identifier, SqlExecutableStatement,
        SqlSchemaManipulationStatement, SqlSchemaStatement, StatementOrDeclaration,
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
            GeneratedParser::parse(Rule::embedded_sql_statement, &apllo_sql).map_err(|e| {
                let reason = format!("{}", e);
                AplloSqlParserError::new(&apllo_sql, reason)
            })?;
        let pair: Pair<Rule> = pairs
            .next()
            .ok_or(AplloSqlParserError::new(&apllo_sql, "Unknown"))?;

        let ast = self.parse_root_embedded_sql_statement(pair, &apllo_sql)?;
        Ok(ast)
    }
}

impl PestParserImpl {
    fn parse_root_embedded_sql_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<AplloAst> {
        match pair.as_rule() {
            Rule::embedded_sql_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_inner_embedded_sql_statement(inner_pair, apllo_sql)?;

                Ok(AplloAst(inner_ast))
            }
            _ => unreachable!(),
        }
    }

    fn parse_inner_embedded_sql_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<EmbeddedSqlStatement> {
        match pair.as_rule() {
            Rule::statement_or_declaration => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_inner_statement_or_declaration(inner_pair, apllo_sql)?;

                Ok(EmbeddedSqlStatement {
                    statement_or_declaration: inner_ast,
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_inner_statement_or_declaration(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<StatementOrDeclaration> {
        match pair.as_rule() {
            Rule::sql_executable_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_inner_sql_executable_statement(inner_pair, apllo_sql)?;

                Ok(StatementOrDeclaration::SqlExecutableStatementVariant(
                    inner_ast,
                ))
            }
            _ => unreachable!(),
        }
    }

    fn parse_inner_sql_executable_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlExecutableStatement> {
        match pair.as_rule() {
            Rule::sql_schema_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_inner_sql_schema_statement(inner_pair, apllo_sql)?;

                Ok(SqlExecutableStatement::SqlSchemaStatementVariant(inner_ast))
            }
            _ => unreachable!(),
        }
    }

    fn parse_inner_sql_schema_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlSchemaStatement> {
        match pair.as_rule() {
            Rule::sql_schema_manipulation_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();
                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast =
                    self.parse_inner_sql_schema_manipulation_statement(inner_pair, apllo_sql)?;

                Ok(SqlSchemaStatement::SqlSchemaManipulationStatementVariant(
                    inner_ast,
                ))
            }
            _ => unreachable!(),
        }
    }

    fn parse_inner_sql_schema_manipulation_statement(
        &self,
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlSchemaManipulationStatement> {
        match pair.as_rule() {
            Rule::drop_table_statement => {
                let mut pairs: Pairs<Rule> = pair.into_inner();

                let inner_pair: Pair<Rule> = pairs
                    .next()
                    .ok_or(AplloSqlParserError::new(apllo_sql, "Unknown"))?;

                let inner_ast = self.parse_inner_drop_table_statement(inner_pair, apllo_sql)?;

                Ok(SqlSchemaManipulationStatement::DropTableStatementVariant(
                    inner_ast,
                ))
            }
            _ => unreachable!(),
        }
    }

    fn parse_inner_drop_table_statement(
        &self,
        pair: Pair<Rule>,
        _apllo_sql: &str,
    ) -> AplloSqlParserResult<DropTableStatement> {
        match pair.as_rule() {
            Rule::identifier => {
                let s = pair.as_str().to_string();
                Ok(DropTableStatement {
                    table_name: Identifier(s),
                })
            }
            _ => unreachable!(),
        }
    }
}
