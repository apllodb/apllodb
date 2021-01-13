use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb, SqlType,
    TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{
    MethodsWithDb, MethodsWithTx, MethodsWithoutDb, StorageEngine,
};

#[test]
fn test_generic_program() {
    #[allow(dead_code)]
    fn use_database<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess Engine,
        session: &'sess SessionWithoutDb,
    ) -> ApllodbResult<SessionWithDb> {
        let no_db = engine.without_db(&session);
        let session = no_db.use_database(DatabaseName::new("dummy")?)?;
        Ok(session)
    }

    #[allow(dead_code)]
    fn begin<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess Engine,
        session: &'sess SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        let db = engine.with_db(&session);
        let session = db.begin()?;
        Ok(session)
    }

    #[allow(dead_code)]
    fn create_table<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess Engine,
        session: &'sess SessionWithTx,
    ) -> ApllodbResult<()> {
        let tx = engine.with_tx(&session);
        tx.create_table(
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
}
