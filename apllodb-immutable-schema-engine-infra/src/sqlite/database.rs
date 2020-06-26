use super::{sqlite_error::map_sqlite_err, transaction::VTableDao};
use apllodb_shared_components::{
    data_structure::DatabaseName, error::ApllodbResult, traits::Database,
};
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

    fn sqlite_db_path(db_name: &DatabaseName) -> String {
        format!("immutable_schema_{}.sqlite3", db_name) // FIXME: path from configuration
    }

    fn connect_sqlite(db_name: &DatabaseName) -> ApllodbResult<rusqlite::Connection> {
        let path = Self::sqlite_db_path(&db_name);
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
impl SqliteDatabase {
    pub(crate) fn new_for_test() -> ApllodbResult<Self> {
        use uuid::Uuid;

        let db_name = format!("{}", Uuid::new_v4());
        let db_name = DatabaseName::new(db_name)?;

        Self::new(db_name)
    }

    /// Creates new connection to SQLite.
    pub(in crate::sqlite) fn dup(&self) -> ApllodbResult<Self> {
        let conn = Self::connect_sqlite(&self.name)?;
        Ok(Self {
            name: self.name.clone(),
            sqlite_conn: conn,
        })
    }
}

// #[cfg(test)]
// impl Drop for SqliteDatabase {
//     fn drop(&mut self) {
//         let path = Self::sqlite_db_path(&self.name);

//         std::fs::remove_file(&path)
//             .or_else(|ioerr| match ioerr.kind() {
//                 std::io::ErrorKind::NotFound => Ok(()),
//                 _ => Err(ioerr),
//             })
//             .unwrap();
//     }
// }
