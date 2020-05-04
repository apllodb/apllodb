use super::generated_parser::Rule;
use crate::{
    apllo_ast::{DataType, Identifier},
    apllo_sql_parser::{AplloSqlParserError, AplloSqlParserResult},
};
use pest::iterators::{Pair, Pairs};
use std::collections::VecDeque;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(super) struct FnParseParams<'a> {
    pub(super) apllo_sql: &'a str,

    // collected from Pairs.
    //
    // Pairs itself cannot be used as this struct field:
    // An AST node who has multiple children can call parse_self!() / parse_leaf_string!() macro twice or more.
    // But Pairs::next() takes this field's ownership so it fails in 2nd macro call.
    // On the other hand, VecDeque::pop_front() just borrows this field and returns ownership of Pair.
    pub(super) children_pairs: VecDeque<Pair<'a, Rule>>,
}

/// Parse the next child term as `child_term` by `child_parser`.
///
/// Returns Ok(None) when either of the following cases:
///  - no child term left.
///  - the next child term does not match $child_term.
///
/// # Failures
/// - When no child term left.
/// - When the next child term does not match $child_term.
/// - Raises Err from `child_parser` as-is.
pub(super) fn parse_child<T, ChildRet>(
    params: &mut FnParseParams,
    child_term: Rule,
    child_parser: impl Fn(FnParseParams) -> AplloSqlParserResult<ChildRet>,
    ret_closure: impl Fn(ChildRet) -> T,
) -> AplloSqlParserResult<T> {
    let child_pair: Pair<Rule> =
        params
            .children_pairs
            .pop_front()
            .ok_or(AplloSqlParserError::new(
                params.apllo_sql,
                "Tried to parse a term but nothing left.",
            ))?;

    if child_pair.as_rule() == child_term {
        let grand_children_pairs: Pairs<Rule> = child_pair.into_inner();

        let child_params = FnParseParams {
            apllo_sql: params.apllo_sql,
            children_pairs: grand_children_pairs.collect(),
        };
        let child_ast = child_parser(child_params)?;

        Ok(ret_closure(child_ast))
    } else {
        eprintln!(
            "Hit to unexpected rule: {:?}\n\
        Pair: {}\n\
        ",
            child_pair.as_rule(),
            child_pair
        );
        unreachable!();
    }
}

/// Try to parse the next child term as `child_term` by `child_parser`.
///
/// Returns Ok(None) when either of the following cases:
/// - no child term left.
/// - the next child term does not match $child_term.
///
/// # Failures
/// Raises Err from `child_parser` as-is.
pub(super) fn try_parse_child<T, ChildRet>(
    params: &mut FnParseParams,
    child_term: Rule,
    child_parser: impl Fn(FnParseParams) -> AplloSqlParserResult<ChildRet>,
    ret_closure: impl Fn(ChildRet) -> T,
) -> AplloSqlParserResult<Option<T>> {
    if let Some(child_pair) = params.children_pairs.pop_front() {
        if child_pair.as_rule() == child_term {
            let grand_children_pairs: Pairs<Rule> = child_pair.into_inner();

            let child_params = FnParseParams {
                apllo_sql: params.apllo_sql,
                children_pairs: grand_children_pairs.collect(),
            };
            let child_ast = child_parser(child_params)?;

            Ok(Some(ret_closure(child_ast)))
        } else {
            params.children_pairs.push_front(child_pair);
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

/// Parses children sequence by `child_parser` while next child matches `child_term`.
///
/// # Failures
/// Raises Err from `child_parser` as-is.
pub(super) fn parse_child_seq<T, ChildRet>(
    params: &mut FnParseParams,
    child_term: Rule,
    child_parser: &impl Fn(FnParseParams) -> AplloSqlParserResult<ChildRet>,
    ret_closure: &impl Fn(ChildRet) -> T,
) -> AplloSqlParserResult<Vec<T>> {
    let mut children = Vec::<T>::new();
    while let Some(child) = try_parse_child(params, child_term, child_parser, ret_closure)? {
        children.push(child);
    }
    Ok(children)
}

pub(super) fn parse_identifier(params: &mut FnParseParams) -> AplloSqlParserResult<Identifier> {
    let s = parse_leaf_string(params)?;
    Ok(Identifier(s))
}

pub(super) fn parse_data_type(params: &mut FnParseParams) -> AplloSqlParserResult<DataType> {
    let s = parse_leaf_string(params)?;
    match s.as_str() {
        "INT" => Ok(DataType::IntVariant),
        x => {
            eprintln!("Unexpected data type parsed: {}", x);
            unreachable!();
        }
    }
}

fn parse_leaf_string(params: &mut FnParseParams) -> AplloSqlParserResult<String> {
    let child_pair = params
        .children_pairs
        .pop_front()
        .ok_or(AplloSqlParserError::new(
            params.apllo_sql,
            "Expected to parse a leaf string but no term left.",
        ))?;
    let s = child_pair.as_str().to_string();
    Ok(s)
}
