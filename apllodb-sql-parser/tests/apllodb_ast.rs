use apllodb_sql_parser::ApllodbSqlParser;
use apllodb_test_support::setup::setup_test_logger;
use ctor::ctor;

#[ctor]
fn test_setup() {
    setup_test_logger();
}

// https://rust-lang.github.io/api-guidelines/interoperability.html#data-structures-implement-serdes-serialize-deserialize-c-serde
#[cfg(feature = "serde")]
#[test]
fn test_api_guidelines_c_serde() {
    use apllodb_sql_parser::ApllodbAst;
    use serde::{Deserialize, Serialize};

    fn assert_serialize<T: Serialize>() {}
    assert_serialize::<ApllodbAst>();

    fn assert_deserialize<'a, T: Deserialize<'a>>() {}
    assert_deserialize::<ApllodbAst>();
}

// https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync
#[test]
fn test_api_guidelines_c_send_sync() {
    use apllodb_sql_parser::ApllodbAst;

    fn assert_send<T: Send>() {}
    assert_send::<ApllodbAst>();

    fn assert_sync<T: Sync>() {}
    assert_sync::<ApllodbAst>();
}

#[test]
fn test_case_insensitivity() {
    let sqls = vec!["SELECT id FROM t", "select id from t"];

    for sql in sqls {
        let parser = ApllodbSqlParser::default();
        parser.parse(sql).expect("should be parsed correctly");
    }
}
