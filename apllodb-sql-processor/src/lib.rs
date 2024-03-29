#![deny(missing_debug_implementations, missing_docs)]

//! SQL processor.
//!
//! Takes subtree of [ApllodbAst](apllodb_sql_parser::ApllodbAst) and executes SQL.
//! If passed SQL requires access to tables, SQL processor calls storage engine APIs.

#[macro_use]
extern crate derive_new;

pub(crate) mod aliaser;
pub(crate) mod ast_translator;
pub(crate) mod attribute;
pub(crate) mod condition;
pub(crate) mod correlation;
pub(crate) mod field;
pub(crate) mod records;
pub(crate) mod select;
pub(crate) mod sql_processor;

pub use records::{record::Record, record_index::RecordIndex, Records};
pub use sql_processor::{
    sql_processor_context::SqlProcessorContext, success::SqlProcessorSuccess, SqlProcessor,
};

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

#[cfg(test)]
mod tests {
    use apllodb_test_support::setup::setup_test_logger;
    use ctor::ctor;

    #[cfg_attr(test, ctor)]
    fn test_setup() {
        setup_test_logger();
    }
}
