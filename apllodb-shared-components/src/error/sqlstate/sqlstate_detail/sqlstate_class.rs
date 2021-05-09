mod sqlstate_category;
mod sqlstate_subclass;

use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in super::super) struct SqlStateClass {
    pub(super) class: String,
    pub(super) class_text: String,
}

impl SqlStateClass {
    pub(in super::super) fn new(class: &str, class_text: &str) -> Self {
        Self {
            class: class.to_string(),
            class_text: class_text.to_string(),
        }
    }
}
