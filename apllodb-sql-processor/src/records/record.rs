use std::sync::Arc;

use apllodb_shared_components::{ApllodbResult, SqlConvertible};
use apllodb_storage_engine_interface::Row;

use super::{record_index::RecordIndex, record_schema::RecordSchema};

/// Record. Clients, servers, and SQL Processor use this.
pub struct Record {
    pub(crate) schema: Arc<RecordSchema>,
    pub(crate) row: Row,
}

impl Record {
    /// Get Rust value from rec field.
    ///
    /// Returns `None` for NULL.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb-shared-components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get<T: SqlConvertible>(&self, index: &RecordIndex) -> ApllodbResult<Option<T>> {
        let (pos, _) = self.schema.index(index)?;
        self.record.get(pos)
    }
}
