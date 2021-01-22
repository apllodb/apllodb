mod test_support;

use crate::test_support::setup;
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    ApllodbErrorKind, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    ColumnName, ColumnReference, DatabaseName, SessionWithoutDb, SqlType, TableConstraintKind,
    TableConstraints, TableName,
};

#[async_std::test]
async fn test_wait_lock() -> ApllodbResult<()> {
    setup();

    let engine = ApllodbImmutableSchemaEngine::new();
    let db = DatabaseName::new("test_wait_lock")?;

    let session1 = engine
        .without_db_methods()
        .use_database(SessionWithoutDb::default(), db.clone())
        .await?;
    let session2 = engine
        .without_db_methods()
        .use_database(SessionWithoutDb::default(), db.clone())
        .await?;

    let session_tx1 = engine.with_db_methods().begin_transaction(session1).await?;
    let session_tx2 = engine.with_db_methods().begin_transaction(session2).await?;

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

    // tx1 (inside session1) is created earlier than tx2 (inside session2) but tx2 issues CREATE TABLE command in prior to tx1.
    // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
    let session_tx2 = engine
        .with_tx_methods()
        .create_table(session_tx2, t_name.clone(), tc.clone(), coldefs.clone())
        .await?;
    match engine
        .with_tx_methods()
        .create_table(session_tx1, t_name.clone(), tc.clone(), coldefs)
        .await
    {
        // Internally, new record is trying to be INSERTed but it is made wait by tx2.
        // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
        Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DeadlockDetected),
        Ok(_) => panic!("should rollback"),
    }

    engine
        .with_tx_methods()
        .commit_transaction(session_tx2)
        .await?;

    Ok(())
}
