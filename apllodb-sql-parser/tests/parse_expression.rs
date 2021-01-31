use apllodb_sql_parser::{ApllodbAst, ApllodbSqlParser, apllodb_ast::{Command, Expression, SelectCommand, UnaryOperator}};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_constant_accepted() {
    let expression_vs_expected_ast: Vec<(&str, Expression)> = vec![
        ("0", Expression::factory_integer("0")),
        (
            // u128::MAX + 1
            "340282366920938463463374607431768211457",
            Expression::factory_integer("340282366920938463463374607431768211457"),
        ),
        (
            "-1",
            Expression::factory_uni_op(UnaryOperator::Minus, Expression::factory_integer("1")),
        ),
    ];

    let parser = ApllodbSqlParser::default();

    for (expression, expected_ast) in expression_vs_expected_ast {
        match parser.parse(format!("SELECT {}", expression)) {
            Ok(ApllodbAst(Command::SelectCommandVariant(SelectCommand {
                select_fields, ..
            }))) => {
                let fields = select_fields.into_vec();
                assert_eq!(fields.len(), 1);
                assert_eq!(&fields[0].expression, &expected_ast);
            }
            x => panic!("{:#?}", x),
        }
    }
}
