use super::super::PestParserImpl;
use crate::apllo_ast::NonEmptyVec;
use crate::apllo_ast::{
    ColumnConstraint, ColumnName, Command, CreateTableColumnDefinition, CreateTableCommand,
    DataType, Identifier, IntegerType, TableName,
};
use crate::parser_interface::ParserLike;
use crate::AplloAst;

macro_rules! create_table {
    ($table_name: expr, $column_definitions: expr $(,)?) => {
        CreateTableCommand {
            table_name: TableName(Identifier($table_name.to_string())),
            create_table_column_definitions: NonEmptyVec::new($column_definitions),
        }
    };
}

macro_rules! coldef {
    ($column_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {
        CreateTableColumnDefinition {
            column_name: ColumnName(Identifier($column_name.to_string())),
            data_type: $data_type,
            column_constraints: $column_constraints,
        }
    };
}

#[test]
fn test_create_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, CreateTableCommand)> = vec![
        (
            "CREATE TABLE t (id INTEGER)",
            create_table!(
                "t",
                vec![coldef!(
                    "id",
                    DataType::IntegerTypeVariant(IntegerType::IntegerVariant),
                    vec![]
                )]
            ),
        ),
        (
            "CREATE TABLE t (id INTEGER NOT NULL, c1 INTEGER)",
            create_table!(
                "t",
                vec![
                    coldef!(
                        "id",
                        DataType::IntegerTypeVariant(IntegerType::IntegerVariant),
                        vec![ColumnConstraint::NotNullVariant]
                    ),
                    coldef!(
                        "c1",
                        DataType::IntegerTypeVariant(IntegerType::IntegerVariant),
                        vec![]
                    ),
                ],
            ),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(AplloAst(Command::CreateTableCommandVariant(create_table_command))) => {
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
