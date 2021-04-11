use std::sync::Arc;

use super::{record_index::RecordIndex, record_schema::RecordSchema};
use apllodb_shared_components::{
    ApllodbResult, RPos, Schema, SchemaIndex, SqlConvertible, SqlValue,
};
use apllodb_storage_engine_interface::Row;

/// Record. Clients, servers, and SQL Processor use this.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Record {
    pub(crate) schema: Arc<RecordSchema>,
    pub(crate) row: Row,
}

impl Record {
    pub(crate) fn new(schema: Arc<RecordSchema>, row: Row) -> Self {
        Self { schema, row }
    }

    /// Get Rust value from rec field.
    ///
    /// Returns `None` for NULL.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb-shared-components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get<T: SqlConvertible>(&self, index: &RecordIndex) -> ApllodbResult<Option<T>> {
        self.row.get(self.pos(index)?)
    }

    pub(crate) fn get_sql_value(&self, index: &SchemaIndex) -> ApllodbResult<&SqlValue> {
        self.row
            .get_sql_value(self.pos(&RecordIndex::Name(index.clone()))?)
    }

    fn pos(&self, index: &RecordIndex) -> ApllodbResult<RPos> {
        match index {
            RecordIndex::Pos(pos) => Ok(*pos),
            RecordIndex::Name(index) => {
                let (pos, _) = self.schema.index(index)?;
                Ok(pos)
            }
        }
    }
}
