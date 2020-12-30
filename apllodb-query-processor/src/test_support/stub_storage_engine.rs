use std::collections::VecDeque;

use super::mock_tx::MockTx;
use apllodb_shared_components::{
    ApllodbResult, ColumnName, ColumnReference, ColumnValue, Database, DatabaseName, Record,
    SqlValue,
};
use apllodb_storage_engine_interface::{
    PrimaryKey, Row, StorageEngine, Transaction, TransactionBuilder, TransactionId,
};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct StubPrimaryKey;
impl PrimaryKey for StubPrimaryKey {
    fn get_sql_value(&self, _column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
        unimplemented!()
    }
}

#[derive(Clone, PartialEq, Debug, new)]
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

#[derive(Clone, PartialEq, Debug, new)]
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

#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct StubTxBuilder;
impl TransactionBuilder for StubTxBuilder {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct StubTransactionId;
impl TransactionId for StubTransactionId {}

#[derive(Debug)]
pub(crate) struct StubStorageEngine;
impl StorageEngine for StubStorageEngine {
    type Tx = MockTx;
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
    pub(crate) fn begin() -> ApllodbResult<MockTx> {
        let ctx = MockTx::begin_context();
        ctx.expect().returning(|_| Ok(MockTx::new()));

        MockTx::begin(StubTxBuilder::new())
    }
}
