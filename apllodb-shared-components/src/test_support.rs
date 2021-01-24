//! Testing support commonly used among apllodb repository.

use std::sync::Once;

static INIT: Once = Once::new();

#[cfg(test)]
pub(crate) fn setup() {
    setup_test_logger();
}

/// setup env_logger for test.
pub fn setup_test_logger() {
    INIT.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}
