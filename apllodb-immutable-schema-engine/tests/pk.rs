mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, FieldIndex, RecordIterator, SqlType, SqlValue, TableConstraintKind,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, Transaction};

#[test]
fn test_compound_pk() -> ApllodbResult<()> {
    setup();

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = &TableName::new("address")?;

    let c_country_code_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("country_code")?),
            SqlType::small_int(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let c_postal_code_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("postal_code")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::new(vec![])?,
    );
    let coldefs = vec![c_country_code_def.clone(), c_postal_code_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![
            c_country_code_def
                .column_data_type()
                .column_ref()
                .as_column_name()
                .clone(),
            c_postal_code_def
                .column_data_type()
                .column_ref()
                .as_column_name()
                .clone(),
        ],
    }])?;

    tx.create_table(&t_name, &tc, coldefs)?;

    tx.insert(
        &t_name,
        RecordIterator::new(vec![
            record! {
                FieldIndex::InColumnReference(c_country_code_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::small_int(), &100i16)?,
                FieldIndex::InColumnReference(c_postal_code_def.column_data_type().column_ref().clone()) => SqlValue::pack(SqlType::integer(), &1000001i32)?
            }
        ])
    )?;

    let records = tx.select(
        &t_name,
        ProjectionQuery::ColumnNames(vec![c_postal_code_def
            .column_data_type()
            .column_ref()
            .as_column_name()
            .clone()]),
    )?;
    for record in records {
        assert_eq!(record.get::<i16>(&FieldIndex::InColumnReference(c_country_code_def.column_data_type().column_ref().clone()))?, Some(100i16), "although `country_code` is not specified in SELECT projection, it's available since it's a part of PK");
        assert_eq!(
            record.get::<i32>(&FieldIndex::InColumnReference(
                c_postal_code_def.column_data_type().column_ref().clone()
            ))?,
            Some(1000001i32)
        );
    }

    tx.commit()?;

    Ok(())
}
