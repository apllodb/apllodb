#[macro_export]
macro_rules! alter_table_action_drop_column {
    ($col_name: expr $(,)?) => {{
        use crate::column_name;
        use crate::data_structure::AlterTableAction;

        AlterTableAction::DropColumn {
            column_name: column_name!($col_name),
        }
    }};
}

#[macro_export]
macro_rules! column_constraints {
    ($($column_constraint_kind: expr $(,)?)*) => {{
        use crate::data_structure::{ColumnConstraints, ColumnConstraintKind};

        let kinds: Vec<ColumnConstraintKind> = vec![
            $($column_constraint_kind,)*
        ];
        ColumnConstraints::new(kinds).unwrap()
    }}
}

#[macro_export]
macro_rules! column_definition {
    ($col_name: expr, $data_type: expr, $column_constraints: expr $(,)?) => {{
        use crate::column_name;
        use crate::data_structure::ColumnDefinition;

        ColumnDefinition::new(column_name!($col_name), $data_type, $column_constraints).unwrap()
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
        use crate::data_structure::ColumnName;

        ColumnName::new($col_name).unwrap()
    }};
}

#[macro_export]
macro_rules! database_name {
    ($col_name: expr) => {{
        use crate::data_structure::{DatabaseName, ShortName};

        DatabaseName::from(ShortName::new($col_name).unwrap())
    }};
}

#[macro_export]
macro_rules! data_type {
    ($kind: expr, $nullable: expr $(,)?) => {{
        use crate::data_structure::DataType;

        DataType::new($kind, $nullable)
    }};
}

#[macro_export]
macro_rules! const_expr {
    ($constant: expr) => {{
        use crate::data_structure::{Constant, Expression};

        Expression::ConstantVariant(Constant::from($constant))
    }};
}

/// Expression factory from column name.
#[macro_export]
macro_rules! column_name_expr {
    ($col_name: expr) => {{
        use crate::data_structure::{ColumnName, Expression};

        Expression::ColumnNameVariant(ColumnName::new($col_name).unwrap())
    }};
}

#[macro_export]
macro_rules! t_pk {
    ($($col_name: expr $(,)?)*) => {{
        use crate::column_name;
        use crate::data_structure::TableConstraintKind;

        TableConstraintKind::PrimaryKey {
            column_names: vec![
                $(
                    column_name!($col_name),
                )*
            ],
        }
    }}
}

#[macro_export]
macro_rules! t_unique {
    ($($col_name: expr $(,)?)*) => {{
        use crate::column_name;
        use crate::data_structure::TableConstraintKind;

        TableConstraintKind::Unique {
            column_names: vec![
                $(
                    column_name!($col_name),
                )*
            ],
        }
    }}
}

#[macro_export]
macro_rules! table_constraints {
    ($($table_constraint_kind: expr $(,)?)*) => {{
        use crate::data_structure::{TableConstraintKind, TableConstraints};

        let kinds: Vec<TableConstraintKind> = vec![
            $($table_constraint_kind,)*
        ];
        TableConstraints::new(kinds).unwrap()
    }}
}

#[macro_export]
macro_rules! table_name {
    ($table_name: expr) => {{
        use crate::data_structure::TableName;

        TableName::new($table_name).unwrap()
    }};
}
