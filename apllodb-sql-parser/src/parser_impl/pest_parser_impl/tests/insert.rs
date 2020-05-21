use super::super::PestParserImpl;
use crate::apllodb_ast::NonEmptyVec;
use crate::apllodb_ast::{
    Alias, ColumnName, Command, Constant, Expression, Identifier, InsertCommand, IntegerConstant,
    NumericConstant, TableName,
};
use crate::parser_interface::ParserLike;
use crate::ApllodbAst;

macro_rules! insert {
    ($table_name: expr, $alias: expr, $column_names: expr, $expressions: expr $(,)?) => {
        InsertCommand {
            table_name: TableName(Identifier($table_name.to_string())),
            alias: $alias.map(|a: &str| Alias(Identifier(a.to_string()))),
            column_names: NonEmptyVec::new(
                $column_names
                    .iter()
                    .map(|c| ColumnName(Identifier(c.to_string())))
                    .collect(),
            ),
            expressions: NonEmptyVec::new(
                $expressions
                    .iter()
                    .map(|e| {
                        Expression::ConstantVariant(Constant::NumericConstantVariant(
                            NumericConstant::IntegerConstantVariant(IntegerConstant(e.to_string())),
                        ))
                    })
                    .collect(),
            ),
        }
    };
}

#[test]
fn test_insert_accepted() {
    let sql_vs_expected_ast: Vec<(&str, InsertCommand)> = vec![
        (
            "INSERT INTO t (id, c1) VALUES (1, 123)",
            insert!("t", None, vec!["id", "c1"], vec!["1", "123"]),
        ),
        (
            "INSERT INTO long_table_name AS t (id, c1) VALUES (1, 123)",
            insert!(
                "long_table_name",
                Some("t"),
                vec!["id", "c1"],
                vec!["1", "123"]
            ),
        ),
        (
            // acceptable syntactically though the number of columns and expressions are different.
            "INSERT INTO t (id, c1) VALUES (1, 123, 456)",
            insert!("t", None, vec!["id", "c1"], vec!["1", "123", "456"]),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::InsertCommandVariant(insert_command))) => {
                assert_eq!(insert_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as INSERT but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_insert_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack parentheses.
        "INSERT INTO t (id) VALUES 1",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
