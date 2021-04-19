use serde::{Deserialize, Serialize};

/// A constraint parameter in a column definition.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ColumnConstraintKind {
    // TODO: DEFAULT, CHECK, ...
}
