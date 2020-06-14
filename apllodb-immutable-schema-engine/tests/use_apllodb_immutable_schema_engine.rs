use apllodb_shared_components::error::ApllodbResult;

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};
    use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;

    let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new("db")?)?;
    let mut tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db)?;
    tx.create_table(&TableName::new("t")?, &TableConstraints::default(), &vec![])?;
    tx.abort()?;

    Ok(())
}
