use std::collections::HashMap;

use apllodb_shared_components::{
    ApllodbResult, ColumnReference, FieldIndex, Record, RecordIterator, SqlValue,
};
use apllodb_sql_parser::apllodb_ast::{self, Command};
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    ast_translator::AstTranslator,
    query::query_plan::query_plan_tree::query_plan_node::{
        LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf,
    },
};

use self::{
    modification_executor::ModificationExecutor,
    modification_plan::{
        modification_plan_tree::{
            modification_plan_node::{InsertNode, ModificationPlanNode},
            ModificationPlanTree,
        },
        ModificationPlan,
    },
};

pub(crate) mod modification_executor;
pub(crate) mod modification_plan;

/// Processes ÎNSERT/UPDATE/DELETE command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct ModificationProcessor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> ModificationProcessor<'exe, Engine> {
    /// Executes parsed INSERT/UPDATE/DELETE command.
    pub fn run(&self, command: Command) -> ApllodbResult<()> {
        match command {
            Command::InsertCommandVariant(ic) => {
                if ic.alias.is_some() {
                    unimplemented!();
                }

                let table_name = AstTranslator::table_name(ic.table_name)?;

                let column_names = ic.column_names.into_vec();
                let expressions = ic.expressions.into_vec();

                if column_names.len() != expressions.len() {
                    unimplemented!();
                }

                let constant_values: Vec<SqlValue> = expressions
                    .into_iter()
                    .map(|expression| match expression {
                        apllodb_ast::Expression::ConstantVariant(c) => AstTranslator::constant(c),
                        _ => unimplemented!(),
                    })
                    .collect::<ApllodbResult<_>>()?;

                let fields: HashMap<FieldIndex, SqlValue> = column_names
                    .into_iter()
                    .zip(constant_values)
                    .into_iter()
                    .map(|(cn, sql_value)| {
                        let col_ref = ColumnReference::new(
                            table_name.clone(),
                            AstTranslator::column_name(cn)?,
                        );
                        let field = FieldIndex::InColumnReference(col_ref);
                        Ok((field, sql_value))
                    })
                    .collect::<ApllodbResult<_>>()?;

                let record = Record::new(fields);
                let records = RecordIterator::new(vec![record]);

                let plan_node = ModificationPlanNode::Insert(InsertNode {
                    table_name,
                    child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::DirectInput { records },
                    }),
                });

                let plan = ModificationPlan::new(ModificationPlanTree::new(plan_node));
                let executor = ModificationExecutor::<'_, Engine>::new(self.tx);
                executor.run(plan)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{ApllodbResult, Record, RecordIterator, TableName};
    use apllodb_sql_parser::ApllodbSqlParser;
    use mockall::predicate::eq;

    use crate::test_support::{setup, test_models::People, test_storage_engine::TestStorageEngine};

    use super::ModificationProcessor;

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum<'test> {
        in_insert_sql: &'test str,
        expected_insert_table: TableName,
        expected_insert_records: Vec<Record>,
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_modification_processor_with_sql() -> ApllodbResult<()> {
        setup();

        let t_people_r1 = People::record(1, 13);

        let mut tx = TestStorageEngine::begin()?;

        let parser = ApllodbSqlParser::new();

        let test_data: Vec<TestDatum> = vec![TestDatum::new(
            "INSERT INTO people (id, age) VALUES (1, 13)",
            People::table_name(),
            vec![t_people_r1.clone()],
        )];

        for test_datum in test_data {
            log::debug!("testing with SQL: {}", test_datum.in_insert_sql);

            let ast = parser.parse(test_datum.in_insert_sql).unwrap();

            // mocking insert()
            tx.expect_insert()
                .with(
                    eq(test_datum.expected_insert_table),
                    eq(RecordIterator::new(test_datum.expected_insert_records)),
                )
                .returning(|_, _| Ok(()));

            let processor = ModificationProcessor::<'_, TestStorageEngine>::new(&tx);
            processor.run(ast.0)?;
        }

        Ok(())
    }
}
