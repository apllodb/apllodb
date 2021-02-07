pub(crate) mod row_column_ref_schema;

use crate::row::immutable_row::ImmutableRow;
use std::fmt::Debug;

/// Row iterator from a single version.
pub trait VersionRowIterator: Iterator<Item = ImmutableRow> + Debug + Sized {}
