use apllodb_shared_components::{
    test_support::factory::random_id, ApllodbResult, Expression, Schema, SqlType,
};
use std::collections::HashSet;

use crate::{
    column::{column_data_type::ColumnDataType, column_name::ColumnName},
    table::table_name::TableName,
    table_column_name::TableColumnName,
    Row, RowSchema, RowSelectionQuery, Rows, SingleTableCondition,
};
use apllodb_shared_components::{RPos, SchemaIndex};

impl TableName {
    /// randomly generate a table name
    pub fn random() -> Self {
        Self::new(random_id()).unwrap()
    }
}

impl TableName {
    pub fn factory(table_name: &str) -> Self {
        Self::new(table_name).unwrap()
    }
}

impl ColumnName {
    pub fn factory(column_name: &str) -> Self {
        Self::new(column_name).unwrap()
    }
}

impl TableColumnName {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(
            TableName::factory(table_name),
            ColumnName::factory(column_name),
        )
    }
}

impl ColumnDataType {
    pub fn factory(column_name: &str, sql_type: SqlType, nullable: bool) -> Self {
        Self::new(ColumnName::factory(column_name), sql_type, nullable)
    }
}

impl Rows {
    /// Horizontally shrink records. Order of columns are kept between input Row and output.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(self, indexes: &HashSet<SchemaIndex>) -> ApllodbResult<Self> {
        let new_schema = self.as_schema().projection(indexes)?;

        let projection_positions = indexes
            .iter()
            .map(|idx| {
                let (pos, _) = self.as_schema().index(idx)?;
                Ok(pos)
            })
            .collect::<ApllodbResult<HashSet<RPos>>>()?;

        let new_inner: Vec<Row> = self
            .map(|row| row.projection(&projection_positions))
            .collect();

        Ok(Self::new(new_schema, new_inner))
    }

    /// Filter Rows. Note that production code should not filter Rows after scan (for performance).
    pub fn selection(self, selection_query: &RowSelectionQuery) -> Self {
        match selection_query {
            RowSelectionQuery::FullScan => self,
            RowSelectionQuery::Condition(c) => self.filter_by_condition(c),
        }
    }

    fn filter_by_condition(self, condition: &SingleTableCondition) -> Self {
        fn eval_expression(schema: &RowSchema, row: Row, expr: &Expression) -> ApllodbResult<bool> {
            let sql_value = expr.to_sql_value_for_expr_with_index(&|index| {
                let (pos, _) = schema.index(index)?;
                row.get_sql_value(pos).map(|v| v.clone())
            })?;
            sql_value.to_bool()
        }

        let schema = self.as_schema().clone();

        let rows: Vec<Row> = self
            .filter(|row| eval_expression(&schema, row.clone(), condition.as_expression()).unwrap())
            .collect();

        Self::new(schema, rows)
    }
}
