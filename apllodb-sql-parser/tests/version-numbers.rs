use apllodb_test_support::setup::setup_test_logger;
use ctor::ctor;

#[ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
