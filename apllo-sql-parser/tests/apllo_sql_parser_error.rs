// https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync
#[test]
fn test_api_guidelines_c_send_sync() {
    use apllo_sql_parser::error::AplloSqlParserError;

    fn assert_send<T: Send>() {}
    assert_send::<AplloSqlParserError>();

    fn assert_sync<T: Sync>() {}
    assert_sync::<AplloSqlParserError>();
}
