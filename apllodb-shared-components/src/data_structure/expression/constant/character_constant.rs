use serde::{Deserialize, Serialize};

use crate::Constant;

/// Character(s) constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CharacterConstant {
    /// Text constant.
    TextConstantVariant(TextConstant),
}

/// Text constant (arbitrary length; UTF-8).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TextConstant(String);
impl TextConstant {
    /// Get as &str
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Constant {
    fn from(v: String) -> Self {
        Self::CharacterConstantVariant(CharacterConstant::TextConstantVariant(TextConstant(v)))
    }
}
