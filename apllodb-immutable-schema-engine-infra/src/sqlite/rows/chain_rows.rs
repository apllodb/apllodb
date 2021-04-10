use apllodb_storage_engine_interface::{Row, Rows};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct ChainRows;

impl ChainRows {
    pub(crate) fn chain(vec_rows: Vec<Rows>) -> Rows {
        assert!(!vec_rows.is_empty());

        let mut base = vec_rows.first().expect("checked").clone();
        let schema = base.as_schema().clone();

        let all_rows: Vec<Row> = vec_rows.into_iter().fold(vec![], |vec_row, rows| {
            assert_eq!(rows.as_schema(), &schema);
            let mut to_add: Vec<Row> = rows.collect();
            vec_row.append(&mut to_add);
            vec_row
        });
        Rows::new(schema, all_rows)
    }
}
