use super::{ExistingNavi, Navi};
use crate::sqlite::row_iterator::SqliteRowIterator;
use apllodb_shared_components::{data_structure::TableName, error::ApllodbResult};

// TODO 消す
#[derive(Clone, Eq, PartialEq, Debug, new)]
pub(in crate::sqlite::transaction::sqlite_tx) struct NaviCollection {
    navi_table_name: TableName,
    row_iter: SqliteRowIterator,
}

impl Iterator for NaviCollection {
    type Item = ApllodbResult<ExistingNavi>;

    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next().map(|r| {
            let navi = Navi::from_navi_row(&self.navi_table_name, &r)?;

            match navi {
                Navi::Exist(existing_navi) => Ok(existing_navi),
                _ => panic!(
                    "NaviCollection unexpectedly encounters non-Exist Navi record: {:?}",
                    navi
                ),
            }
        })
    }
}
