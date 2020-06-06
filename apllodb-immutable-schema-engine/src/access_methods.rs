mod ddl;

/// AccessMethods implementation.
#[derive(Clone, Debug)]
pub struct AccessMethods;

#[cfg(test)]
mod tests {
    use super::AccessMethods;
    use crate::{
        column_constraints, column_definition, column_definitions, column_name, data_type,
        table_constraints, table_name,
        transaction::{Database, SqliteTx},
    };
    use apllodb_shared_components::{
        data_structure::{AlterTableAction, DataTypeKind},
        error::ApllodbResult,
    };
    use apllodb_storage_manager_interface::AccessMethodsDdl;

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

        // AccessMethods::insert(
        //     &mut tx,
        //     &tn,
        //     hmap! { column_name!("id") => 1, column_name!("c") => 1 },
        // )?;

        AccessMethods::alter_table(
            &mut tx,
            &tn,
            &AlterTableAction::DropColumn {
                column_name: column_name!("c"),
            },
        )?;

        // AccessMethods::insert(&mut tx, &tn, hmap! { column_name!("id") => 2 })?;

        // // Selects both v1's record (id=1) and v2's record (id=2),
        // // although v2 does not have column "c".
        // let records = AccessMethods::select(&mut tx, &tn, vec![column_name!("id"), column_name!("c")])?;

        // for r in records {
        //     let id: u64 = r.get("id")?;
        //     match  id {
        //         1 => assert_eq!(r.get::<u64>("c"), 1),
        //         2 => {
        //             match r.get::<u64>("c") {
        //                 Err(DataTypeMismatch) => ,
        //                 _ => panic!("should be DataTypeMismatch"),
        //             };
        //             assert_eq!(r.get::<Option<u64>>("c"), None);
        //         }
        //         _ => unreachable!(),
        //     }
        // }

        Ok(())
    }
}
