use apllodb_immutable_schema_engine::ApllodbImmutableSchemaDb;
use apllodb_shared_components::{ApllodbResult, Database, DatabaseName};

pub struct TestDatabase(pub ApllodbImmutableSchemaDb);

impl TestDatabase {
    pub fn new() -> ApllodbResult<Self> {
        use uuid::Uuid;

        let db_name = format!("{}", Uuid::new_v4());
        let db_name = DatabaseName::new(db_name)?;

        let db = ApllodbImmutableSchemaDb::use_database(db_name)?;
        Ok(Self(db))
    }

    #[allow(dead_code)]
    pub fn dup(&self) -> ApllodbResult<Self> {
        let db_name = self.0.name();
        let db = ApllodbImmutableSchemaDb::use_database(db_name.clone())?;
        Ok(Self(db))
    }
}

#[cfg(test)]
impl Drop for TestDatabase {
    fn drop(&mut self) {
        let path = self.0.sqlite_db_path();

        std::fs::remove_file(&path)
            .or_else(|ioerr| match ioerr.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(ioerr),
            })
            .unwrap();
    }
}
