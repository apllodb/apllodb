mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    ApllodbErrorKind, ApllodbResult, ColumnConstraints, ColumnDefinition, ColumnName,
    ColumnReference, DataType, DataTypeKind, TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

#[test]
fn test_wait_lock() -> ApllodbResult<()> {
    setup();

    let mut db1 = TestDatabase::new()?;
    let mut db2 = db1.dup()?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c1_def.column_ref().as_column_name().clone()],
    }])?;

    let tx1 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db1.0)?;
    let tx2 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db2.0)?;

    // tx1 is created earlier than tx2 but tx2 issues CREATE TABLE command in prior to tx1.
    // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
    tx2.create_table(&t_name, &tc, &coldefs)?;
    match tx1.create_table(&t_name, &tc, &coldefs) {
        // Internally, new record is trying to be INSERTed but it is made wait by tx2.
        // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
        Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DeadlockDetected),
        Ok(_) => panic!("should rollback"),
    }

    tx1.commit()?; // it's ok to commit tx1 although it already aborted by error.
    tx2.commit()?;

    Ok(())
}

#[test]
fn test_tx_id_order() -> ApllodbResult<()> {
    setup();

    let mut db1 = TestDatabase::new()?;
    let mut db2 = db1.dup()?;

    let tx1 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db1.0)?;
    let tx2 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db2.0)?;

    assert!(tx1.id() < tx2.id());

    Ok(())
}
