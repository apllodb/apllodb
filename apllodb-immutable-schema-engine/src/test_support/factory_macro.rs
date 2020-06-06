mod alter_table_action {
    /// AlterTableAction::DropColumn factory.
    #[macro_export]
    macro_rules! alter_table_action_drop_column {
        ($col_name: expr $(,)?) => {{
            use crate::column_name;
            use apllodb_shared_components::data_structure::AlterTableAction;

            AlterTableAction::DropColumn {
                column_name: column_name!($col_name),
            }
        }};
    }
}

mod column_constraints {
    /// ColumnConstraints factory.
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
    /// ColumnDefinition factory.
    #[macro_export]
    macro_rules! column_definition {
        ($col_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {{
            use crate::column_name;
            use apllodb_shared_components::data_structure::ColumnDefinition;

            ColumnDefinition::new(column_name!($col_name), $data_type, $column_constraints).unwrap()
        }};
    }

    /// Vec<ColumnDefinition> factory.
    #[macro_export]
    macro_rules! column_definitions {
        ($($column_definition: expr $(,)?)*) => {{
            vec![
                $(
                    $column_definition,
                )*
            ]
        }}
    }
}

mod column_name {
    /// ColumnName factory.
    #[macro_export]
    macro_rules! column_name {
        ($col_name: expr) => {{
            use apllodb_shared_components::data_structure::{ColumnName, ShortName};

            ColumnName::from(ShortName::new($col_name).unwrap())
        }};
    }
}

mod database_name {
    /// DatabaseName factory.
    #[macro_export]
    macro_rules! database_name {
        ($col_name: expr) => {{
            use apllodb_shared_components::data_structure::{DatabaseName, ShortName};

            DatabaseName::from(ShortName::new($col_name).unwrap())
        }};
    }
}

mod data_type {
    /// DataType factory.
    #[macro_export]
    macro_rules! data_type {
        ($kind: expr, $nullable: expr $(,)?) => {{
            use apllodb_shared_components::data_structure::DataType;

            DataType::new($kind, $nullable)
        }};
    }
}

mod expression {
    /// DataType factory.
    #[macro_export]
    macro_rules! const_expr {
        ($constant: expr) => {{
            use apllodb_shared_components::data_structure::{Constant, Expression};

            Expression::ConstantVariant(Constant::from($constant))
        }};
    }
}

mod table_constraint_kind {
    /// TableConstraintKind::PrimaryKey factory.
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

    /// TableConstraintKind::Unique factory.
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
    /// TableConstraints factory.
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

mod table_name {
    /// TableName factory.
    #[macro_export]
    macro_rules! table_name {
        ($table_name: expr) => {{
            use apllodb_shared_components::data_structure::{ShortName, TableName};

            TableName::from(ShortName::new($table_name).unwrap())
        }};
    }
}
