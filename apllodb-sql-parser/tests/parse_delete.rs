use apllodb_sql_parser::{
    apllodb_ast::{Command, DeleteCommand},
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_delete_accepted() {
    let sql_vs_expected_ast: Vec<(&str, DeleteCommand)> =
        vec![("DELETE FROM t", DeleteCommand::factory("t", None, None))];
    vec![(
        "DELETE FROM long_table_name AS t",
        DeleteCommand::factory("long_table_name", Some("t"), None),
    )];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::DeleteCommandVariant(delete_command))) => {
                assert_eq!(delete_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as DELETE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_delete_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack FROM.
        "DELETE t",
    ];

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
