#![deny(warnings, missing_docs, missing_debug_implementations)]

//! apllodb's storage engine interface.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllodb-storage-engine-interface = "0.1"
//! ```
//!
//! # Boundary of Responsibility with Storage Engine
//!
//! A storage engine is an implementation of this interface crate.
//!
//! This crate provides:
//!
//! - Access Methods traits related to:
//!   - apllodb-DDL
//!   - apllodb-DML
//!   - Transaction
//!   - Getting catalog
//! - Traits of records and record iterators.
//! - Catalog data structure with read-only APIs.
//!
//! And a storage engine MUST provide:
//!
//! - Access Methods implementation.
//! - Implementation of records and record iterators.
//! - Ways to materialize tables and records.
//!
//! # Example: implementing a storage engine
//!
//! `EmptyStorageEngine` does no effective work but it just compiles and runs.
//!
//! ```rust
// example storage engine implementation.
//! pub mod empty_storage_engine {
//!     pub use db::EmptyDatabase;
//!     pub use engine::EmptyStorageEngine;
//!     pub use row::EmptyRowIterator;
//!     pub use tx::{EmptyTx, EmptyTxBuilder};
//!
//!     mod db {
//!         use apllodb_shared_components::Database;
//!
//!         pub struct EmptyDatabase;
//!         impl Database for EmptyDatabase {
//!             fn name(&self) -> &apllodb_shared_components::DatabaseName {
//!                 unimplemented!()
//!             }
//!         }
//!         impl EmptyDatabase {
//!             pub(super) fn new() -> Self {
//!                 Self
//!             }
//!         }
//!     }
//!
//!     mod row {
//!         use apllodb_shared_components::{
//!             ApllodbResult, ColumnName, ColumnReference, ColumnValue, SqlValue,
//!         };
//!         use apllodb_storage_engine_interface::{PrimaryKey, Row};
//!         use serde::{Deserialize, Serialize};
//!
//!         #[derive(
//!             Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
//!         )]
//!         pub struct EmptyPrimaryKey;
//!         impl PrimaryKey for EmptyPrimaryKey {
//!             fn get_sql_value(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
//!                 unimplemented!()
//!             }
//!         }
//!
//!         pub struct EmptyRow;
//!         impl Row for EmptyRow {
//!             fn get_sql_value(&mut self, colref: &ColumnReference) -> ApllodbResult<SqlValue> {
//!                 unimplemented!()
//!             }
//!
//!             fn append(&mut self, colvals: Vec<ColumnValue>) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!         }
//!
//!         #[derive(Debug)]
//!         pub struct EmptyRowIterator;
//!         impl Iterator for EmptyRowIterator {
//!             type Item = EmptyRow;
//!
//!             fn next(&mut self) -> Option<Self::Item> {
//!                 unimplemented!()
//!             }
//!         }
//!     }
//!
//!     mod tx {
//!         use apllodb_shared_components::{
//!             AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, DatabaseName,
//!             Expression, TableConstraints, TableName,
//!         };
//!         use apllodb_storage_engine_interface::{ProjectionQuery, Transaction, TransactionBuilder, TransactionId};
//!         use std::collections::HashMap;
//!
//!         use super::{EmptyRowIterator, EmptyStorageEngine};
//!
//!         #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//!         pub struct EmptyTxBuilder;
//!         impl TransactionBuilder for EmptyTxBuilder {}
//!
//!         #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//!         pub struct EmptyTransactionId;
//!         impl TransactionId for EmptyTransactionId {}
//!
//!         #[derive(Debug)]
//!         pub struct EmptyTx;
//!         impl Transaction<EmptyStorageEngine> for EmptyTx {
//!             fn id(&self) -> &EmptyTransactionId {
//!                 unimplemented!()
//!             }
//!
//!             fn begin(_builder: EmptyTxBuilder) -> ApllodbResult<Self> {
//!                 Ok(Self)
//!             }
//!
//!             fn commit(self) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!
//!             fn abort(self) -> ApllodbResult<()> {
//!                 Ok(())
//!             }
//!
//!             fn database_name(&self) -> &DatabaseName {
//!                 unimplemented!()
//!             }
//!
//!             fn create_table(
//!                 &self,
//!                 table_name: &TableName,
//!                 table_constraints: &TableConstraints,
//!                 column_definitions: &[ColumnDefinition],
//!             ) -> ApllodbResult<()> {
//!                 Ok(())
//!             }
//!
//!             fn alter_table(
//!                 &self,
//!                 table_name: &TableName,
//!                 action: &AlterTableAction,
//!             ) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!
//!             fn drop_table(&self, table_name: &TableName) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!
//!             fn select(
//!                 &self,
//!                 table_name: &TableName,
//!                 projection: ProjectionQuery,
//!             ) -> ApllodbResult<EmptyRowIterator> {
//!                 unimplemented!()
//!             }
//!
//!             fn insert(
//!                 &self,
//!                 table_name: &TableName,
//!                 column_values: HashMap<ColumnName, Expression>,
//!             ) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!
//!             fn update(
//!                 &self,
//!                 table_name: &TableName,
//!                 column_values: HashMap<ColumnName, Expression>,
//!             ) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!
//!             fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
//!                 unimplemented!()
//!             }
//!         }
//!     }
//!
//!     mod engine {
//!         use super::{
//!             row::EmptyRow, tx::EmptyTransactionId, EmptyDatabase, EmptyRowIterator, EmptyTx, EmptyTxBuilder,
//!         };
//!         use apllodb_shared_components::{ApllodbResult, DatabaseName};
//!         use apllodb_storage_engine_interface::StorageEngine;
//!
//!         #[derive(Debug)]
//!         pub struct EmptyStorageEngine;
//!         impl StorageEngine for EmptyStorageEngine {
//!             type Tx = EmptyTx;
//!             type TxBuilder = EmptyTxBuilder;
//!             type TID = EmptyTransactionId;
//!             type Db = EmptyDatabase;
//!             type R = EmptyRow;
//!             type RowIter = EmptyRowIterator;
//!
//!             fn use_database(database_name: &DatabaseName) -> ApllodbResult<EmptyDatabase> {
//!                 Ok(EmptyDatabase::new())
//!             }
//!         }
//!     }
//! }
//!
//! use apllodb_shared_components::ApllodbResult;
//!
//! fn main() -> ApllodbResult<()> {
//!     use apllodb_shared_components::{
//!         ColumnConstraints, ColumnDefinition, ColumnName, ColumnReference, DataType, DataTypeKind,
//!         DatabaseName, TableConstraintKind, TableConstraints, TableName,
//!     };
//!     use apllodb_storage_engine_interface::{StorageEngine, Transaction};
//!
//!     // `use` only `EmptyStorageEngine` from `empty_storage_engine`.
//!     // `EmptyDatabase` and `EmptyTx` are usable without `use`.
//!     use empty_storage_engine::{EmptyStorageEngine, EmptyTx, EmptyTxBuilder};
//!
//!     let mut db = EmptyStorageEngine::use_database(&DatabaseName::new("db")?)?;
//!     let builder = EmptyTxBuilder;
//!     let tx = EmptyTx::begin(builder)?;
//!
//!     let table_name = TableName::new("t")?;
//!
//!     let c1_def = ColumnDefinition::new(
//!         ColumnReference::new(table_name.clone(), ColumnName::new("c1")?),
//!         DataType::new(DataTypeKind::Integer, false),
//!         ColumnConstraints::default(),
//!     );
//!
//!     tx.create_table(
//!         &table_name,
//!         &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
//!             column_names: vec![c1_def.column_ref().as_column_name().clone()],
//!         }])?,
//!         &[],
//!     )?;
//!
//!     tx.abort()?;
//!
//!     Ok(())
//! }
//! ```

mod query;
mod row;
mod transaction;

use std::fmt::Debug;

pub use crate::query::projection::ProjectionQuery;
pub use crate::row::{pk::PrimaryKey, Row};
pub use crate::transaction::{transaction_id::TransactionId, Transaction, TransactionBuilder};

use apllodb_shared_components::{ApllodbResult, Database, DatabaseName};

/// An storage engine implementation must implement this trait and included associated-types.
pub trait StorageEngine: Sized + Debug {
    /// Transaction.
    type Tx: Transaction<Self>;

    /// Transaction builder to begin a Transaction.
    type TxBuilder: TransactionBuilder;

    /// Transaction ID.
    type TID: TransactionId;

    /// Database.
    type Db: Database;

    /// Row.
    type R: Row;

    /// Iterator of `Self::R`s returned from [select()](crate::Transaction::select) method.
    type RowIter: Iterator<Item = Self::R> + Debug;

    /// Specify database to use and return database object.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<Self::Db>;
}
