use super::ShortName;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Column name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(ShortName);

impl From<ShortName> for ColumnName {
    fn from(name: ShortName) -> Self {
        Self(name)
    }
}

impl Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod factory {
    use super::ColumnName;
    use crate::{data_structure::ShortName, error::AplloResult};

    impl ColumnName {
        pub(crate) fn create<S: Into<String>>(name: S) -> AplloResult<Self> {
            let short_name = ShortName::new(name.into())?;
            Ok(Self::from(short_name))
        }
    }
}
