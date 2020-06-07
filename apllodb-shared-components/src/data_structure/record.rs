use super::SqlValue;
use field_index::FieldIndex;
use std::collections::HashMap;

mod field_index;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Record {
    fields: HashMap<FieldIndex, SqlValue>,
}
