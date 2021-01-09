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
//! // example storage engine implementation.
//!
//! # use std::collections::HashMap;
//! # use apllodb_shared_components::{
//! #     AlterTableAction, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName, ColumnReference, DatabaseName,
//! #     Expression, RecordIterator, SqlType, TableConstraintKind, TableConstraints, TableName,
//! # };
//! use apllodb_storage_engine_interface::{Database, ProjectionQuery, StorageEngine, Transaction, TransactionId};
//!
//! struct EmptyDatabase;
//! impl Database for EmptyDatabase {
//!     fn name(&self) -> &apllodb_shared_components::DatabaseName {
//!         unimplemented!()
//!     }
//! }
//!
//! #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//! struct EmptyTransactionId;
//! impl TransactionId for EmptyTransactionId {}
//!
//! #[derive(Debug)]
//! struct EmptyTx;
//! impl Transaction<EmptyStorageEngine> for EmptyTx {
//!     type Db = EmptyDatabase;
//!     type TID = EmptyTransactionId;
//!
//!     fn id(&self) -> &EmptyTransactionId {
//!         unimplemented!()
//!     }
//!
//!     fn begin(db: EmptyDatabase) -> ApllodbResult<Self> {
//!         Ok(Self)
//!     }
//!
//!     fn commit(self) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//!
//!     fn abort(self) -> ApllodbResult<()> {
//!         Ok(())
//!     }
//!
//!     fn database_name(&self) -> &DatabaseName {
//!         unimplemented!()
//!     }
//!
//!     fn create_table(
//!         &self,
//!         table_name: &TableName,
//!         table_constraints: &TableConstraints,
//!         column_definitions: Vec<ColumnDefinition>,
//!     ) -> ApllodbResult<()> {
//!         Ok(())
//!     }
//!
//!     fn alter_table(
//!         &self,
//!         table_name: &TableName,
//!         action: &AlterTableAction,
//!     ) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//!
//!     fn drop_table(&self, table_name: &TableName) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//!
//!     fn select(
//!         &self,
//!         table_name: &TableName,
//!         projection: ProjectionQuery,
//!     ) -> ApllodbResult<RecordIterator> {
//!         unimplemented!()
//!     }
//!
//!     fn insert(
//!         &self,
//!         table_name: &TableName,
//!         records: RecordIterator,
//!     ) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//!
//!     fn update(
//!         &self,
//!         table_name: &TableName,
//!         column_values: HashMap<ColumnName, Expression>,
//!     ) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//!
//!     fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//! }
//!
//! #[derive(Debug)]
//! struct EmptyStorageEngine;
//! impl StorageEngine for EmptyStorageEngine {
//!     type Db = EmptyDatabase;
//!     type Tx = EmptyTx;
//!
//!     fn use_database(database_name: &DatabaseName) -> ApllodbResult<EmptyDatabase> {
//!         Ok(EmptyDatabase)
//!     }
//! }
//!
//! fn main() -> ApllodbResult<()> {
//!     let db = EmptyStorageEngine::use_database(&DatabaseName::new("db")?)?;
//!     let tx = EmptyTx::begin(db)?;
//!
//!     let table_name = TableName::new("t")?;
//!
//!     let c1_def = ColumnDefinition::new(
//!         ColumnDataType::new(
//!             ColumnReference::new(table_name.clone(), ColumnName::new("c1")?),
//!             SqlType::integer(),
//!             false
//!         ),
//!         ColumnConstraints::default(),
//!     );
//!
//!     ApllodbImmutableSchemaDDL::create_table(tx.create_table(mut tx
//!         &table_name,
//!         &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
//!             column_names: vec![c1_def.column_data_type().column_ref().as_column_name().clone()],
//!         }])?,
//!         vec![],
//!     )?;
//!
//!     tx.abort()?;
//!
//!     Ok(())
//! }
//! ```

mod ddl_methods;
mod dml_methods;

pub use crate::{
    ddl_methods::DDLMethods,
    dml_methods::{projection::ProjectionQuery, DMLMethods},
};

use apllodb_shared_components::{Database, Transaction};
use std::fmt::Debug;

/// An storage engine implementation must implement this trait and included associated-types.
pub trait StorageEngine: Debug + Sized {
    /// Database.
    type Db: Database;

    /// Transaction.
    type Tx: Transaction;

    /// DDL access methods.
    type DDL: DDLMethods<Self>;

    /// DML access methods.
    type DML: DMLMethods<Self>;
}
