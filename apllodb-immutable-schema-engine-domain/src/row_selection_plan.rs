use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    version_revision_resolver::vrr_entries::VrrEntries,
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
