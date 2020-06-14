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
                AlterTableAction, ColumnDefinition, ColumnName, Expression, TableConstraints,
                TableName,
            },
            error::ApllodbResult,
        };
        use apllodb_storage_engine_interface::Transaction;
        use std::collections::HashMap;

        pub struct EmptyTx;
        impl<'tx> Transaction<'tx> for EmptyTx {
            type Db = EmptyDatabase;
            type RowIter = EmptyRowIterator;

            fn begin(db: &'tx mut Self::Db) -> ApllodbResult<Self> {
                Ok(Self)
            }

            fn commit(self) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn abort(self) -> ApllodbResult<()> {
                Ok(())
            }

            fn database(&'tx self) -> &Self::Db {
                unimplemented!()
            }

            fn create_table(
                &'tx mut self,
                table_name: &TableName,
                table_constraints: &TableConstraints,
                column_definitions: &[ColumnDefinition],
            ) -> ApllodbResult<()> {
                Ok(())
            }

            fn alter_table(
                &'tx mut self,
                table_name: &TableName,
                action: &AlterTableAction,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn drop_table(&'tx mut self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn select(
                &'tx mut self,
                table_name: &TableName,
                column_names: &[ColumnName],
            ) -> ApllodbResult<Self::RowIter> {
                unimplemented!()
            }

            fn insert(
                &'tx mut self,
                table_name: &TableName,
                column_values: HashMap<ColumnName, Expression>,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn update(&'tx mut self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn delete(&'tx mut self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }
        }
    }

    mod engine {
        use super::{EmptyDatabase, EmptyTx};
        use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
        use apllodb_storage_engine_interface::StorageEngine;

        pub struct EmptyStorageEngine;
        impl<'tx> StorageEngine<'tx> for EmptyStorageEngine {
            type Tx = EmptyTx;

            fn use_database(database_name: &DatabaseName) -> ApllodbResult<EmptyDatabase> {
                Ok(EmptyDatabase::new())
            }

            fn begin_transaction(db: &mut EmptyDatabase) -> ApllodbResult<Self::Tx> {
                use apllodb_storage_engine_interface::Transaction;

                Self::Tx::begin(db)
            }
        }
    }
}

use apllodb_shared_components::error::ApllodbResult;

#[test]
fn test_empty_storage_engine() -> ApllodbResult<()> {
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    // `use` only `EmptyStorageEngine` from `empty_storage_engine`.
    // `EmptyDatabase` and `EmptyTx` are usable without `use`.
    use empty_storage_engine::EmptyStorageEngine;

    let mut db = EmptyStorageEngine::use_database(&DatabaseName::new("db")?)?;
    let mut tx = EmptyStorageEngine::begin_transaction(&mut db)?;
    tx.create_table(&TableName::new("t")?, &TableConstraints::default(), &vec![])?;
    tx.abort()?;

    Ok(())
}
