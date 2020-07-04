use super::Navi;
use apllodb_immutable_schema_engine_domain::VersionNumber;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite::transaction::sqlite_tx) struct NaviCollection {}

impl Iterator for NaviCollection {
    type Item = Navi;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl NaviCollection {
    pub(in crate::sqlite::transaction::sqlite_tx) fn group_by_version_number(
        &self,
    ) -> Vec<(VersionNumber, Self)> {
        todo!()
    }
}
