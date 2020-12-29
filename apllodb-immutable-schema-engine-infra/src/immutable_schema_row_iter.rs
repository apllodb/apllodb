use std::collections::VecDeque;

use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow, row_iter::ImmutableSchemaRowIterator,
};

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    sqlite::{row_iterator::SqliteRowIterator, sqlite_types::SqliteTypes},
};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ImmutableSchemaRowIter(VecDeque<SqliteRowIterator>);

impl Iterator for ImmutableSchemaRowIter {
    type Item = ImmutableRow;

    fn next(&mut self) -> Option<Self::Item> {
        let ver_row_iter = self.0.front_mut()?;
        ver_row_iter.next().or_else(|| {
            let _ = self
                .0
                .remove(0)
                .expect("ver_row_iter exists so self.0 has first element");
            self.next()
        })
    }
}
impl<'tx, 'db: 'tx>
    ImmutableSchemaRowIterator<'tx, 'db, ApllodbImmutableSchemaEngine<'db>, SqliteTypes>
    for ImmutableSchemaRowIter
{
    fn chain_versions(iters: impl IntoIterator<Item = SqliteRowIterator>) -> Self {
        Self(iters.into_iter().collect())
    }
}
