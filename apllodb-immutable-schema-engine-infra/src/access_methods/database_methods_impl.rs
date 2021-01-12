use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::DatabaseMethods;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct DatabaseMethodsImpl;

impl DatabaseMethods for DatabaseMethodsImpl {
    fn use_database_core(&self, database_name: &DatabaseName) -> ApllodbResult<()> {
        todo!()
    }
}
