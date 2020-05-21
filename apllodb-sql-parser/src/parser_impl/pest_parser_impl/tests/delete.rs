use super::super::PestParserImpl;
use crate::apllodb_ast::{Alias, Command, DeleteCommand, Identifier, TableName};
use crate::parser_interface::ParserLike;
use crate::ApllodbAst;

macro_rules! delete {
    ($table_name: expr, $alias: expr $(,)?) => {
        DeleteCommand {
            table_name: TableName(Identifier($table_name.to_string())),
            alias: $alias.map(|a: &str| Alias(Identifier(a.to_string()))),
            where_condition: None,
        }
    };
}

#[test]
fn test_delete_accepted() {
    let sql_vs_expected_ast: Vec<(&str, DeleteCommand)> =
        vec![("DELETE FROM t", delete!("t", None))];
    vec![(
        "DELETE FROM long_table_name AS t",
        delete!("long_table_name", Some("t")),
    )];

    let parser = PestParserImpl::new();

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
fn test_insert_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack FROM.
        "DELETE t",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
