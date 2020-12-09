use super::{CNAME_REVISION, CNAME_ROWID, CNAME_VERSION_NUMBER};
use crate::sqlite::sqlite_rowid::SqliteRowid;
use apllodb_immutable_schema_engine_domain::{
    row::{
        immutable_row::ImmutableRow,
        pk::{apparent_pk::ApparentPrimaryKey, full_pk::revision::Revision},
    },
    version::version_number::VersionNumber,
    vtable::VTable,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, ColumnReference, TableName},
    error::ApllodbResult,
};

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

impl Navi {
    pub(in crate::sqlite::transaction::sqlite_tx) fn from_navi_row(
        table_name: &TableName,
        r: &ImmutableRow,
    ) -> ApllodbResult<Self> {
        use apllodb_storage_engine_interface::Row;

        let rowid = SqliteRowid(r.get::<i64>(&ColumnReference::new(
            table_name.clone(),
            ColumnName::new(CNAME_ROWID)?,
        ))?);
        let revision = Revision::from(r.get::<i64>(&ColumnReference::new(
            table_name.clone(),
            ColumnName::new(CNAME_REVISION)?,
        ))?);
        let opt_version_number = r
            .get::<Option<i64>>(&ColumnReference::new(
                table_name.clone(),
                ColumnName::new(CNAME_VERSION_NUMBER)?,
            ))?
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

/// Does not have PrimaryKey for performance
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx) struct ExistingNavi {
    pub(in crate::sqlite::transaction::sqlite_tx) rowid: SqliteRowid,
    pub(in crate::sqlite::transaction::sqlite_tx) revision: Revision,
    pub(in crate::sqlite::transaction::sqlite_tx) version_number: VersionNumber,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx) struct ExistingNaviWithPK {
    pub(in crate::sqlite::transaction::sqlite_tx) navi: ExistingNavi,
    pub(in crate::sqlite::transaction::sqlite_tx) pk: ApparentPrimaryKey,
}

/// `Some()` only when `r` is Navi::Exist.
impl ExistingNaviWithPK {
    pub(in crate::sqlite::transaction::sqlite_tx) fn from_navi_row(
        vtable: &VTable,
        r: ImmutableRow,
    ) -> ApllodbResult<Option<Self>> {
        let ret = if let Navi::Exist(existing_navi) = Navi::from_navi_row(vtable.table_name(), &r)? {
            Some(ExistingNaviWithPK {
                navi: existing_navi,
                pk: ApparentPrimaryKey::from_table_and_immutable_row(vtable, r)?,
            })
        } else {
            None
        };
        Ok(ret)
    }
}
