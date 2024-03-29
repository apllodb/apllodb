mod test_support;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::{
    sqlite_database_cleaner::SqliteDatabaseCleaner, test_setup,
};
use apllodb_shared_components::{
    ApllodbError, ApllodbResult, DatabaseName, Session, SessionWithoutDb, SqlState, SqlType,
};
use apllodb_storage_engine_interface::{
    ColumnConstraints, ColumnDataType, ColumnDefinition, StorageEngine, TableConstraintKind,
    TableConstraints, TableName, WithDbMethods, WithTxMethods, WithoutDbMethods,
};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_wait_lock() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let db = DatabaseName::random();
    let _db_cleaner = SqliteDatabaseCleaner::new(db.clone());

    let _ = engine
        .without_db()
        .create_database(Session::default(), db.clone())
        .await?;

    let session1 = engine
        .without_db()
        .use_database(SessionWithoutDb::default(), db.clone())
        .await?;
    let session2 = engine
        .without_db()
        .use_database(SessionWithoutDb::default(), db.clone())
        .await?;

    let session_tx1 = engine.with_db().begin_transaction(session1).await?;
    let session_tx2 = engine.with_db().begin_transaction(session2).await?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::factory("c1", SqlType::integer(), false),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c1_def.column_data_type().column_name().clone()],
    }])?;

    // tx1 (inside session1) is created earlier than tx2 (inside session2) but tx2 issues CREATE TABLE command in prior to tx1.
    // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
    let session_tx2 = engine
        .with_tx()
        .create_table(session_tx2, t_name.clone(), tc.clone(), coldefs.clone())
        .await?;
    match engine
        .with_tx()
        .create_table(session_tx1, t_name.clone(), tc.clone(), coldefs)
        .await
    {
        // Internally, new record is trying to be INSERTed but it is made wait by tx2.
        // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
        Err(e) => assert_eq!(
            ApllodbError::from(e).kind(),
            &SqlState::TransactionRollbackDeadlock
        ),
        Ok(_) => panic!("should rollback"),
    }

    engine.with_tx().commit_transaction(session_tx2).await?;

    Ok(())
}
