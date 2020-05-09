use super::column::ColumnDataType;
use apllo_shared_components::data_structure::{ColumnConstraints, ColumnName};
use serde::{Deserialize, Serialize};

/// Describes an action (diff) to create next version.
///
/// Corresponding to T_action.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum NextVersionAction {
    AddColumn {
        column_data_type: ColumnDataType,
        column_constraints: ColumnConstraints,
    },
    DropColumn {
        column: ColumnName,
    },
}
