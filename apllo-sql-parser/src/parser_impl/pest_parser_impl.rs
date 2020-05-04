mod generated_parser;

#[cfg(test)]
mod tests;

use crate::{
    apllo_ast::{
        ColumnDefinition, DataType, DropTableStatement, EmbeddedSqlStatement, Identifier,
        SqlExecutableStatement, SqlSchemaDefinitionStatement, SqlSchemaManipulationStatement,
        SqlSchemaStatement, StatementOrDeclaration, TableContentsSource, TableDefinition,
        TableElement, TableElementList,
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
use std::collections::VecDeque;

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

    // collected from Pairs.
    //
    // Pairs itself cannot be used as this struct field:
    // An AST node who has multiple children can call parse_self!() / parse_leaf_string!() macro twice or more.
    // But Pairs::next() takes this field's ownership so it fails in 2nd macro call.
    // On the other hand, VecDeque::pop_front() just borrows this field and returns ownership of Pair.
    children_pairs: VecDeque<Pair<'a, Rule>>,
}

/// Returns:
/// Result<T: Return type of $ret_closure>
macro_rules! parse_self {
    ($params: expr,
        [
            $((  // Possible `Rule`s
                $child_term: ident,
                $child_parser: ident,
                $ret_closure: expr$(,)?
            )$(,)?)+
        ]
    ) => {{
        let child_pair: Pair<Rule> = $params.children_pairs.pop_front()
            .ok_or(
                AplloSqlParserError::new(
                    $params.apllo_sql,
                    "Tried to parse a term but nothing left.",
                )
            )?;

        match child_pair.as_rule() {
            $(  // Possible `Rule`s
            Rule::$child_term => {
                let grand_children_pairs: Pairs<Rule> = child_pair.into_inner();

                let child_params =  FnParseParams {
                    apllo_sql: $params.apllo_sql,
                    children_pairs: grand_children_pairs.collect(),
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

/// Returns:
/// Result<Option<T: Return type of $ret_closure>>>
///   None when either of the following cases:
///   - no term left.
///   - the next Pair does not match $child_term.
macro_rules! try_parse_self {
    ($params: expr,
        [
            $((  // Possible `Rule`s
                $child_term: ident,
                $child_parser: ident,
                $ret_closure: expr$(,)?
            )$(,)?)+
        ]
    ) => {{
        if let Some(child_pair) = $params.children_pairs.pop_front() {
            match child_pair.as_rule() {
                $(  // Possible `Rule`s
                Rule::$child_term => {
                    let grand_children_pairs: Pairs<Rule> = child_pair.into_inner();

                    let child_params =  FnParseParams {
                        apllo_sql: $params.apllo_sql,
                        children_pairs: grand_children_pairs.collect(),
                    };
                    let child_ast = Self::$child_parser(child_params)?;

                    Ok(Some($ret_closure(child_ast)))
                }
                )+
                _ => {
                    $params.children_pairs.push_front(child_pair);
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }};
}

/// Returns:
/// Result<String>
macro_rules! _parse_leaf_string {
    ($params: expr) => {{
        let child_pair: Pair<Rule> =
            $params
                .children_pairs
                .pop_front()
                .ok_or(AplloSqlParserError::new(
                    $params.apllo_sql,
                    "Expected to parse a leaf string but no term left.",
                ))?;
        let s = child_pair.as_str().to_string();
        Ok(s)
    }};
}

/// Returns:
/// Result<Identifier>
macro_rules! parse_identifier {
    ($params: expr) => {{
        let s = _parse_leaf_string!($params)?;
        Ok(Identifier(s))
    }};
}

/// Returns:
/// Result<DataType>
macro_rules! parse_data_type {
    ($params: expr) => {{
        let s = _parse_leaf_string!($params)?;
        match s.as_str() {
            "INT" => Ok(DataType::IntVariant),
            x => {
                eprintln!("Unexpected data type parsed: {}", x);
                unreachable!();
            }
        }
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

        let mut params = FnParseParams {
            apllo_sql: &apllo_sql,
            children_pairs: pairs.collect(),
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
    fn parse_table_definition(mut params: FnParseParams) -> AplloSqlParserResult<TableDefinition> {
        let table_name = parse_identifier!(params)?;
        let table_contents_source = parse_self!(
            params,
            [(
                table_contents_source,
                parse_table_contents_source,
                |inner_ast| inner_ast,
            ),]
        )?;
        Ok(TableDefinition {
            table_name,
            table_contents_source,
        })
    }

    fn parse_table_contents_source(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<TableContentsSource> {
        parse_self!(
            params,
            [(table_element_list, parse_table_element_list, |inner_ast| {
                TableContentsSource::TableElementListVariant(inner_ast)
            },),]
        )
    }

    fn parse_table_element_list(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<TableElementList> {
        let head_table_element = parse_self!(
            params,
            [(table_element, parse_table_element, |inner_ast| inner_ast,),]
        )?;

        let mut tail_table_elements: Vec<TableElement> = vec![];
        while let Some(table_element) = try_parse_self!(
            params,
            [(table_element, parse_table_element, |inner_ast| inner_ast,),]
        )? {
            tail_table_elements.push(table_element);
        }

        Ok(TableElementList {
            head_table_element,
            tail_table_elements: vec![],
        })
    }

    fn parse_table_element(mut params: FnParseParams) -> AplloSqlParserResult<TableElement> {
        parse_self!(
            params,
            [(column_definition, parse_column_definition, |inner_ast| {
                TableElement::ColumnDefinitionVariant(inner_ast)
            }),]
        )
    }

    fn parse_column_definition(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<ColumnDefinition> {
        let column_name = parse_identifier!(params)?;
        let data_type = parse_data_type!(params)?;
        // TODO: これだと制約のあるカラムに対応できていない
        Ok(ColumnDefinition {
            column_name,
            data_type,
            column_constraint_definitions: vec![],
        })
    }

    fn parse_drop_table_statement(
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<DropTableStatement> {
        let table_name = parse_identifier!(params)?;
        Ok(DropTableStatement { table_name })
    }

    fn parse_embedded_sql_statement(
        mut params: FnParseParams,
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
        mut params: FnParseParams,
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
        mut params: FnParseParams,
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
        mut params: FnParseParams,
    ) -> AplloSqlParserResult<SqlSchemaDefinitionStatement> {
        parse_self!(
            params,
            [(table_definition, parse_table_definition, |inner_ast| {
                SqlSchemaDefinitionStatement::TableDefinitionVariant(inner_ast)
            },),]
        )
    }

    fn parse_sql_schema_statement(
        mut params: FnParseParams,
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
        mut params: FnParseParams,
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
}
