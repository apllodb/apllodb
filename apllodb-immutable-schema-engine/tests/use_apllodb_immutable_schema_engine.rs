use apllodb_shared_components::{
    data_structure::{ColumnConstraints, ColumnDefinition, ColumnName, DataType, DataTypeKind},
    error::ApllodbResult,
};

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(
        "db_test_use_apllodb_immutable_schema_engine",
    )?)?;
    let mut tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db)?;
    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::default(),
        &vec![ColumnDefinition::new(
            ColumnName::new("c1")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::default(),
        )?],
    )?;
    tx.abort()?;

    Ok(())
}
