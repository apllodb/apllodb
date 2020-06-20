//! Data structures used among multiple components such as Query Processor, Storage Engine, and Client.

mod alter_table_action;
mod column;
mod database;
mod expression;
mod record;
mod table;
mod validation_helper;
mod value;

pub use alter_table_action::AlterTableAction;
pub use column::{
    ColumnConstraintKind, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    DataType, DataTypeKind,
};
pub use database::DatabaseName;
pub use expression::{Constant, Expression, IntegerConstant, NumericConstant};
pub use record::{FieldIndex, Record};
pub use table::{TableConstraintKind, TableConstraints, TableName};
pub use value::SqlValue;
