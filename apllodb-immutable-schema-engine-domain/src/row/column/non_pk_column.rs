mod column_data_type;
mod column_definition;
mod column_name;

pub use column_data_type::NonPKColumnDataType;
pub use column_definition::NonPKColumnDefinition;
pub use column_name::NonPKColumnName;

use crate::PKColumnNames;
use apllodb_shared_components::data_structure::{ColumnDefinition, ColumnName};

pub fn filter_non_pk_column_names(
    column_names: &[ColumnName],
    apk_column_names: &PKColumnNames,
) -> Vec<NonPKColumnName> {
    let apk_column_names = apk_column_names.column_names();

    column_names
        .iter()
        .filter_map(|cn| {
            if apk_column_names.contains(cn) {
                None
            } else {
                Some(NonPKColumnName(cn.clone()))
            }
        })
        .collect()
}

pub fn filter_non_pk_column_definitions(
    column_definitions: &[ColumnDefinition],
    apk_column_names: &PKColumnNames,
) -> Vec<NonPKColumnDefinition> {
    let apk_column_names = apk_column_names.column_names();

    column_definitions
        .iter()
        .filter_map(|cd| {
            if apk_column_names.contains(cd.column_name()) {
                None
            } else {
                Some(NonPKColumnDefinition(cd.clone()))
            }
        })
        .collect()
}
