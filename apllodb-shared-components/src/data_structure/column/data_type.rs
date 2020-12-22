use super::data_type_kind::DataTypeKind;
use serde::{Deserialize, Serialize};

/// Data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct DataType {
    kind: DataTypeKind,
    nullable: bool,
}

impl DataType {
    /// Constructor
    pub fn new(kind: DataTypeKind, nullable: bool) -> Self {
        Self { kind, nullable }
    }

    /// Ref to DataTypeKind
    pub fn kind(&self) -> &DataTypeKind {
        &self.kind
    }

    /// `false` if the column is `NOT NULL`, otherwise `true`.
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
