mod ddl;
mod dml;

/// AccessMethods implementation.
#[derive(Clone, Debug)]
pub struct AccessMethods;

#[cfg(test)]
mod tests {
    use super::AccessMethods;
    use crate::{
        column_constraints, column_definition, column_definitions, column_name, column_name_expr,
        const_expr, data_type, hmap, table_constraints, table_name,
        transaction::{Database, SqliteTx},
    };
    use apllodb_shared_components::{
        data_structure::{AlterTableAction, DataTypeKind, FieldIndex},
        error::{ApllodbErrorKind, ApllodbResult},
    };
    use apllodb_storage_manager_interface::{AccessMethodsDdl, AccessMethodsDml};

    #[test]
    fn test_success_select_all_from_2_versions() -> ApllodbResult<()> {
        let mut db = Database::new_for_test()?;
        let mut tx = SqliteTx::begin(&mut db)?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(
            column_definition!(
                "id",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
            column_definition!(
                "c",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
        );

        AccessMethods::create_table(&mut tx, &tn, &tc, &coldefs)?;

        AccessMethods::insert(
            &mut tx,
            &tn,
            hmap! { column_name!("id") => const_expr!(1), column_name!("c") => const_expr!(1) },
        )?;

        AccessMethods::alter_table(
            &mut tx,
            &tn,
            &AlterTableAction::DropColumn {
                column_name: column_name!("c"),
            },
        )?;

        AccessMethods::insert(&mut tx, &tn, hmap! { column_name!("id") => const_expr!(2) })?;

        // Selects both v1's record (id=1) and v2's record (id=2),
        // although v2 does not have column "c".
        let records =
            AccessMethods::select(&mut tx, &tn, &vec![column_name!("id"), column_name!("c")])?;

        for rec_res in records {
            let r = rec_res?;
            let id: i64 = r.get(&column_name!("id"))?;
            match id {
                1 => assert_eq!(r.get::<i64>(&column_name!("c"))?, 1),
                2 => {
                    match r.get::<i64>(&column_name!("c")) {
                        Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
                        _ => unreachable!(),
                    };
                    assert_eq!(r.get::<Option<i64>>(&column_name!("c"))?, None);
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}
