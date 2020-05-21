use super::super::PestParserImpl;
use crate::apllodb_ast::{Command, DropTableCommand, Identifier, TableName};
use crate::parser_interface::ParserLike;
use crate::ApllodbAst;

macro_rules! drop_table {
    ($table_name: expr $(,)?) => {
        DropTableCommand {
            table_name: TableName(Identifier($table_name.to_string())),
        }
    };
}

#[test]
fn test_drop_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, DropTableCommand)> = vec![
        ("DROP TABLE t", drop_table!("t")),
        ("DROP TABLE t;", drop_table!("t")),
        ("  DROP\tTABLE\nt ", drop_table!("t")),
        ("DROP TABLE 机", drop_table!("机")),
        ("DROP TABLE 🍙", drop_table!("🍙")),
        // Keyword is case-sensitive.
        ("DROP TABLE drop", drop_table!("drop")),
    ];

    let parser = PestParserImpl::new();

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

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
