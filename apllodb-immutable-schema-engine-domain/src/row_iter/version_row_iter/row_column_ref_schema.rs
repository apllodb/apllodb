use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, CorrelationReference,
    FieldReference, FullFieldReference, RecordFieldRefSchema, TableName,
};
use apllodb_storage_engine_interface::AliasDef;
use serde::{Deserialize, Serialize};

/// Internally has similar structure as `Vec<ColumnColumn>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RowColumnRefSchema {
    table_name: TableName,
    column_names: Vec<ColumnName>,
}

impl RowColumnRefSchema {
    /// Constructor
    pub fn new(table_name: TableName, column_names: Vec<ColumnName>) -> Self {
        Self {
            table_name,
            column_names,
        }
    }

    pub fn into_column_names(self) -> Vec<ColumnName> {
        self.column_names
    }

    pub fn into_record_schema(self, alias_def: AliasDef) -> RecordFieldRefSchema {
        let correlation_reference = match alias_def.table_alias() {
            None => CorrelationReference::TableNameVariant(self.table_name),
            Some(table_alias) => CorrelationReference::TableAliasVariant {
                table_name: self.table_name,
                alias_name: table_alias.clone(),
            },
        };

        let ffrs: Vec<FullFieldReference> = self
            .column_names
            .into_iter()
            .map(|column_name| {
                let column_alias = alias_def.column_aliases().get(&column_name);
                let field_reference = match column_alias {
                    None => FieldReference::ColumnNameVariant(column_name),
                    Some(column_alias) => FieldReference::ColumnAliasVariant {
                        column_name,
                        alias_name: column_alias.clone(),
                    },
                };
                FullFieldReference::new(correlation_reference.clone(), field_reference)
            })
            .collect();

        RecordFieldRefSchema::new(ffrs)
    }

    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same [ColumnReference](apllodb_shared_components::ColumnReference) is already in this row.
    pub(crate) fn append(&mut self, column_name: ColumnName) -> ApllodbResult<()> {
        if self
            .column_names
            .iter()
            .find(|cn| cn == &&column_name)
            .is_some()
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{}` is already in this row", column_name.as_str()),
                None,
            ))
        } else {
            self.column_names.push(column_name);
            Ok(())
        }
    }

    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb-shared-components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` does not exist in this row.
    pub(crate) fn resolve_index_with_rm(
        &mut self,
        column_name: &ColumnName,
    ) -> ApllodbResult<usize> {
        let idx = self
            .column_names
            .iter()
            .position(|cn| cn == column_name)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!(
                        "column named `{}` does not exist in this row",
                        column_name.as_str()
                    ),
                    None,
                )
            })?;

        self.column_names.remove(idx);
        Ok(idx)
    }
}
