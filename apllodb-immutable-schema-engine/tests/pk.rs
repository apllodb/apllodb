mod test_support;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_immutable_schema_engine_infra::test_support::test_setup;
use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, FieldIndex,
    FullFieldReference, NNSqlValue, RecordIterator, SqlType, SqlValue, TableConstraintKind,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{record, test_support::session_with_tx};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine, WithTxMethods};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_compound_pk() -> ApllodbResult<()> {
    let engine = ApllodbImmutableSchemaEngine::default();
    let session = session_with_tx(&engine).await?;

    let t_name = &TableName::new("address")?;

    let c_country_code_def = ColumnDefinition::new(
        ColumnDataType::factory("country_code", SqlType::small_int(), false),
        ColumnConstraints::new(vec![])?,
    );
    let c_postal_code_def = ColumnDefinition::new(
        ColumnDataType::factory("postal_code", SqlType::integer(), false),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c_country_code_def.clone(), c_postal_code_def.clone()];

    let ffr_country_code = FullFieldReference::factory_table(
        t_name.as_str(),
        c_country_code_def.column_data_type().column_name().as_str(),
    );
    let ffr_postal_code = FullFieldReference::factory_table(
        t_name.as_str(),
        c_postal_code_def.column_data_type().column_name().as_str(),
    );

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![
            c_country_code_def.column_data_type().column_name().clone(),
            c_postal_code_def.column_data_type().column_name().clone(),
        ],
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
            RecordIterator::new(vec![record! {
                ffr_country_code .clone() => SqlValue::NotNull(NNSqlValue::SmallInt(100)),
                ffr_postal_code.clone() => SqlValue::NotNull(NNSqlValue::Integer(1000001))
            }]),
        )
        .await?;

    let (records, session) = engine
        .with_tx()
        .select(
            session,
            t_name.clone(),
            ProjectionQuery::ColumnNames(vec![c_postal_code_def
                .column_data_type()
                .column_name()
                .clone()]),
        )
        .await?;

    for record in records {
        assert_eq!(
            record.get::<i16>(
                &FieldIndex::InFullFieldReference(
                    ffr_country_code.clone()
                )
            )?, 
            Some(100i16), 
            "although `country_code` is not specified in SELECT projection, it's available since it's a part of PK"
        );
        assert_eq!(
            record.get::<i32>(&FieldIndex::InFullFieldReference(ffr_postal_code.clone()))?,
            Some(1000001i32),
        );
    }

    engine.with_tx().commit_transaction(session).await?;

    Ok(())
}
