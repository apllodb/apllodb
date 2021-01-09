#![deny(warnings, missing_debug_implementations, missing_docs)]

//! SQL processor.
//!
//! Takes subtree of [ApllodbAst](apllodb_sql_parser::ApllodbAst) and executes SQL.
//! If passed SQL requires access to tables, SQL processor calls storage engine APIs.
//!
//! # Examples
//!
//! ```
//! use crate::my_engine::{MyDatabase, MyStorageEngine, MyTx};
//! use apllodb_shared_components::{ApllodbResult, RecordIterator, Transaction};
//! use apllodb_sql_parser::{apllodb_ast::Command, ApllodbAst, ApllodbSqlParser};
//! use apllodb_sql_processor::{DDLProcessor, ModificationProcessor, QueryProcessor};
//! use my_engine::{MyDDL, MyDML};
//!
//! fn process_ast(tx: &mut MyTx, ast: ApllodbAst) -> ApllodbResult<()> {
//!     let ddl = MyDDL;
//!     let dml = MyDML;
//!
//!     let command = ast.0;
//!     match command {
//!         Command::SelectCommandVariant(select_command) => {
//!             let processor = QueryProcessor::<'_, MyStorageEngine>::new(&dml);
//!             // Here gets records from MyStorageEngine!
//!             let records: RecordIterator = processor.run(tx, select_command)?;
//!         }
//!         Command::InsertCommandVariant(_)
//!         | Command::UpdateCommandVariant(_)
//!         | Command::DeleteCommandVariant(_) => {
//!             let processor = ModificationProcessor::<'_, MyStorageEngine>::new(&dml);
//!             processor.run(tx, command)?;
//!         }
//!         Command::CreateTableCommandVariant(_)
//!         | Command::AlterTableCommandVariant(_)
//!         | Command::DropTableCommandVariant(_) => {
//!             let processor = DDLProcessor::<'_, MyStorageEngine>::new(&ddl);
//!             processor.run(tx, command)?;
//!         }
//!     };
//!     Ok(())
//! }
//!
//! fn main() -> ApllodbResult<()> {
//!     let parser = ApllodbSqlParser::new();
//!
//!     let db = MyDatabase;
//!     let mut tx = MyTx::begin(db)?;
//!
//!     process_ast(
//!         &mut tx,
//!         parser
//!             .parse("CREATE TABLE t (id INTEGER, c INTEGER, PRIMARY KEY (id))")
//!             .unwrap(),
//!     )?;
//!     process_ast(&mut tx, parser.parse("SELECT id, c FROM t").unwrap())?;
//!     process_ast(
//!         &mut tx,
//!         parser
//!             .parse("INSERT INTO t (id, c) VALUES (1, 13)")
//!             .unwrap(),
//!     )?;
//!
//!     Ok(())
//! }
//!
//! mod my_engine {
//!     // ...
//! #     use apllodb_shared_components::{
//! #         AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, Database, DatabaseName,
//! #         Expression, Record, RecordIterator, TableConstraints, TableName, Transaction, TransactionId,
//! #     };
//! #     use apllodb_storage_engine_interface::{
//! #         DDLMethods, DMLMethods, ProjectionQuery, StorageEngine,
//! #     };
//! #     use std::collections::HashMap;
//! #
//! #     #[derive(Debug)]
//! #     pub struct MyDatabase;
//! #     impl Database for MyDatabase {
//! #         fn use_database(name: DatabaseName) -> ApllodbResult<Self> {
//! #             Ok(Self)
//! #         }
//! #
//! #         fn name(&self) -> &apllodb_shared_components::DatabaseName {
//! #             unimplemented!()
//! #         }
//! #     }
//! #
//! #     #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//! #     pub struct MyTransactionId;
//! #     impl TransactionId for MyTransactionId {}
//! #
//! #     #[derive(Debug)]
//! #     pub struct MyTx;
//! #     impl Transaction for MyTx {
//! #         type Db = MyDatabase;
//! #         type RefDb = MyDatabase;
//! #         type TID = MyTransactionId;
//! #
//! #         fn id(&self) -> &MyTransactionId {
//! #             unimplemented!()
//! #         }
//! #
//! #         fn begin(_db: MyDatabase) -> ApllodbResult<Self> {
//! #             Ok(Self)
//! #         }
//! #
//! #         fn commit(self) -> ApllodbResult<()> {
//! #             unimplemented!()
//! #         }
//! #
//! #         fn abort(self) -> ApllodbResult<()> {
//! #             unimplemented!()
//! #         }
//! #
//! #         fn database_name(&self) -> &DatabaseName {
//! #             unimplemented!()
//! #         }
//! #     }
//! #
//! #     #[derive(Debug)]
//! #     pub struct MyDDL;
//! #     impl DDLMethods<MyStorageEngine> for MyDDL {
//! #         fn create_table(
//! #             &self,
//! #             tx: &mut MyTx,
//! #             table_name: &TableName,
//! #             table_constraints: &TableConstraints,
//! #             column_definitions: Vec<ColumnDefinition>,
//! #         ) -> ApllodbResult<()> {
//! #             Ok(())
//! #         }
//! #
//! #         fn alter_table(
//! #             &self,
//! #             tx: &mut MyTx,
//! #             table_name: &TableName,
//! #             action: &AlterTableAction,
//! #         ) -> ApllodbResult<()> {
//! #             todo!()
//! #         }
//! #
//! #         fn drop_table(&self, tx: &mut MyTx, table_name: &TableName) -> ApllodbResult<()> {
//! #             todo!()
//! #         }
//! #     }
//! #
//! #     #[derive(Debug)]
//! #     pub struct MyDML;
//! #     impl DMLMethods<MyStorageEngine> for MyDML {
//! #         fn select(
//! #             &self,
//! #             tx: &mut MyTx,
//! #             table_name: &TableName,
//! #             projection: ProjectionQuery,
//! #         ) -> ApllodbResult<RecordIterator> {
//! #             Ok(RecordIterator::new(Vec::<Record>::new()))
//! #         }
//! #
//! #         fn insert(
//! #             &self,
//! #             tx: &mut MyTx,
//! #             table_name: &TableName,
//! #             records: RecordIterator,
//! #         ) -> ApllodbResult<()> {
//! #             Ok(())
//! #         }
//! #
//! #         fn update(
//! #             &self,
//! #             tx: &mut MyTx,
//! #             table_name: &TableName,
//! #             column_values: HashMap<ColumnName, Expression>,
//! #         ) -> ApllodbResult<()> {
//! #             todo!()
//! #         }
//! #
//! #         fn delete(&self, tx: &mut MyTx, table_name: &TableName) -> ApllodbResult<()> {
//! #             todo!()
//! #         }
//! #     }
//! #
//! #     #[derive(Debug)]
//! #     pub struct MyStorageEngine;
//! #     impl StorageEngine for MyStorageEngine {
//! #         type Db = MyDatabase;
//! #         type Tx = MyTx;
//! #         type DDL = MyDDL;
//! #         type DML = MyDML;
//! #     }
//! }
//! ```

#[macro_use]
extern crate derive_new;

pub(crate) mod ast_translator;
pub(crate) mod sql_processor;

#[cfg(test)]
pub(crate) mod test_support;
