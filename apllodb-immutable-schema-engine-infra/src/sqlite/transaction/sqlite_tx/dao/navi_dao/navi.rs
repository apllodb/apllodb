use apllodb_immutable_schema_engine_domain::{Revision, VersionNumber};
use crate::sqlite::sqlite_rowid::SqliteRowid;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite) enum Navi {
    /// Record exists in navi table and it is not DELETEd.
    Exist {
        rowid: SqliteRowid,
        revision: Revision,
        version_number: VersionNumber,
    },
    /// Record does not exist (never has been INSERTed) in navi table.
    NotExist,
    /// Record had been INSERTEd in navi table but it is DELETEd now.
    Deleted {
        rowid: SqliteRowid,
        revision: Revision,
    },
}
