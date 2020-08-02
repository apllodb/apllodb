use super::PKColumnName;
use apllodb_shared_components::data_structure::{ColumnDataType, ColumnName, DataType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct PKColumnDataType(ColumnDataType);

impl From<ColumnDataType> for PKColumnDataType {
    fn from(cdt: ColumnDataType) -> Self {
        Self(cdt)
    }
}

impl PKColumnDataType {
    pub fn new(column_name: PKColumnName, data_type: DataType) -> Self {
        let cdt = ColumnDataType::new(
            ColumnName::new(column_name.as_str().to_string())
                .expect("column_name str has already passed validation"),
            data_type,
        );
        Self(cdt)
    }

    pub fn column_data_type(&self) -> &ColumnDataType {
        &self.0
    }

    /// Ref to column name.
    pub fn column_name(&self) -> PKColumnName {
        PKColumnName::from(self.0.column_name().clone())
    }

    /// Ref to data type.
    pub fn data_type(&self) -> &DataType {
        &self.0.data_type()
    }
}
