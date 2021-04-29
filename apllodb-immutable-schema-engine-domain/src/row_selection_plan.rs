use apllodb_shared_components::{ApllodbResult, SchemaIndex, SqlValue};
use apllodb_storage_engine_interface::{RowSelectionQuery, SingleTableCondition};

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    version_revision_resolver::vrr_entries::VrrEntries, vtable::VTable,
};

/// Has selection plan aiming for highest performance.
/// Except FullScan strategy, each plan refers to primary keys or VRR.
#[derive(Clone, PartialEq, Debug)]
pub enum RowSelectionPlan<Types: ImmutableSchemaAbstractTypes> {
    /// Full scan: Selects all Rows from a table.
    FullScan,
    /// VRR probe: Points primary keys with revision and version information. Efficient for low selectivity.
    VrrProbe(VrrEntries<Types>),
    // TODO ScanFilter(Expression): Filter-in matching Rows while scanning all Rows. Efficient for high selectivity since this avoids random I/O.
    // TODO PkRange(...): More efficient for low-mid selectivity than ScanFilter.
}

impl<Types: ImmutableSchemaAbstractTypes> RowSelectionPlan<Types> {
    /// Constructor.
    ///
    /// If RowSelectionQuery consists only of primary keys, this constructor does not access VersionRepository
    /// but just makes translation from RowSelectionQuery into SelectionResult.
    /// Otherwise, this constructor accesses VersionRepository and collect VrrEntries.
    pub fn new(query: &RowSelectionQuery) -> ApllodbResult<Self> {
        let ret = match query {
            RowSelectionQuery::FullScan => Self::FullScan,
            RowSelectionQuery::Condition(cond) => {
                todo!()
            }
        };
        Ok(ret)
    }

    fn new_from_condition(
        vtable: &VTable,
        condition: &SingleTableCondition,
    ) -> ApllodbResult<Self> {
        todo!()
    }
}
