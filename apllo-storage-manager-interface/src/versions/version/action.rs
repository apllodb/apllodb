use super::column::ColumnDataType;
use apllo_shared_components::{ColumnConstraint, ColumnName};
use serde::{Deserialize, Serialize};

/// Describes an action (diff) to create next version.
///
/// Corresponding to T_action.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum NextVersionAction {
    AddColumn {
        column_data_type: ColumnDataType,
        column_constraints: Vec<ColumnConstraint>,
    },
    DropColumn {
        column: ColumnName,
    },
}
