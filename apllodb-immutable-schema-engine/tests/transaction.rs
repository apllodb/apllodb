mod test_support;

use crate::test_support::setup;
use apllodb_shared_components::{
    ApllodbErrorKind, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    ColumnName, ColumnReference, SqlType, TableConstraintKind, TableConstraints, TableName,
};

#[test]
fn test_wait_lock() -> ApllodbResult<()> {
    setup();

    // let mut db1 = TestDatabase::new()?;
    // let mut db2 = db1.dup()?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c1_def
            .column_data_type()
            .column_ref()
            .as_column_name()
            .clone()],
    }])?;

    // let mut tx1 = ApllodbImmutableSchemaTx::begin(&mut db1.0)?;
    // let mut tx2 = ApllodbImmutableSchemaTx::begin(&mut db2.0)?;

    // let ddl = ApllodbImmutableSchemaDDL::default();

    // // tx1 is created earlier than tx2 but tx2 issues CREATE TABLE command in prior to tx1.
    // // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
    // ddl.create_table(&mut tx2, &t_name, &tc, coldefs.clone())?;
    // match ddl.create_table(&mut tx1, &t_name, &tc, coldefs) {
    //     // Internally, new record is trying to be INSERTed but it is made wait by tx2.
    //     // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
    //     Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DeadlockDetected),
    //     Ok(_) => panic!("should rollback"),
    // }

    // tx1.commit()?; // it's ok to commit tx1 although it already aborted by error.
    // tx2.commit()?;

    Ok(())
}

#[test]
fn test_tx_id_order() -> ApllodbResult<()> {
    setup();

    // let mut db1 = TestDatabase::new()?;
    // let mut db2 = db1.dup()?;

    // let tx1 = ApllodbImmutableSchemaTx::begin(&mut db1.0)?;
    // let tx2 = ApllodbImmutableSchemaTx::begin(&mut db2.0)?;

    // assert!(tx1.id() < tx2.id());

    Ok(())
}
