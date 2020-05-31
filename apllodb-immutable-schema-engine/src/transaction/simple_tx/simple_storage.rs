use super::{lock_manager::TableRwToken, objects::TableObj};
use apllodb_shared_components::error::ApllodbResult;

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
    pub(super) fn load_table(_token: &TableRwToken) -> ApllodbResult<TableObj> {
        todo!()
    }

    pub(super) fn flush_objects_atomically(_table_objects: Vec<TableObj>) -> ApllodbResult<()> {
        // TODO will use https://github.com/untitaker/rust-atomicwrites
        todo!()
    }
}
