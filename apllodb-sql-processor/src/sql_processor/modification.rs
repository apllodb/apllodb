use std::{collections::HashMap, rc::Rc};

use apllodb_shared_components::{
    ApllodbResult, ColumnReference, FieldIndex, Record, RecordIterator, SessionWithTx, SqlValue,
};
use apllodb_sql_parser::apllodb_ast::{self, Command};
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    ast_translator::AstTranslator,
    sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ModificationProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> ModificationProcessor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

    /// Executes parsed INSERT/UPDATE/DELETE command.
    pub async fn run(
        &self,
        session: SessionWithTx,
        command: Command,
    ) -> ApllodbResult<SessionWithTx> {
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
                let executor = ModificationExecutor::new(self.engine.clone());
                executor.run(session, plan).await
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{ApllodbResult, Record, TableName};

    use crate::test_support::{setup, test_models::People};

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

        // let parser = ApllodbSqlParser::new();

        let test_data: Vec<TestDatum> = vec![TestDatum::new(
            "INSERT INTO people (id, age) VALUES (1, 13)",
            People::table_name(),
            vec![t_people_r1.clone()],
        )];

        for test_datum in test_data {
            log::debug!("testing with SQL: {}", test_datum.in_insert_sql);

            // let ast = parser.parse(test_datum.in_insert_sql).unwrap();

            // let mut tx = TestTx;
            // let mut dml = MockDML::new();

            // // mocking insert()
            // dml.expect_insert()
            //     .with(
            //         always(),
            //         eq(test_datum.expected_insert_table),
            //         eq(RecordIterator::new(test_datum.expected_insert_records)),
            //     )
            //     .returning(|_, _, _| Ok(()));

            // let processor = ModificationProcessor::<TestStorageEngine>::new(&dml);
            // processor.run(&mut tx, ast.0)?;
        }

        Ok(())
    }
}
