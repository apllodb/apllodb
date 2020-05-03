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

        let params = FnParseParams {
            apllo_sql: &apllo_sql,
            pair,
        };

        let ast = Self::parse_root_embedded_sql_statement(params)?;
        Ok(ast)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct FnParseParams<'a> {
    apllo_sql: &'a str,
    pair: Pair<'a, Rule>,
}

macro_rules! parse_inner {
    ($(
        {
            $params: expr,
            $self_term: ident,
            $inner_parser: ident,
            $ret_closure: expr,
        }$(,)?
    ),*) => {{
        $(
            match $params.pair.as_rule() {
                Rule::$self_term => {
                    let mut pairs: Pairs<Rule> = $params.pair.into_inner();
                    let inner_pair: Pair<Rule> = pairs
                        .next()
                        .ok_or(AplloSqlParserError::new($params.apllo_sql, "Unknown"))?;

                    let inner_params =  FnParseParams {
                        apllo_sql: $params.apllo_sql,
                        pair: inner_pair,
                    };
                    let inner_ast = Self::$inner_parser(inner_params)?;

                    Ok($ret_closure(inner_ast))
                }
                _ => unreachable!(),
            }
        )*
    }};
}

impl PestParserImpl {
    fn parse_root_embedded_sql_statement(params: FnParseParams) -> AplloSqlParserResult<AplloAst> {
        parse_inner!(
            {
                params,
                embedded_sql_statement,
                parse_inner_embedded_sql_statement,
                |inner_ast| AplloAst(inner_ast),
            },
        )
    }

    fn parse_inner_embedded_sql_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<EmbeddedSqlStatement> {
        parse_inner!(
            {
                params,
                statement_or_declaration,
                parse_inner_statement_or_declaration,
                |inner_ast| EmbeddedSqlStatement { statement_or_declaration: inner_ast },
            },
        )
    }

    fn parse_inner_statement_or_declaration(
        params: FnParseParams,
    ) -> AplloSqlParserResult<StatementOrDeclaration> {
        parse_inner!(
            {
                params,
                sql_executable_statement,
                parse_inner_sql_executable_statement,
                |inner_ast| StatementOrDeclaration::SqlExecutableStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_sql_executable_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlExecutableStatement> {
        parse_inner!(
            {
                params,
                sql_schema_statement,
                parse_inner_sql_schema_statement,
                |inner_ast| SqlExecutableStatement::SqlSchemaStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_sql_schema_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaStatement> {
        parse_inner!(
            {
                params,
                sql_schema_manipulation_statement,
                parse_inner_sql_schema_manipulation_statement,
                |inner_ast| SqlSchemaStatement::SqlSchemaManipulationStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_sql_schema_manipulation_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaManipulationStatement> {
        parse_inner!(
            {
                params,
                drop_table_statement,
                parse_inner_drop_table_statement,
                |inner_ast| SqlSchemaManipulationStatement::DropTableStatementVariant(inner_ast),
            },
        )
    }

    fn parse_inner_drop_table_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<DropTableStatement> {
        match params.pair.as_rule() {
            Rule::identifier => {
                let s = params.pair.as_str().to_string();
                Ok(DropTableStatement {
                    table_name: Identifier(s),
                })
            }
            _ => unreachable!(),
        }
    }
}
