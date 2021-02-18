use std::fmt::Display;

use crate::{traits::correlation::Correlation, AliasName, CorrelationName, FromItem, TableName};
use serde::{Deserialize, Serialize};

/// Table name with (optional) alias.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableWithAlias {
    /// table name
    pub table_name: TableName,

    /// alias
    pub alias: Option<AliasName>,
}

impl Display for TableWithAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.alias {
            Some(alias) => alias.as_str(),
            None => self.table_name.as_str(),
        };
        write!(f, "{}", s)
    }
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

impl Correlation for TableWithAlias {
    fn is_named(&self, correlation_name: &CorrelationName) -> bool {
        let cn = correlation_name.as_str();
        self.table_name.as_str() == cn
            || self
                .alias
                .as_ref()
                .map_or_else(|| false, |alias| alias.as_str() == cn)
    }
}
