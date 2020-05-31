use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use apllodb_storage_manager_interface::DbCtxLike;
use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs::OpenOptions;

pub(super) struct Materializer {
    db_name: DatabaseName,
}

impl Materializer {
    pub(super) fn new<D: DbCtxLike>(db: &D) -> ApllodbResult<Self> {
        let db_name = db.name().clone();

        let slf = Self { db_name };
        slf.create_db_if_not_exists()?;
        Ok(slf)
    }

    pub(super) fn read_db(&self) -> ApllodbResult<String> {
        use std::io::Read;

        let path = self.table_objects_file_path();
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub(super) fn write_db_atomically(&self, contents: String) -> ApllodbResult<()> {
        use std::io::Write;

        let path = self.table_objects_file_path();
        let af = AtomicFile::new(path, AllowOverwrite);

        af.write(|f| f.write_all(contents.as_bytes()))
            .map_err(std::io::Error::from)?;

        Ok(())
    }

    fn table_objects_file_path(&self) -> String {
        format!("{}.ss", self.db_name)
    }

    fn create_db_if_not_exists(&self) -> ApllodbResult<()> {
        let path = self.table_objects_file_path();
        OpenOptions::new().write(true).create(true).open(path)?;
        Ok(())
    }
}
