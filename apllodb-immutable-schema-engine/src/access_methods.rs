mod ddl;

/// AccessMethods implementation.
#[derive(Clone, Debug)]
pub struct AccessMethods;

// #[cfg(test)]
// mod tests {
//     use super::AccessMethods;
//     use crate::{
//         column_constraints, column_definition, column_definitions, data_type, table_constraints,
//         table_name,
//         transaction::{Database, SqliteTx},
//     };
//     use apllodb_shared_components::{data_structure::{AlterTableAction, DataTypeKind}, error::ApllodbResult};
//     use apllodb_storage_manager_interface::AccessMethodsDdl;

//     #[test]
//     fn test_success_select_all_from_2_versions() -> ApllodbResult<()> {
//         let mut db = Database::new_for_test()?;
//         let mut tx = SqliteTx::begin(&mut db)?;

//         let tn = &table_name!("t");
//         let tc = table_constraints!();
//         let coldefs = column_definitions!(
//             column_definition!(
//                 "id",
//                 data_type!(DataTypeKind::Integer, false),
//                 column_constraints!()
//             ),
//             column_definition!(
//                 "c_not_null",
//                 data_type!(DataTypeKind::Integer, false),
//                 column_constraints!()
//             ),
//             column_definition!(
//                 "c_nullable",
//                 data_type!(DataTypeKind::Integer, true),
//                 column_constraints!()
//             ),
//         );

//         AccessMethods::create_table(&mut tx, &tn, &tc, &coldefs)?;

//         AccessMethods::alter_table(&mut tx, &tn, &AlterTableAction::DropColumn {column_name: column_name!("")})?;
//     }
// }
