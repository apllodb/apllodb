use apllodb_sql_parser::{
    apllodb_ast::{ColumnReference, Command, Expression, FromItem, SelectCommand, SelectField},
    ApllodbAst, ApllodbSqlParser,
};
use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_select_accepted() {
    let sql_vs_expected_ast: Vec<(&str, SelectCommand)> = vec![
        (
            "SELECT id FROM t",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                vec![FromItem::factory("t", None)],
                None,
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id, c1 FROM t",
            SelectCommand::factory(
                vec![
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                        None,
                    ),
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(None, "c1")),
                        None,
                    ),
                ],
                vec![FromItem::factory("t", None)],
                None,
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id, c1 FROM t1, t2",
            SelectCommand::factory(
                vec![
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                        None,
                    ),
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(None, "c1")),
                        None,
                    ),
                ],
                vec![FromItem::factory("t1", None), FromItem::factory("t2", None)],
                None,
                None,
                None,
                None,
            ),
        ),
    ];

    let parser = ApllodbSqlParser::default();

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

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
