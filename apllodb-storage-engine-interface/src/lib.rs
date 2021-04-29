#![deny(warnings, missing_debug_implementations)]

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
//! # Testing support
//!
//! Testing supports are available with `"test-support"` feature.
//!
//! ```toml
//! [dependencies]
//! apllodb-storage-engine-interface = {version = "...", features = ["test-support"]}
//! ```
//!
//! List of features for testing:
//!
//! - `MockStorageEngine`, `Mock*Methods` (access methods mock) structs generated by [mockall](https://docs.rs/mockall/).
//! - Models and fixtures.
//!
//! See [test_support module level doc](crate::test_support) for detail.

#[macro_use]
extern crate derive_new;

mod access_methods;
mod alter_table_action;
mod column;
mod row_projection_query;
mod row_selection_query;
mod rows;
mod single_table_condition;
mod table;
mod table_column_name;

pub use access_methods::{
    with_db_methods::WithDbMethods, with_tx_methods::WithTxMethods,
    without_db_methods::WithoutDbMethods,
};
pub use alter_table_action::AlterTableAction;
pub use column::{
    column_constraint_kind::ColumnConstraintKind, column_constraints::ColumnConstraints,
    column_data_type::ColumnDataType, column_definition::ColumnDefinition, column_name::ColumnName,
};
pub use row_projection_query::RowProjectionQuery;
pub use row_selection_query::RowSelectionQuery;
pub use rows::{row::Row, row_schema::RowSchema, Rows};
pub use single_table_condition::SingleTableCondition;
pub use table::{
    table_constraint_kind::TableConstraintKind, table_constraints::TableConstraints,
    table_name::TableName,
};
pub use table_column_name::TableColumnName;

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

#[cfg(feature = "test-support")]
use mockall::automock;

#[cfg(feature = "test-support")]
use access_methods::{
    with_db_methods::MockWithDbMethods, with_tx_methods::MockWithTxMethods,
    without_db_methods::MockWithoutDbMethods,
};

/// Storage engine interface.
#[cfg_attr(
    feature = "test-support", 
    automock(
        type WithoutDb = MockWithoutDbMethods;
        type WithDb = MockWithDbMethods;
        type WithTx = MockWithTxMethods;
    )
)]
pub trait StorageEngine {
    /// Access methods that take [SessionWithoutDb](apllodb-shared-components::SessionWithoutDb).
    type WithoutDb: WithoutDbMethods;

    /// Access methods that take [SessionWithDb](apllodb-shared-components::SessionWithDb).
    type WithDb: WithDbMethods;

    /// Access methods that take [SessionWithTx](apllodb-shared-components::SessionWithTx).
    type WithTx: WithTxMethods;

    fn without_db(&self) -> Self::WithoutDb;

    fn with_db(&self) -> Self::WithDb;

    fn with_tx(&self) -> Self::WithTx;
}

#[cfg(test)]
mod tests {
    use apllodb_test_support::setup::setup_test_logger;
    use ctor::ctor;

    #[cfg_attr(test, ctor)]
    fn test_setup() {
        setup_test_logger();
    }
}
