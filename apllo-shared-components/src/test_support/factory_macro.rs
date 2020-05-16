#[macro_export]
macro_rules! pk {
    ($($col_name: expr $(,)?)*) => {
        TableConstraintKind::PrimaryKey {
            column_names: vec![
                $(
                    ColumnName::create($col_name).unwrap(),
                )*
            ],
        }
    };
}

#[macro_export]
macro_rules! unique {
    ($($col_name: expr $(,)?)*) => {
        TableConstraintKind::Unique {
            column_names: vec![
                $(
                    ColumnName::create($col_name).unwrap(),
                )*
            ],
        }
    };
}
