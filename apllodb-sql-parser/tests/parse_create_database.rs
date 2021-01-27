use apllodb_sql_parser::{
    apllodb_ast::{Command, CreateDatabaseCommand},
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_create_database_accepted() {
    let sql_vs_expected_ast: Vec<(&str, CreateDatabaseCommand)> =
        vec![("CREATE DATABASE d", CreateDatabaseCommand::factory("d"))];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::CreateDatabaseCommandVariant(create_database_command))) => {
                assert_eq!(create_database_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as CREATE DATABASE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
