use super::data_type_kind::DataTypeKind;
use serde::{Deserialize, Serialize};

/// Data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct DataType {
    kind: DataTypeKind,
    nullable: bool,
}

impl DataType {
    pub fn new(kind: DataTypeKind, nullable: bool) -> Self {
        Self { kind, nullable }
    }

    pub fn kind(&self) -> &DataTypeKind {
        &self.kind
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
