pub(crate) mod mock_tx_select;

use std::collections::HashMap;

use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, DatabaseName, Expression,
    RecordIterator, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, Transaction};

use super::test_storage_engine::{TestDatabase, TestStorageEngine, TestTransactionId};

use mockall::mock;

mock! {
    pub(crate) Tx {}

    impl std::fmt::Debug for Tx {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error>;
    }

    impl Transaction<TestStorageEngine> for Tx {
        type Db = TestDatabase;
        type TID = TestTransactionId;

        fn id(&self) -> &TestTransactionId;

        fn begin(db: TestDatabase) -> ApllodbResult<Self>;

        fn commit(self) -> ApllodbResult<()>;

        fn abort(self) -> ApllodbResult<()>;

        fn database_name(&self) -> &DatabaseName;

        fn create_table(
            &self,
            table_name: &TableName,
            table_constraints: &TableConstraints,
            column_definitions: Vec<ColumnDefinition>,
        ) -> ApllodbResult<()>;

        fn alter_table(
            &self,
            table_name: &TableName,
            action: &AlterTableAction,
        ) -> ApllodbResult<()>;

        fn drop_table(&self, _table_name: &TableName) -> ApllodbResult<()>;

        fn select(
            &self,
            table_name: &TableName,
            projection: ProjectionQuery,
        ) -> ApllodbResult<RecordIterator>;

        fn insert(
            &self,
            table_name: &TableName,
            records: RecordIterator,
        ) -> ApllodbResult<()>;

        fn update(
            &self,
            table_name: &TableName,
            column_values: HashMap<ColumnName, Expression>,
        ) -> ApllodbResult<()>;

        fn delete(&self, _table_name: &TableName) -> ApllodbResult<()>;
    }
}
