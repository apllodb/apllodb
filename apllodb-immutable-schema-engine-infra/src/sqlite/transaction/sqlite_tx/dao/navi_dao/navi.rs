use crate::sqlite::sqlite_rowid::SqliteRowid;
use apllodb_immutable_schema_engine_domain::{Revision, VersionNumber};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx) enum Navi {
    /// Record exists in navi table and it is not DELETEd.
    Exist(ExistingNavi),
    /// Record does not exist (never has been INSERTed) in navi table.
    NotExist,
    /// Record had been INSERTEd in navi table but it is DELETEd now.
    Deleted {
        rowid: SqliteRowid,
        revision: Revision,
    },
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx) struct ExistingNavi {
    pub(in crate::sqlite::transaction::sqlite_tx) rowid: SqliteRowid,
    pub(in crate::sqlite::transaction::sqlite_tx) revision: Revision,
    pub(in crate::sqlite::transaction::sqlite_tx) version_number: VersionNumber,
}

impl Navi {
    pub(in crate::sqlite::transaction::sqlite_tx) fn rowid(&self) -> ApllodbResult<&SqliteRowid> {
        match self {
            Navi::Exist(ExistingNavi { rowid, .. }) | Navi::Deleted { rowid, .. } => Ok(rowid),
            Navi::NotExist => Err(ApllodbError::new(
                ApllodbErrorKind::DataException,
                "reference to rowid for NotExist navi record",
                None,
            )),
        }
    }
}
