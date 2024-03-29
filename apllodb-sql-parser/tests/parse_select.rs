use apllodb_sql_parser::{
    apllodb_ast::{
        ColumnReference, Command, Condition, Correlation, Expression, FromItem, OrderBy, Ordering,
        SelectCommand, SelectField, UnaryOperator,
    },
    ApllodbAst, ApllodbSqlParser,
};
use apllodb_test_support::setup::setup_test_logger;
use pretty_assertions::assert_eq;

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
                Some(FromItem::factory_tn("t", None)),
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
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id, t.c1 FROM t",
            SelectCommand::factory(
                vec![
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                        None,
                    ),
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(
                            Some(Correlation::factory("t")),
                            "c1",
                        )),
                        None,
                    ),
                ],
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id, s.c1 FROM t AS s",
            SelectCommand::factory(
                vec![
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                        None,
                    ),
                    SelectField::factory(
                        Expression::factory_colref(ColumnReference::factory(
                            Some(Correlation::factory("s")),
                            "c1",
                        )),
                        None,
                    ),
                ],
                Some(FromItem::factory_tn("t", Some("s"))),
                None,
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id", // OK as syntax (but NG for semantically)
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                None,
                None,
                None,
                None,
                None,
            ),
        ),
        // Selection
        (
            "SELECT c FROM t WHERE c",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "c")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                Some(Condition {
                    expression: Expression::factory_colref(ColumnReference::factory(None, "c")),
                }),
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id FROM t WHERE id = 1",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                Some(Condition {
                    expression: Expression::factory_eq(
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                        Expression::factory_integer("1"),
                    ),
                }),
                None,
                None,
                None,
            ),
        ),
        (
            "SELECT id FROM t WHERE id = -1",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                Some(Condition {
                    expression: Expression::factory_eq(
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                        Expression::factory_uni_op(
                            UnaryOperator::Minus,
                            Expression::factory_integer("1"),
                        ),
                    ),
                }),
                None,
                None,
                None,
            ),
        ),
        // Sort
        (
            "SELECT id FROM t ORDER BY id",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                Some(vec![OrderBy::factory_colref(
                    ColumnReference::factory(None, "id"),
                    None,
                )]),
            ),
        ),
        (
            "SELECT id FROM t ORDER BY id ASC",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                Some(vec![OrderBy::factory_colref(
                    ColumnReference::factory(None, "id"),
                    Some(Ordering::AscVariant),
                )]),
            ),
        ),
        (
            "SELECT id FROM t ORDER BY id DESC",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                Some(vec![OrderBy::factory_colref(
                    ColumnReference::factory(None, "id"),
                    Some(Ordering::DescVariant),
                )]),
            ),
        ),
        (
            "SELECT id FROM t ORDER BY -id",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                Some(vec![OrderBy::factory_expr(
                    Expression::factory_uni_op(
                        UnaryOperator::Minus,
                        Expression::factory_colref(ColumnReference::factory(None, "id")),
                    ),
                    None,
                )]),
            ),
        ),
        (
            "SELECT id FROM t ORDER BY age ASC, id DESC",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "id")),
                    None,
                )],
                Some(FromItem::factory_tn("t", None)),
                None,
                None,
                None,
                Some(vec![
                    OrderBy::factory_colref(
                        ColumnReference::factory(None, "age"),
                        Some(Ordering::AscVariant),
                    ),
                    OrderBy::factory_colref(
                        ColumnReference::factory(None, "id"),
                        Some(Ordering::DescVariant),
                    ),
                ]),
            ),
        ),
        // Join
        (
            "SELECT t.id FROM t INNER JOIN s ON t.id = s.t_id",
            SelectCommand::factory(
                vec![SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(
                        Some(Correlation::factory("t")),
                        "id",
                    )),
                    None,
                )],
                Some(FromItem::factory_inner_join(
                    FromItem::factory_tn("t", None),
                    FromItem::factory_tn("s", None),
                    Expression::factory_eq(
                        Expression::factory_colref(ColumnReference::factory(
                            Some(Correlation::factory("t")),
                            "id",
                        )),
                        Expression::factory_colref(ColumnReference::factory(
                            Some(Correlation::factory("s")),
                            "t_id",
                        )),
                    ),
                )),
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
    ];

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
