pub mod column_data_type;
pub mod column_definition;
pub mod column_name;

use apllodb_shared_components::data_structure::{ColumnDefinition, ColumnName};
use column_definition::NonPKColumnDefinition;
use column_name::NonPKColumnName;
use super::pk_column::column_name::PKColumnName;

pub fn filter_non_pk_column_names(
    column_names: &[ColumnName],
    apk_column_names: &[PKColumnName],
) -> Vec<NonPKColumnName> {
    let apk_column_names: Vec<String> = apk_column_names
        .iter()
        .map(|pk_cn| pk_cn.as_str().to_string())
        .collect();

    column_names
        .iter()
        .filter_map(|cn| {
            if apk_column_names.contains(&cn.as_str().to_string()) {
                None
            } else {
                Some(NonPKColumnName::from(cn.clone()))
            }
        })
        .collect()
}

pub fn filter_non_pk_column_definitions(
    column_definitions: &[ColumnDefinition],
    apk_column_names: &[PKColumnName],
) -> Vec<NonPKColumnDefinition> {
    let apk_column_names: Vec<String> = apk_column_names
        .iter()
        .map(|pk_cn| pk_cn.as_str().to_string())
        .collect();

    column_definitions
        .iter()
        .filter_map(|cd| {
            if apk_column_names.contains(&cd.column_name().as_str().to_string()) {
                None
            } else {
                Some(NonPKColumnDefinition::from(cd.clone()))
            }
        })
        .collect()
}
