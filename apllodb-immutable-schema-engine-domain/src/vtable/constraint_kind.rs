use crate::row::column::pk_column::column_data_type::PKColumnDataType;
use apllodb_shared_components::{
    data_structure::{ColumnDefinition, ColumnName, TableConstraintKind},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{Deserialize, Serialize};

/// A constraint parameter that set of record (not each record) must satisfy.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(super) enum TableWideConstraintKind {
    /// PRIMARY KEY ({column_name}, ...)
    PrimaryKey {
        column_data_types: Vec<PKColumnDataType>,
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
                    let pk_cd = column_definitions.iter().find(|cd| cd.column_name() == pk_cn).ok_or_else(||
                        ApllodbError::new(
                            ApllodbErrorKind::InvalidTableDefinition,
                            format!("column `{}` does not exist in ColumnDefinition while it is declared as PRIMARY KEY", pk_cn),
                            None,
                        )
                    )?;
                    Ok(PKColumnDataType::from(pk_cd.column_data_type()))
                }).collect::<ApllodbResult<Vec<PKColumnDataType>>>()?;

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
