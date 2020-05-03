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

        let ast = Self::parse_root_embedded_sql_statement(pair, &apllo_sql)?;
        Ok(ast)
    }
}

macro_rules! parse_inner {
    ($(
        {
            $self_pair: expr,
            $self_term: ident,
            $inner_parser: ident,
            $apllo_sql: ident,
            $ret_closure: expr,
        }$(,)?
    ),*) => {{
        $(
            match $self_pair.as_rule() {
                Rule::$self_term => {
                    let mut pairs: Pairs<Rule> = $self_pair.into_inner();
                    let inner_pair: Pair<Rule> = pairs
                        .next()
                        .ok_or(AplloSqlParserError::new($apllo_sql, "Unknown"))?;

                    let inner_ast = Self::$inner_parser(inner_pair, $apllo_sql)?;

                    Ok($ret_closure(inner_ast))
                }
                _ => unreachable!(),
            }
        )*
    }};
}

impl PestParserImpl {
    fn parse_root_embedded_sql_statement(
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<AplloAst> {
        parse_inner!(
            {
                pair,
                embedded_sql_statement,
                parse_inner_embedded_sql_statement,
                apllo_sql,
                |inner_ast| AplloAst(inner_ast),
            },
        )
    }

    fn parse_inner_embedded_sql_statement(
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<EmbeddedSqlStatement> {
        parse_inner!(
            {
                pair,
                statement_or_declaration,
                parse_inner_statement_or_declaration,
                apllo_sql,
                |inner_ast| EmbeddedSqlStatement { statement_or_declaration: inner_ast },
            },
        )
    }

    fn parse_inner_statement_or_declaration(
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<StatementOrDeclaration> {
        parse_inner!(
            {
                pair,
                sql_executable_statement,
                parse_inner_sql_executable_statement,
                apllo_sql,
                |inner_ast| StatementOrDeclaration::SqlExecutableStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_sql_executable_statement(
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlExecutableStatement> {
        parse_inner!(
            {
                pair,
                sql_schema_statement,
                parse_inner_sql_schema_statement,
                apllo_sql,
                |inner_ast| SqlExecutableStatement::SqlSchemaStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_sql_schema_statement(
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlSchemaStatement> {
        parse_inner!(
            {
                pair,
                sql_schema_manipulation_statement,
                parse_inner_sql_schema_manipulation_statement,
                apllo_sql,
                |inner_ast| SqlSchemaStatement::SqlSchemaManipulationStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_sql_schema_manipulation_statement(
        pair: Pair<Rule>,
        apllo_sql: &str,
    ) -> AplloSqlParserResult<SqlSchemaManipulationStatement> {
        parse_inner!(
            {
                pair,
                drop_table_statement,
                parse_inner_drop_table_statement,
                apllo_sql,
                |inner_ast| SqlSchemaManipulationStatement::DropTableStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_drop_table_statement(
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
