use apllodb_sql_parser::{
    apllodb_ast::{ColumnReference, Command, Expression, UpdateCommand},
    ApllodbAst, ApllodbSqlParser,
};
use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_update_accepted() {
    let sql_vs_expected_ast: Vec<(&str, UpdateCommand)> = vec![
        (
            "UPDATE t SET c1 = 123",
            UpdateCommand::factory("t", None, "c1", Expression::factory_integer("123"), None),
        ),
        (
            "UPDATE long_table_name AS t SET c1 = 123",
            UpdateCommand::factory(
                "long_table_name",
                Some("t"),
                "c1",
                Expression::factory_integer("123"),
                None,
            ),
        ),
        (
            "UPDATE t SET c1 = c2",
            UpdateCommand::factory(
                "t",
                None,
                "c1",
                Expression::factory_colref(ColumnReference::factory(None, "c2")),
                None,
            ),
        ),
    ];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::UpdateCommandVariant(update_command))) => {
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

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
