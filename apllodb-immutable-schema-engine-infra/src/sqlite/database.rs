use crate::error::InfraError;

use crate::sqlite::transaction::sqlite_tx::{
    version::dao::version_metadata_dao::VersionMetadataDao,
    vtable::vtable_metadata_dao::VTableMetadataDao,
};
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName};
use sqlx::{migrate::MigrateDatabase, Connection};
use std::{path::PathBuf, str::FromStr, time::Duration};

/// Database context.
#[derive(Debug)]
pub(crate) struct SqliteDatabase {
    sqlite_pool: sqlx::SqlitePool,
    name: DatabaseName,
}

impl SqliteDatabase {
    /// Create a database.
    ///
    /// # Failures
    ///
    /// - [DuplicateDatabase](apllodb_shared_components::ApllodbErrorKind::DuplicateDatabase) when:
    ///   - specified database already exists
    pub(crate) async fn create_database(name: DatabaseName) -> ApllodbResult<()> {
        let path = Self::sqlite_db_path(&name);
        let path = path.to_str().expect("should be valid unicode");

        if sqlx::Sqlite::database_exists(path)
            .await
            .map_err(InfraError::from)?
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateDatabase,
                format!("database {:?} already exists", name),
                None,
            ))
        } else {
            log::debug!("creates new database file: {}", path);

            let opt = sqlx::sqlite::SqliteConnectOptions::from_str(&path)
                .map_err(InfraError::from)?
                .create_if_missing(true);
            let mut conn = sqlx::SqliteConnection::connect_with(&opt)
                .await
                .map_err(InfraError::from)?;

            VTableMetadataDao::create_table(&mut conn).await?;
            VersionMetadataDao::create_table(&mut conn).await?;

            conn.close().await.map_err(InfraError::from)?;
            Ok(())
        }
    }

    /// Start using database.
    ///
    /// # Failures
    ///
    /// - [UndefinedObject](apllodb_shared_components::ApllodbErrorKind::UndefinedObject) when:
    ///   - database named `name` has not been created yet.
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    pub(crate) async fn use_database(name: DatabaseName) -> ApllodbResult<Self> {
        let pool = Self::connect_existing_sqlite(&name).await?;
        Ok(Self {
            name,
            sqlite_pool: pool,
        })
    }

    pub(crate) fn name(&self) -> &DatabaseName {
        &self.name
    }

    pub(crate) fn sqlite_db_path(db_name: &DatabaseName) -> PathBuf {
        PathBuf::from(format!("immutable_schema_{}.sqlite3", db_name.as_str())) // FIXME: path from configuration
    }

    async fn connect_existing_sqlite(db_name: &DatabaseName) -> ApllodbResult<sqlx::SqlitePool> {
        let path = Self::sqlite_db_path(&db_name);
        let path = path.to_str().expect("should be valid unicode");

        log::debug!("using `{}` as backend db", path);

        let opt = sqlx::sqlite::SqliteConnectOptions::from_str(&path)
            .map_err(InfraError::from)?
            .create_if_missing(false)
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
