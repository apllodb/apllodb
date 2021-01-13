use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb, SqlType,
    TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{
    DDLMethods, DatabaseMethods, StorageEngine, TransactionMethods,
};

#[test]
fn test_generic_program() {
    fn use_database<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess Engine,
        session: &'sess SessionWithoutDb,
    ) -> ApllodbResult<SessionWithDb> {
        let db = engine.db(&session);
        let session = db.use_database(DatabaseName::new("dummy")?)?;
        Ok(session)
    }

    fn begin<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess Engine,
        session: &'sess SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        let tx = engine.tx(&session);
        let session = tx.begin()?;
        Ok(session)
    }

    fn create_table<'sess, Engine: StorageEngine<'sess>>(
        engine: &'sess Engine,
        session: &'sess SessionWithTx,
    ) -> ApllodbResult<()> {
        let ddl = engine.ddl(&session);
        ddl.create_table(
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
