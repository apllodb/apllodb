use apllodb_shared_components::{Record, SqlValues};

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::LeafPlanOperation;

impl LeafPlanOperation {
    pub fn factory_insert_values(records: Vec<Record>) -> Self {
        assert!(!records.is_empty());

        let r = records.first().unwrap();
        let table_name = r.as_table_name();
        let column_names = r.as_column_names();

        let values: Vec<SqlValues> = records.into_iter().map(SqlValues::from).collect();

        Self::InsertValues {
            table_name,
            column_names,
            values,
        }
    }
}
