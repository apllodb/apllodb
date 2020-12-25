use apllodb_shared_components::{ColumnDefinition, ColumnName};

pub fn filter_non_pk_column_names(
    all_column_names: &[ColumnName],
    apk_column_names: &[ColumnName],
) -> Vec<ColumnName> {
    let apk_column_names: Vec<String> = apk_column_names
        .iter()
        .map(|pk_cn| pk_cn.as_str().to_string())
        .collect();

    all_column_names
        .iter()
        .filter_map(|cn| {
            if apk_column_names.contains(&cn.as_str().to_string()) {
                None
            } else {
                Some(cn.clone())
            }
        })
        .collect()
}

pub fn filter_non_pk_column_definitions(
    all_column_definitions: &[ColumnDefinition],
    apk_column_names: &[ColumnName],
) -> Vec<ColumnDefinition> {
    let apk_column_names: Vec<String> = apk_column_names
        .iter()
        .map(|pk_cn| pk_cn.as_str().to_string())
        .collect();

    all_column_definitions
        .iter()
        .filter_map(|cd| {
            if apk_column_names.contains(&cd.column_ref().as_column_name().as_str().to_string()) {
                None
            } else {
                Some(cd.clone())
            }
        })
        .collect()
}
