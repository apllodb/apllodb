mod test_support;

use crate::test_support::setup;
use apllodb_shared_components::{
    ApllodbErrorKind, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    ColumnName, ColumnReference, Expression, FieldIndex, RecordIterator, SqlType, SqlValue,
    TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::ProjectionQuery;

#[test]
fn test_create_table_success() -> ApllodbResult<()> {
    setup();

    // let mut db = TestDatabase::new()?;
    // let mut tx = ApllodbImmutableSchemaTx::begin(&mut db.0)?;

    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::default(),
    );

    // let ddl = ApllodbImmutableSchemaDDL::default();

    // ddl.create_table(
    //     &mut tx,
    //     &t_name,
    //     &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
    //         column_names: vec![c1_def
    //             .column_data_type()
    //             .column_ref()
    //             .as_column_name()
    //             .clone()],
    //     }])?,
    //     vec![c1_def],
    // )?;
    // tx.abort()?;

    Ok(())
}

#[test]
fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
    setup();

    // let mut db = TestDatabase::new()?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c1_def
            .column_data_type()
            .column_ref()
            .as_column_name()
            .clone()],
    }])?;

    // let mut tx = ApllodbImmutableSchemaTx::begin(&mut db.0)?;

    // let ddl = ApllodbImmutableSchemaDDL::default();

    // ddl.create_table(&mut tx, &t_name, &tc, coldefs.clone())?;
    // match ddl.create_table(&mut tx, &t_name, &tc, coldefs) {
    //     // Internally, new record is trying to be INSERTed but it is made wait by tx2.
    //     // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
    //     Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DuplicateTable),
    //     Ok(_) => panic!("should rollback"),
    // }
    Ok(())
}

#[test]
fn test_insert() -> ApllodbResult<()> {
    setup();

    // let mut db = TestDatabase::new()?;
    // let mut tx = ApllodbImmutableSchemaTx::begin(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("id")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def
            .column_data_type()
            .column_ref()
            .as_column_name()
            .clone()],
    }])?;

    // let ddl = ApllodbImmutableSchemaDDL::default();
    // let dml = ApllodbImmutableSchemaDML::default();

    // ddl.create_table(&mut tx, &t_name, &tc, coldefs)?;

    // dml.insert(
    //     &mut tx,
    //     &t_name,
    //     RecordIterator::new(vec![record! {
    //         FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?,
    //         FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &100i32)?
    //     }]),
    // )?;

    // let mut records = dml.select(&mut tx, &t_name, ProjectionQuery::All)?;

    // let record = records.next().unwrap();
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c_id_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(1)
    // );
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c1_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(100)
    // );

    // assert!(records.next().is_none());

    // tx.commit()?;

    Ok(())
}

#[test]
fn test_update() -> ApllodbResult<()> {
    setup();

    // let mut db = TestDatabase::new()?;
    // let mut tx = ApllodbImmutableSchemaTx::begin(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("id")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def
            .column_data_type()
            .column_ref()
            .as_column_name()
            .clone()],
    }])?;

    // let ddl = ApllodbImmutableSchemaDDL::default();
    // let dml = ApllodbImmutableSchemaDML::default();

    // ddl.create_table(&mut tx, &t_name, &tc, coldefs)?;

    // dml.insert(
    //     &mut tx,
    //     &t_name,
    //     RecordIterator::new(vec![record! {
    //         FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?,
    //         FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &100i32)?
    //     }]),
    // )?;
    // let mut records = dml.select(&mut tx, &t_name, ProjectionQuery::All)?;
    // let record = records.next().unwrap();
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c_id_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(1)
    // );
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c1_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(100)
    // );
    // assert!(records.next().is_none());

    // // update non-PK
    // dml.update(
    //     &mut tx,
    //     &t_name,
    //     hmap! {
    //         c1_def.column_data_type().column_ref().as_column_name().clone() => Expression::ConstantVariant(SqlValue::pack(SqlType::integer(), &200)?)
    //     },
    // )?;
    // let mut records = dml.select(&mut tx, &t_name, ProjectionQuery::All)?;
    // let record = records.next().unwrap();
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c_id_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(1)
    // );
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c1_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(200)
    // );
    // assert!(records.next().is_none());

    // // update PK
    // dml.update(
    //     &mut tx,
    //     &t_name,
    //     hmap! {
    //         c_id_def.column_data_type().column_ref().as_column_name().clone() => Expression::ConstantVariant(SqlValue::pack(SqlType::integer(), &2)?)
    //     },
    // )?;
    // let mut records = dml.select(&mut tx, &t_name, ProjectionQuery::All)?;
    // let record = records.next().unwrap();
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c_id_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(2)
    // );
    // assert_eq!(
    //     record.get::<i32>(&FieldIndex::InColumnReference(
    //         c1_def.column_data_type().column_ref().clone()
    //     ))?,
    //     Some(200)
    // );
    // assert!(records.next().is_none());

    // tx.commit()?;

    Ok(())
}

#[test]
fn test_delete() -> ApllodbResult<()> {
    setup();

    // let mut db = TestDatabase::new()?;
    // let mut tx = ApllodbImmutableSchemaTx::begin(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("id")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def
            .column_data_type()
            .column_ref()
            .as_column_name()
            .clone()],
    }])?;

    // let ddl = ApllodbImmutableSchemaDDL::default();
    // let dml = ApllodbImmutableSchemaDML::default();

    // ddl.create_table(&mut tx, &t_name, &tc, coldefs)?;

    // dml.insert(
    //     &mut tx,
    //     &t_name,
    //     RecordIterator::new(vec![record! {
    //         FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?,
    //         FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &100i32)?
    //     }]),
    // )?;

    // let rows = dml.select(
    //     &mut tx,
    //     &t_name,
    //     ProjectionQuery::ColumnNames(vec![c_id_def
    //         .column_data_type()
    //         .column_ref()
    //         .as_column_name()
    //         .clone()]),
    // )?;
    // assert_eq!(rows.count(), 1);

    // dml.delete(&mut tx, &t_name)?;
    // let rows = dml.select(
    //     &mut tx,
    //     &t_name,
    //     ProjectionQuery::ColumnNames(vec![c_id_def
    //         .column_data_type()
    //         .column_ref()
    //         .as_column_name()
    //         .clone()]),
    // )?;
    // assert_eq!(rows.count(), 0);

    // tx.commit()?;

    Ok(())
}
