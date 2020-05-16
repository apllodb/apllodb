#[macro_export]
macro_rules! pk {
    ($($col_name: expr $(,)?)*) => {
        {
            use crate::data_structure::{ColumnName, TableConstraintKind};

            TableConstraintKind::PrimaryKey {
                column_names: vec![
                    $(
                        ColumnName::create($col_name).unwrap(),
                    )*
                ],
            }
        }
    };
}

#[macro_export]
macro_rules! unique {
    ($($col_name: expr $(,)?)*) => {
        {
            use crate::data_structure::{ColumnName, TableConstraintKind};

            TableConstraintKind::Unique {
                column_names: vec![
                    $(
                        ColumnName::create($col_name).unwrap(),
                    )*
                ],
            }
        }
    };
}
