use super::sqlite_table_name::SqliteTableNameForTable;
use apllodb_shared_components::{
    data_structure::DatabaseName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_manager_interface::DbCtxLike;

/// Database context.
#[derive(Debug)]
pub struct Database {
    pub(in crate::transaction::sqlite_tx) sqlite_conn: rusqlite::Connection,
    name: DatabaseName,
}

impl DbCtxLike for Database {
    fn name(&self) -> &DatabaseName {
        &self.name
    }
}

impl Database {
    #[allow(dead_code)]
    /// Constructor.
    ///
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    pub(in crate::transaction::sqlite_tx) fn new(db_name: DatabaseName) -> ApllodbResult<Self> {
        let conn = Self::connect_sqlite(&db_name)?;
        let slf = Self {
            name: db_name,
            sqlite_conn: conn,
        };
        slf.create_metadata_table_if_not_exist()?;
        Ok(slf)
    }

    fn sqlite_db_path(db_name: &DatabaseName) -> String {
        format!("immutable_schema_{}.sqlite3", db_name) // FIXME: path from configuration
    }

    fn connect_sqlite(db_name: &DatabaseName) -> ApllodbResult<rusqlite::Connection> {
        let path = Self::sqlite_db_path(&db_name);
        let conn = rusqlite::Connection::open(path).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on creating connection"),
                Some(Box::new(e)),
            )
        })?;
        Ok(conn)
    }

    pub(in crate::transaction::sqlite_tx) fn sqlite_conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.sqlite_conn
    }

    fn create_metadata_table_if_not_exist(&self) -> ApllodbResult<()> {
        let metadata_table = SqliteTableNameForTable::name();
        let sql = format!(
            "
CREATE TABLE IF NOT EXISTS {} (
  table_name TEXT PRIMARY KEY,
  table_wide_constraints TEXT
)
        ",
            &metadata_table,
        );

        self.sqlite_conn
            .execute(sql.as_str(), rusqlite::params![])
            .map(|_| ())
            .map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::IoError,
                    format!(
                        "backend sqlite3 raised an error on creating metadata table `{}`",
                        metadata_table
                    ),
                    Some(Box::new(e)),
                )
            })?;
        Ok(())
    }
}

#[cfg(test)]
impl Database {
    pub(crate) fn new_for_test() -> ApllodbResult<Self> {
        use apllodb_shared_components::data_structure::ShortName;
        use uuid::Uuid;

        let db_name = format!("{}", Uuid::new_v4());
        let db_name = DatabaseName::from(ShortName::new(db_name)?);

        Self::new(db_name)
    }

    /// Creates new connection to SQLite.
    pub(in crate::transaction::sqlite_tx) fn dup(&self) -> ApllodbResult<Self> {
        let conn = Self::connect_sqlite(&self.name)?;
        Ok(Self {
            name: self.name.clone(),
            sqlite_conn: conn,
        })
    }
}

#[cfg(test)]
impl Drop for Database {
    fn drop(&mut self) {
        let path = Self::sqlite_db_path(&self.name);

        std::fs::remove_file(&path)
            .or_else(|ioerr| match ioerr.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(ioerr),
            })
            .unwrap();
    }
}
