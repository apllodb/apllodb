mod builder;

pub use builder::RowBuilder;

use apllodb_shared_components::traits::SqlConvertible;
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Row representation used in storage engine.
/// Row, unlike `Record`, does not deal with `Expression`s.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Row {  //<- いやー、traiででよさそう
    columns: HashMap<ColumnName, SqlValue>,
}

impl Row {
    /// Get value from column.
    pub fn get<T: SqlConvertible>(&self, column_name: &ColumnName) -> ApllodbResult<T> {
        let sql_value = self.columns.get(column_name).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column name: `{:?}`", column_name),
                None,
            )
        })?;
        Ok(sql_value.unpack()?)
    }
}
