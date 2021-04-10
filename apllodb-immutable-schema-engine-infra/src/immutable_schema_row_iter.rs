use std::collections::VecDeque;

use apllodb_immutable_schema_engine_domain::row::immutable_row::ImmutableRow;
use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::{Row, RowSchema, Rows};

use crate::sqlite::row_iterator::SqliteRowIterator;

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

impl ImmutableSchemaRowIter {
    fn chain_versions(iters: impl IntoIterator<Item = SqliteRowIterator>) -> Self {
        Self(iters.into_iter().collect())
    }

    fn into_rows(self, schema: RowSchema) -> ApllodbResult<Rows> {
        if self.0.is_empty() {
            Ok(Rows::new(schema, Vec::<Row>::new()))
        } else {
            let mut rows: Vec<Row> = vec![];

            for row_iter in self.0 {
                let mut rs: Vec<Row> = row_iter.map(|mut row| row.row).collect();
                rows.append(&mut rs);
            }

            Ok(Rows::new(schema, rows))
        }
    }

    fn schema(&self) -> RowSchema {
        if self.0.is_empty() {
            RowSchema::empty()
        } else {
            let row_iter = self.0.front().unwrap();
            row_iter.as_schema().clone()
        }
    }
}
