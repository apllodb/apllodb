mod column_constraints {
    #[macro_export]
    macro_rules! column_constraints {
        ($($column_constraint_kind: expr $(,)?)*) => {{
            use apllo_shared_components::data_structure::{ColumnConstraints, ColumnConstraintKind};

            let kinds: Vec<ColumnConstraintKind> = vec![
                $($column_constraint_kind,)*
            ];
            ColumnConstraints::new(kinds).unwrap()
        }}
    }
}

mod column_definition {
    #[macro_export]
    macro_rules! column_definition {
        ($col_name: expr, $column_constraints: expr $(,)?) => {{
            use apllo_shared_components::data_structure::{ColumnName, DataType, DataTypeKind};

            ColumnDefinition::new(
                ColumnName::from(ShortName::new($col_name).unwrap()),
                DataType::new(DataTypeKind::Integer, false),
                $column_constraints,
            )
            .unwrap()
        }};
    }
}

mod table_constraint_kind {
    #[macro_export]
    macro_rules! t_pk {
        ($($col_name: expr $(,)?)*) => {
            {
                use apllo_shared_components::data_structure::{ColumnName, TableConstraintKind};

                TableConstraintKind::PrimaryKey {
                    column_names: vec![
                        $(
                            ColumnName::from(ShortName::new($col_name).unwrap()),
                        )*
                    ],
                }
            }
        }
    }

    #[macro_export]
    macro_rules! t_unique {
        ($($col_name: expr $(,)?)*) => {
            {
                use apllo_shared_components::data_structure::{ColumnName, TableConstraintKind};

                TableConstraintKind::Unique {
                    column_names: vec![
                        $(
                            ColumnName::from(ShortName::new($col_name).unwrap()),
                        )*
                    ],
                }
            }
        }
    }
}

mod table_constraints {
    #[macro_export]
    macro_rules! table_constraints {
        ($($table_constraint_kind: expr $(,)?)*) => {{
            use apllo_shared_components::data_structure::{TableConstraintKind, TableConstraints};

            let kinds: Vec<TableConstraintKind> = vec![
                $($table_constraint_kind,)*
            ];
            TableConstraints::new(kinds).unwrap()
        }}
    }
}
