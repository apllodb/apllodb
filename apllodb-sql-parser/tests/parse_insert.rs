use apllodb_sql_parser::{
    apllodb_ast::{Command, Expression, InsertCommand, InsertValue},
    ApllodbAst, ApllodbSqlParser,
};
use pretty_assertions::assert_eq;

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_insert_accepted() {
    let sql_vs_expected_ast: Vec<(&str, InsertCommand)> = vec![
        (
            "INSERT INTO t (id, c1) VALUES (1, 123)",
            InsertCommand::factory(
                "t",
                None,
                vec!["id", "c1"],
                vec![InsertValue::factory(vec![
                    Expression::factory_integer("1"),
                    Expression::factory_integer("123"),
                ])],
            ),
        ),
        (
            r#"INSERT INTO t (c) VALUES ("abc")"#,
            InsertCommand::factory(
                "t",
                None,
                vec!["c"],
                vec![InsertValue::factory(vec![Expression::factory_text(
                    r#"abc"#,
                )])],
            ),
        ),
        (
            r#"INSERT INTO t (c) VALUES ("abc🍣 '@\")"#,
            InsertCommand::factory(
                "t",
                None,
                vec!["c"],
                vec![InsertValue::factory(vec![Expression::factory_text(
                    r#"abc🍣 '@\"#,
                )])],
            ),
        ),
        (
            "INSERT INTO long_table_name AS t (id, c1) VALUES (1, 123)",
            InsertCommand::factory(
                "long_table_name",
                Some("t"),
                vec!["id", "c1"],
                vec![InsertValue::factory(vec![
                    Expression::factory_integer("1"),
                    Expression::factory_integer("123"),
                ])],
            ),
        ),
        (
            // acceptable syntactically though the number of columns and expressions are different.
            "INSERT INTO t (id, c1) VALUES (1, 123, 456)",
            InsertCommand::factory(
                "t",
                None,
                vec!["id", "c1"],
                vec![InsertValue::factory(vec![
                    Expression::factory_integer("1"),
                    Expression::factory_integer("123"),
                    Expression::factory_integer("456"),
                ])],
            ),
        ),
        (
            "INSERT INTO t (id, c1) VALUES (1, 123), (2, 456)",
            InsertCommand::factory(
                "t",
                None,
                vec!["id", "c1"],
                vec![
                    InsertValue::factory(vec![
                        Expression::factory_integer("1"),
                        Expression::factory_integer("123"),
                    ]),
                    InsertValue::factory(vec![
                        Expression::factory_integer("2"),
                        Expression::factory_integer("456"),
                    ]),
                ],
            ),
        ),
    ];

    let parser = ApllodbSqlParser::default();

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

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
