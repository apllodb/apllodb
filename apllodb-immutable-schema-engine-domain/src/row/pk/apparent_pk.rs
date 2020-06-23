use apllodb_shared_components::data_structure::{ColumnName, SqlValue};
use apllodb_storage_engine_interface::PrimaryKey;
use serde::{Deserialize, Serialize};

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    column_name: ColumnName,
    value: SqlValue,
}

impl PrimaryKey for ApparentPrimaryKey {
    fn column_name(&self) -> &ColumnName {
        &self.column_name
    }
}