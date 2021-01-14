#![cfg(feature = "test-support")]

use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb, SqlType,
    TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{
    test_support::TestStorageEngine, MethodsWithDb, MethodsWithTx, MethodsWithoutDb, StorageEngine,
};

#[test]
fn test_generic_program() -> ApllodbResult<()> {
    #[allow(dead_code)]
    fn use_database<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess mut Engine,
        session: SessionWithoutDb,
    ) -> ApllodbResult<SessionWithDb> {
        let no_db = engine.without_db();
        let session = no_db.use_database(session, DatabaseName::new("dummy")?)?;
        Ok(session)
    }

    #[allow(dead_code)]
    fn begin<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess mut Engine,
        session: SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        let db = engine.with_db();
        let session = db.begin(session)?;
        Ok(session)
    }

    #[allow(dead_code)]
    fn create_table<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess mut Engine,
        session: &SessionWithTx,
    ) -> ApllodbResult<()> {
        let tx = engine.with_tx();
        tx.create_table(
            session,
            &TableName::new("t")?,
            &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                column_names: vec![ColumnName::new("id")?],
            }])?,
            vec![ColumnDefinition::new(
                ColumnDataType::new(
                    ColumnReference::new(TableName::new("t")?, ColumnName::new("id")?),
                    SqlType::big_int(),
                    false,
                ),
                ColumnConstraints::new(vec![])?,
            )],
        )
    }

    #[allow(dead_code)]
    fn server_code() -> ApllodbResult<()> {
        // here injects real impl of StorageEngine
        let mut engine = TestStorageEngine::default();
        let session = SessionWithoutDb::default();

        let session = use_database(&mut engine, session)?;
        let session = begin(&mut engine, session)?;
        create_table(&mut engine, &session)?;

        Ok(())
    }
}
