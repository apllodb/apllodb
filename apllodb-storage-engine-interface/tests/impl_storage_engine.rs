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
        use apllodb_shared_components::{
            data_structure::ColumnReference,
            data_structure::{ColumnName, SqlValue},
            error::ApllodbResult,
        };
        use apllodb_storage_engine_interface::{PrimaryKey, Row};
        use serde::{Deserialize, Serialize};

        #[derive(
            Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
        )]
        pub struct EmptyPrimaryKey;
        impl PrimaryKey for EmptyPrimaryKey {
            fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
                unimplemented!()
            }
        }

        pub struct EmptyRow;
        impl Row for EmptyRow {
            fn get_sql_value(&mut self, colref: &ColumnReference) -> ApllodbResult<SqlValue> {
                unimplemented!()
            }

            fn append(
                &mut self,
                colvals: Vec<apllodb_shared_components::data_structure::ColumnValue>,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }
        }

        #[derive(Debug)]
        pub struct EmptyRowIterator;
        impl Iterator for EmptyRowIterator {
            type Item = EmptyRow;

            fn next(&mut self) -> Option<Self::Item> {
                unimplemented!()
            }
        }
    }

    mod tx {
        use apllodb_shared_components::{
            data_structure::{
                AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression,
                TableConstraints, TableName,
            },
            error::ApllodbResult,
        };
        use apllodb_storage_engine_interface::{Transaction, TransactionId};
        use std::collections::HashMap;

        use super::{EmptyDatabase, EmptyRowIterator, EmptyStorageEngine};

        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct EmptyTransactionId;
        impl TransactionId for EmptyTransactionId {}

        #[derive(Debug)]
        pub struct EmptyTx;
        impl<'tx, 'db: 'tx> Transaction<'tx, 'db, EmptyStorageEngine> for EmptyTx {
            fn id(&self) -> &EmptyTransactionId {
                unimplemented!()
            }

            fn begin(db: &'db mut EmptyDatabase) -> ApllodbResult<Self> {
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
            ) -> ApllodbResult<EmptyRowIterator> {
                unimplemented!()
            }

            fn insert(
                &self,
                table_name: &TableName,
                column_values: HashMap<ColumnName, Expression>,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn update(
                &self,
                table_name: &TableName,
                column_values: HashMap<ColumnName, Expression>,
            ) -> ApllodbResult<()> {
                unimplemented!()
            }

            fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
                unimplemented!()
            }
        }
    }

    mod engine {
        use super::{
            row::EmptyRow, tx::EmptyTransactionId, EmptyDatabase, EmptyRowIterator, EmptyTx,
        };
        use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
        use apllodb_storage_engine_interface::StorageEngine;

        #[derive(Debug)]
        pub struct EmptyStorageEngine;
        impl<'tx, 'db: 'tx> StorageEngine<'tx, 'db> for EmptyStorageEngine {
            type Tx = EmptyTx;
            type TID = EmptyTransactionId;
            type Db = EmptyDatabase;
            type R = EmptyRow;
            type RowIter = EmptyRowIterator;

            fn use_database(database_name: &DatabaseName) -> ApllodbResult<EmptyDatabase> {
                Ok(EmptyDatabase::new())
            }
        }
    }
}

use apllodb_shared_components::{
    data_structure::ColumnReference,
    data_structure::{
        ColumnConstraints, ColumnDefinition, ColumnName, DataType, DataTypeKind,
        TableConstraintKind,
    },
    error::ApllodbResult,
};
use empty_storage_engine::{EmptyDatabase, EmptyTx};

#[test]
fn test_empty_storage_engine() -> ApllodbResult<()> {
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    // `use` only `EmptyStorageEngine` from `empty_storage_engine`.
    // `EmptyDatabase` and `EmptyTx` are usable without `use`.
    use empty_storage_engine::EmptyStorageEngine;

    let mut db: EmptyDatabase = EmptyStorageEngine::use_database(&DatabaseName::new("db")?)?;
    let tx: EmptyTx = EmptyStorageEngine::begin_transaction(&mut db)?;

    let c1_def = ColumnDefinition::new(
        ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::default(),
    )?;

    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_names: vec![c1_def.column_ref().as_column_name().clone()],
        }])?,
        &vec![],
    )?;

    tx.abort()?;

    Ok(())
}
