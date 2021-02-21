use std::{collections::HashMap, rc::Rc};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, CorrelationName, Expression,
    FieldIndex, FieldReference, FromItem, FullFieldReference, Ordering, Record,
    RecordFieldRefSchema, RecordIterator, SelectFieldReference, Session, SessionWithTx,
    SqlValueHashKey, SqlValues, TableName, TableWithAlias,
};
use apllodb_storage_engine_interface::{AliasDef, ProjectionQuery, StorageEngine, WithTxMethods};

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
    BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(super) struct PlanNodeExecutor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> PlanNodeExecutor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

    pub(super) async fn run_leaf(
        &self,
        session: SessionWithTx,
        op_leaf: LeafPlanOperation,
    ) -> ApllodbSessionResult<(RecordIterator, SessionWithTx)> {
        match op_leaf {
            LeafPlanOperation::InsertValues {
                table_name,
                column_names,
                values,
            } => {
                let from_item = FromItem::TableVariant(TableWithAlias {
                    table_name: table_name.clone(),
                    alias: None,
                });

                let res_ffrs: ApllodbResult<Vec<FullFieldReference>> = column_names
                    .into_iter()
                    .map(|column_name| {
                        let sfr = SelectFieldReference::new(
                            Some(CorrelationName::from(table_name.clone())),
                            FieldReference::from(column_name),
                        );
                        sfr.resolve(Some(from_item.clone()))
                        //.map_err(|e| )
                    })
                    .collect();

                match res_ffrs {
                    Ok(ffrs) => {
                        let schema = RecordFieldRefSchema::new(Some(from_item), ffrs);
                        Ok((RecordIterator::new(schema, values), session))
                    }
                    Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
                }
            }
            LeafPlanOperation::SeqScan {
                table_name,
                projection,
                alias_def,
            } => {
                self.seq_scan(session, table_name, projection, alias_def)
                    .await
            }
        }
    }

    pub(super) fn run_unary(
        &self,
        op_unary: UnaryPlanOperation,
        input_left: RecordIterator,
    ) -> ApllodbResult<RecordIterator> {
        match op_unary {
            UnaryPlanOperation::Projection { fields } => self.projection(input_left, fields),
            UnaryPlanOperation::Selection { condition } => self.selection(input_left, condition),
            UnaryPlanOperation::Sort { field_orderings } => self.sort(input_left, field_orderings),
        }
    }

    pub(super) fn run_binary(
        &self,
        op_binary: BinaryPlanOperation,
        input_left: RecordIterator,
        input_right: RecordIterator,
    ) -> ApllodbResult<RecordIterator> {
        match op_binary {
            // TODO type cast
            BinaryPlanOperation::HashJoin {
                left_field,
                right_field,
            } => self.hash_join(input_left, input_right, &left_field, &right_field),
        }
    }

    async fn seq_scan(
        &self,
        session: SessionWithTx,
        table_name: TableName,
        projection: ProjectionQuery,
        alias_def: AliasDef,
    ) -> ApllodbSessionResult<(RecordIterator, SessionWithTx)> {
        self.engine
            .with_tx()
            .select(session, table_name, projection, alias_def)
            .await
    }

    /// # Failures
    ///
    /// Failures from [Record::projection()](apllodb_shared_components::Record::projection).
    fn projection(
        &self,
        input_left: RecordIterator,
        fields: Vec<FieldIndex>,
    ) -> ApllodbResult<RecordIterator> {
        input_left.projection(&fields)
    }

    fn selection(
        &self,
        input_left: RecordIterator,
        condition: Expression,
    ) -> ApllodbResult<RecordIterator> {
        input_left.selection(&condition)
    }

    fn sort(
        &self,
        input_left: RecordIterator,
        field_orderings: Vec<(FieldIndex, Ordering)>,
    ) -> ApllodbResult<RecordIterator> {
        input_left.sort(&field_orderings)
    }

    /// Join algorithm using hash table.
    /// It can be used with join keys' equality (like `ON t.id = s.t_id`).
    /// This algorithm's time-complexity is `max[O(len(input_left)), O(len(input_right))]` but uses relatively large memory.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in any record.
    fn hash_join(
        &self,
        input_left: RecordIterator,
        input_right: RecordIterator,
        left_field: &FieldIndex,
        right_field: &FieldIndex,
    ) -> ApllodbResult<RecordIterator> {
        // TODO Create hash table from smaller input.
        let mut hash_table = HashMap::<SqlValueHashKey, Vec<Record>>::new();

        for left_record in input_left {
            let left_sql_value = left_record.get_sql_value(&left_field)?;
            hash_table
                .entry(SqlValueHashKey::from(left_sql_value))
                // FIXME Clone less. If join keys are unique, no need for clone.
                .and_modify(|records| records.push(left_record.clone()))
                .or_insert_with(|| vec![left_record]);
        }

        let mut ret = Vec::<Record>::new();

        for right_record in input_right {
            let right_sql_value = right_record.get_sql_value(&right_field)?;
            if let Some(left_records) = hash_table.get(&SqlValueHashKey::from(right_sql_value)) {
                ret.append(
                    &mut left_records
                        .iter()
                        .map(|left_record| left_record.clone().join(right_record.clone()))
                        .collect::<Vec<Record>>(),
                );
            }
        }

        let it = if ret.is_empty() {
            RecordIterator::new(
                RecordFieldRefSchema::new(None, vec![]),
                Vec::<SqlValues>::new(),
            )
        } else {
            let r = ret.first().unwrap();
            let schema = r.schema();
            RecordIterator::new(schema.clone(), ret)
        };

        Ok(it)
    }
}
