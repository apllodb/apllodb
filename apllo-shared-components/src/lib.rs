#![deny(
    warnings,
    // missing_docs,
    missing_debug_implementations)]

//! TBD
//!
//! まずは、TableNameとか、クエリプロセッサやストレージマネージャなどどの持ち物でもないstructを置いていく

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableName(pub String);
