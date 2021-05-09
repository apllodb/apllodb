use std::sync::Arc;

use crate::field::aliased_field_name::AliasedFieldName;

use super::{record_index::RecordIndex, record_schema::RecordSchema};
use apllodb_shared_components::{
    SqlState, ApllodbResult, RPos, Schema, SchemaIndex, SqlConvertible, SqlValue,
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
    /// - [NameErrorNotFound](apllodb-shared-components::SqlState::NameErrorNotFound) when:
    ///   - Specified field does not exist in this record.
    pub fn get<T: SqlConvertible>(&self, index: &RecordIndex) -> ApllodbResult<Option<T>> {
        self.row.get(self.pos(index)?)
    }

    /// Get sequence of field name vs SqlValue.
    pub fn into_name_values(self) -> Vec<(String, SqlValue)> {
        self.schema
            .to_aliased_field_names()
            .into_iter()
            .map(|afn| format!("{}", SchemaIndex::from(&afn)))
            .zip(self.row.into_values())
            .collect()
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

    pub(crate) fn helper_get_sql_value(
        &self,
        joined_name: &AliasedFieldName,
    ) -> Option<ApllodbResult<SqlValue>> {
        self.get_sql_value(&SchemaIndex::from(joined_name))
            .map_or_else(
                |e| {
                    if matches!(e.kind(), SqlState::NameErrorNotFound) {
                        None
                    } else {
                        Some(Err(e))
                    }
                },
                |sql_value| Some(Ok(sql_value.clone())),
            )
    }
}
