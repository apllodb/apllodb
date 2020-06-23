// example storage engine implementation.
#[allow(unused_variables)]
pub mod empty_storage_engine {
    pub use db::EmptyDatabase;
    pub use engine::EmptyStorageEngine;
    pub use row::EmptyRowIterator;
    pub use tx::EmptyTx;

    mod db {
        use apllodb_shared_components::traits::Database;

        pub struct EmptyDatabase;
        impl Database for EmptyDatabase {
            fn name(&self) -> &apllodb_shared_components::data_structure::DatabaseName {
                unimplemented!()
            }
        }
        impl EmptyDatabase {
            pub(super) fn new() -> Self {
                Self
            }
        }
    }

    mod row {
        use apllodb_shared_components::error::ApllodbResult;
        use apllodb_storage_engine_interface::Row;

        pub struct EmptyRowIterator;
        impl Iterator for EmptyRowIterator {
            type Item = ApllodbResult<Row>;

            fn next(&mut self) -> Option<Self::Item> {
                unimplemented!()
            }
        }
    }

    mod tx {
        use super::{EmptyDatabase, EmptyRowIterator};
        use apllodb_shared_components::{
            data_structure::{
                AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression,
                TableConstraints, TableName,
            },
            error::ApllodbResult,
        };
        use apllodb_storage_engine_interface::{Transaction, TransactionId};
        use std::collections::HashMap;

        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct EmptyTransactionId;
        impl TransactionId for EmptyTransactionId {}

        pub struct EmptyTx;
        impl<'tx, 'db: 'tx> Transaction<'tx, 'db> for EmptyTx {
            type TID = EmptyTransactionId;
            type Db = EmptyDatabase;
            type RowIter = EmptyRowIterator;

            fn id(&self) -> &Self::TID {
                unimplemented!()
            }

            fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self> {
                Ok(Self)
            }

            fn commit(self) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn abort(self) -> ApllodbResult<()> {
                Ok(())
            }

            fn database_name(&self) -> &DatabaseName {
                unimplemented!()
            }

            fn create_table(
                &self,
                table_name: &TableName,
                table_constraints: &TableConstraints,
                column_definitions: &[ColumnDefinition],
            ) -> ApllodbResult<()> {
                Ok(())
            }

            fn alter_table(
                &self,
                table_name: &TableName,
                action: &AlterTableAction,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn drop_table(&self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn select(
                &self,
                table_name: &TableName,
                column_names: &[ColumnName],
            ) -> ApllodbResult<Self::RowIter> {
                unimplemented!()
            }

            fn insert(
                &self,
                table_name: &TableName,
                column_values: HashMap<ColumnName, Expression>,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn update(&self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }
        }
    }

    mod engine {
        use super::{EmptyDatabase, EmptyTx};
        use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
        use apllodb_storage_engine_interface::StorageEngine;

        pub struct EmptyStorageEngine;
        impl<'tx, 'db: 'tx> StorageEngine<'tx, 'db> for EmptyStorageEngine {
            type Tx = EmptyTx;

            fn use_database(database_name: &DatabaseName) -> ApllodbResult<EmptyDatabase> {
                Ok(EmptyDatabase::new())
            }
        }
    }
}

use apllodb_shared_components::{
    data_structure::{ColumnName, TableConstraintKind},
    error::ApllodbResult,
};

#[test]
fn test_empty_storage_engine() -> ApllodbResult<()> {
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    // `use` only `EmptyStorageEngine` from `empty_storage_engine`.
    // `EmptyDatabase` and `EmptyTx` are usable without `use`.
    use empty_storage_engine::EmptyStorageEngine;

    let mut db = EmptyStorageEngine::use_database(&DatabaseName::new("db")?)?;
    let tx = EmptyStorageEngine::begin_transaction(&mut db)?;
    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_names: vec![ColumnName::new("c1")?],
        }])?,
        &vec![],
    )?;
    tx.abort()?;

    Ok(())
}
