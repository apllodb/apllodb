mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    data_structure::{
        ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName, Constant, DataType,
        DataTypeKind, Expression, TableConstraintKind, TableConstraints, TableName,
    },
    error::{ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

#[test]
fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
    setup();

    let mut db = TestDatabase::new()?;

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

    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

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
fn test_insert() -> ApllodbResult<()> {
    setup();

    use apllodb_storage_engine_interface::Row;

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

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
         c1_def.column_name().clone() => Expression::ConstantVariant(Constant::from(100))
        },
    )?;

    let mut rows = tx.select(
        &t_name,
        &vec![c_id_def.column_name().clone(), c1_def.column_name().clone()],
    )?;

    let row = rows.next().unwrap()?;
    assert_eq!(row.get::<i32>(c_id_def.column_name())?, 1);
    assert_eq!(row.get::<i32>(c1_def.column_name())?, 100);

    assert!(rows.next().is_none());

    tx.commit()?;

    Ok(())
}
