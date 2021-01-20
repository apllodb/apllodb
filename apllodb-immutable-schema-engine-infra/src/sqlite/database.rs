use crate::error::InfraError;

use super::transaction::sqlite_tx::vtable::dao::VTableDao;
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use sqlx::{ConnectOptions, Connection};
use std::{str::FromStr, time::Duration};

/// Database context.
#[derive(Debug)]
pub struct SqliteDatabase {
    sqlite_conn: sqlx::SqliteConnection,
    name: DatabaseName,
}

impl SqliteDatabase {
    /// Start using database.
    ///
    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    pub(crate) async fn use_database(name: DatabaseName) -> ApllodbResult<Self> {
        let mut conn = Self::connect_sqlite(&name).await?;

        VTableDao::create_table_if_not_exist(&mut conn).await?;

        Ok(Self {
            name,
            sqlite_conn: conn,
        })
    }

    pub(crate) fn name(&self) -> &DatabaseName {
        &self.name
    }

    pub(crate) fn sqlite_db_path(&self) -> String {
        Self::_sqlite_db_path(&self.name)
    }

    fn _sqlite_db_path(db_name: &DatabaseName) -> String {
        format!("sqlite://immutable_schema_{}.sqlite3", db_name.as_str()) // FIXME: path from configuration
    }

    async fn connect_sqlite(db_name: &DatabaseName) -> ApllodbResult<sqlx::SqliteConnection> {
        let path = Self::_sqlite_db_path(&db_name);
        log::debug!("using `{}` as backend db", path);

        let conn = sqlx::sqlite::SqliteConnectOptions::from_str(&path)
            .map_err(InfraError::from)?
            .create_if_missing(true)
            .busy_timeout(Duration::from_secs(1))
            .connect()
            .await
            .map_err(InfraError::from)?;

        Ok(conn)
    }

    pub(in crate::sqlite) fn sqlite_conn(&mut self) -> &mut sqlx::SqliteConnection {
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
