use super::super::PestParserImpl;
use crate::apllo_ast::{
    Alias, ColumnName, ColumnReference, Command, Constant, Expression, Identifier, IntegerConstant,
    NumericConstant, TableName, UpdateCommand,
};
use crate::parser_interface::ParserLike;
use crate::AplloAst;

macro_rules! update {
    ($table_name: expr, $alias: expr, $column_name: expr, $expression: expr $(,)?) => {
        UpdateCommand {
            table_name: TableName(Identifier($table_name.to_string())),
            alias: $alias.map(|a: &str| Alias(Identifier(a.to_string()))),
            column_name: ColumnName(Identifier($column_name.to_string())),
            expression: $expression,
            where_condition: None,
        }
    };
}

macro_rules! int_expr {
    ($int_str: expr) => {
        Expression::ConstantVariant(Constant::NumericConstantVariant(
            NumericConstant::IntegerConstantVariant(IntegerConstant($int_str.to_string())),
        ))
    };
}

macro_rules! colref_expr {
    ($colref_str: expr) => {
        Expression::ColumnReferenceVariant(ColumnReference {
            correlation: None,
            column_name: ColumnName(Identifier($colref_str.to_string())),
        })
    };
}

#[test]
fn test_update_accepted() {
    let sql_vs_expected_ast: Vec<(&str, UpdateCommand)> = vec![
        (
            "UPDATE t SET c1 = 123",
            update!("t", None, "c1", int_expr!("123")),
        ),
        (
            "UPDATE long_table_name AS t SET c1 = 123",
            update!("long_table_name", Some("t"), "c1", int_expr!("123")),
        ),
        (
            "UPDATE t SET c1 = c2",
            update!("t", None, "c1", colref_expr!("c2")),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(AplloAst(Command::UpdateCommandVariant(update_command))) => {
                assert_eq!(update_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as UPDATE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_update_rejected() {
    let sqls: Vec<&str> = vec![];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
