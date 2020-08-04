mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_shared_components::{
    data_structure::{
        ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName, DataType, DataTypeKind,
        TableConstraintKind,
    },
    error::ApllodbResult,
};

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
    use apllodb_shared_components::data_structure::{TableConstraints, TableName};
    use apllodb_storage_engine_interface::{StorageEngine, Transaction};

    setup();

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;
    tx.create_table(
        &TableName::new("t")?,
        &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
            column_data_types: vec![ColumnDataType::new(
                ColumnName::new("c1")?,
                DataType::new(DataTypeKind::Integer, false),
            )],
        }])?,
        &vec![ColumnDefinition::new(
            ColumnName::new("c1")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::default(),
        )?],
    )?;
    tx.abort()?;

    Ok(())
}
