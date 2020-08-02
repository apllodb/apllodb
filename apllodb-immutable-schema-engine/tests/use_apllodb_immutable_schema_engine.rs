use apllodb_shared_components::{
    data_structure::{
        ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName, DataType, DataTypeKind,
        TableConstraintKind,
    },
    error::ApllodbResult,
};

fn setup() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(
        "db_test_use_apllodb_immutable_schema_engine",
    )?)?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db)?;
    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_data_types: vec![ColumnDataType::new(
                ColumnName::new("c1")?,
                DataType::new(DataTypeKind::Integer, false),
            )],
        }])?,
        &vec![ColumnDefinition::new(
            ColumnName::new("c1")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::default(),
        )?],
    )?;
    tx.abort()?;

    Ok(())
}

// -------------------    #[test]
#[test]
fn test_tx_id_order() -> ApllodbResult<()> {
    use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
    use apllodb_shared_components::data_structure::DatabaseName;
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    setup();

    let mut db1 =
        ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new("db_test_tx_id_order")?)?;
    let mut db2 =
        ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new("db_test_tx_id_order")?)?;

    let tx1 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db1)?;
    let tx2 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db2)?;

    assert!(tx1.id() < tx2.id());

    Ok(())
}
