use super::super::PestParserImpl;
use crate::apllodb_ast::NonEmptyVec;
use crate::apllodb_ast::{
    ColumnName, ColumnReference, Command, Expression, FromItem, Identifier, SelectCommand,
    SelectField, TableName,
};
use crate::parser_interface::ParserLike;
use crate::ApllodbAst;

macro_rules! select {
    ($select_fields: expr, $from_items: expr $(,)?) => {
        SelectCommand {
            select_fields: NonEmptyVec::new($select_fields),
            from_items: NonEmptyVec::new($from_items),
            where_condition: None,
            grouping_elements: None,
            having_conditions: None,
            order_bys: None,
        }
    };
}

macro_rules! select_field {
    ($expression: expr $(,)?) => {
        SelectField {
            expression: Expression::ColumnReferenceVariant(ColumnReference {
                correlation: None,
                column_name: ColumnName(Identifier($expression.to_string())),
            }),
            alias: None,
        }
    };
}

macro_rules! from_item {
    ($table_name: expr $(,)?) => {
        FromItem {
            table_name: TableName(Identifier($table_name.to_string())),
            alias: None,
        }
    };
}

#[test]
fn test_select_accepted() {
    let sql_vs_expected_ast: Vec<(&str, SelectCommand)> = vec![
        (
            "SELECT id FROM t",
            select!(vec![select_field!("id")], vec![from_item!("t")],),
        ),
        (
            "SELECT id, c1 FROM t",
            select!(
                vec![select_field!("id"), select_field!("c1")],
                vec![from_item!("t")],
            ),
        ),
        (
            "SELECT id, c1 FROM t1, t2",
            select!(
                vec![select_field!("id"), select_field!("c1")],
                vec![from_item!("t1"), from_item!("t2")],
            ),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::SelectCommandVariant(select_command))) => {
                assert_eq!(select_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as SELECT but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_select_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack select_field.
        "SELECT FROM t",
        // Lack from_item.
        "SELECT id",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
