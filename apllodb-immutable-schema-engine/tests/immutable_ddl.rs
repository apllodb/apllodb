mod test_support;

use crate::test_support::setup;
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    ColumnName, ColumnReference, FieldIndex, RecordIterator, SqlType, SqlValue,
    TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{
    DMLMethods, MethodsWithDb, MethodsWithTx, MethodsWithoutDb, ProjectionQuery, StorageEngine,
};

use pretty_assertions::assert_eq;
use test_support::database::test_database_name;

#[test]
fn test_success_select_column_available_only_in_1_of_2_versions() -> ApllodbResult<()> {
    setup();

    let mut engine = ApllodbImmutableSchemaEngine::default();
    let db = ApllodbImmutableSchemaEngine::without_db(&mut engine);
    let tx = ApllodbImmutableSchemaEngine::with_db(&mut engine);
    let ddl = ApllodbImmutableSchemaEngine::with_tx(&mut engine);
    let dml = ApllodbImmutableSchemaEngine::dml(&mut engine);
    let mut session = db.use_database(test_database_name())?;

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

    TransactionMethodsImpl::begin(&mut tx, &mut session)?;

    // v1
    // | id | c1 |
    // |----|----|
    ddl.create_table(&mut session, &t_name, &tc, coldefs)?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    dml.insert(
        &mut session,
        &t_name,
        RecordIterator::new(vec![record! {
            FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?,
            FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?
        }]),
    )?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    //
    // v2
    // | id |
    // |----|
    ddl.alter_table(
        &mut session,
        &t_name,
        &AlterTableAction::DropColumn {
            column_name: c1_def
                .column_data_type()
                .column_ref()
                .as_column_name()
                .clone(),
        },
    )?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    //
    // v2
    // | id |
    // |----|
    // | 2  |
    dml.insert(
        &mut session,
        &t_name,
        RecordIterator::new(vec![
            record! { FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &2i32)? },
        ]),
    )?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    // | 3  | 3  |
    //
    // v2
    // | id |
    // |----|
    // | 2  |
    dml.insert(
        &mut session,
        &t_name,
        RecordIterator::new(vec![record! {
            FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &3i32)?,
            FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &3i32)?
        }]),
    )?;

    // Selects both v1's record (id=1) and v2's record (id=2),
    // although v2 does not have column "c".
    let records = dml.select(&mut session, &t_name, ProjectionQuery::All)?;

    assert_eq!(records.clone().count(), 3);

    for record in records {
        let id: i32 = record
            .get(&FieldIndex::InColumnReference(
                c_id_def.column_data_type().column_ref().clone(),
            ))?
            .unwrap();
        match id {
            1 => assert_eq!(
                record.get::<i32>(&FieldIndex::InColumnReference(
                    c1_def.column_data_type().column_ref().clone()
                ))?,
                Some(1)
            ),
            3 => assert_eq!(
                record.get::<i32>(&FieldIndex::InColumnReference(
                    c1_def.column_data_type().column_ref().clone()
                ))?,
                Some(3)
            ),
            2 => {
                // Can fetch column `c1` from v2 and it's value is NULL.
                assert_eq!(
                    record.get::<i32>(&FieldIndex::InColumnReference(
                        c1_def.column_data_type().column_ref().clone()
                    ))?,
                    None
                );
            }
            _ => unreachable!(),
        }
    }

    tx.commit(&mut session)?;

    Ok(())
}
