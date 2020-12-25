use super::data_type_kind::DataTypeKind;
use serde::{Deserialize, Serialize};

/// Data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct DataType {
    kind: DataTypeKind,
    nullable: bool,
}

impl DataType {
    /// Ref to DataTypeKind
    pub fn kind(&self) -> &DataTypeKind {
        &self.kind
    }

    /// `false` if the column is `NOT NULL`, otherwise `true`.
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
