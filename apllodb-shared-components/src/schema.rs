pub(crate) mod schema_index;
pub(crate) mod schema_name;

use crate::{ApllodbResult, RecordPos};

use self::{schema_index::SchemaIndex, schema_name::SchemaName};

/// Schema defines structure of records / rows.
///
/// Main purpose of schema is to resolve fields' / columns' position in records / rows to extract values from them.
///
/// While rows, used in storage-engine, consist of tables' column values,
/// records have higher level of fields like field references, constants, and operations.
///
/// So a schema for rows consist of sequence of "table_name.column_name" but a schema for records may include unnamed field.
pub trait Schema {
    type Name: SchemaName;
    type Index: SchemaIndex;

    fn index(&self, idx: &Self::Index) -> ApllodbResult<(RecordPos, Self::Name)>;
}
