impl Row {
    pub fn factory(sql_values: Vec<SqlValue>) -> Self {
        Self::new(SqlValues::new(sql_values))
    }

    /// WARN: internal SqlValues might get different from RecordFieldRefSchema
    pub fn naive_join(self, right: Self) -> Self {
        let mut sql_values = self.into_values();
        for right_sql_value in right.into_values() {
            sql_values.append(right_sql_value);
        }
        Self::new(sql_values)
    }
}
