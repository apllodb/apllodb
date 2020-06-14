use apllodb_shared_components::data_structure::{DatabaseName, TableName};
use serde::{Deserialize, Serialize};

/// ID of VTable
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VTableId {
    pub(in crate::vtable) database_name: DatabaseName,
    pub(in crate::vtable) table_name: TableName,
}

impl VTableId {
    pub(in crate::vtable) fn new(database_name: &DatabaseName, table_name: &TableName) -> Self {
        Self {
            database_name: database_name.clone(),
            table_name: table_name.clone(),
        }
    }
}

#[cfg(test)]
impl VTableId {
    pub(crate) fn gen() -> Self {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let database_name = DatabaseName::new(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .collect::<String>(),
        )
        .unwrap();

        let table_name = TableName::new(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .collect::<String>(),
        )
        .unwrap();

        Self::new(&database_name, &table_name)
    }
}
