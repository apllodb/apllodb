use apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_id::VRRId;

use serde::{Deserialize, Serialize};

/// See: https://www.sqlite.org/lang_createtable.html#rowid
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SqliteRowid(pub i64);

impl VRRId for SqliteRowid {}
