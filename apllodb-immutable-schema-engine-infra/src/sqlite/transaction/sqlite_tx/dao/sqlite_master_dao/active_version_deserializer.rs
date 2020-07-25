use apllodb_immutable_schema_engine_domain::{ActiveVersion, VTable, row::column::non_pk_column::NonPKColumnDataType};
use apllodb_shared_components::{
    data_structure::{ColumnName,  DataType, DataTypeKind, ColumnDataType},
    error::{ApllodbError, ApllodbResult, ApllodbErrorKind},
};
use apllodb_sql_parser::{
    apllodb_ast::{Command, CreateTableCommand,   self},
    ApllodbAst, ApllodbSqlParser,
};
use serde::{Deserialize, Serialize};
use crate::sqlite::transaction::sqlite_tx::dao::version_dao;
use version_dao::SqliteTableNameForVersion;


#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(super) struct ActiveVersionDeserializer{
    create_version_table_sql: String,
}

impl ActiveVersionDeserializer {
    pub(super) fn new<S: Into<String>>(create_version_table_sql: S) -> Self {
        Self{create_version_table_sql: create_version_table_sql.into()}
    }

    /// # Failures
    ///
    /// - [UndefinedTable](a.html) when:
    ///   - Version defined in this CreateTableSqlForVersion is deactivated.
    pub(super) fn into_active_version(&self, vtable: &VTable) -> ApllodbResult<ActiveVersion> {
        use apllodb_immutable_schema_engine_domain::Entity;

        let parser = ApllodbSqlParser::new();
        let ast = parser.parse(&self.create_version_table_sql).map_err(|e| 
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("SQLite's `{}` table somehow hold string that apllodb-sql-parser cannot parse: {}", super::TNAME, &self.create_version_table_sql),
                Some(Box::new(e))
            )
        )?;

        match ast {
            ApllodbAst(Command::CreateTableCommandVariant(CreateTableCommand {
                table_name,
                column_definitions,
            })) => {
                let sqlite_table_name = {
                    let id = table_name.0;
                    SqliteTableNameForVersion::from(id.0)
                };
                let version_number = sqlite_table_name.to_version_number();
         
                let non_pk_column_data_types: Vec<NonPKColumnDataType> = column_definitions
                .as_vec()
                .iter()
                .filter(|cd| {
                    !Self::is_control_column(cd)
                })
                .map(|cd| {
                    let column_name_res = {
                        let id = &cd.column_name.0;
                        ColumnName::new(id.0.as_str())
                    };
                    let data_type = {
                        let not_null = cd.column_constraints.contains(&apllodb_ast::ColumnConstraint::NotNullVariant);
                        let data_type_kind = match cd.data_type {
                            apllodb_ast::DataType::IntegerTypeVariant(apllodb_ast::IntegerType::SmallIntVariant) => DataTypeKind::SmallInt,
                            apllodb_ast::DataType::IntegerTypeVariant(apllodb_ast::IntegerType::IntegerVariant) => DataTypeKind::Integer,
                            apllodb_ast::DataType::IntegerTypeVariant(apllodb_ast::IntegerType::BigIntVariant) => DataTypeKind::BigInt,
                        };
                        DataType::new(data_type_kind, !not_null)
                    };

                    column_name_res.map(|column_name| {
                        let cdt = ColumnDataType::new(column_name, data_type);
                        NonPKColumnDataType(cdt)
                    }
                )
                })
                .collect::<ApllodbResult<Vec<NonPKColumnDataType>>>()?;

                ActiveVersion::new(vtable.id(), &version_number, &non_pk_column_data_types)
            }
            _ => unreachable!(format!(
                "SQLite's `{}` table somehow hold wrong non CREATE TABLE statement: {}",
                super::TNAME,
                &self.create_version_table_sql
            )),
        }

    }

    fn is_control_column(column_definition: &apllodb_ast::ColumnDefinition) -> bool {
        let id = &column_definition.column_name.0;
        id.0.as_str() == version_dao::CNAME_NAVI_ROWID

    }
}

#[cfg(test)]
mod tests {
    use super::ActiveVersionDeserializer;
    use apllodb_immutable_schema_engine_domain::{ ActiveVersion};
    use apllodb_shared_components::{
        column_constraints, column_definition, data_structure::{TableConstraints, DataTypeKind, ColumnDefinition}, data_type,
        error::ApllodbResult, table_constraints, t_pk, table_name, database_name,
    };
    use apllodb_immutable_schema_engine_domain::{VTable, Entity, row::column::non_pk_column::NonPKColumnDataType};
    use crate::{test_support::setup, sqlite::transaction::sqlite_tx::dao::version_dao::CreateTableSqlForVersionTestWrapper};

    #[test]
    fn test_from_into() -> ApllodbResult<()> {
        setup();

        let testset: Vec<(Vec<ColumnDefinition>, TableConstraints)> = vec![
            (
            vec![column_definition!(
                "c1",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            )],
            table_constraints!(t_pk!("c1")),
        )
        
        // TODO more samples
        ];

        for t in testset {
            let database_name = database_name!("db");
            let table_name = table_name!("tbl");
            let vtable = VTable::create(&database_name, &table_name, &t.1, &t.0)?;
            let non_pk_column_data_types: Vec<NonPKColumnDataType> = t.0.iter().map(|cd| {
                let cdt = cd.column_data_type();
                NonPKColumnDataType(cdt)
            }).collect();
            let version = ActiveVersion::initial(vtable.id(), &non_pk_column_data_types)?;

            let sql = CreateTableSqlForVersionTestWrapper::from(&version);
            let sql = sql.as_str();
            let deser = ActiveVersionDeserializer::new(sql);

            assert_eq!(deser.into_active_version(&vtable)?, version);
        }

        Ok(())
    }
}
