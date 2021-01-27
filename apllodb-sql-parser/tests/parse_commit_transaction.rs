use apllodb_sql_parser::{apllodb_ast::Command, ApllodbAst, ApllodbSqlParser};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_commit_transaction_accepted() {
    let sqls: Vec<&str> = vec!["COMMIT", "COMMIT TRANSACTION"];

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::CommitTransactionCommandVariant)) => {}
            Ok(ast) => panic!(
                "'{}' should be parsed as COMMIT but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
