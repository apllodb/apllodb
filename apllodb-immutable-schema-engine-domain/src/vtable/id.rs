use apllodb_shared_components::DatabaseName;
use apllodb_storage_engine_interface::TableName;
use serde::{Deserialize, Serialize};

/// ID of VTable
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VTableId {
    pub(in crate::vtable) database_name: DatabaseName,
    pub(in crate::vtable) table_name: TableName,
}

impl VTableId {
    pub fn new(database_name: &DatabaseName, table_name: &TableName) -> Self {
        Self {
            database_name: database_name.clone(),
            table_name: table_name.clone(),
        }
    }

    pub fn database_name(&self) -> &DatabaseName {
        &self.database_name
    }

    pub fn table_name(&self) -> &TableName {
        &self.table_name
    }
}

#[cfg(test)]
impl VTableId {
    pub(crate) fn new_for_test() -> Self {
        Self::new(&DatabaseName::random(), &TableName::random())
    }
}
