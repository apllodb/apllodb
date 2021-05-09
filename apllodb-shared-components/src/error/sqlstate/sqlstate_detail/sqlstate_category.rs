use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum SqlStateCategory {
    S,
    W,
    N,
    X,
}

impl Display for SqlStateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::S => "Success",
            Self::W => "Warning",
            Self::N => "No data",
            Self::X => "Exception",
        };
        write!(f, "{}", s)
    }
}
