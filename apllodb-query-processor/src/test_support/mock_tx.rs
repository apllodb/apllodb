use std::collections::HashMap;

use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, DatabaseName, Expression,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, Transaction};

use super::stub_storage_engine::{
    StubRowIterator, StubStorageEngine, StubTransactionId, StubTxBuilder,
};

use mockall::mock;

mock! {
    pub(crate) Tx {}

    impl std::fmt::Debug for Tx {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error>;
    }

    impl Transaction<StubStorageEngine> for Tx {
        fn id(&self) -> &StubTransactionId;

        fn begin(_builder: StubTxBuilder) -> ApllodbResult<Self>;

        fn commit(self) -> ApllodbResult<()>;

        fn abort(self) -> ApllodbResult<()>;

        fn database_name(&self) -> &DatabaseName;

        fn create_table(
            &self,
            table_name: &TableName,
            table_constraints: &TableConstraints,
            column_definitions: &[ColumnDefinition],
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
        ) -> ApllodbResult<StubRowIterator>;

        fn insert(
            &self,
            table_name: &TableName,
            column_values: HashMap<ColumnName, Expression>,
        ) -> ApllodbResult<()>;

        fn update(
            &self,
            table_name: &TableName,
            column_values: HashMap<ColumnName, Expression>,
        ) -> ApllodbResult<()>;

        fn delete(&self, _table_name: &TableName) -> ApllodbResult<()>;
    }
}
