use super::{NonPKColumnDefinition, NonPKColumnName};
use apllodb_shared_components::data_structure::{ColumnDataType, ColumnName, DataType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct NonPKColumnDataType(ColumnDataType);

impl From<&NonPKColumnDefinition> for NonPKColumnDataType {
    fn from(non_pk_column_definition: &NonPKColumnDefinition) -> Self {
        let cdt = ColumnDataType::from(&non_pk_column_definition.0);
        Self(cdt)
    }
}
impl From<ColumnDataType> for NonPKColumnDataType {
    fn from(cdt: ColumnDataType) -> Self {
        Self(cdt)
    }
}

impl NonPKColumnDataType {
    pub fn new(column_name: NonPKColumnName, data_type: DataType) -> Self {
        let cdt = ColumnDataType::new(
            ColumnName::new(column_name.as_str().to_string())
                .expect("column_name str has already passed validation"),
            data_type,
        );
        Self(cdt)
    }

    /// Ref to column name.
    pub fn column_name(&self) -> NonPKColumnName {
        NonPKColumnName::from(self.0.column_name().clone())
    }

    /// Ref to data type.
    pub fn data_type(&self) -> &DataType {
        &self.0.data_type()
    }    
}
