use super::data_type_kind::DataTypeKind;
use serde::{Deserialize, Serialize};

/// Data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct DataType {
    kind: DataTypeKind,
    nullable: bool,
}
