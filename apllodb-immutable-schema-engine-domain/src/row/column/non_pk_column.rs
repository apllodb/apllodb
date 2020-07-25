use crate::PKColumnNames;
use apllodb_shared_components::data_structure::{ColumnDataType, ColumnDefinition, ColumnName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct NonPKColumnName(pub ColumnName);

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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct NonPKColumnDataType(pub ColumnDataType);
impl From<&NonPKColumnDefinition> for NonPKColumnDataType {
    fn from(non_pk_column_definition: &NonPKColumnDefinition) -> Self {
        let cdt = ColumnDataType::from(&non_pk_column_definition.0);
        Self(cdt)
    }
}
impl NonPKColumnDataType {
    /// Ref to column name.
    pub fn column_name(&self) -> NonPKColumnName {
        NonPKColumnName(self.0.column_name().clone())
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct NonPKColumnDefinition(pub ColumnDefinition);

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
