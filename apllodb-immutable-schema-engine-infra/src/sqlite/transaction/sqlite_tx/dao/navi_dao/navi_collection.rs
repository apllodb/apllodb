use super::{ExistingNavi, Navi, CNAME_VERSION_NUMBER};
use crate::sqlite::row_iterator::SqliteRowIterator;
use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow, version::version_number::VersionNumber,
};
use apllodb_shared_components::{data_structure::ColumnName, error::ApllodbResult};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Eq, PartialEq, Debug, new)]
pub(in crate::sqlite::transaction::sqlite_tx) struct NaviCollection {
    row_iter: SqliteRowIterator,
}

impl Iterator for NaviCollection {
    type Item = ApllodbResult<ExistingNavi>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::convert::TryFrom;

        self.row_iter.next().map(|row_res| {
            let r = row_res?;
            let navi = Navi::try_from(r)?;

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

impl NaviCollection {
    pub(in crate::sqlite::transaction::sqlite_tx) fn group_by_version_number(
        self,
    ) -> ApllodbResult<Vec<(VersionNumber, Self)>> {
        use apllodb_storage_engine_interface::Row;

        let mut h: HashMap<VersionNumber, VecDeque<ImmutableRow>> = HashMap::new();

        for row in self.row_iter {
            let r = row?;
            let version_number = r
                .get::<Option<i64>>(&ColumnName::new(CNAME_VERSION_NUMBER)?)?
                .map(VersionNumber::from)
                .expect("NaviCollection should not hold Navi::Deleted inside");

            h.entry(version_number)
                .and_modify(|rows| {
                    let r = r.clone(); // don't hold r's ownership for or_insert_with.
                    rows.push_back(r);
                })
                .or_insert_with(move || {
                    let mut rows = VecDeque::new();
                    rows.push_back(r);
                    rows
                });
        }

        Ok(h.into_iter()
            .map(|(version_number, rows)| {
                (
                    version_number,
                    NaviCollection {
                        row_iter: SqliteRowIterator::from(rows),
                    },
                )
            })
            .collect())
    }
}
