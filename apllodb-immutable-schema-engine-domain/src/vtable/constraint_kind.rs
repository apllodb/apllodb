use apllodb_shared_components::data_structure::{
    ColumnConstraintKind, ColumnDataType, ColumnDefinition, ColumnName, TableConstraintKind,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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

impl From<&TableConstraintKind> for TableWideConstraintKind {
    fn from(tck: &TableConstraintKind) -> Self {
        match tck {
            TableConstraintKind::PrimaryKey { column_data_types } => Self::PrimaryKey {
                column_data_types: column_data_types.clone(),
            },
            TableConstraintKind::Unique { column_names } => Self::Unique {
                column_names: column_names.clone(),
            },
        }
    }
}

impl TryFrom<&ColumnDefinition> for TableWideConstraintKind {
    /// Simply means that the ColumnDefinition does not include any version set constraint.
    type Error = ();
    fn try_from(cd: &ColumnDefinition) -> Result<Self, Self::Error> {
        let column_name = cd.column_name();

        #[allow(clippy::never_loop)]
        for kind in cd.column_constraints().kinds() {
            match kind {
                ColumnConstraintKind::PrimaryKey => {
                    return Ok(Self::PrimaryKey {
                        column_data_types: vec![cd.column_data_type().clone()],
                    })
                }
                ColumnConstraintKind::Unique => {
                    return Ok(Self::Unique {
                        column_names: vec![column_name.clone()],
                    })
                }
            }
        }

        Err(())
    }
}
