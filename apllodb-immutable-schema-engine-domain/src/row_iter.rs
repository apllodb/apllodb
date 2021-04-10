pub mod version_row_iter;

use std::fmt::Debug;

use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::{RowSchema, Rows};

use crate::{abstract_types::ImmutableSchemaAbstractTypes, row::immutable_row::ImmutableRow};

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIterator<Types: ImmutableSchemaAbstractTypes>:
    Iterator<Item = ImmutableRow> + Debug
{
    /// ref to schema
    fn schema(&self) -> RowSchema;

    /// Chain iterators from multiple versions.
    fn chain_versions(iters: impl IntoIterator<Item = Types::VersionRowIter>) -> Self;

    ///  Into<Rows>.
    fn into_rows(self, schema: RowSchema) -> ApllodbResult<Rows>;
}
