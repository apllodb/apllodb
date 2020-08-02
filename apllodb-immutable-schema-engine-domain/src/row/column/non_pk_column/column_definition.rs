use super::NonPKColumnName;
use apllodb_shared_components::data_structure::{ColumnConstraints, ColumnDefinition, DataType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct NonPKColumnDefinition(ColumnDefinition);

impl From<ColumnDefinition> for NonPKColumnDefinition {
    fn from(cd: ColumnDefinition) -> Self {
        Self(cd)
    }
}

impl NonPKColumnDefinition {
    /// Ref to ColumnName.
    pub fn column_name(&self) -> NonPKColumnName {
        let cn = self.0.column_name().clone();
        NonPKColumnName::from(cn)
    }

    /// Ref to DataType.
    pub fn data_type(&self) -> &DataType {
        &self.0.data_type()
    }

    /// Ref to ColumnConstraints.
    pub fn column_constraints(&self) -> &ColumnConstraints {
        &self.0.column_constraints()
    }
}
