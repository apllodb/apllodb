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
/// 1. `try_lock()` "T" and (if no other transaction does not have lock to "T", ) get [TableRwToken](foobar.html).
///
/// 2. Show the token to `load_table()` and then get reference to [TableObj](foobar.html) of "T" from disk.
///    Note that only a transaction who acquires [TableRwToken](foobar.html) for "T" can load [TableObj](foobar.html) of "T".
///
///     ```text
///                  &TableObj "T"
///                       ^
///                +------|--------------------------------+
///                |      |                                |
///                |  TableObj "T"                         |
///     [memory]   |      ^                                |
///     ===========|======|================================|=================================
///      [disk]    |      |                                |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +--- -------- SimpleStorage ------------+
///     ```
///
/// 3. Get [Table](foobar.html) "T" from [TableObj](foobar.html) "T".
///   Construct another [Table](foobar.html) "T" instance.
///   
///
///     ```text
///                    Table "T" -> (new) Table "T" -> (new) TableObj "T"
///                       ^
///                       |
///                  &mut TableObj "T"
///                       ^
///                +------|--------------------------------+
///                |      |                                |
///                |  TableObj "T"                         |
///     [memory]   |      ^                                |
///     ===========|======|================================|=================================
///      [disk]    |      |                                |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +--- -------- SimpleStorage ------------+
///     ```
///
/// 4. Call [TableObj::update()](foobar.html) to overwrite "T" in memory.
///
///     ```text
///                    Table "T" -> (new) Table "T" -> (new) TableObj "T"
///                       ^                                     |
///                       |                                     |
///                  &mut TableObj "T" <------------------------+
///                       ^    |
///                +------|----|---------------------------+
///                |      |    v                           |
///                |  TableObj "T" (dirty)                 |
///     [memory]   |      ^                                |
///     ===========|======|================================|=================================
///      [disk]    |      |                                |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +--- -------- SimpleStorage ------------+
///     ```
///
/// 5. `flush_objects_atomically()` to make dirty "T" durable.
///
///     ```text
///                    Table "T" -> (new) Table "T" -> (new) TableObj "T"
///                       ^                                     |
///                       |                                     |
///                  &mut TableObj "T" <------------------------+
///                       ^    |
///                +------|----|---------------------------+
///                |      |    v                           |
///                |  TableObj "T" (will be dropped)       |
///     [memory]   |      ^    |                           |
///     ===========|======|====|===========================|=================================
///      [disk]    |      |    v                           |
///                |  (serialized representation of "T")   |
///                |                                       |
///                +--- -------- SimpleStorage ------------+
///     ```
///
/// 6. Automatically release (unlock) [TableRwToken](foobar.html) when you drop response of `try_lock()`.
#[derive(Debug)]
pub(crate) struct SimpleStorage;

impl SimpleStorage {
    pub(super) fn load_table(_token: &TableRwToken) -> ApllodbResult<TableObj> {
        todo!()
    }

    pub(super) fn flush_objects_atomically(_table_objects: Vec<TableObj>) -> ApllodbResult<()> {
        // TODO will use https://github.com/untitaker/rust-atomicwrites
        todo!()
    }
}
