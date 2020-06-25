use apllodb_shared_components::data_structure::{ColumnName, SqlValue};
use apllodb_storage_engine_interface::PrimaryKey;
use serde::{Deserialize, Serialize};

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    column_names: Vec<ColumnName>,

    // real "key" of a record.
    sql_values: Vec<SqlValue>,
}

impl PrimaryKey for ApparentPrimaryKey {
    fn column_names(&self) -> &[ColumnName] {
        &self.column_names
    }
}
