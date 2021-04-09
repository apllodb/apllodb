use serde::{Deserialize, Serialize};

/// 0-origin position index of a field/column.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize, new,
)]
pub struct RPos(usize);

impl RPos {
    /// To raw u16.
    pub fn to_usize(&self) -> usize {
        self.0
    }

    /// Increment pos.
    pub fn inc(&mut self) {
        self.0 = self.0 + 1
    }
}
