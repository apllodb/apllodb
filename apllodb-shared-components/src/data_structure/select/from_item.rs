use serde::{Deserialize, Serialize};

use crate::{Expression, TableWithAlias};

/// FROM ...
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum FromItem {
    /// T (AS TA)?
    TableVariant(TableWithAlias),

    /// T (AS TA)? INNER JOIN ... ON ...
    JoinVariant {
        /// join type
        join_type: JoinType,

        /// T
        left: Box<FromItem>,

        /// ...
        right: Box<FromItem>,

        /// ...
        on: Expression,
    },
}

/// JOIN type
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum JoinType {
    /// INNER JOIN
    InnerJoin,
}
