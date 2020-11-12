use apllodb_immutable_schema_engine_application::use_case::{
    transaction::{
        alter_table::{AlterTableUseCase, AlterTableUseCaseInput},
        create_table::{CreateTableUseCase, CreateTableUseCaseInput},
        delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
        full_scan::{FullScanUseCase, FullScanUseCaseInput},
        insert::{InsertUseCase, InsertUseCaseInput},
        update_all::{UpdateAllUseCase, UpdateAllUseCaseInput},
    },
    UseCase,
};
use apllodb_immutable_schema_engine_domain::transaction::ImmutableSchemaTransaction;
use apllodb_shared_components::{
    data_structure::{
        AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression, TableConstraints,
        TableName,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::{Transaction};
use std::{collections::HashMap};

use crate::{
    immutable_schema_row_iter::ImmutableSchemaRowIter,
    sqlite::{
        database::SqliteDatabase,
        sqlite_types::SqliteTypes,
        transaction::{sqlite_tx::SqliteTx, tx_id::TxId},
    },
};

use super::{ApllodbImmutableSchemaEngine};

#[derive(Debug, new)]
pub struct TransactionController<'db> {
    tx: SqliteTx<'db>,
}

impl<'db> Transaction<'db, ApllodbImmutableSchemaEngine> for TransactionController<'db> {
    fn id(&self) -> &TxId {
        self.tx.id()
    }

    fn begin(db: &'db mut SqliteDatabase) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let tx = SqliteTx::begin(db)?;
        Ok(Self::new(tx))
    }

    fn commit(self) -> ApllodbResult<()> {
        self.tx.commit()
    }
    fn abort(self) -> ApllodbResult<()> {
        self.tx.abort()
    }

    fn database_name(&self) -> &DatabaseName {
        self.tx.database_name()
    }

    fn create_table(
        &self,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: CreateTableUseCaseInput<'_, '_, '_, ApllodbImmutableSchemaEngine, SqliteTypes> =
            CreateTableUseCaseInput::new(
                &self.tx,
                &database_name,
                table_name,
                table_constraints,
                column_definitions,
            );
        let _ = CreateTableUseCase::run(input)?;

        Ok(())
    }

    fn alter_table(&self, table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: AlterTableUseCaseInput<'_, '_, '_, ApllodbImmutableSchemaEngine, SqliteTypes> =
            AlterTableUseCaseInput::new(&self.tx, &database_name, table_name, action);
        let _ = AlterTableUseCase::run(input)?;

        Ok(())
    }

    fn drop_table(&self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn select(
        &self,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let database_name = self.database_name().clone();
        let input: FullScanUseCaseInput<'_, '_, '_, ApllodbImmutableSchemaEngine, SqliteTypes> =
            FullScanUseCaseInput::new(&self.tx, &database_name, table_name, &column_names);
        let output = FullScanUseCase::run(input)?;

        Ok(output.row_iter)
    }

    fn insert(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: InsertUseCaseInput<'_, '_, '_, ApllodbImmutableSchemaEngine, SqliteTypes> =
            InsertUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = InsertUseCase::run(input)?;

        Ok(())
    }

    fn update(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: UpdateAllUseCaseInput<'_, '_, '_, ApllodbImmutableSchemaEngine, SqliteTypes> =
            UpdateAllUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = UpdateAllUseCase::run(input)?;

        Ok(())
    }

    fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: DeleteAllUseCaseInput<'_, '_, '_, ApllodbImmutableSchemaEngine, SqliteTypes> =
            DeleteAllUseCaseInput::new(&self.tx, &database_name, table_name);
        let _ = DeleteAllUseCase::run(input)?;

        Ok(())
    }
}
