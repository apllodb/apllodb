#[macro_export]
macro_rules! apk_column_names {
    [ $( $column_name: expr $(,)? )* ] => {{
        let mut column_names: Vec<apllodb_shared_components::data_structure::ColumnName> = vec![];

        $(
            column_names.push(apllodb_shared_components::data_structure::ColumnName::new($column_name).unwrap());
        )*

        $crate::ApparentPrimaryKeyColumnNames::new(column_names)
    }};
}

#[macro_export]
macro_rules! apparent_pk {
    [ $( ( $column_name: expr, $sql_value: expr $(,)? ), )* ] => {{
        let mut pk_column_names: Vec<$crate::row::column::pk_column::PKColumnName> = vec![];
        let mut sql_values: Vec<apllodb_shared_components::data_structure::SqlValue> = vec![];

        $(
            pk_column_names.push($column_name);
            sql_values.push($sql_value);
        )*

        $crate::ApparentPrimaryKey::new(pk_column_names, sql_values)
    }};
}

#[macro_export]
macro_rules! non_pk_column_data_type {
    ($col_name: expr, $data_type: expr $(,)?) => {{
        let column_data_type = apllodb_shared_components::column_data_type!($col_name, $data_type);
        $crate::row::column::non_pk_column::NonPKColumnDataType::from(column_data_type)
    }};
}

#[macro_export]
macro_rules! non_pk_column_definition {
    ($col_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {{
        let column_definition = apllodb_shared_components::column_definition!(
            $col_name,
            $data_type,
            $column_constraints
        );
        $crate::row::column::non_pk_column::NonPKColumnDefinition(column_definition)
    }};
}

#[macro_export]
macro_rules! non_pk_column_name {
    ($col_name: expr) => {{
        let column_name = apllodb_shared_components::column_name!($col_name);
        $crate::row::column::non_pk_column::NonPKColumnName::from(column_name)
    }};
}

#[macro_export]
macro_rules! pk_column_data_type {
    ($col_name: expr, $data_type: expr $(,)?) => {{
        let column_data_type = apllodb_shared_components::column_data_type!($col_name, $data_type);
        $crate::row::column::pk_column::PKColumnDataType::from(column_data_type)
    }};
}

#[macro_export]
macro_rules! pk_column_definition {
    ($col_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {{
        let column_definition = apllodb_shared_components::column_definition!(
            $col_name,
            $data_type,
            $column_constraints
        );
        $crate::row::column::pk_column::PKColumnDefinition(column_definition)
    }};
}

#[macro_export]
macro_rules! pk_column_name {
    ($col_name: expr) => {{
        let column_name = apllodb_shared_components::column_name!($col_name);
        $crate::row::column::pk_column::PKColumnName::from(column_name)
    }};
}

#[macro_export]
macro_rules! vtable_id {
    ($database_name: expr, $table_name: expr $(,)?) => {{
        let database_name =
            apllodb_shared_components::data_structure::DatabaseName::new($database_name).unwrap();
        let table_name =
            apllodb_shared_components::data_structure::TableName::new($table_name).unwrap();

        $crate::VTableId::new(&database_name, &table_name)
    }};
    () => {{
        use rand::Rng;

        let database_name = apllodb_shared_components::data_structure::DatabaseName::new(
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .filter(|&c| 'a' <= c && c <= 'z')
                .take(10)
                .collect::<String>(),
        )
        .unwrap();
        let table_name = apllodb_shared_components::data_structure::TableName::new(
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .filter(|&c| 'a' <= c && c <= 'z')
                .take(10)
                .collect::<String>(),
        )
        .unwrap();

        $crate::VTableId::new(&database_name, &table_name)
    }};
}
