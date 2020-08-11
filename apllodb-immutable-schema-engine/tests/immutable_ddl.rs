mod test_support;

use crate::test_support::{database::TestDatabase, setup};
use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{
    data_structure::{
        AlterTableAction, ColumnConstraints, ColumnDefinition, ColumnName, Constant, DataType,
        DataTypeKind, Expression, TableConstraintKind, TableConstraints, TableName,
    },
    error::{ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

#[test]
fn test_success_select_all_from_2_versions() -> ApllodbResult<()> {
    setup();

    use apllodb_storage_engine_interface::Row;

    let mut db = TestDatabase::new()?;
    let tx = ApllodbImmutableSchemaEngine::begin_transaction(&mut db.0)?;

    let t_name = &TableName::new("t")?;

    let c_id_def = ColumnDefinition::new(
        ColumnName::new("id")?,
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let c1_def = ColumnDefinition::new(
        ColumnName::new("c1")?,
        DataType::new(DataTypeKind::Integer, false),
        ColumnConstraints::new(vec![])?,
    )?;
    let coldefs = vec![c_id_def.clone(), c1_def.clone()];

    let tc = TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
        column_names: vec![c_id_def.column_name().clone()],
    }])?;

    tx.create_table(&t_name, &tc, &coldefs)?;

    tx.insert(
        &t_name,
        hmap! {
         c_id_def.column_name().clone() => Expression::ConstantVariant(Constant::from(1)),
         c1_def.column_name().clone() => Expression::ConstantVariant(Constant::from(1))
        },
    )?;

    tx.alter_table(
        &t_name,
        &AlterTableAction::DropColumn {
            column_name: c1_def.column_name().clone(),
        },
    )?;

    tx.insert(
        &t_name,
        hmap! { c_id_def.column_name().clone() => Expression::ConstantVariant(Constant::from(2)) },
    )?;

    // Selects both v1's record (id=1) and v2's record (id=2),
    // although v2 does not have column "c".
    let rows = tx.select(
        &t_name,
        &vec![c_id_def.column_name().clone(), c1_def.column_name().clone()],
    )?;

    assert_eq!(rows.clone().count(), 2);

    for row_res in rows {
        let row = row_res?;
        let id: i32 = row.get(c_id_def.column_name())?;
        match id {
            1 => assert_eq!(row.get::<i32>(c1_def.column_name())?, 1),
            2 => {
                // Cannot fetch column `c` from v2. Note that v2's `c` is different from NULL,
                // although it is treated similarly to NULL in GROUP BY, ORDER BY operations.
                match row.get::<i32>(c1_def.column_name()) {
                    Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::UndefinedColumn),
                    _ => unreachable!(),
                };
            }
            _ => unreachable!(),
        }
    }

    tx.commit()?;

    Ok(())
}
