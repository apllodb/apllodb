use apllodb_immutable_schema_engine_application::use_case::transaction::{
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    full_scan::{FullScanUseCase, FullScanUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
    update_all::{UpdateAllUseCase, UpdateAllUseCaseInput},
};
use apllodb_immutable_schema_engine_application::use_case::TxUseCase;
use apllodb_shared_components::{ApllodbResult, TableName, Transaction};
use apllodb_shared_components::{ColumnName, Expression, RecordIterator};
use apllodb_storage_engine_interface::DMLMethods;
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

use crate::{engine::ApllodbImmutableSchemaEngine, sqlite::sqlite_types::SqliteTypes};
use crate::{runtime, sqlite::transaction::sqlite_tx::SqliteTx};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct DMLMethodsImpl;

impl DMLMethods<ApllodbImmutableSchemaEngine<'_>> for DMLMethodsImpl {
    fn select(
        &self,
        tx: &mut SqliteTx<'_>,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<RecordIterator> {
        let database_name = tx.database_name().clone();
        let input = FullScanUseCaseInput::new(&database_name, table_name, projection);
        let output = runtime().block_on(FullScanUseCase::<
            '_,
            ApllodbImmutableSchemaEngine,
            SqliteTypes,
        >::run(
            &tx.vtable_repo(), &tx.version_repo(), input
        ))?;

        Ok(RecordIterator::new(output.row_iter))
    }

    fn insert(
        &self,
        tx: &mut SqliteTx<'_>,
        table_name: &TableName,
        records: RecordIterator,
    ) -> ApllodbResult<()> {
        let database_name = tx.database_name().clone();
        let input = InsertUseCaseInput::new(&database_name, table_name, records);
        let _ = runtime().block_on(
            InsertUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
                &tx.vtable_repo(),
                &tx.version_repo(),
                input,
            ),
        )?;
        Ok(())
    }

    fn update(
        &self,
        tx: &mut SqliteTx<'_>,
        table_name: &TableName,
        column_values: std::collections::HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = tx.database_name().clone();
        let input = UpdateAllUseCaseInput::new(&database_name, table_name, column_values);
        let _ = runtime().block_on(UpdateAllUseCase::<
            '_,
            ApllodbImmutableSchemaEngine,
            SqliteTypes,
        >::run(&tx.vtable_repo(), &tx.version_repo(), input))?;

        Ok(())
    }

    fn delete(&self, tx: &mut SqliteTx<'_>, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = tx.database_name().clone();
        let input = DeleteAllUseCaseInput::new(&database_name, table_name);
        let _ = runtime().block_on(DeleteAllUseCase::<
            '_,
            ApllodbImmutableSchemaEngine,
            SqliteTypes,
        >::run(&tx.vtable_repo(), &tx.version_repo(), input))?;

        Ok(())
    }
}
