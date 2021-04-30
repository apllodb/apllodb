//use crate::sqlite::database::SqliteDatabase;
use apllodb_shared_components::DatabaseName;
//use glob::glob;

/// Removes sqlite3 database file on drop().
///
/// Use this in "panic guard" pattern.
#[derive(Debug)]
pub struct SqliteDatabaseCleaner(DatabaseName);

impl SqliteDatabaseCleaner {
    pub fn new(database_name: DatabaseName) -> Self {
        Self(database_name)
    }
}

impl Drop for SqliteDatabaseCleaner {
    fn drop(&mut self) {
        // TODO コメントイン

        // let sqlite3_path = SqliteDatabase::sqlite_db_path(&self.0);
        // let sqlite3_files_pattern = format!("{}*", sqlite3_path.as_os_str().to_str().unwrap());

        // for entry in glob(&sqlite3_files_pattern).unwrap() {
        //     if let Ok(path) = entry {
        //         log::debug!(
        //             "SqliteDatabaseCleaner: found {}. removing...",
        //             path.as_os_str().to_str().unwrap()
        //         );

        //         std::fs::remove_file(&path)
        //             .or_else(|ioerr| match ioerr.kind() {
        //                 std::io::ErrorKind::NotFound => Ok(()),
        //                 _ => Err(ioerr),
        //             })
        //             .unwrap();

        //         log::debug!("SqliteDatabaseCleaner: done");
        //     }
        // }
    }
}
