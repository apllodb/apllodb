use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct Revision(u64);

impl Revision {
    pub fn initial() -> Self {
        Self(1)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

impl From<i64> for Revision {
    fn from(v: i64) -> Self {
        Self(v as u64)
    }
}
