mod column_name {
    #[macro_export]
    macro_rules! column_name {
        ($col_name: expr) => {{
            use crate::data_structure::ColumnName;

            ColumnName::new($col_name).unwrap()
        }};
    }
}

mod table_constraint_kind {
    #[macro_export]
    macro_rules! pk {
        ($($col_name: expr $(,)?)*) => {
            {
                use crate::{column_name, data_structure::TableConstraintKind};

                TableConstraintKind::PrimaryKey {
                    column_names: vec![
                        $(
                            column_name!($col_name),
                        )*
                    ],
                }
            }
        }
    }

    #[macro_export]
    macro_rules! unique {
        ($($col_name: expr $(,)?)*) => {
            {
                use crate::{column_name, data_structure::TableConstraintKind};

                TableConstraintKind::Unique {
                    column_names: vec![
                        $(
                            column_name!($col_name),
                        )*
                    ],
                }
            }
        }
    }
}
