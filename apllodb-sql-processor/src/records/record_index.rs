use serde::{Deserialize, Serialize};

use apllodb_shared_components::{RPos, SchemaIndex};

/// Key to extract an SqlValue from a Record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum RecordIndex {
    /// 0-origin index
    Pos(RPos),

    /// Fuzzy reference to a field.
    ///
    /// Given the following SQL:
    ///
    /// ```sql
    /// SELECT c1, ta.c2 AS c2a FROM t AS ta;
    /// ```
    ///
    /// First field can be extracted either by:
    ///
    /// - `RecordIndex::from("c1")`
    /// - `RecordIndex::from("t.c1")`
    /// - `RecordIndex::from("ta.c1")`
    ///
    /// and second can be:
    ///
    /// - `RecordIndex::from("c2")`
    /// - `RecordIndex::from("t.c2")`
    /// - `RecordIndex::from("ta.c2")`
    /// - `RecordIndex::from("c2a")`
    Name(SchemaIndex),
}

impl From<usize> for RecordIndex {
    fn from(raw_pos: usize) -> Self {
        Self::Pos(RPos::new(raw_pos))
    }
}

impl From<&str> for RecordIndex {
    fn from(s: &str) -> Self {
        Self::Name(SchemaIndex::from(s))
    }
}
