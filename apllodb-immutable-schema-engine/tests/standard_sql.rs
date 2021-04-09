mod test_support;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::{session_with_tx, test_setup};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnConstraints, ColumnDataType,
    ColumnDefinition, Expression, FieldIndex, FullFieldReference, NnSqlValue, RecordFieldRefSchema,
    SqlType, SqlValue, SqlValues, TableConstraintKind, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{RowProjectionQuery, StorageEngine, WithTxMethods};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_table_success() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::factory("c1", SqlType::integer(), false),
        ColumnConstraints::default(),
    );

    let session = engine
        .with_tx()
        .create_table(
            session,
            t_name.clone(),
            TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                column_names: vec![c1_def.column_data_type().column_name().clone()],
            }])?,
            vec![c1_def],
        )
        .await?;

    engine.with_tx().abort_transaction(session).await?;

    Ok(())
}

#[async_std::test]
async fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = &TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::factory("c1", SqlType::integer(), false),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c1_def.column_data_type().column_name().clone()],
    }])?;

    let session = engine
        .with_tx()
        .create_table(session, t_name.clone(), tc.clone(), coldefs.clone())
        .await?;
    match engine
        .with_tx()
        .create_table(session, t_name.clone(), tc, coldefs.clone())
        .await
    {
        // Internally, new record is trying to be INSERTed but it is made wait by tx2.
        // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
        Err(e) => assert_eq!(
            ApllodbError::from(e).kind(),
            &ApllodbErrorKind::DuplicateTable
        ),
        Ok(_) => panic!("should rollback"),
    }

    Ok(())
}

#[async_std::test]
async fn test_insert() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = &TableName::new("t")?;

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

    let session = engine
        .with_tx()
        .create_table(session, t_name.clone(), tc, coldefs)
        .await?;

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
                SqlValue::NotNull(NnSqlValue::Integer(100)),
            ])],
        )
        .await?;

    let (mut records, session) = engine
        .with_tx()
        .select(session, t_name.clone(), RowProjectionQuery::All)
        .await?;

    let schema = records.as_schema().clone();
    let id_idx = schema.resolve_index(&FieldIndex::from(
        c_id_def.column_data_type().column_name().as_str(),
    ))?;
    let c1_idx = schema.resolve_index(&FieldIndex::from(
        c1_def.column_data_type().column_name().as_str(),
    ))?;

    let record = records.next().unwrap();
    assert_eq!(record.get::<i32>(id_idx)?, Some(1));
    assert_eq!(record.get::<i32>(c1_idx)?, Some(100));

    assert!(records.next().is_none());

    engine.with_tx().commit_transaction(session).await?;

    Ok(())
}

#[async_std::test]
async fn test_update() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = &TableName::new("t")?;

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

    let session = engine
        .with_tx()
        .create_table(session, t_name.clone(), tc.clone(), coldefs)
        .await?;

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
                SqlValue::NotNull(NnSqlValue::Integer(100)),
            ])],
        )
        .await?;

    let (mut records, session) = engine
        .with_tx()
        .select(session, t_name.clone(), RowProjectionQuery::All)
        .await?;

    {
        let schema = records.as_schema().clone();
        let id_idx = schema.resolve_index(&FieldIndex::from(
            c_id_def.column_data_type().column_name().as_str(),
        ))?;
        let c1_idx = schema.resolve_index(&FieldIndex::from(
            c1_def.column_data_type().column_name().as_str(),
        ))?;

        let record = records.next().unwrap();
        assert_eq!(record.get::<i32>(id_idx)?, Some(1));
        assert_eq!(record.get::<i32>(c1_idx)?, Some(100));
        assert!(records.next().is_none());
    }

    // update non-PK
    let session = engine.with_tx().update(
        session,
        t_name.clone(),
        hmap! {
            c1_def.column_data_type().column_name().clone() => Expression::ConstantVariant(SqlValue::NotNull(NnSqlValue::Integer(200)))
        },
    ).await?;
    let (mut records, session) = engine
        .with_tx()
        .select(session, t_name.clone(), RowProjectionQuery::All)
        .await?;

    {
        let schema = records.as_schema().clone();
        let id_idx = schema.resolve_index(&FieldIndex::from(
            c_id_def.column_data_type().column_name().as_str(),
        ))?;
        let c1_idx = schema.resolve_index(&FieldIndex::from(
            c1_def.column_data_type().column_name().as_str(),
        ))?;

        let record = records.next().unwrap();
        assert_eq!(record.get::<i32>(id_idx)?, Some(1));
        assert_eq!(record.get::<i32>(c1_idx)?, Some(200));
        assert!(records.next().is_none());
    }

    // update PK
    let session =engine.with_tx().
    update(
        session,
        t_name.clone(),
        hmap! {
            c_id_def.column_data_type().column_name().clone() => Expression::ConstantVariant(SqlValue::NotNull(NnSqlValue::Integer(2)))
        },
    ).await?;
    let (mut records, session) = engine
        .with_tx()
        .select(session, t_name.clone(), RowProjectionQuery::All)
        .await?;
    {
        let schema = records.as_schema().clone();
        let id_idx = schema.resolve_index(&FieldIndex::from(
            c_id_def.column_data_type().column_name().as_str(),
        ))?;
        let c1_idx = schema.resolve_index(&FieldIndex::from(
            c1_def.column_data_type().column_name().as_str(),
        ))?;
        let record = records.next().unwrap();
        assert_eq!(record.get::<i32>(id_idx)?, Some(2));
        assert_eq!(record.get::<i32>(c1_idx)?, Some(200));
        assert!(records.next().is_none());
    }

    engine.with_tx().commit_transaction(session).await?;

    Ok(())
}

#[async_std::test]
async fn test_delete() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = &TableName::new("t")?;

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

    let session = engine
        .with_tx()
        .create_table(session, t_name.clone(), tc.clone(), coldefs)
        .await?;

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
                SqlValue::NotNull(NnSqlValue::Integer(100)),
            ])],
        )
        .await?;

    let (rows, session) = engine
        .with_tx()
        .select(
            session,
            t_name.clone(),
            RowProjectionQuery::Columns(RecordFieldRefSchema::factory(vec![
                FullFieldReference::factory(
                    t_name.as_str(),
                    c_id_def.column_data_type().column_name().as_str(),
                ),
            ])),
        )
        .await?;
    assert_eq!(rows.count(), 1);

    let session = engine.with_tx().delete(session, t_name.clone()).await?;
    let (rows, session) = engine
        .with_tx()
        .select(
            session,
            t_name.clone(),
            RowProjectionQuery::Columns(RecordFieldRefSchema::factory(vec![
                FullFieldReference::factory(
                    t_name.as_str(),
                    c_id_def.column_data_type().column_name().as_str(),
                ),
            ])),
        )
        .await?;
    assert_eq!(rows.count(), 0);

    engine.with_tx().commit_transaction(session).await?;

    Ok(())
}
