use super::sqlite_table_name_for_version::SqliteTableNameForVersion;
use apllodb_immutable_schema_engine_domain::{VTableId, ActiveVersion};
use apllodb_shared_components::{
    data_structure::{DatabaseName,  ColumnName, ColumnDefinition, DataType, DataTypeKind, ColumnConstraints, TableConstraints},
    error::{ApllodbError, ApllodbResult, ApllodbErrorKind},
};
use apllodb_sql_parser::{
    apllodb_ast::{Command, CreateTableCommand,   self},
    ApllodbAst, ApllodbSqlParser,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(super) struct CreateTableSqlForVersion(String);

impl CreateTableSqlForVersion {
    pub(super) fn new<S: Into<String>>(sql: S) -> Self {
        Self(sql.into())
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&ActiveVersion> for CreateTableSqlForVersion {
    fn from(version: &ActiveVersion) -> Self {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::Entity;

        let version_table_name = SqliteTableNameForVersion::new(version.id(), true);

        let sql = format!(
            "
CREATE TABLE {} (
  {}
)
        ",
            version_table_name.as_str(),
            version
                .column_data_types()
                .iter()
                .map(|cdt| cdt.to_sql_string())
                .collect::<Vec<String>>()
                .join(",\n  ")
        );

        // TODO materialize Version::constraints

        Self(sql)
    }
}

impl CreateTableSqlForVersion {
    /// # Failures
    ///
    /// - [UndefinedTable](a.html) when:
    ///   - Version defined in this CreateTableSqlForVersion is deactivated.
    pub(super) fn into_active_version(&self, database_name: &DatabaseName) -> ApllodbResult<ActiveVersion> {
        let parser = ApllodbSqlParser::new();
        let ast = parser.parse(self.as_str()).map_err(|e| 
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("SQLite's sqlite_master table somehow hold string that apllodb-sql-parser cannot parse: {}", self.as_str()),
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
                let table_name = sqlite_table_name.to_table_name();
                let version_number = sqlite_table_name.to_version_number();
                let vtable_id = VTableId::new(database_name, &table_name);
         
                let column_definitions: Vec<ColumnDefinition> = column_definitions
                .as_vec()
                .iter()
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
                    // TODO Support ColumnConstraint other than NotNullVariant.
                    let column_constraints = ColumnConstraints::default();

                    column_name_res.and_then(|column_name| ColumnDefinition::new(column_name, data_type, column_constraints))
                })
                .collect::<ApllodbResult<Vec<ColumnDefinition>>>()?;

                // TODO TableConstraints
                let table_constraints = TableConstraints::default();
                
                ActiveVersion::new(&vtable_id, &version_number, &column_definitions, &table_constraints)
            }
            _ => unreachable!(format!(
                "SQLite's sqlite_master table somehow hold wrong non CREATE TABLE statement: {}",
                self.as_str()
            )),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::CreateTableSqlForVersion;
    use apllodb_immutable_schema_engine_domain::{vtable_id, ActiveVersion, VTableId};
    use apllodb_shared_components::{
        column_constraints, column_definition, data_structure::DataTypeKind, data_type,
        error::ApllodbResult, table_constraints,
    };
    use apllodb_immutable_schema_engine_domain::Entity;

    #[test]
    fn test_from_into() -> ApllodbResult<()> {
        let testset: Vec<ActiveVersion> = vec![ActiveVersion::initial(
            &vtable_id!(),
            &vec![column_definition!(
                "c1",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            )],
            &table_constraints!(),
        )?
        // TODO more samples
        ];

        for version in testset {
            let sql = CreateTableSqlForVersion::from(&version);
            let database_name = version.id().vtable_id().database_name();

            assert_eq!(sql.into_active_version(database_name)?, version);
        }

        Ok(())
    }
}
