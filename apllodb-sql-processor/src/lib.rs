#![deny(warnings, missing_debug_implementations, missing_docs)]

//! SQL processor.
//!
//! Takes subtree of [ApllodbAst](apllodb_sql_parser::ApllodbAst) and executes SQL.
//! If passed SQL requires access to tables, SQL processor calls storage engine APIs.
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//! use apllodb_shared_components::{AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, DatabaseName, Expression, Record, RecordIterator, TableConstraints, TableName};
//! use apllodb_storage_engine_interface::{
//!     Database, ProjectionQuery, StorageEngine, Transaction, TransactionId,
//! };
//! use apllodb_sql_parser::{ApllodbAst, ApllodbSqlParser, apllodb_ast::Command};
//! use apllodb_sql_processor::{DDLProcessor, ModificationProcessor, QueryProcessor};
//!
//! fn process_ast(tx: &MyTx, ast: ApllodbAst) -> ApllodbResult<()> {
//!     let command = ast.0;
//!     match command {
//!         Command::SelectCommandVariant(select_command) => {
//!             let processor = QueryProcessor::<'_, MyStorageEngine>::new(&tx);
//!             // Here gets records from MyStorageEngine!
//!             let records: RecordIterator = processor.run(select_command)?;
//!         },
//!         Command::InsertCommandVariant(_) | Command::UpdateCommandVariant(_) | Command::DeleteCommandVariant(_) => {
//!             let processor = ModificationProcessor::<'_, MyStorageEngine>::new(&tx);
//!             processor.run(command)?;
//!         }
//!         Command::CreateTableCommandVariant(_) | Command::AlterTableCommandVariant(_) | Command::DropTableCommandVariant(_) => {
//!             let processor = DDLProcessor::<'_, MyStorageEngine>::new(&tx);
//!             processor.run(command)?;
//!         }
//!     };
//!     Ok(())
//! }
//!
//! fn main() -> ApllodbResult<()> {
//!     let parser = ApllodbSqlParser::new();
//!
//!     let db = MyDatabase;
//!     let tx = MyTx::begin(db)?;
//!
//!     process_ast(&tx, parser.parse("CREATE TABLE t (id INTEGER, c INTEGER)").unwrap())?;
//!     process_ast(&tx, parser.parse("SELECT id, c FROM t").unwrap())?;
//!     process_ast(&tx, parser.parse("INSERT INTO t (id, c) VALUES (1, 13)").unwrap())?;
//!
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
//! #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//! struct MyTransactionId;
//! impl TransactionId for MyTransactionId {}
//!
//! #[derive(Debug)]
//! struct MyTx;
//! impl Transaction<MyStorageEngine> for MyTx {
//!     type Db = MyDatabase;
//!     type TID = MyTransactionId;
//!
//!     fn id(&self) -> &MyTransactionId {
//!         unimplemented!()
//!     }
//!
//!     fn begin(db: MyDatabase) -> ApllodbResult<Self> {
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
//!         Ok(RecordIterator::new(Vec::<Record>::new()))
//!     }
//!
//!     fn insert(
//!         &self,
//!         table_name: &TableName,
//!         records: RecordIterator,
//!     ) -> ApllodbResult<()> {
//!         Ok(())
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
//!     type Db = MyDatabase;
//!     type Tx = MyTx;
//!
//!     fn use_database(_database_name: &DatabaseName) -> ApllodbResult<MyDatabase> {
//!         Ok(MyDatabase)
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
