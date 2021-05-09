use apllodb_shared_components::{ApllodbError, SqlState, ApllodbResult};
use apllodb_storage_engine_interface::{
    ColumnDataType, ColumnDefinition, ColumnName, TableConstraintKind,
};
use serde::{Deserialize, Serialize};

/// A constraint parameter that set of record (not each record) must satisfy.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(super) enum TableWideConstraintKind {
    /// PRIMARY KEY ({column_name}, ...)
    PrimaryKey {
        column_data_types: Vec<ColumnDataType>,
    },

    /// UNIQUE ({column_name}, ...)
    /// It does not hold data type because data type info are held by Version.
    Unique { column_names: Vec<ColumnName> },
}

impl TableWideConstraintKind {
    pub fn new(
        column_definitions: &[ColumnDefinition],
        tck: &TableConstraintKind,
    ) -> ApllodbResult<Self> {
        let kind = match tck {
            TableConstraintKind::PrimaryKey { column_names } => {
                let pk_column_data_types = column_names.iter().map(|pk_cn| {
                    let pk_cd = column_definitions.iter().find(|cd| cd.column_data_type().column_name() == pk_cn).ok_or_else(||
                        ApllodbError::new(
                            SqlState::InvalidTableDefinition,
                            format!("column `{:?}` does not exist in ColumnDefinition while it is declared as PRIMARY KEY", pk_cn),
                            None,
                        )
                    )?;
                    Ok(pk_cd.column_data_type().clone())
                }).collect::<ApllodbResult<Vec<ColumnDataType>>>()?;

                Self::PrimaryKey {
                    column_data_types: pk_column_data_types,
                }
            }
            TableConstraintKind::Unique { column_names } => Self::Unique {
                column_names: column_names.clone(),
            },
        };
        Ok(kind)
    }
}
