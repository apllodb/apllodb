#[macro_export]
macro_rules! apparent_pk {
    [ $( ( $column_name: expr, $sql_value: expr $(,)? ), )* ] => {{
        use apllodb_shared_components::data_structure::{ColumnName, SqlValue};

        let mut column_names: Vec<ColumnName> = vec![];
        let mut sql_values: Vec<SqlValue> = vec![];

        $(
            column_names.push($column_name);
            sql_values.push($sql_value);
        )*

        ApparentPrimaryKey::new(column_names, sql_values)
    }};
}

#[macro_export]
macro_rules! vtable_id {
    ($database_name: expr, $table_name: expr $(,)?) => {{
        use apllodb_shared_components::data_structure::{DatabaseName, TableName};

        let database_name = DatabaseName::new($database_name).unwrap();
        let table_name = TableName::new($table_name).unwrap();

        VTableId::new(&database_name, &table_name)
    }};
    () => {{
        use apllodb_shared_components::data_structure::{DatabaseName, TableName};
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let database_name = DatabaseName::new(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .filter(|&c| 'a' <= c && c <= 'z')
                .take(10)
                .collect::<String>(),
        )
        .unwrap();
        let table_name = TableName::new(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .filter(|&c| 'a' <= c && c <= 'z')
                .take(10)
                .collect::<String>(),
        )
        .unwrap();

        VTableId::new(&database_name, &table_name)
    }};
}
