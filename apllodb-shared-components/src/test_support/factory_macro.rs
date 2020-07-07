#[macro_export]
macro_rules! alter_table_action_drop_column {
    ($col_name: expr $(,)?) => {{
        $crate::data_structure::AlterTableAction::DropColumn {
            column_name: $crate::column_name!($col_name),
        }
    }};
}

#[macro_export]
macro_rules! column_constraints {
    ($($column_constraint_kind: expr $(,)?)*) => {{
        let kinds: Vec<$crate::data_structure::ColumnConstraintKind> = vec![
            $($column_constraint_kind,)*
        ];
        $crate::data_structure::ColumnConstraints::new(kinds).unwrap()
    }}
}

#[macro_export]
macro_rules! column_definition {
    ($col_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {{
        $crate::data_structure::ColumnDefinition::new(
            $crate::column_name!($col_name),
            $data_type,
            $column_constraints,
        )
        .unwrap()
    }};
}

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

#[macro_export]
macro_rules! column_name {
    ($col_name: expr) => {{
        $crate::data_structure::ColumnName::new($col_name).unwrap()
    }};
}

#[macro_export]
macro_rules! database_name {
    ($db_name: expr) => {{
        $crate::data_structure::DatabaseName::new($db_name).unwrap()
    }};
}

#[macro_export]
macro_rules! data_type {
    ($kind: expr, $nullable: expr $(,)?) => {{
        $crate::data_structure::DataType::new($kind, $nullable)
    }};
}

#[macro_export]
macro_rules! const_expr {
    ($constant: expr) => {{
        $crate::data_structure::Expression::ConstantVariant($crate::data_structure::Constant::from(
            $constant,
        ))
    }};
}

/// Expression factory from column name.
#[macro_export]
macro_rules! column_name_expr {
    ($col_name: expr) => {{
        $crate::data_structure::Expression::ColumnNameVariant(
            $crate::data_structure::ColumnName::new($col_name).unwrap(),
        )
    }};
}

#[macro_export]
macro_rules! t_pk {
    ($($col_name: expr $(,)?)*) => {{
        $crate::data_structure::TableConstraintKind::PrimaryKey {
            column_data_types: vec![
                $(
                    $crate::data_structure::ColumnDataType::from(
                        &$crate::column_definition!(
                            $col_name,
                            $crate::data_structure::DataType::new($crate::data_structure::DataTypeKind::Integer, false),
                            $crate::column_constraints!()
                        )
                    ),
                )*
            ],
        }
    }};
}

#[macro_export]
macro_rules! t_unique {
    ($($col_name: expr $(,)?)*) => {{
        $crate::data_structure::TableConstraintKind::Unique {
            column_names: vec![
                $(
                    $crate::column_name!($col_name),
                )*
            ],
        }
    }}
}

#[macro_export]
macro_rules! table_constraints {
    ($($table_constraint_kind: expr $(,)?)*) => {{
        let kinds: Vec<$crate::data_structure::TableConstraintKind> = vec![
            $($table_constraint_kind,)*
        ];
        $crate::data_structure::TableConstraints::new(kinds).unwrap()
    }}
}

#[macro_export]
macro_rules! table_name {
    ($table_name: expr) => {{
        $crate::data_structure::TableName::new($table_name).unwrap()
    }};
}
