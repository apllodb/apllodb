use std::collections::VecDeque;

use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow,
    row_iter::{
        version_row_iter::row_column_ref_schema::RowColumnRefSchema, ImmutableSchemaRowIterator,
    },
};
use apllodb_shared_components::{RecordFieldRefSchema, RecordIterator, SqlValues};
use apllodb_storage_engine_interface::AliasDef;

use crate::sqlite::{row_iterator::SqliteRowIterator, sqlite_types::SqliteTypes};

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

impl ImmutableSchemaRowIterator<SqliteTypes> for ImmutableSchemaRowIter {
    fn chain_versions(iters: impl IntoIterator<Item = SqliteRowIterator>) -> Self {
        Self(iters.into_iter().collect())
    }

    fn into_record_iterator(self, alias_def: AliasDef) -> RecordIterator {
        if self.0.is_empty() {
            RecordIterator::new(
                RecordFieldRefSchema::new_for_select(vec![]),
                Vec::<SqlValues>::new(),
            )
        } else {
            let record_schema = {
                let row_iter = self.0.front().unwrap();
                let row_schema = row_iter.schema().clone();
                row_schema.into_record_schema(alias_def)
            };

            let mut sql_values: Vec<SqlValues> = vec![];

            for row_iter in self.0 {
                let mut vs: Vec<SqlValues> = row_iter
                    .map(|row| {
                        let col_vals = row.into_zipped();
                        let sql_values = col_vals
                            .into_iter()
                            .map(|(_, sql_value)| sql_value)
                            .collect();
                        SqlValues::new(sql_values)
                    })
                    .collect();
                sql_values.append(&mut vs);
            }

            RecordIterator::new(record_schema, sql_values)
        }
    }

    fn schema(&self) -> RowColumnRefSchema {
        if self.0.is_empty() {
            RowColumnRefSchema::empty()
        } else {
            let row_iter = self.0.front().unwrap();
            row_iter.schema().clone()
        }
    }
}
