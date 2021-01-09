use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnDefinition, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::DDLMethods;

use super::test_storage_engine::{TestStorageEngine, TestTx};

use mockall::mock;

mock! {
    pub(crate) DDL {}

    impl std::fmt::Debug for DDL {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error>;
    }

    impl DDLMethods<TestStorageEngine> for DDL {
        fn create_table(
            &self,
            tx: &mut TestTx,
            table_name: &TableName,
            table_constraints: &TableConstraints,
            column_definitions: Vec<ColumnDefinition>,
        ) -> ApllodbResult<()>;

        fn alter_table(
            &self,
            tx: &mut TestTx,
            table_name: &TableName,
            action: &AlterTableAction,
        ) -> ApllodbResult<()>;

        fn drop_table(&self, tx: &mut TestTx, table_name: &TableName) -> ApllodbResult<()>;
    }
}
