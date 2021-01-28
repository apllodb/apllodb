mod test_support;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::test_setup;
use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, DatabaseName, Session, SessionWithoutDb, SqlType, TableConstraintKind,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{
    StorageEngine, WithDbMethods, WithTxMethods, WithoutDbMethods,
};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();

    let db_name = DatabaseName::new("db")?;
    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::default(),
    );

    let _ = engine
        .without_db()
        .create_database(
            Session::from(SessionWithoutDb::default()),
            db_name.clone(),
        )
        .await?;

    let session = engine
        .without_db()
        .use_database(SessionWithoutDb::default(), db_name)
        .await?;

    log::debug!("SessionWithDb: {:?}", session);

    let session = engine.with_db().begin_transaction(session).await?;

    log::debug!("SessionWithTx: {:?}", session);

    let session = engine
        .with_tx()
        .create_table(
            session,
            t_name,
            TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                column_names: vec![c1_def
                    .column_data_type()
                    .column_ref()
                    .as_column_name()
                    .clone()],
            }])?,
            vec![c1_def],
        )
        .await?;

    engine.with_tx().commit_transaction(session).await?;

    Ok(())
}
