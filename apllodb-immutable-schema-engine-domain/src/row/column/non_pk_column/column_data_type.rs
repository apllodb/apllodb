use apllodb_shared_components::data_structure::ColumnDataType;
use serde::{Deserialize, Serialize};
use super::{NonPKColumnName, NonPKColumnDefinition};

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
