use crate::error::InfraError;

use super::transaction::sqlite_tx::vtable::dao::VTableDao;
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use std::{str::FromStr, time::Duration};

/// Database context.
#[derive(Debug)]
pub struct SqliteDatabase {
    sqlite_pool: sqlx::SqlitePool,
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
        let mut pool = Self::connect_sqlite(&name).await?;

        VTableDao::create_table_if_not_exist(&mut pool).await?;

        Ok(Self {
            name,
            sqlite_pool: pool,
        })
    }

    pub(crate) fn name(&self) -> &DatabaseName {
        &self.name
    }

    fn sqlite_db_path(db_name: &DatabaseName) -> String {
        format!("sqlite://immutable_schema_{}.sqlite3", db_name.as_str()) // FIXME: path from configuration
    }

    async fn connect_sqlite(db_name: &DatabaseName) -> ApllodbResult<sqlx::SqlitePool> {
        let path = Self::sqlite_db_path(&db_name);
        log::debug!("using `{}` as backend db", path);

        let opt = sqlx::sqlite::SqliteConnectOptions::from_str(&path)
            .map_err(InfraError::from)?
            .create_if_missing(true)
            .busy_timeout(Duration::from_secs(1));
        let pool = sqlx::SqlitePool::connect_with(opt)
            .await
            .map_err(InfraError::from)?;

        Ok(pool)
    }

    pub(in crate::sqlite) fn sqlite_pool(&self) -> &sqlx::SqlitePool {
        &self.sqlite_pool
    }
}

#[cfg(test)]
impl Drop for SqliteDatabase {
    fn drop(&mut self) {
        let path = Self::sqlite_db_path(self.name());
        log::warn!("removing {}", path);

        std::fs::remove_file(&path)
            .or_else(|ioerr| match ioerr.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(ioerr),
            })
            .unwrap();
    }
}
