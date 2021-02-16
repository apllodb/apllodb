use serde::{Deserialize, Serialize};

use crate::{AliasName, Expression, TableName};

/// FROM ...
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum FromItem {
    /// T (AS TA)?
    TableNameVariant {
        table_name: TableName,
        alias: Option<AliasName>,
    },

    /// T (AS TA)? INNER JOIN ... ON ...
    JoinVariant {
        join_type: JoinType,
        left: Box<FromItem>,
        right: Box<FromItem>,
        on: Expression,
    },
}

/// JOIN type
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum JoinType {
    /// INNER JOIN
    InnerJoin,
}
