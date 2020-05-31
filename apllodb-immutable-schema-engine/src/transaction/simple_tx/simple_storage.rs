mod materializer;
mod objects;

pub(in crate::transaction::simple_tx) use objects::TableObj;

use super::lock_manager::TableRwToken;
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use apllodb_storage_manager_interface::DbCtxLike;
use materializer::Materializer;

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
    pub(super) fn load_table<D: DbCtxLike>(
        db: &D,
        token: &TableRwToken,
    ) -> ApllodbResult<TableObj> {
        let materializer = Materializer::new(db)?;
        let contents = materializer.read_db()?;

        let deserialized: Vec<TableObj> = serde_yaml::from_str(&contents).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to deserialize database `{}`", db.name()),
                Some(Box::new(e)),
            )
        })?;

        let table_name = token.as_table_name();
        deserialized
            .into_iter()
            .find(|obj| obj.as_table().name() == table_name)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!("undefined table `{}`", table_name),
                    None,
                )
            })
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - IO error happens on writing file.
    pub(super) fn flush_objects_atomically<D: DbCtxLike>(
        db: &D,
        table_objects: Vec<TableObj>,
    ) -> ApllodbResult<()> {
        let materializer = Materializer::new(db)?;

        let contents = serde_yaml::to_string(&table_objects).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to serialize {:?}", &table_objects),
                Some(Box::new(e)),
            )
        })?;

        materializer.write_db_atomically(contents)
    }
}
