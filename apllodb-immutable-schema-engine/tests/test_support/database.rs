use apllodb_shared_components::DatabaseName;
use uuid::Uuid;

pub(crate) fn test_database_name() -> DatabaseName {
    let db_name = format!("{}", Uuid::new_v4());
    DatabaseName::new(db_name).unwrap()
}
