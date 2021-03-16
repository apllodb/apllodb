pub mod fixture;

use apllodb_test_support::setup::setup_test_logger;

/// general test setup sequence
pub fn test_setup() {
    setup_test_logger();
}
