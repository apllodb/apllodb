pub mod version_row_iter;

use std::fmt::Debug;

use apllodb_shared_components::{RecordFieldRefSchema, Records};

use crate::{abstract_types::ImmutableSchemaAbstractTypes, row::immutable_row::ImmutableRow};

use self::version_row_iter::row_column_ref_schema::RowColumnRefSchema;

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIterator<Types: ImmutableSchemaAbstractTypes>:
    Iterator<Item = ImmutableRow> + Debug
{
    /// ref to schema
    fn schema(&self) -> RowColumnRefSchema;

    /// Chain iterators from multiple versions.
    fn chain_versions(iters: impl IntoIterator<Item = Types::VersionRowIter>) -> Self;

    ///  Into<RecordIterator>.
    fn into_record_iterator(self, schema: RecordFieldRefSchema) -> Records;
}
