mod test_support;

use crate::test_support::setup;
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::session_with_tx;
use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    ColumnName, ColumnReference, FieldIndex, RecordIterator, SqlType, SqlValue,
    TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, WithTxMethods};

#[async_std::test]
async fn test_success_select_column_available_only_in_1_of_2_versions() -> ApllodbResult<()> {
    setup();

    let engine = ApllodbImmutableSchemaEngine::new();
    let session = session_with_tx(&engine).await?;

    let t_name = TableName::new("t")?;

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

    // v1
    // | id | c1 |
    // |----|----|
    let session = engine
        .with_tx_methods()
        .create_table(session, t_name.clone(), tc, coldefs)
        .await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    let session = engine.with_tx_methods().insert(session, t_name.clone(),     RecordIterator::new(vec![record! {
        FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?,
        FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1i32)?
    }])).await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    //
    // v2
    // | id |
    // |----|
    let session = engine
        .with_tx_methods()
        .alter_table(
            session,
            t_name.clone(),
            AlterTableAction::DropColumn {
                column_name: c1_def
                    .column_data_type()
                    .column_ref()
                    .as_column_name()
                    .clone(),
            },
        )
        .await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 1  |
    //
    // v2
    // | id |
    // |----|
    // | 2  |
    let session = engine.with_tx_methods().insert(
        session,
        t_name.clone(),
        RecordIterator::new(vec![
            record! { FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &2i32)? },
        ]),
    ).await?;

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
    let session = engine.with_tx_methods().insert(
        session,
t_name.clone(),
        RecordIterator::new(vec![record! {
            FieldIndex::InColumnReference(c_id_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &3i32)?,
            FieldIndex::InColumnReference(c1_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &3i32)?
        }]),
    ).await?;

    // Selects both v1's record (id=1) and v2's record (id=2),
    // although v2 does not have column "c".
    let (records, session) = engine
        .with_tx_methods()
        .select(session, t_name.clone(), ProjectionQuery::All)
        .await?;

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

    engine.with_tx_methods().commit_transaction(session).await?;

    Ok(())
}
