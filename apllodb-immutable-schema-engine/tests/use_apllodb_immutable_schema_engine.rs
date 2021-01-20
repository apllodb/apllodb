mod test_support;

use crate::test_support::setup;
use apllodb_shared_components::{
    ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, ColumnName,
    ColumnReference, SqlType, TableConstraintKind, TableConstraints, TableName,
};

#[test]
fn test_use_apllodb_immutable_schema_engine() -> ApllodbResult<()> {
    setup();

    // let mut db = TestDatabase::new()?;
    // let mut tx = ApllodbImmutableSchemaTx::begin(&mut db.0)?;

    let t_name = TableName::new("t")?;

    let c1_def = ColumnDefinition::new(
        ColumnDataType::new(
            ColumnReference::new(t_name.clone(), ColumnName::new("c1")?),
            SqlType::integer(),
            false,
        ),
        ColumnConstraints::default(),
    );

    // let ddl = ApllodbImmutableSchemaDDL::default();

    // ddl.create_table(
    //     &mut tx,
    //     &t_name,
    //     &TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
    //         column_names: vec![c1_def
    //             .column_data_type()
    //             .column_ref()
    //             .as_column_name()
    //             .clone()],
    //     }])?,
    //     vec![c1_def],
    // )?;
    // tx.abort()?;

    Ok(())
}
