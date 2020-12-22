use apllodb_shared_components::data_structure::{DatabaseName, TableName};
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
        use rand::Rng;

        let database_name = apllodb_shared_components::data_structure::DatabaseName::new(
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .map(char::from)
                .filter(|&c| 'a' <= c && c <= 'z')
                .take(10)
                .collect::<String>(),
        )
        .unwrap();
        let table_name = apllodb_shared_components::data_structure::TableName::new(
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .map(char::from)
                .filter(|&c| 'a' <= c && c <= 'z')
                .take(10)
                .collect::<String>(),
        )
        .unwrap();

        Self::new(&database_name, &table_name)
    }
}
