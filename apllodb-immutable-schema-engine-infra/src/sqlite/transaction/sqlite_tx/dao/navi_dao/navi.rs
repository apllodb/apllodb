use super::{CNAME_REVISION, CNAME_ROWID, CNAME_VERSION_NUMBER};
use crate::sqlite::sqlite_rowid::SqliteRowid;
use apllodb_immutable_schema_engine_domain::{ImmutableRow, Revision, VersionNumber};
use apllodb_shared_components::{
    data_structure::ColumnName,
    error::{ApllodbError, ApllodbResult},
};
use std::convert::TryFrom;

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

impl TryFrom<ImmutableRow> for Navi {
    type Error = ApllodbError;

    fn try_from(r: ImmutableRow) -> ApllodbResult<Self> {
        use apllodb_storage_engine_interface::Row;

        let rowid = SqliteRowid(r.get::<i64>(&ColumnName::new(CNAME_ROWID)?)?);
        let revision = Revision::from(r.get::<i64>(&ColumnName::new(CNAME_REVISION)?)?);
        let opt_version_number = r
            .get::<Option<i64>>(&ColumnName::new(CNAME_VERSION_NUMBER)?)?
            .map(VersionNumber::from);
        match opt_version_number {
            None => Ok(Navi::Deleted { rowid, revision }),
            Some(version_number) => Ok(Navi::Exist(ExistingNavi {
                rowid,
                revision,
                version_number,
            })),
        }
    }
}
