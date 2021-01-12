use apllodb_storage_engine_interface::StorageEngine;

#[test]
fn test_generic_program() {
    #[allow(dead_code)]
    fn call_access_methods<Engine: StorageEngine>() {
        let engine = Engine::default();
        let db = Engine::db(engine);
    }
}
