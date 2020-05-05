// https://rust-lang.github.io/api-guidelines/interoperability.html#data-structures-implement-serdes-serialize-deserialize-c-serde
#[cfg(feature = "serde")]
#[test]
fn test_api_guidelines_c_serde() {
    use apllo_sql_parser::AplloAst;
    use serde::{Deserialize, Serialize};

    fn assert_serialize<T: Serialize>() {}
    assert_serialize::<AplloAst>();

    fn assert_deserialize<'a, T: Deserialize<'a>>() {}
    assert_deserialize::<AplloAst>();
}

// https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync
#[test]
fn test_api_guidelines_c_send_sync() {
    use apllo_sql_parser::AplloAst;

    fn assert_send<T: Send>() {}
    assert_send::<AplloAst>();

    fn assert_sync<T: Sync>() {}
    assert_sync::<AplloAst>();
}
