use super::{CNAME_REVISION, CNAME_ROWID, CNAME_VERSION_NUMBER};
use crate::sqlite::sqlite_rowid::SqliteRowid;
use apllodb_immutable_schema_engine_domain::{
    row::pk::{apparent_pk::ApparentPrimaryKey, full_pk::revision::Revision},
    version::version_number::VersionNumber,
    vtable::VTable,
};
use apllodb_shared_components::{ApllodbResult, Schema, SchemaIndex};
use apllodb_storage_engine_interface::{Row, RowSchema};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) enum Navi {
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
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) fn from_navi_row(
        schema: &RowSchema,
        row: &mut Row,
    ) -> ApllodbResult<Self> {
        let (pos_rowid, _) = schema.index(&SchemaIndex::from(CNAME_ROWID))?;
        let (pos_revision, _) = schema.index(&SchemaIndex::from(CNAME_REVISION))?;
        let (pos_version_number, _) = schema.index(&SchemaIndex::from(CNAME_VERSION_NUMBER))?;

        let rowid = SqliteRowid(row.get::<i64>(pos_rowid)?.expect("must be NOT NULL"));
        let revision = Revision::from(row.get::<i64>(pos_revision)?.expect("must be NOT NULL"));
        let opt_version_number = row.get::<i64>(pos_version_number)?.map(VersionNumber::from);
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
pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) struct ExistingNavi {
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) rowid: SqliteRowid,
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) revision: Revision,
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) version_number:
        VersionNumber,
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) struct ExistingNaviWithPk {
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) navi: ExistingNavi,
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) pk: ApparentPrimaryKey,
}

/// `Some()` only when `r` is Navi::Exist.
impl ExistingNaviWithPk {
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) fn from_navi_row(
        vtable: &VTable,
        schema: &RowSchema,
        mut row: Row,
    ) -> ApllodbResult<Option<Self>> {
        let ret = if let Navi::Exist(existing_navi) = Navi::from_navi_row(schema, &mut row)? {
            Some(ExistingNaviWithPk {
                navi: existing_navi,
                pk: ApparentPrimaryKey::from_table_and_row(vtable, schema, &mut row)?,
            })
        } else {
            None
        };
        Ok(ret)
    }
}
