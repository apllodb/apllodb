use std::fmt::Display;

// https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err
#[test]
fn test_api_guidelines_c_good_err() {
    use apllodb_sql_parser::error::ApllodbSqlParserError;
    use std::error::Error;

    fn assert_error<T: Error + Send + Sync + 'static>() {}
    assert_error::<ApllodbSqlParserError>();

    fn assert_display<T: Display>() {}
    assert_display::<ApllodbSqlParserError>();
}

#[test]
fn test_none_source() {
    use apllodb_sql_parser::ApllodbSqlParser;
    use std::error::Error;

    let parser = ApllodbSqlParser::new();
    match parser.parse("DROP TABLE FROM people") {
        Err(e) => {
            assert!(e.source().is_none(), "No root cause. Just a syntax error.");
        }
        Ok(ast) => panic!("Syntax error should be reported but parsed as: {:?}", ast),
    }
}
