use apllodb_shared_components::{
    data_structure::{
        ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName, DataType, DataTypeKind,
        TableConstraintKind,
    },
    error::{ApllodbErrorKind, ApllodbResult},
};

fn setup() {
    let _ = env_logger::builder().is_test(true).try_init();
}

/// Creates HashMap.
///
/// # Examples
///
/// ```
/// use apllodb_shared_components::hmap;
/// use std::collections::HashMap;
///
/// let h = hmap! { "k" => "v" };
///
/// let mut h2: HashMap<&str, &str> = HashMap::new();
/// h2.insert("k", "v");
///
/// assert_eq!(h, h2);
/// ```
#[macro_export]
macro_rules! hmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
    use apllodb_shared_components::data_structure::{DatabaseName, TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(
        "db_test_use_apllodb_immutable_schema_engine",
    )?)?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db)?;
    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_data_types: vec![ColumnDataType::new(
                ColumnName::new("c1")?,
                DataType::new(DataTypeKind::Integer, false),
            )],
        }])?,
        &vec![ColumnDefinition::new(
            ColumnName::new("c1")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::default(),
        )?],
    )?;
    tx.abort()?;

    Ok(())
}

// -------------------    #[test]

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::data_structure::{
    AlterTableAction, Constant, DatabaseName, Expression, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

#[test]
fn test_tx_id_order() -> ApllodbResult<()> {
    setup();

    let mut db1 =
        ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new("db_test_tx_id_order")?)?;
    let mut db2 =
        ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new("db_test_tx_id_order")?)?;

    let tx1 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db1)?;
    let tx2 = ApllodbImmutableSchemaEngine::begin_transaction(&mut db2)?;

    assert!(tx1.id() < tx2.id());

    Ok(())
}

#[test]
fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
    setup();

    let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(
        "test_create_table_failure_duplicate_table",
    )?)?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnName::new("c1")?,
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_data_types: vec![ColumnDataType::from(&c1_def)],
    }])?;

    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db)?;

    tx.create_table(&t_name, &tc, &coldefs)?;
    match tx.create_table(&t_name, &tc, &coldefs) {
        // Internally, new record is trying to be INSERTed but it is made wait by tx2.
        // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
        Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DuplicateTable),
        Ok(_) => panic!("should rollback"),
    }
    Ok(())
}

#[test]
fn test_success_select_all_from_2_versions() -> ApllodbResult<()> {
    setup();

    use apllodb_storage_engine_interface::Row;

    let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(
        "test_success_select_all_from_2_versions",
    )?)?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnName::new("id")?,
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let c1_def = ColumnDefinition::new(
        ColumnName::new("c1")?,
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_data_types: vec![ColumnDataType::from(&c_id_def)],
    }])?;

    tx.create_table(&t_name, &tc, &coldefs)?;

    tx.insert(
        &t_name,
        hmap! {
         c_id_def.column_name().clone() => Expression::ConstantVariant(Constant::from(1)),
         c1_def.column_name().clone() => Expression::ConstantVariant(Constant::from(1))
        },
    )?;

    tx.alter_table(
        &t_name,
        &AlterTableAction::DropColumn {
            column_name: c1_def.column_name().clone(),
        },
    )?;

    tx.insert(
        &t_name,
        hmap! { c_id_def.column_name().clone() => Expression::ConstantVariant(Constant::from(2)) },
    )?;

    // Selects both v1's record (id=1) and v2's record (id=2),
    // although v2 does not have column "c".
    let rows = tx.select(
        &t_name,
        &vec![c_id_def.column_name().clone(), c1_def.column_name().clone()],
    )?;

    for row_res in rows {
        let row = row_res?;
        let id: i32 = row.get(c_id_def.column_name())?;
        match id {
            1 => assert_eq!(row.get::<i32>(c1_def.column_name())?, 1),
            2 => {
                // Cannot fetch column `c` from v2. Note that v2's `c` is different from NULL,
                // although it is treated similarly to NULL in GROUP BY, ORDER BY operations.
                match row.get::<i32>(c1_def.column_name()) {
                    Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::UndefinedColumn),
                    _ => unreachable!(),
                };
            }
            _ => unreachable!(),
        }
    }

    tx.commit()?;

    Ok(())
}
