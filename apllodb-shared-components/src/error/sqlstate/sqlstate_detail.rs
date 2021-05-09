mod sqlstate_class;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub(super) use self::sqlstate_class::SqlStateClass;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(super) struct SqlStateDetail {
    pub(super) class: Arc<SqlStateClass>,
    pub(super) subclass: String,
    pub(super) subclass_text: String,
}

impl SqlStateDetail {
    pub(super) fn new(class: Arc<SqlStateClass>, subclass: &str, subclass_text: &str) -> Self {
        Self {
            class,
            subclass: subclass.to_string(),
            subclass_text: subclass_text.to_string(),
        }
    }

    pub(super) fn sqlstate(&self) -> String {
        format!("{}{}", &self.class.class, &self.subclass)
    }
}
