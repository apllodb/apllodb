use apllodb_immutable_schema_engine_infra::test_support::clean_test_sqlite3;
use apllodb_test_support::setup::setup_test_logger;

/// general test setup sequence
pub fn test_setup() {
    setup_test_logger();
    clean_test_sqlite3().unwrap();
}
