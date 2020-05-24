mod column_constraints {
    #[macro_export]
    macro_rules! column_constraints {
        ($($column_constraint_kind: expr $(,)?)*) => {{
            use apllodb_shared_components::data_structure::{ColumnConstraints, ColumnConstraintKind};

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
            use crate::column_name;
            use apllodb_shared_components::data_structure::{
                ColumnDefinition, DataType, DataTypeKind,
            };

            ColumnDefinition::new(
                column_name!($col_name),
                DataType::new(DataTypeKind::Integer, false),
                $column_constraints,
            )
            .unwrap()
        }};
    }
}

mod column_name {
    #[macro_export]
    macro_rules! column_name {
        ($col_name: expr) => {{
            use apllodb_shared_components::data_structure::{ColumnName, ShortName};

            ColumnName::from(ShortName::new($col_name).unwrap())
        }};
    }
}

mod table_constraint_kind {
    #[macro_export]
    macro_rules! t_pk {
        ($($col_name: expr $(,)?)*) => {
            {
                use crate::column_name;
                use apllodb_shared_components::data_structure::TableConstraintKind;

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
    macro_rules! t_unique {
        ($($col_name: expr $(,)?)*) => {
            {
                use crate::column_name;
                use apllodb_shared_components::data_structure::TableConstraintKind;

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

mod table_constraints {
    #[macro_export]
    macro_rules! table_constraints {
        ($($table_constraint_kind: expr $(,)?)*) => {{
            use apllodb_shared_components::data_structure::{TableConstraintKind, TableConstraints};

            let kinds: Vec<TableConstraintKind> = vec![
                $($table_constraint_kind,)*
            ];
            TableConstraints::new(kinds).unwrap()
        }}
    }
}

mod version {
    #[macro_export]
    macro_rules! next_version_action_drop_column {
        ($col_name: expr $(,)?) => {{
            use crate::column_name;
            use apllodb_shared_components::data_structure::AlterTableAction;

            AlterTableAction::DropColumn {
                column_name: column_name!($col_name),
            }
        }};
    }
}
