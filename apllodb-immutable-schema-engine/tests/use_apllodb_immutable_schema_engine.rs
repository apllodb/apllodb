mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
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

    let c1_def = ColumnDefinition::new(
        ColumnName::new("c1")?,
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::default(),
    )?;

    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_names: vec![c1_def.column_name().clone()],
        }])?,
        &vec![c1_def],
    )?;
    tx.abort()?;

    Ok(())
}
