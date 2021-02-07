pub mod version_row_iter;

use std::fmt::Debug;

use apllodb_shared_components::RecordIterator;
use apllodb_storage_engine_interface::AliasDef;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, row::immutable_row::ImmutableRow};

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIterator<Types: ImmutableSchemaAbstractTypes>:
    Iterator<Item = ImmutableRow> + Debug
{
    /// Chain iterators from multiple versions.
    fn chain_versions(iters: impl IntoIterator<Item = Types::VersionRowIter>) -> Self;

    ///  Into<RecordIterator>.
    fn into_record_iterator(self, alias_def: AliasDef) -> RecordIterator;
}
