use std::collections::HashMap;

use apllodb_shared_components::{
    ApllodbResult, ColumnName, ColumnReference, Constant, FieldIndex, SqlValue, TableName,
};
use apllodb_sql_parser::apllodb_ast::{self, Command};
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    ast_translator::AstTranslator,
    query::query_plan::query_plan_tree::query_plan_node::{
        LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf,
    },
    QueryProcessor,
};

use self::modification_plan::modification_plan_tree::modification_plan_node::{
    InsertNode, ModificationPlanNode,
};

pub(crate) mod modification_executor;
pub(crate) mod modification_plan;

/// Processes ÎNSERT/UPDATE/DELETE command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct ModificationProcessor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> ModificationProcessor<'exe, Engine> {
    pub fn run(&self, command: Command) -> ApllodbResult<()> {
        let query_processor = QueryProcessor::new(self.tx);

        match command {
            Command::InsertCommandVariant(ic) => {
                if ic.alias.is_some() {
                    unimplemented!();
                }

                let table_name = TableName::new(ic.table_name.0 .0)?;

                let column_names = ic.column_names.into_vec();
                let expressions = ic.expressions.into_vec();

                if column_names.len() != expressions.len() {
                    unimplemented!();
                }

                let constant_values: Vec<Constant> = expressions
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
                    .map(|(cn, constant)| {
                        let col_ref =
                            ColumnReference::new(table_name.clone(), ColumnName::new(cn.0 .0)?);
                        let field = FieldIndex::InColumnReference(col_ref);
                        let sql_value = SqlValue::try_from(apllodb_ast::Expression::ConstantVariant(constant))?;
                        Ok((field, sql_value))
                    })
                    .collect::<ApllodbResult<_>>()?;

                let plan_node = ModificationPlanNode::Insert(InsertNode {
                    table_name,
                    child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::DirectInput { records },
                    }),
                });
            }
            _ => unimplemented!(),
        }

        //let input = query_processor.run()

        let plan = QueryPlan::try_from(select_command)?;

        // TODO plan optimization -> QueryPlan

        let executor = QueryExecutor::<'_, Engine>::new(self.tx);
        executor.run(plan)
    }
}
