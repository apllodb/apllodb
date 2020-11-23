use super::{ExistingNavi};
use crate::sqlite::row_iterator::SqliteRowIterator;
use apllodb_immutable_schema_engine_domain::{
    version::version_number::VersionNumber,
};
use apllodb_shared_components::{error::ApllodbResult};


#[derive(Clone, Eq, PartialEq, Debug, new)]
pub(in crate::sqlite::transaction::sqlite_tx) struct NaviCollection {
    row_iter: SqliteRowIterator,
}

impl Iterator for NaviCollection {
    type Item = ApllodbResult<ExistingNavi>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // TODO 不要のはず
    }
}

impl NaviCollection {
    pub(in crate::sqlite::transaction::sqlite_tx) fn group_by_version_number(
        self,
    ) -> ApllodbResult<Vec<(VersionNumber, Self)>> {
        todo!()
        // TODO この関数は不要になったはず
    }
}
