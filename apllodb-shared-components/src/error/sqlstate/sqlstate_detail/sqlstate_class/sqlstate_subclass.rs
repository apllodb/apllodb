use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(super) struct SqlStateSubclass {
    class: Arc<SqlStateClass>,
    subclass: String,
    subclass_text: String,
}

impl SqlStateSubclass {
    pub(super) fn new(class: Arc<SqlStateClass>, subclass: &str, subclass_text: &str) -> Self {
        Self::validate_subclass(subclass);
        Self {
            class,
            subclass: subclass.to_string(),
            subclass_text: subclass_text.to_string(),
        }
    }

    fn validate_subclass(subclass: &str) {}
}
