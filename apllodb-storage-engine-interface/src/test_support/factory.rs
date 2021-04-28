use apllodb_shared_components::{
    test_support::factory::random_id, ApllodbResult, Schema, SchemaIndex, SqlType, SqlValue,
};

use crate::{
    column::{column_data_type::ColumnDataType, column_name::ColumnName},
    table::table_name::TableName,
    table_column_name::TableColumnName,
    Row, RowSelectionQuery, Rows,
};

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
    /// Filter Rows. Note that production code should not filter Rows after scan (for performance).
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn selection(self, selection_query: &RowSelectionQuery) -> ApllodbResult<Self> {
        match selection_query {
            RowSelectionQuery::FullScan => Ok(self),
            RowSelectionQuery::Probe { column, value } => self.probe(&column, &value),
        }
    }

    fn probe(self, index: &SchemaIndex, value: &SqlValue) -> ApllodbResult<Self> {
        let schema = self.as_schema().clone();
        let (pos, _) = schema.index(index)?;

        let rows = self
            .filter_map(|row| {
                let v = row
                    .get_sql_value(pos)
                    .expect("valid RPos must find an SqlValue");
                if v == value {
                    Some(row)
                } else {
                    None
                }
            })
            .collect::<Vec<Row>>();

        Ok(Self::new(schema, rows))
    }
}
