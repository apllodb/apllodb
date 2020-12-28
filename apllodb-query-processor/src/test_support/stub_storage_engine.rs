pub(crate) mod stub_data;

pub(crate) use engine::StubStorageEngine;
pub(crate) use row::StubRowIterator;
pub(crate) use stub_data::StubData;

mod db {
    use apllodb_shared_components::Database;

    pub(crate) struct StubDatabase;
    impl Database for StubDatabase {
        fn name(&self) -> &apllodb_shared_components::DatabaseName {
            unimplemented!()
        }
    }
    impl StubDatabase {
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
    pub(crate) struct StubPrimaryKey;
    impl PrimaryKey for StubPrimaryKey {
        fn get_sql_value(&self, _column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
            unimplemented!()
        }
    }

    #[derive(Clone, Eq, PartialEq, Debug, new)]
    pub(crate) struct StubRow(pub(super) Record);
    impl Row for StubRow {
        fn get_sql_value(&mut self, _colref: &ColumnReference) -> ApllodbResult<SqlValue> {
            unimplemented!()
        }

        fn append(&mut self, _colvals: Vec<ColumnValue>) -> ApllodbResult<()> {
            unimplemented!()
        }
    }
    impl Into<Record> for StubRow {
        fn into(self) -> Record {
            self.0
        }
    }

    #[derive(Clone, Eq, PartialEq, Debug, new)]
    pub(crate) struct StubRowIterator(VecDeque<StubRow>);
    impl Iterator for StubRowIterator {
        type Item = StubRow;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop_front()
        }
    }
    impl From<Vec<Record>> for StubRowIterator {
        fn from(records: Vec<Record>) -> Self {
            Self(records.into_iter().map(StubRow::new).collect())
        }
    }
}

mod tx {
    use apllodb_shared_components::{
        AlterTableAction, ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnDefinition,
        ColumnName, ColumnReference, DatabaseName, Expression, FieldIndex, TableConstraints,
        TableName,
    };
    use apllodb_storage_engine_interface::{
        ProjectionQuery, Transaction, TransactionBuilder, TransactionId,
    };
    use std::collections::{HashMap, HashSet, VecDeque};

    use super::{
        row::{StubRow, StubRowIterator},
        StubData, StubStorageEngine,
    };

    #[derive(Clone, Eq, PartialEq, Debug, new)]
    pub(crate) struct StubTxBuilder {
        stub_data: StubData,
    }
    impl TransactionBuilder for StubTxBuilder {}

    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub(crate) struct StubTransactionId;
    impl TransactionId for StubTransactionId {}

    #[derive(Debug, new)]
    pub(crate) struct StubTx {
        stub_data: StubData,
    }
    impl Transaction<StubStorageEngine> for StubTx {
        fn id(&self) -> &StubTransactionId {
            unimplemented!()
        }

        fn begin(builder: StubTxBuilder) -> ApllodbResult<Self> {
            Ok(Self::new(builder.stub_data))
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
            table_name: &TableName,
            projection: ProjectionQuery,
        ) -> ApllodbResult<StubRowIterator> {
            let stub_table = self
                .stub_data
                .tables
                .iter()
                .find(|stub_table| stub_table.table_name == *table_name)
                .ok_or_else(|| {
                    ApllodbError::new(
                        ApllodbErrorKind::UndefinedTable,
                        format!("table `{:?}` is undefined in StubTx", table_name),
                        None,
                    )
                })?;

            let row_iter = stub_table.rows.clone();

            match projection {
                ProjectionQuery::All => Ok(row_iter),
                ProjectionQuery::ColumnNames(column_names) => {
                    let fields: HashSet<FieldIndex> = column_names
                        .into_iter()
                        .map(|cn| FieldIndex::from(ColumnReference::new(table_name.clone(), cn)))
                        .collect();

                    let projected_rows: VecDeque<StubRow> = row_iter
                        .map(|row| {
                            let record = row.0.projection(&fields)?;
                            Ok(StubRow::new(record))
                        })
                        .collect::<ApllodbResult<_>>()?;

                    Ok(StubRowIterator::new(projected_rows))
                }
            }
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
        db::StubDatabase,
        row::{StubRow, StubRowIterator},
        tx::{StubTransactionId, StubTx, StubTxBuilder},
        StubData,
    };
    use apllodb_shared_components::{ApllodbResult, DatabaseName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    #[derive(Debug)]
    pub(crate) struct StubStorageEngine;
    impl StorageEngine for StubStorageEngine {
        type Tx = StubTx;
        type TxBuilder = StubTxBuilder;
        type TID = StubTransactionId;
        type Db = StubDatabase;
        type R = StubRow;
        type RowIter = StubRowIterator;

        fn use_database(_database_name: &DatabaseName) -> ApllodbResult<StubDatabase> {
            Ok(StubDatabase::new())
        }
    }
    impl StubStorageEngine {
        pub(crate) fn begin_stub_tx(stub: StubData) -> ApllodbResult<StubTx> {
            StubTx::begin(StubTxBuilder::new(stub))
        }
    }
}
