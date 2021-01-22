mod test_support;

use std::{cell::RefCell, rc::Rc};

use crate::test_support::setup;
use apllodb_immutable_schema_engine::{
    ApllodbImmutableSchemaEngine, SqliteDatabasePool, SqliteTxPool,
};
use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, DatabaseName, SessionWithoutDb, SqlType, TableName,
};

#[tokio::test]
async fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    setup();

    let db_pool = Rc::new(RefCell::new(SqliteDatabasePool::default()));
    let tx_pool = Rc::new(RefCell::new(SqliteTxPool::default()));
    let engine = ApllodbImmutableSchemaEngine::new(db_pool, tx_pool);

    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::default(),
    );

    let session = engine
        .without_db_methods()
        .use_database(SessionWithoutDb::default(), DatabaseName::new("xyzw")?)
        .await?;

    log::debug!("SessionWithDb: {:?}", session);

    // let session = client
    //     .begin_transaction(context::current(), session)
    //     .await??;

    // log::debug!("SessionWithTx: {:?}", session);

    // let ddl = ApllodbImmutableSchemaDDL::default();

    // ddl.create_table(
    //     &mut tx,
    //     &t_name,
    //     &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
    //         column_names: vec![c1_def
    //             .column_data_type()
    //             .column_ref()
    //             .as_column_name()
    //             .clone()],
    //     }])?,
    //     vec![c1_def],
    // )?;
    // tx.abort()?;

    //h.join_handle.join().expect("server thread panic-ed:");

    // TODO Kill server thread

    Ok(())
}
