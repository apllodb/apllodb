use super::super::PestParserImpl;
use crate::apllodb_ast::NonEmptyVec;
use crate::apllodb_ast::{
    Action, AddColumn, AlterTableCommand, ColumnConstraint, ColumnDefinition, ColumnName, Command,
    DataType, DropColumn, Identifier, IntegerType, TableName,
};
use crate::parser_interface::ParserLike;
use crate::ApllodbAst;

macro_rules! alter_table {
    ($table_name: expr, $actions: expr $(,)?) => {
        AlterTableCommand {
            table_name: TableName(Identifier($table_name.to_string())),
            actions: NonEmptyVec::new($actions),
        }
    };
}

macro_rules! add_column {
    ($column_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {
        Action::AddColumnVariant(AddColumn {
            column_definition: ColumnDefinition {
                column_name: ColumnName(Identifier($column_name.to_string())),
                data_type: $data_type,
                column_constraints: $column_constraints,
            },
        })
    };
}

macro_rules! drop_column {
    ($column_name: expr $(,)?) => {
        Action::DropColumnVariant(DropColumn {
            column_name: ColumnName(Identifier($column_name.to_string())),
        })
    };
}

#[test]
fn test_alter_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, AlterTableCommand)> = vec![
        (
            "ALTER TABLE t ADD COLUMN c1 INTEGER",
            alter_table!(
                "t",
                vec![add_column!(
                    "c1",
                    DataType::IntegerTypeVariant(IntegerType::IntegerVariant),
                    vec![]
                )]
            ),
        ),
        (
            "ALTER TABLE t ADD COLUMN c1 INTEGER NOT NULL, DROP COLUMN c2",
            alter_table!(
                "t",
                vec![
                    add_column!(
                        "c1",
                        DataType::IntegerTypeVariant(IntegerType::IntegerVariant),
                        vec![ColumnConstraint::NotNullVariant]
                    ),
                    drop_column!("c2")
                ]
            ),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::AlterTableCommandVariant(alter_table_command))) => {
                assert_eq!(alter_table_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as ALTER TABLE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_alter_table_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack action.
        "ALTER TABLE t",
        // Lack data-type.
        "ALTER TABLE t ADD COLUMN c1",
        // Should not be parenthesized.
        "ALTER TABLE t (ADD COLUMN c1 INTEGER)",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
