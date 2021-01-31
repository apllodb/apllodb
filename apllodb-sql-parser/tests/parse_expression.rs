use apllodb_sql_parser::{
    apllodb_ast::{Command, Constant, Expression, SelectCommand},
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_constant_accepted() {
    let constant_vs_expected_ast: Vec<(&str, Constant)> =
        vec![("0", Constant::factory_integer("0"))];

    let parser = ApllodbSqlParser::default();

    for (constant, expected_ast) in constant_vs_expected_ast {
        match parser.parse(format!("SELECT {}", constant)) {
            Ok(ApllodbAst(Command::SelectCommandVariant(SelectCommand {
                select_fields, ..
            }))) => {
                let fields = select_fields.into_vec();
                assert_eq!(fields.len(), 1);

                match &fields[0].expression {
                    Expression::ConstantVariant(c) => {
                        assert_eq!(c, &expected_ast);
                    }
                    _ => panic!(),
                }
            }
            x => panic!("{:#?}", x),
        }
    }
}
