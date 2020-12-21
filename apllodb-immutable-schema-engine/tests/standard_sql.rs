mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    data_structure::{
        ColumnConstraints, ColumnDefinition, ColumnName, ColumnReference, Constant, DataType,
        DataTypeKind, Expression, TableConstraintKind, TableConstraints, TableName,
    },
    error::{ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::{ProjectionQuery, Row, StorageEngine, Transaction};

#[test]
fn test_create_table_success() -> ApllodbResult<()> {
    setup();

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::default(),
    )?;

    tx.create_table(
        &t_name,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_names: vec![c1_def.column_ref().as_column_name().clone()],
        }])?,
        &[c1_def],
    )?;
    tx.abort()?;

    Ok(())
}

#[test]
fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
    setup();

    let mut db = TestDatabase::new()?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c1_def.column_ref().as_column_name().clone()],
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

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("id")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let c1_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def.column_ref().as_column_name().clone()],
    }])?;

    tx.create_table(&t_name, &tc, &coldefs)?;

    tx.insert(
        &t_name,
        hmap! {
         c_id_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(1)),
         c1_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(100))
        },
    )?;

    let mut rows = tx.select(&t_name, ProjectionQuery::All)?;

    let mut row = rows.next().unwrap();
    assert_eq!(row.get::<i32>(c_id_def.column_ref())?, 1);
    assert_eq!(row.get::<i32>(c1_def.column_ref())?, 100);

    assert!(rows.next().is_none());

    tx.commit()?;

    Ok(())
}

#[test]
fn test_update() -> ApllodbResult<()> {
    setup();

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("id")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let c1_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def.column_ref().as_column_name().clone()],
    }])?;

    tx.create_table(&t_name, &tc, &coldefs)?;

    tx.insert(
        &t_name,
        hmap! {
         c_id_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(1)),
         c1_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(100))
        },
    )?;
    let mut rows = tx.select(&t_name, ProjectionQuery::All)?;
    let mut row = rows.next().unwrap();
    assert_eq!(row.get::<i32>(c_id_def.column_ref())?, 1);
    assert_eq!(row.get::<i32>(c1_def.column_ref())?, 100);
    assert!(rows.next().is_none());

    // update non-PK
    tx.update(
        &t_name,
        hmap! {
            c1_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(200))
        },
    )?;
    let mut rows = tx.select(&t_name, ProjectionQuery::All)?;
    let mut row = rows.next().unwrap();
    assert_eq!(row.get::<i32>(c_id_def.column_ref())?, 1);
    assert_eq!(row.get::<i32>(c1_def.column_ref())?, 200);
    assert!(rows.next().is_none());

    // update PK
    tx.update(
        &t_name,
        hmap! {
            c_id_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(2))
        },
    )?;
    let mut rows = tx.select(&t_name, ProjectionQuery::All)?;
    let mut row = rows.next().unwrap();
    assert_eq!(row.get::<i32>(c_id_def.column_ref())?, 2);
    assert_eq!(row.get::<i32>(c1_def.column_ref())?, 200);
    assert!(rows.next().is_none());

    tx.commit()?;

    Ok(())
}

#[test]
fn test_delete() -> ApllodbResult<()> {
    setup();

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("id")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let c1_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def.column_ref().as_column_name().clone()],
    }])?;

    tx.create_table(&t_name, &tc, &coldefs)?;

    tx.insert(
        &t_name,
        hmap! {
         c_id_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(1)),
         c1_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(100))
        },
    )?;

    let rows = tx.select(
        &t_name,
        ProjectionQuery::ColumnNames(vec![c_id_def.column_ref().as_column_name().clone()]),
    )?;
    assert_eq!(rows.count(), 1);

    tx.delete(&t_name)?;
    let rows = tx.select(
        &t_name,
        ProjectionQuery::ColumnNames(vec![c_id_def.column_ref().as_column_name().clone()]),
    )?;
    assert_eq!(rows.count(), 0);

    tx.commit()?;

    Ok(())
}
