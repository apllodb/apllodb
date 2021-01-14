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
//! # Examples
//!
//! `MyStorageEngine` does no effective work but it just compiles and runs.
//!
//! ```
//! // example storage engine implementation.
//!
//! use apllodb_shared_components::{
//!     AlterTableAction, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
//!     ColumnName, ColumnReference, Database, DatabaseName, Expression, Record, RecordIterator,
//!     SqlType, TableConstraintKind, TableConstraints, TableName, Transaction, TransactionId,
//! };
//! use apllodb_storage_engine_interface::{DDLMethods, DMLMethods, ProjectionQuery, StorageEngine};
//! use std::collections::HashMap;
//!
//! #[derive(Debug)]
//! pub struct MyDatabase;
//! impl Database for MyDatabase {
//!     fn use_database(name: DatabaseName) -> ApllodbResult<Self> {
//!         Ok(Self)
//!     }
//!
//!     fn name(&self) -> &apllodb_shared_components::DatabaseName {
//!         unimplemented!()
//!     }
//! }
//!
//! #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//! pub struct MyTransactionId;
//! impl TransactionId for MyTransactionId {}
//!
//! #[derive(Debug)]
//! pub struct MyTx;
//! impl Transaction for MyTx {
//!     type Db = MyDatabase;
//!     type RefDb = MyDatabase;
//!     type TID = MyTransactionId;
//!
//!     fn id(&self) -> &MyTransactionId {
//!         unimplemented!()
//!     }
//!
//!     fn begin(_db: MyDatabase) -> ApllodbResult<Self> {
//!         Ok(Self)
//!     }
//!
//!     fn commit(self) -> ApllodbResult<()> {
//!         Ok(())
//!     }
//!
//!     fn abort(self) -> ApllodbResult<()> {
//!         Ok(())
//!     }
//!
//!     fn database_name(&self) -> &DatabaseName {
//!         unimplemented!()
//!     }
//! }
//!
//! #[derive(Debug)]
//! pub struct MyDDL;
//! impl DDLMethods<MyStorageEngine> for MyDDL {
//!     fn create_table(
//!         &self,
//!         tx: &mut MyTx,
//!         table_name: &TableName,
//!         table_constraints: &TableConstraints,
//!         column_definitions: Vec<ColumnDefinition>,
//!     ) -> ApllodbResult<()> {
//!         Ok(())
//!     }
//!
//!     fn alter_table(
//!         &self,
//!         tx: &mut MyTx,
//!         table_name: &TableName,
//!         action: &AlterTableAction,
//!     ) -> ApllodbResult<()> {
//!         todo!()
//!     }
//!
//!     fn drop_table(&self, tx: &mut MyTx, table_name: &TableName) -> ApllodbResult<()> {
//!         todo!()
//!     }
//! }
//!
//! #[derive(Debug)]
//! pub struct MyDML;
//! impl DMLMethods<MyStorageEngine> for MyDML {
//!     fn select(
//!         &self,
//!         tx: &mut MyTx,
//!         table_name: &TableName,
//!         projection: ProjectionQuery,
//!     ) -> ApllodbResult<RecordIterator> {
//!         Ok(RecordIterator::new(Vec::<Record>::new()))
//!     }
//!
//!     fn insert(
//!         &self,
//!         tx: &mut MyTx,
//!         table_name: &TableName,
//!         records: RecordIterator,
//!     ) -> ApllodbResult<()> {
//!         Ok(())
//!     }
//!
//!     fn update(
//!         &self,
//!         tx: &mut MyTx,
//!         table_name: &TableName,
//!         column_values: HashMap<ColumnName, Expression>,
//!     ) -> ApllodbResult<()> {
//!         todo!()
//!     }
//!
//!     fn delete(&self, tx: &mut MyTx, table_name: &TableName) -> ApllodbResult<()> {
//!         todo!()
//!     }
//! }
//!
//! #[derive(Debug)]
//! pub struct MyStorageEngine;
//! impl StorageEngine for MyStorageEngine {
//!     type Db = MyDatabase;
//!     type Tx = MyTx;
//!     type DDL = MyDDL;
//!     type DML = MyDML;
//! }
//!
//! fn main() -> ApllodbResult<()> {
//!     let ddl = MyDDL;
//!
//!     let db = MyDatabase::use_database(DatabaseName::new("db")?)?;
//!     let mut tx = MyTx::begin(db)?;
//!
//!     let table_name = TableName::new("t")?;
//!
//!     let c1_def = ColumnDefinition::new(
//!         ColumnDataType::new(
//!             ColumnReference::new(table_name.clone(), ColumnName::new("c1")?),
//!             SqlType::integer(),
//!             false,
//!         ),
//!         ColumnConstraints::default(),
//!     );
//!
//!     ddl.create_table(
//!         &mut tx,
//!         &table_name,
//!         &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
//!             column_names: vec![c1_def
//!                 .column_data_type()
//!                 .column_ref()
//!                 .as_column_name()
//!                 .clone()],
//!         }])?,
//!         vec![],
//!     )?;
//!
//!     tx.abort()?;
//!
//!     Ok(())
//! }
//! ```

pub(crate) mod access_methods;

use apllodb_shared_components::{SessionWithDb, SessionWithTx, SessionWithoutDb};

pub use crate::access_methods::{
    methods_with_db::MethodsWithDb,
    methods_with_tx::{projection::ProjectionQuery, MethodsWithTx},
    methods_without_db::MethodsWithoutDb,
};

use std::fmt::Debug;

/// An storage engine implementation must implement this trait and included associated-types.
///
/// # Lifetime parameters
///
/// - `'sess`: shorthand of `'session`. Each access methods returned from this trait's associated functions (like [db()](Self::db))
///   knows lifetime of a session from `&'sess self` and returns an instance that may die at `'sess`'s drop.
pub trait StorageEngine<'sess>: Default + Debug + Sized {
    /// Access methods without open database.
    type MethWithoutDb: MethodsWithoutDb;

    /// Access methods with open database (without transaction).
    type MethWithDb: MethodsWithDb;

    /// Access methods with open transaction.
    type MethWithTx: MethodsWithTx;

    /// MethodsWithoutDb implementation.
    fn without_db(&'sess self) -> Self::MethWithoutDb;

    /// MethodsWithDb implementation.
    fn with_db(&'sess self) -> Self::MethWithDb;

    /// MethodsWithTx implementation.
    fn with_tx(&'sess self) -> Self::MethWithTx;
}

#[cfg(any(test, feature = "test_support"))]
pub mod test_support;
