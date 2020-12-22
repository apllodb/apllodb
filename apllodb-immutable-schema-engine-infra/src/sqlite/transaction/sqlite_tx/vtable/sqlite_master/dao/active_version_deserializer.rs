use apllodb_immutable_schema_engine_domain::{
    version::active_version::ActiveVersion, vtable::VTable,
};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, ColumnReference, DataType, DataTypeKind},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_sql_parser::{
    apllodb_ast::{self, Command, CreateTableCommand},
    ApllodbAst, ApllodbSqlParser,
};
use serde::{Deserialize, Serialize};

use crate::sqlite::transaction::sqlite_tx::version::dao::{
    sqlite_table_name_for_version::SqliteTableNameForVersion, CNAME_NAVI_ROWID,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(super) struct ActiveVersionDeserializer {
    create_version_table_sql: String,
}

impl ActiveVersionDeserializer {
    pub(super) fn new<S: Into<String>>(create_version_table_sql: S) -> Self {
        Self {
            create_version_table_sql: create_version_table_sql.into(),
        }
    }

    /// # Failures
    ///
    /// - [UndefinedTable](a.html) when:
    ///   - Version defined in this CreateTableSqlForVersion is deactivated.
    pub(super) fn to_active_version(&self, vtable: &VTable) -> ApllodbResult<ActiveVersion> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

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

                let non_pk_column_data_types: Vec<ColumnDataType> = column_definitions
                    .as_vec()
                    .iter()
                    .filter(|cd| !Self::is_control_column(cd))
                    .map(|cd| {
                        let column_name_res = {
                            let id = &cd.column_name.0;
                            ColumnName::new(id.0.as_str())
                        };
                        let data_type = {
                            let not_null = cd
                                .column_constraints
                                .contains(&apllodb_ast::ColumnConstraint::NotNullVariant);
                            let data_type_kind = match cd.data_type {
                                apllodb_ast::DataType::IntegerTypeVariant(
                                    apllodb_ast::IntegerType::SmallIntVariant,
                                ) => DataTypeKind::SmallInt,
                                apllodb_ast::DataType::IntegerTypeVariant(
                                    apllodb_ast::IntegerType::IntegerVariant,
                                ) => DataTypeKind::Integer,
                                apllodb_ast::DataType::IntegerTypeVariant(
                                    apllodb_ast::IntegerType::BigIntVariant,
                                ) => DataTypeKind::BigInt,
                            };
                            DataType::new(data_type_kind, !not_null)
                        };

                        column_name_res.map(|column_name| {
                            ColumnDataType::new(
                                ColumnReference::new(vtable.table_name().clone(), column_name),
                                data_type,
                            )
                        })
                    })
                    .collect::<ApllodbResult<Vec<ColumnDataType>>>()?;

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
        id.0.as_str() == CNAME_NAVI_ROWID
    }
}

#[cfg(test)]
mod tests {
    use super::ActiveVersionDeserializer;
    use crate::sqlite::transaction::sqlite_tx::version::dao::CreateTableSqlForVersionTestWrapper;
    use crate::test_support::setup;
    use apllodb_immutable_schema_engine_domain::{
        entity::Entity, version::active_version::ActiveVersion, vtable::VTable,
    };
    use apllodb_shared_components::{
        data_structure::ColumnDataType,
        data_structure::ColumnReference,
        data_structure::{
            ColumnConstraints, ColumnDefinition, ColumnName, DataType, DataTypeKind, DatabaseName,
            TableConstraintKind, TableConstraints, TableName,
        },
        error::ApllodbResult,
    };

    #[test]
    fn test_from_into() -> ApllodbResult<()> {
        setup();

        let c1_def = ColumnDefinition::new(
            ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?),
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::new(vec![])?,
        )?;

        let testset: Vec<(Vec<ColumnDefinition>, TableConstraints)> = vec![
            (
                vec![c1_def.clone()],
                TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                    column_names: vec![c1_def.column_ref().as_column_name().clone()],
                }])?,
            ), // TODO more samples
        ];

        for t in testset {
            let database_name = DatabaseName::new("db")?;
            let table_name = TableName::new("t")?;
            let vtable = VTable::create(&database_name, &table_name, &t.1, &t.0)?;
            let non_pk_column_data_types: Vec<ColumnDataType> =
                t.0.iter().map(|cd| cd.column_data_type()).collect();
            let version = ActiveVersion::initial(vtable.id(), &non_pk_column_data_types)?;

            let sql = CreateTableSqlForVersionTestWrapper::from(&version);
            let sql = sql.as_str();
            let deser = ActiveVersionDeserializer::new(sql);

            assert_eq!(deser.to_active_version(&vtable)?, version);
        }

        Ok(())
    }
}
