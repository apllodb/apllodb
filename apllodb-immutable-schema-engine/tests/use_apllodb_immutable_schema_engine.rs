mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    data_structure::ColumnReference,
    data_structure::{
        ColumnConstraints, ColumnDefinition, ColumnName, DataType, DataTypeKind,
        TableConstraintKind, TableConstraints, TableName,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
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
