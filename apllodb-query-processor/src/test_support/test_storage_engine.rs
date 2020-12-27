pub(crate) use engine::TestStorageEngine;

mod db {
    use apllodb_shared_components::Database;

    pub(crate) struct TestDatabase;
    impl Database for TestDatabase {
        fn name(&self) -> &apllodb_shared_components::DatabaseName {
            unimplemented!()
        }
    }
    impl TestDatabase {
        pub(super) fn new() -> Self {
            Self
        }
    }
}

mod row {
    use std::collections::VecDeque;

    use apllodb_shared_components::{
        ApllodbResult, ColumnName, ColumnReference, ColumnValue, Record, SqlValue,
    };
    use apllodb_storage_engine_interface::{PrimaryKey, Row};
    use serde::{Deserialize, Serialize};

    #[derive(
        Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
    )]
    pub(crate) struct TestPrimaryKey;
    impl PrimaryKey for TestPrimaryKey {
        fn get_sql_value(&self, _column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
            unimplemented!()
        }
    }

    #[derive(Clone, Eq, PartialEq, Debug, new)]
    pub(crate) struct TestRow(Record);
    impl Row for TestRow {
        fn get_sql_value(&mut self, _colref: &ColumnReference) -> ApllodbResult<SqlValue> {
            unimplemented!()
        }

        fn append(&mut self, _colvals: Vec<ColumnValue>) -> ApllodbResult<()> {
            unimplemented!()
        }
    }
    impl Into<Record> for TestRow {
        fn into(self) -> Record {
            self.0
        }
    }

    #[derive(Debug, new)]
    pub(crate) struct TestRowIterator(VecDeque<TestRow>);
    impl Iterator for TestRowIterator {
        type Item = TestRow;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop_front()
        }
    }
}

mod tx {
    use apllodb_shared_components::{
        AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, DataType, DataTypeKind,
        DatabaseName, Expression, Record, SqlValue, TableConstraints, TableName,
    };
    use apllodb_storage_engine_interface::{
        ProjectionQuery, Transaction, TransactionBuilder, TransactionId,
    };
    use std::collections::HashMap;

    use crate::record;

    use super::{
        row::{TestRow, TestRowIterator},
        TestStorageEngine,
    };

    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub(crate) struct TestTxBuilder;
    impl TransactionBuilder for TestTxBuilder {}

    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub(crate) struct TestTransactionId;
    impl TransactionId for TestTransactionId {}

    #[derive(Debug)]
    pub(crate) struct TestTx;
    impl Transaction<TestStorageEngine> for TestTx {
        fn id(&self) -> &TestTransactionId {
            unimplemented!()
        }

        fn begin(_builder: TestTxBuilder) -> ApllodbResult<Self> {
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
            _table_name: &TableName,
            _table_constraints: &TableConstraints,
            _column_definitions: &[ColumnDefinition],
        ) -> ApllodbResult<()> {
            Ok(())
        }

        fn alter_table(
            &self,
            _table_name: &TableName,
            _action: &AlterTableAction,
        ) -> ApllodbResult<()> {
            unimplemented!()
        }

        fn drop_table(&self, _table_name: &TableName) -> ApllodbResult<()> {
            unimplemented!()
        }

        fn select(
            &self,
            _table_name: &TableName,
            _projection: ProjectionQuery,
        ) -> ApllodbResult<TestRowIterator> {
            let testset: Vec<Record> = vec![
                record! {
                    "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
                    "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &13i32)?
                },
                record! {
                    "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &2i32)?,
                    "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &70i32)?
                },
                record! {
                    "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
                    "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &35i32)?
                },
            ];

            Ok(TestRowIterator::new(
                testset.into_iter().map(TestRow::new).collect(),
            ))
        }

        fn insert(
            &self,
            _table_name: &TableName,
            _column_values: HashMap<ColumnName, Expression>,
        ) -> ApllodbResult<()> {
            unimplemented!()
        }

        fn update(
            &self,
            _table_name: &TableName,
            _column_values: HashMap<ColumnName, Expression>,
        ) -> ApllodbResult<()> {
            unimplemented!()
        }

        fn delete(&self, _table_name: &TableName) -> ApllodbResult<()> {
            unimplemented!()
        }
    }
}

mod engine {

    use super::{
        db::TestDatabase,
        row::{TestRow, TestRowIterator},
        tx::{TestTransactionId, TestTx, TestTxBuilder},
    };
    use apllodb_shared_components::{ApllodbResult, DatabaseName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    #[derive(Debug)]
    pub(crate) struct TestStorageEngine;
    impl StorageEngine for TestStorageEngine {
        type Tx = TestTx;
        type TxBuilder = TestTxBuilder;
        type TID = TestTransactionId;
        type Db = TestDatabase;
        type R = TestRow;
        type RowIter = TestRowIterator;

        fn use_database(_database_name: &DatabaseName) -> ApllodbResult<TestDatabase> {
            Ok(TestDatabase::new())
        }
    }
    impl TestStorageEngine {
        pub(crate) fn begin() -> ApllodbResult<TestTx> {
            TestTx::begin(TestTxBuilder)
        }
    }
}
