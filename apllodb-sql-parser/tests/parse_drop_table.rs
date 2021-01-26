use apllodb_sql_parser::{
    apllodb_ast::{Command, DropTableCommand},
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_drop_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, DropTableCommand)> = vec![
        ("DROP TABLE t", DropTableCommand::factory("t")),
        ("DROP TABLE t;", DropTableCommand::factory("t")),
        ("  DROP\tTABLE\nt ", DropTableCommand::factory("t")),
        ("DROP TABLE 机", DropTableCommand::factory("机")),
        ("DROP TABLE 🍙", DropTableCommand::factory("🍙")),
        // Keyword is case-sensitive.
        ("DROP TABLE drop", DropTableCommand::factory("drop")),
    ];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::DropTableCommandVariant(drop_table_command))) => {
                assert_eq!(drop_table_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as DROP TABLE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_drop_table_rejected() {
    let sqls: Vec<&str> = vec![
        // Keyword is case-sensitive.
        "drop table t",
        // Does not accept trailing letter.
        "DROP TABLE t x",
        // Does not accept 2nd statement.
        "DROP TABLE t; DROP TABLE t2;",
        // Does not accept heading letter.
        "x DROP TABLE t",
        // Does not accept illegal white space.
        "DROP　TABLE t",
        // Does not accept keyword as identifier.
        "DROP TABLE CREATE",
    ];

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
