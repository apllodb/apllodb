pub(crate) mod macros;
pub(crate) mod stub_storage_engine;

pub(crate) fn setup() {
    let _ = env_logger::builder().is_test(true).try_init();
}
