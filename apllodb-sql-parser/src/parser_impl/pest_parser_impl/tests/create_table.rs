use super::{super::PestParserImpl, create_table, te_coldef, te_pk};
use crate::apllodb_ast::{ColumnConstraint, Command, CreateTableCommand, DataType};
use crate::parser_interface::ParserLike;
use crate::ApllodbAst;
use pretty_assertions::assert_eq;

#[test]
fn test_create_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, CreateTableCommand)> = vec![
        (
            "CREATE TABLE t (id INTEGER)",
            create_table("t", vec![te_coldef("id", DataType::integer(), vec![])]),
        ),
        (
            "CREATE TABLE t (id INTEGER NOT NULL, c1 INTEGER)",
            create_table(
                "t",
                vec![
                    te_coldef(
                        "id",
                        DataType::integer(),
                        vec![ColumnConstraint::NotNullVariant],
                    ),
                    te_coldef("c1", DataType::integer(), vec![]),
                ],
            ),
        ),
        (
            "CREATE TABLE t (id INTEGER, c1 INTEGER, PRIMARY KEY (id, c1))",
            create_table(
                "t",
                vec![
                    te_coldef("id", DataType::integer(), vec![]),
                    te_coldef("c1", DataType::integer(), vec![]),
                    te_pk(vec!["id", "c1"]),
                ],
            ),
        ),
        (
            "CREATE TABLE t (id INTEGER, PRIMARY KEY (id), c1 INTEGER)",
            create_table(
                "t",
                vec![
                    te_coldef("id", DataType::integer(), vec![]),
                    te_pk(vec!["id"]),
                    te_coldef("c1", DataType::integer(), vec![]),
                ],
            ),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::CreateTableCommandVariant(create_table_command))) => {
                assert_eq!(create_table_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as CREATE TABLE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_create_table_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack data-type.
        "CREATE TABLE t (c1)",
        // Lack data-type again.
        "CREATE TABLE t (c1 NOT NULL)",
        // Should be parenthesized.
        "CREATE TABLE t c1 INTEGER NOT NULL",
        // `NOT NULL` is a keyword, only a space is allowed.
        "CREATE TABLE t (c1 INTEGER NOT  NULL)",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
