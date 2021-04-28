use std::collections::{HashMap, HashSet};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use apllodb_storage_engine_interface::{
    ColumnName, RowProjectionQuery, RowSchema, TableColumnName,
};
use serde::{Deserialize, Serialize};

use crate::{
    entity::Entity,
    row::pk::apparent_pk::ApparentPrimaryKey,
    version::{active_versions::ActiveVersions, id::VersionId},
    vtable::VTable,
};

/// Has filters by primary keys.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SelectionResult {
    /// FullScan    
    FullScan,
    /// PK probe
    Probe(Vec<ApparentPrimaryKey>),  // TODO これ PrimaryKey にする
    // TODO Range with lower-bound PK & upper-bound PK
}

impl SelectionResult {
    /// Constructor.
    ///
    /// If RowSelectionQuery consists only of primary keys, this constructor does not access Rows
    /// but just makes translation from RowSelectionQuery into SelectionResult.
    /// Otherwise, this constructor accesses Rows and collect PrimaryKeys
    pub fn new(
        vtable: &VTable,
        active_versions: ActiveVersions,
        query: &RowProjectionQuery,
    ) -> ApllodbResult<Self> {
    }
}
