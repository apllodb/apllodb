pub mod database;
mod macros;

pub(crate) fn setup() {
    let _ = env_logger::builder()
        .is_test(false) // To enable color. Logs are not captured by test framework.
        .try_init();
}
