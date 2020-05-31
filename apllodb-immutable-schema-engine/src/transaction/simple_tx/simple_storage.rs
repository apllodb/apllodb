use super::{lock_manager::TableRwToken, objects::TableObj};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs::File;

/// Storage for [SimpleTx](foobar.html).
///
/// Really simple to implement and really poor in performance.
///
/// # Objects lifecycle
///
/// Say you are trying to update table "T" in a transaction.
///
/// 1. [SimpleTx::get_table()](foobar.html) transparently returns `Table` instance from disk,
///    while [SimpleStorage::load_table()](foobar.html) is internally called and `TableObj` gets owned by `SimpleTx` instance.
///
///     ```text
///                    Table "T"
///                       ^
///          .get_table() |
///                +------|--------------------------------+
///                |      |                                |
///                |  TableObj "T"                         |
///                |      ^                                |
///                +------|------- SimpleTx ---------------+
///     [memory]          |
///     ==================|==================================================================
///      [disk]           |
///         .load_table() |
///                +------|--------------------------------+
///                |      |                                |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +------------ SimpleStorage ------------+
///     ```
///
/// 2. Construct another [Table](foobar.html) "T" instance.
///    [SimpleTx::put_table()](foobar.html) to overwrite "T" in memory.
///
///     ```text
///                    Table "T" -> (new) Table "T"
///                       ^                |
///          .get_table() |                | .put_table()
///                +------|----------------|---------------+
///                |      |                v               |
///                |  TableObj "T" (dirty)                 |
///                |      ^                                |
///                +------|------- SimpleTx ---------------+
///     [memory]          |
///     ==================|==================================================================
///      [disk]           |
///         .load_table() |
///                +------|--------------------------------+
///                |      |                                |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +------------ SimpleStorage ------------+
///     ```
///
/// 3. `SimpleTx::commit()` to make dirty "T" durable.
///
///     ```text
///                    Table "T" -> (new) Table "T"
///                       ^                |
///          .get_table() |                | .put_table()
///                +------|----------------|---------------+
///                |      |                v               |
///                |  TableObj "T" (dropped)               | .commit()
///                |      ^    |                           |
///                +------|----|-- SimpleTx ---------------+
///     [memory]          |    |
///     ==================|====|=============================================================
///      [disk]           |    |
///         .load_table() |    | .flush_objects_atomically()
///                +------|----|---------------------------+
///                |      |    v                           |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +------------ SimpleStorage ------------+
///     ```
#[derive(Debug)]
pub(crate) struct SimpleStorage;

impl SimpleStorage {
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table does not exist in disk.
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - IO error happens on reading file.
    /// - [DeserializationError](error/enum.ApllodbErrorKind.html#variant.DeserializationError) when:
    ///   - Failed to deserialize table metadata.
    pub(super) fn load_table(_token: &TableRwToken) -> ApllodbResult<TableObj> {
        use std::io::Read;

        let path = Self::table_objects_file_path();

        let mut file = File::open(path.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let deserialized: TableObj = serde_yaml::from_str(&contents).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to deserialize `{}`", path),
                Some(Box::new(e)),
            )
        })?;
        Ok(deserialized)
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - IO error happens on writing file.
    pub(super) fn flush_objects_atomically(table_objects: Vec<TableObj>) -> ApllodbResult<()> {
        use std::io::Write;

        //let contents: &[u8] = "TODO".as_bytes();
        let contents = serde_yaml::to_string(&table_objects).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to serialize {:?}", &table_objects),
                Some(Box::new(e)),
            )
        })?;

        let path = Self::table_objects_file_path();
        let af = AtomicFile::new(path, AllowOverwrite);

        af.write(|f| f.write_all(contents.as_bytes()))
            .map_err(std::io::Error::from)?;

        Ok(())
    }

    fn table_objects_file_path() -> String {
        // FIXME set from configuration
        "table_objects.ss".to_string()
    }
}
