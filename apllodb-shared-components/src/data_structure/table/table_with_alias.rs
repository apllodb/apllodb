use crate::{AliasName, FromItem, TableName};
use serde::{Deserialize, Serialize};

/// Table name with (optional) alias.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableWithAlias {
    /// table name
    pub table_name: TableName,

    /// alias
    pub alias: Option<AliasName>,
}

impl From<&FromItem> for Vec<TableWithAlias> {
    fn from(from_item: &FromItem) -> Self {
        match from_item {
            FromItem::TableVariant(table_with_alias) => {
                vec![table_with_alias.clone()]
            }
            FromItem::JoinVariant { left, right, .. } => {
                let mut left_tables = Self::from(left.as_ref());
                let mut right_tables = Self::from(right.as_ref());
                left_tables.append(&mut right_tables);
                left_tables
            }
        }
    }
}
