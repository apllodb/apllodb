#![deny(warnings, missing_debug_implementations, missing_docs)]

//! SQL processor.
//!
//! Takes subtree of [ApllodbAst](apllodb_sql_parser::ApllodbAst) and executes SQL.
//! If passed SQL requires access to tables, SQL processor calls storage engine APIs.
//!
//! # Examples
//!
//! ## SELECT query
//!
//! ```
//! use std::collections::HashMap;
//! use apllodb_shared_components::{AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, Database, DatabaseName, Expression, Record, RecordIterator, TableConstraints, TableName};
//! use apllodb_storage_engine_interface::{
//!     ProjectionQuery, StorageEngine, Transaction, TransactionBuilder, TransactionId,
//! };
//! use apllodb_sql_parser::{ApllodbSqlParser, apllodb_ast::Command};
//! use apllodb_sql_processor::QueryProcessor;
//!
//! fn main() -> ApllodbResult<()> {
//!     let parser = ApllodbSqlParser::new();
//!     let tx = MyStorageEngine::begin()?;
//!
//!     let ast = parser.parse("SELECT id, c FROM t")
//!         .expect("syntactically valid SQL");
//!     match ast.0 {
//!         Command::SelectCommandVariant(select_command) => {
//!             let processor = QueryProcessor::<'_, MyStorageEngine>::new(&tx);
//!             // Here gets records from MyStorageEngine!
//!             let records: RecordIterator = processor.run(select_command)?;
//!         },
//!         _ => todo!(),
//!     };
//!     Ok(())
//! }
//!     
//! // simple storage engine implementation follows
//!
//! struct MyDatabase;
//! impl Database for MyDatabase {
//!     fn name(&self) -> &apllodb_shared_components::DatabaseName {
//!         unimplemented!()
//!     }
//! }
//! impl MyDatabase {
//!     fn new() -> Self {
//!         Self
//!     }
//! }
//!
//! #[derive(Clone, PartialEq, Debug)]
//! struct MyTxBuilder;
//! impl TransactionBuilder for MyTxBuilder {}
//!
//! #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//! struct MyTransactionId;
//! impl TransactionId for MyTransactionId {}
//!
//! #[derive(Debug)]
//! struct MyTx;
//! impl Transaction<MyStorageEngine> for MyTx {
//!     fn id(&self) -> &MyTransactionId {
//!         unimplemented!()
//!     }
//!
//!     fn begin(_builder: MyTxBuilder) -> ApllodbResult<Self> {
//!         Ok(Self)
//!     }
//!
//!     fn commit(self) -> ApllodbResult<()> {
//!         unimplemented!()
//!     }
//!
//!     fn abort(self) -> ApllodbResult<()> {
//!         unimplemented!()
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
//!         unimplemented!()
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
//!         Ok(RecordIterator::new(Vec::<Record>::new()))
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
//! struct MyStorageEngine;
//! impl StorageEngine for MyStorageEngine {
//!     type Tx = MyTx;
//!     type TxBuilder = MyTxBuilder;
//!     type TID = MyTransactionId;
//!     type Db = MyDatabase;
//!
//!     fn use_database(_database_name: &DatabaseName) -> ApllodbResult<MyDatabase> {
//!         Ok(MyDatabase)
//!     }
//! }
//! impl MyStorageEngine {
//!     fn begin() -> ApllodbResult<MyTx> {
//!         MyTx::begin(MyTxBuilder)
//!     }
//! }
//! ```

#[macro_use]
extern crate derive_new;

pub(crate) mod ast_translator;
pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;

pub use ddl::DDLProcessor;
pub use modification::ModificationProcessor;
pub use query::QueryProcessor;

#[cfg(test)]
pub(crate) mod test_support;
