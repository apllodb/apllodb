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
