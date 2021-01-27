use apllodb_sql_parser::{
    apllodb_ast::{Command, UseDatabaseCommand},
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_use_database_accepted() {
    let sql_vs_expected_ast: Vec<(&str, UseDatabaseCommand)> =
        vec![("USE DATABASE d", UseDatabaseCommand::factory("d"))];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::UseDatabaseCommandVariant(use_database_command))) => {
                assert_eq!(use_database_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as USE DATABASE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
