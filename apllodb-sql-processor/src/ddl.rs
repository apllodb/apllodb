use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, TableConstraintKind, TableConstraints,
};
use apllodb_sql_parser::apllodb_ast::{Command, TableElement};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

use crate::ast_translator::AstTranslator;

/// Processes DDL command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct DDLProcessor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> DDLProcessor<'exe, Engine> {
    /// Executes DDL command.
    pub fn run(&self, command: Command) -> ApllodbResult<()> {
        match command {
            Command::CreateTableCommandVariant(cc) => {
                let table_name = AstTranslator::table_name(cc.table_name)?;

                let column_definitions: Vec<ColumnDefinition> = cc
                    .table_elements
                    .as_vec()
                    .iter()
                    .filter_map(|table_element| {
                        if let TableElement::ColumnDefinitionVariant(cd) = table_element {
                            Some(cd)
                        } else {
                            None
                        }
                    })
                    .map(|cd| AstTranslator::column_definition(cd.clone(), table_name.clone()))
                    .collect::<ApllodbResult<_>>()?;

                let table_constraints: Vec<TableConstraintKind> = cc
                    .table_elements
                    .as_vec()
                    .iter()
                    .filter_map(|table_element| {
                        if let TableElement::TableConstraintVariant(tc) = table_element {
                            Some(tc)
                        } else {
                            None
                        }
                    })
                    .map(|tc| AstTranslator::table_constraint(tc.clone()))
                    .collect::<ApllodbResult<_>>()?;

                self.tx.create_table(
                    &table_name,
                    &TableConstraints::new(table_constraints)?,
                    column_definitions,
                )
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{
        ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, SqlType,
        TableConstraints, TableName,
    };
    use apllodb_sql_parser::ApllodbSqlParser;
    use mockall::predicate::eq;

    use crate::{
        test_support::{setup, test_models::People, test_storage_engine::TestStorageEngine},
        DDLProcessor,
    };

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum<'test> {
        in_create_table_sql: &'test str,
        expected_table_name: TableName,
        expected_table_constraints: TableConstraints,
        expected_column_definitions: Vec<ColumnDefinition>,
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_ddl_processor_with_sql() -> ApllodbResult<()> {
        setup();

        let parser = ApllodbSqlParser::new();

        let test_data: Vec<TestDatum> = vec![TestDatum::new(
            "
            CREATE TABLE people (
                id INTEGER, 
                age INTEGER
            )",
            People::table_name(),
            TableConstraints::default(),
            vec![
                ColumnDefinition::new(
                    ColumnDataType::new(People::colref_id(), SqlType::integer(), true),
                    ColumnConstraints::default(),
                ),
                ColumnDefinition::new(
                    ColumnDataType::new(People::colref_age(), SqlType::integer(), true),
                    ColumnConstraints::default(),
                ),
            ],
        )];

        for test_datum in test_data {
            log::debug!("testing with SQL: {}", test_datum.in_create_table_sql);

            let ast = parser.parse(test_datum.in_create_table_sql).unwrap();

            let mut tx = TestStorageEngine::begin()?;

            // mocking create_table()
            tx.expect_create_table()
                .with(
                    eq(test_datum.expected_table_name),
                    eq(test_datum.expected_table_constraints),
                    eq(test_datum.expected_column_definitions),
                )
                .returning(|_, _, _| Ok(()));

            let processor = DDLProcessor::<'_, TestStorageEngine>::new(&tx);
            processor.run(ast.0)?;
        }

        Ok(())
    }
}
