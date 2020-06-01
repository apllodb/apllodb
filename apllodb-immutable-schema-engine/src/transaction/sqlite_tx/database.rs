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
        let path = format!("immutable_schema_{}.sqlite3", db_name); // FIXME: path from configuration
        let conn = rusqlite::Connection::open(path).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on creating connection"),
                Some(Box::new(e)),
            )
        })?;
        Ok(Self {
            name: db_name,
            sqlite_conn: conn,
        })
    }

    pub(in crate::transaction::sqlite_tx) fn sqlite_conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.sqlite_conn
    }

    fn create_metadata_table_if_not_exist(&mut self) -> ApllodbResult<()> {
        self.conn.
    }

    fn metadata_table_name() -> String {
        "_table_metadata".to_string()
    }
}
