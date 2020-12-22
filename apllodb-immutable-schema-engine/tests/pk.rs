mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    data_structure::ColumnReference,
    data_structure::{
        ColumnConstraints, ColumnDefinition, ColumnName, Constant, DataType, DataTypeKind,
        Expression, TableConstraintKind, TableConstraints, TableName,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine, Transaction};

#[test]
fn test_compound_pk() -> ApllodbResult<()> {
    setup();

    use apllodb_storage_engine_interface::Row;

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = &TableName::new("address")?;

    let c_country_code_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("country_code")?),
        DataType::new(DataTypeKind::SmallInt, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let c_postal_code_def = ColumnDefinition::new(
        ColumnReference::new(t_name.clone(), ColumnName::new("postal_code")?),
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c_country_code_def.clone(), c_postal_code_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![
            c_country_code_def.column_ref().as_column_name().clone(),
            c_postal_code_def.column_ref().as_column_name().clone(),
        ],
    }])?;

    tx.create_table(&t_name, &tc, &coldefs)?;

    tx.insert(
        &t_name,
        hmap! { c_country_code_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(100i16)),
            c_postal_code_def.column_ref().as_column_name().clone() => Expression::ConstantVariant(Constant::from(1000001i32) )},
    )?;

    let row_iter = tx.select(
        &t_name,
        ProjectionQuery::ColumnNames(vec![c_postal_code_def
            .column_ref()
            .as_column_name()
            .clone()]),
    )?;
    for mut row in row_iter {
        assert_eq!(row.get::<i16>(c_country_code_def.column_ref())?, 100i16, "although `country_code` is not specified in SELECT projection, it's available since it's a part of PK");
        assert_eq!(row.get::<i32>(c_postal_code_def.column_ref())?, 1000001i32);
    }

    tx.commit()?;

    Ok(())
}
