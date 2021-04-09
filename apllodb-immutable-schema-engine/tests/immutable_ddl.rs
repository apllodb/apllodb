mod test_support;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::{session_with_tx, test_setup};
use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    FieldIndex, NnSqlValue, SqlType, SqlValue, SqlValues, TableConstraintKind, TableConstraints,
    TableName,
};
use apllodb_storage_engine_interface::{RowProjectionQuery, StorageEngine, WithTxMethods};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_success_select_column_available_only_in_1_of_2_versions() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnDataType::factory("id", SqlType::integer(), false),
        ColumnConstraints::new(vec![])?,
    );
    let c1_def = ColumnDefinition::new(
        ColumnDataType::factory("c1", SqlType::integer(), false),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def.column_data_type().column_name().clone()],
    }])?;

    // v1
    // | id | c1 |
    // |----|----|
    let session = engine
        .with_tx()
        .create_table(session, t_name.clone(), tc, coldefs)
        .await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 10 |
    let session = engine
        .with_tx()
        .insert(
            session,
            t_name.clone(),
            vec![
                c_id_def.column_data_type().column_name().clone(),
                c1_def.column_data_type().column_name().clone(),
            ],
            vec![SqlValues::new(vec![
                SqlValue::NotNull(NnSqlValue::Integer(1)),
                SqlValue::NotNull(NnSqlValue::Integer(10)),
            ])],
        )
        .await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 10 |
    //
    // v2
    // | id |
    // |----|
    let session = engine
        .with_tx()
        .alter_table(
            session,
            t_name.clone(),
            AlterTableAction::DropColumn {
                column_name: c1_def.column_data_type().column_name().clone(),
            },
        )
        .await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 10 |
    //
    // v2
    // | id |
    // |----|
    // | 2  |
    let session = engine
        .with_tx()
        .insert(
            session,
            t_name.clone(),
            vec![c_id_def.column_data_type().column_name().clone()],
            vec![SqlValues::new(vec![SqlValue::NotNull(
                NnSqlValue::Integer(2),
            )])],
        )
        .await?;

    // v1
    // | id | c1 |
    // |----|----|
    // | 1  | 10 |
    // | 3  | 30 |
    //
    // v2
    // | id |
    // |----|
    // | 2  |
    let session = engine
        .with_tx()
        .insert(
            session,
            t_name.clone(),
            vec![
                c_id_def.column_data_type().column_name().clone(),
                c1_def.column_data_type().column_name().clone(),
            ],
            vec![SqlValues::new(vec![
                SqlValue::NotNull(NnSqlValue::Integer(3)),
                SqlValue::NotNull(NnSqlValue::Integer(30)),
            ])],
        )
        .await?;

    // Selects both v1's record (id=1) and v2's record (id=2),
    // although v2 does not have column "c".
    let (records, session) = engine
        .with_tx()
        .select(session, t_name.clone(), RowProjectionQuery::All)
        .await?;

    assert_eq!(records.clone().count(), 3);

    let schema = records.as_schema().clone();
    let id_idx = schema.resolve_index(&FieldIndex::from(
        c_id_def.column_data_type().column_name().as_str(),
    ))?;
    let c1_idx = schema.resolve_index(&FieldIndex::from(
        c1_def.column_data_type().column_name().as_str(),
    ))?;

    for record in records {
        let id: i32 = record.get(id_idx)?.unwrap();
        match id {
            1 => assert_eq!(record.get::<i32>(c1_idx)?, Some(10)),
            3 => assert_eq!(record.get::<i32>(c1_idx)?, Some(30)),
            2 => {
                // Can fetch column `c1` from v2 and it's value is NULL.
                assert_eq!(record.get::<i32>(c1_idx)?, None);
            }
            _ => unreachable!(),
        }
    }

    engine.with_tx().commit_transaction(session).await?;

    Ok(())
}
