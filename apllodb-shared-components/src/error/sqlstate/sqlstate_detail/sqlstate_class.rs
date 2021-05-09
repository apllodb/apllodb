use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in super::super) struct SqlStateClass {
    pub(in super::super) class: String,
    pub(in super::super) class_text: String,
}

impl SqlStateClass {
    pub(in super::super) fn new(class: &str, class_text: &str) -> Self {
        Self {
            class: class.to_string(),
            class_text: class_text.to_string(),
        }
    }
}
