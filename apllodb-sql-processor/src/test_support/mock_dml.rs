pub(crate) mod mock_tx_select;

use std::collections::HashMap;

use apllodb_shared_components::{ApllodbResult, ColumnName, Expression, RecordIterator, TableName};
use apllodb_storage_engine_interface::{DMLMethods, ProjectionQuery};

use super::test_storage_engine::{TestStorageEngine, TestTx};

use mockall::mock;

mock! {
    pub(crate) DML {}

    impl std::fmt::Debug for DML {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error>;
    }

    impl DMLMethods<TestStorageEngine> for DML {
        fn select(
            &self,
            tx: &mut TestTx,
            table_name: &TableName,
            projection: ProjectionQuery,
        ) -> ApllodbResult<RecordIterator>;

        fn insert(
            &self,
            tx: &mut TestTx,
            table_name: &TableName,
            records: RecordIterator,
        ) -> ApllodbResult<()>;

        fn update(
            &self,
            tx: &mut TestTx,
            table_name: &TableName,
            column_values: HashMap<ColumnName, Expression>,
        ) -> ApllodbResult<()>;

        fn delete(&self, tx: &mut TestTx, table_name: &TableName) -> ApllodbResult<()>;
    }
}
