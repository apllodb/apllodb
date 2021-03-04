mod test_support;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::test_setup;
use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition,
    NNSqlValue, SqlType, SqlValue, SqlValues, TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{test_support::session_with_tx, AliasDef};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine, WithTxMethods};

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

    let (id_idx, c1_idx): (usize, usize) = (0, 1);

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
    // | 1  | 1  |
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
                SqlValue::NotNull(NNSqlValue::Integer(1)),
                SqlValue::NotNull(NNSqlValue::Integer(1)),
            ])],
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
    // | 1  | 1  |
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
                NNSqlValue::Integer(2),
            )])],
        )
        .await?;

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
                SqlValue::NotNull(NNSqlValue::Integer(3)),
                SqlValue::NotNull(NNSqlValue::Integer(3)),
            ])],
        )
        .await?;

    // Selects both v1's record (id=1) and v2's record (id=2),
    // although v2 does not have column "c".
    let (records, session) = engine
        .with_tx()
        .select(
            session,
            t_name.clone(),
            ProjectionQuery::All,
            AliasDef::default(),
        )
        .await?;

    assert_eq!(records.clone().count(), 3);

    for record in records {
        let id: i32 = record.get(id_idx)?.unwrap();
        match id {
            1 => assert_eq!(record.get::<i32>(c1_idx)?, Some(1)),
            3 => assert_eq!(record.get::<i32>(c1_idx)?, Some(3)),
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
