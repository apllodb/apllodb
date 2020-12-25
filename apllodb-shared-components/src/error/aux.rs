use super::sqlstate::SqlState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(super) struct ApllodbErrorAux {
    pub(super) sqlstate: SqlState,
    pub(super) errcode: String,
}
