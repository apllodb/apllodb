use ctor::ctor;
use std::sync::Once;

/// setup env_logger for test.
#[cfg_attr(any(feature = "setup-all", feature = "setup-setup_logger"), ctor)]
pub fn setup_test_logger() {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}
