// https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err
#[test]
fn test_api_guidelines_c_good_err() {
    use apllo_shared_components::error::AplloError;
    use std::error::Error;
    use std::fmt::Display;

    fn assert_error<T: Error + Send + Sync + 'static>() {}
    assert_error::<AplloError>();

    fn assert_display<T: Display>() {}
    assert_display::<AplloError>();
}
