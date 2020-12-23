use super::{sqlite_error::map_sqlite_err, transaction::sqlite_tx::vtable::dao::VTableDao};
use apllodb_shared_components::{ApllodbResult, Database, DatabaseName};
use std::time::Duration;

/// Database context.
#[derive(Debug)]
pub struct SqliteDatabase {
    sqlite_conn: rusqlite::Connection,
    name: DatabaseName,
}

impl Database for SqliteDatabase {
    fn name(&self) -> &DatabaseName {
        &self.name
    }
}

impl SqliteDatabase {
    /// Constructor.
    ///
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    pub(crate) fn new(db_name: DatabaseName) -> ApllodbResult<Self> {
        let conn = Self::connect_sqlite(&db_name)?;

        VTableDao::create_table_if_not_exist(&conn)?;

        Ok(Self {
            name: db_name,
            sqlite_conn: conn,
        })
    }

    pub fn sqlite_db_path(&self) -> String {
        Self::_sqlite_db_path(&self.name)
    }

    fn _sqlite_db_path(db_name: &DatabaseName) -> String {
        format!("immutable_schema_{}.sqlite3", db_name.as_str()) // FIXME: path from configuration
    }

    fn connect_sqlite(db_name: &DatabaseName) -> ApllodbResult<rusqlite::Connection> {
        let path = Self::_sqlite_db_path(&db_name);
        let conn = rusqlite::Connection::open(path).map_err(|e| {
            map_sqlite_err(e, "backend sqlite3 raised an error on creating connection")
        })?;

        conn.busy_timeout(Duration::from_secs(1))
            .map_err(|e| map_sqlite_err(e, "failed to set busy timeout to SQLite"))?;

        Ok(conn)
    }

    pub(in crate::sqlite) fn sqlite_conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.sqlite_conn
    }
}

#[cfg(test)]
impl Drop for SqliteDatabase {
    fn drop(&mut self) {
        let path = self.sqlite_db_path();

        std::fs::remove_file(&path)
            .or_else(|ioerr| match ioerr.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(ioerr),
            })
            .unwrap();
    }
}
