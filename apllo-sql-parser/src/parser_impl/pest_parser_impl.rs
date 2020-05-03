mod generated_parser;

#[cfg(test)]
mod tests;

use crate::{
    apllo_ast::{
        DropTableStatement, EmbeddedSqlStatement, Identifier, SqlExecutableStatement,
        SqlSchemaDefinitionStatement, SqlSchemaManipulationStatement, SqlSchemaStatement,
        StatementOrDeclaration, TableContentsSource, TableDefinition, TableElementList,
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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct FnParseParams<'a> {
    apllo_sql: &'a str,
    children_pairs: Pairs<'a, Rule>,
}

macro_rules! parse_self {
    ($params: expr,
        [
            $((  // Possible `Rule`s
                $child_term: ident,
                $child_parser: ident,
                $ret_closure: expr$(,)?
            ),)+
        ]
    ) => {{
        let mut children_pairs = $params.children_pairs;
        let child_pair: Pair<Rule> = children_pairs.next()
            .ok_or(
                AplloSqlParserError::new(
                    $params.apllo_sql,
                    "Expected any terms are left unparsed but nothing left.",
                )
            )?;

        match child_pair.as_rule() {
            $(  // Possible `Rule`s
            Rule::$child_term => {
                let grand_children_pairs: Pairs<Rule> = child_pair.into_inner();

                let child_params =  FnParseParams {
                    apllo_sql: $params.apllo_sql,
                    children_pairs: grand_children_pairs,
                };
                let child_ast = Self::$child_parser(child_params)?;

                Ok($ret_closure(child_ast))
            }
            )+
            rule => {
                eprintln!("Hit to unexpected rule: {:?}\n\
                Pair: {}\n\
                ", rule, child_pair);
                unreachable!();
            }
        }
    }};
}

macro_rules! parse_identifier {
    ($params: expr, $ret_closure: expr) => {{
        let mut children_pairs = $params.children_pairs;
        let child_pair: Pair<Rule> = children_pairs.next().ok_or(AplloSqlParserError::new(
            $params.apllo_sql,
            format!(
                "Expected a rule '{:?}' but it does not appear.",
                Rule::identifier
            ),
        ))?;
        let child_pair_as_string: String = child_pair.as_str().to_string();
        Ok($ret_closure(Identifier(child_pair_as_string)))
    }};
}

impl ParserLike for PestParserImpl {
    fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloSqlParserResult<AplloAst> {
        let apllo_sql = apllo_sql.into();

        let pairs: Pairs<Rule> = GeneratedParser::parse(Rule::embedded_sql_statement, &apllo_sql)
            .map_err(|e| {
            let reason = format!("{}", e);
            AplloSqlParserError::new(&apllo_sql, reason)
        })?;

        let params = FnParseParams {
            apllo_sql: &apllo_sql,
            children_pairs: pairs,
        };

        parse_self!(
            params,
            [(
                embedded_sql_statement,
                parse_embedded_sql_statement,
                |inner_ast| AplloAst(inner_ast)
            ),]
        )
    }
}

impl PestParserImpl {
    fn parse_table_definition(_params: FnParseParams) -> AplloSqlParserResult<TableDefinition> {
        todo!()

        // let table_name = parse_identifier!(params, |inner_ast| inner_ast,)?;
        // let table_contents_source = parse_self!(
        //     params,
        //     {
        //         table_contents_source,
        //         parse_table_contents_source,
        //         |inner_ast| inner_ast,
        //     }
        // )?;
        // Ok(TableDefinition {
        //     table_name,
        //     table_contents_source,
        // })
    }

    fn _parse_table_contents_source(
        _params: FnParseParams,
    ) -> AplloSqlParserResult<TableContentsSource> {
        todo!()

        // parse_self!(
        //     params,
        //     [
        //         {
        //             (
        //                 table_element_list
        //                 _parse_table_element_list,
        //                 |inner_ast| TableContentsSource::TableElementListVariant(inner_ast),
        //             ),
        //         },
        //     ]
        // )
    }

    fn _parse_table_element_list(_params: FnParseParams) -> AplloSqlParserResult<TableElementList> {
        todo!()
    }

    fn parse_embedded_sql_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<EmbeddedSqlStatement> {
        parse_self!(
            params,
            [(
                statement_or_declaration,
                parse_statement_or_declaration,
                |inner_ast| EmbeddedSqlStatement {
                    statement_or_declaration: inner_ast
                },
            ),]
        )
    }

    fn parse_statement_or_declaration(
        params: FnParseParams,
    ) -> AplloSqlParserResult<StatementOrDeclaration> {
        parse_self!(
            params,
            [(
                sql_executable_statement,
                parse_sql_executable_statement,
                |inner_ast| StatementOrDeclaration::SqlExecutableStatementVariant(inner_ast),
            ),]
        )
    }

    fn parse_sql_executable_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlExecutableStatement> {
        parse_self!(
            params,
            [(
                sql_schema_statement,
                parse_sql_schema_statement,
                |inner_ast| SqlExecutableStatement::SqlSchemaStatementVariant(inner_ast),
            ),]
        )
    }

    fn parse_sql_schema_definition_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaDefinitionStatement> {
        parse_self!(
            params,
            [(table_definition, parse_table_definition, |inner_ast| {
                SqlSchemaDefinitionStatement::TableDefinitionVariant(inner_ast)
            },),]
        )
    }

    fn parse_sql_schema_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaStatement> {
        parse_self!(
            params,
            [
                (
                    sql_schema_definition_statement,
                    parse_sql_schema_definition_statement,
                    |inner_ast| SqlSchemaStatement::SqlSchemaDefinitionStatementVariant(inner_ast),
                ),
                (
                    sql_schema_manipulation_statement,
                    parse_sql_schema_manipulation_statement,
                    |inner_ast| SqlSchemaStatement::SqlSchemaManipulationStatementVariant(
                        inner_ast
                    ),
                ),
            ]
        )
    }

    fn parse_sql_schema_manipulation_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaManipulationStatement> {
        parse_self!(
            params,
            [(
                drop_table_statement,
                parse_drop_table_statement,
                |inner_ast| SqlSchemaManipulationStatement::DropTableStatementVariant(inner_ast),
            ),]
        )
    }

    fn parse_drop_table_statement(
        params: FnParseParams,
    ) -> AplloSqlParserResult<DropTableStatement> {
        parse_identifier!(params, |inner_ast| DropTableStatement {
            table_name: inner_ast,
        })
    }
}
