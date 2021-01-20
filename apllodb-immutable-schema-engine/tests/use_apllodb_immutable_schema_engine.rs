#![cfg(feature = "test-support")]

mod test_support;

use crate::test_support::setup;
use apllodb_immutable_schema_engine::{
    test_support::{make_client, spawn_server},
    ApllodbImmutableSchemaEngine,
};
use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, DatabaseName, SessionWithoutDb, SqlType, TableConstraintKind,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use futures::{future, prelude::*};
use tarpc::{
    client, context,
    server::{self, Handler},
};
use tokio::task;

#[tokio::test]
async fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    setup();

    let h = spawn_server().unwrap();

    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::default(),
    );

    let mut client = make_client(h.connect_addr).await?;

    let session = client
        .use_database(
            context::current(),
            SessionWithoutDb::default(),
            DatabaseName::new("xyzw")?,
        )
        .await??;

    log::debug!("session: {:?}", session);

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

    h.join_handle.join().expect("server thread panic-ed:");

    // TODO Kill server thread

    Ok(())
}
