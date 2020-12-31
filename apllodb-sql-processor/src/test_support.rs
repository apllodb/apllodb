pub(crate) mod macros;
pub(crate) mod mock_tx;
pub(crate) mod test_models;
pub(crate) mod test_storage_engine;
pub(crate) mod utility_functions;

pub(crate) fn setup() {
    let _ = env_logger::builder().is_test(true).try_init();
}
