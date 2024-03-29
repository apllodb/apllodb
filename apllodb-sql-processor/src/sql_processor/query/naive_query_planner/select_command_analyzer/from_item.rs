use super::SelectCommandAnalyzer;
use crate::{
    ast_translator::AstTranslator,
    correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_name::CorrelationName,
    },
    records::record_schema::RecordSchema,
    sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
        node_id::QueryPlanNodeId,
        node_kind::{QueryPlanNodeBinary, QueryPlanNodeKind},
        node_repo::QueryPlanNodeRepository,
        operation::BinaryPlanOperation,
    },
};
use apllodb_shared_components::{ApllodbError, ApllodbResult, Expression, SchemaIndex};
use apllodb_sql_parser::apllodb_ast;

impl SelectCommandAnalyzer {
    pub(in super::super) fn from_item_correlations(
        &self,
    ) -> ApllodbResult<Vec<AliasedCorrelationName>> {
        if let Some(ast_from_item) = self.ast_from_item() {
            Self::ast_from_item_into_correlation_references(ast_from_item)
        } else {
            Ok(vec![])
        }
    }

    /// Indexes appear in JOIN ... *ON ...* condition.
    pub(in super::super) fn from_item_indexes(&self) -> ApllodbResult<Vec<SchemaIndex>> {
        if let Some(ast_from_item) = self.ast_from_item() {
            Self::ast_from_item_into_indexes(ast_from_item)
        } else {
            Ok(vec![])
        }
    }

    /// Creates JOIN nodes recursively (post-order DFS) from apllodb_ast::FromItem.
    ///
    /// Say current FromItem is a JoinVariant:
    /// If left child is a TableVariant, then its QueryPlanNodeId is found from QueryPlanNodeRepository by CorrelationIndex.
    /// If right child is a JoinVariant, then its QueryPlanNodeId is create from this function recursively.
    pub(in super::super) fn create_join_nodes(
        &self,
        node_repo: &QueryPlanNodeRepository,
    ) -> ApllodbResult<()> {
        /// returns NodeId of node created from cur_from_item
        fn rec_create(
            cur_from_item: &apllodb_ast::FromItem,
            widest_schema: &RecordSchema,
            node_repo: &QueryPlanNodeRepository,
        ) -> ApllodbResult<QueryPlanNodeId> {
            let from_item_correlations =
                SelectCommandAnalyzer::ast_from_item_into_correlation_references(cur_from_item)?;
            let joined_schema = widest_schema.filter_by_correlations(&from_item_correlations);

            match cur_from_item {
                apllodb_ast::FromItem::TableNameVariant { table_name, .. } => {
                    let corr_name = CorrelationName::TableNameVariant(AstTranslator::table_name(
                        table_name.clone(),
                    )?);
                    let node_id = node_repo.find_correlation_node(&corr_name)?;
                    Ok(node_id)
                }
                apllodb_ast::FromItem::JoinVariant {
                    join_type,
                    left,
                    right,
                    on,
                } => {
                    let left_node_id = rec_create(&*left, widest_schema, node_repo)?;
                    let right_node_id = rec_create(&*right, widest_schema, node_repo)?;
                    let mid_node_id =
                        node_repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                            left: left_node_id,
                            right: right_node_id,
                            op: SelectCommandAnalyzer::join_variant_into_join_op(
                                join_type,
                                on,
                                joined_schema,
                                &from_item_correlations,
                            )?,
                        }));
                    Ok(mid_node_id)
                }
            }
        }

        if let Some(ast_from_item) = self.ast_from_item() {
            rec_create(ast_from_item, &self.widest_schema()?, node_repo).map(|_| ())
        } else {
            Ok(())
        }
    }

    fn ast_from_item(&self) -> Option<&apllodb_ast::FromItem> {
        self.select_command.from_item.as_ref()
    }
    fn ast_from_item_into_correlation_references(
        ast_from_item: &apllodb_ast::FromItem,
    ) -> ApllodbResult<Vec<AliasedCorrelationName>> {
        match ast_from_item {
            apllodb_ast::FromItem::TableNameVariant { table_name, alias } => {
                let table_name = AstTranslator::table_name(table_name.clone())?;
                let correlation_name = CorrelationName::TableNameVariant(table_name);
                let correlation_alias = match alias {
                    Some(a) => Some(AstTranslator::correlation_alias(a.clone())?),
                    None => None,
                };
                let aliased_correlation_name =
                    AliasedCorrelationName::new(correlation_name, correlation_alias);

                Ok(vec![aliased_correlation_name])
            }
            apllodb_ast::FromItem::JoinVariant { left, right, .. } => {
                let mut left_corr_ref = Self::ast_from_item_into_correlation_references(left)?;
                let mut right_corr_ref = Self::ast_from_item_into_correlation_references(right)?;
                left_corr_ref.append(&mut right_corr_ref);
                Ok(left_corr_ref)
            }
        }
    }

    fn ast_from_item_into_indexes(
        ast_from_item: &apllodb_ast::FromItem,
    ) -> ApllodbResult<Vec<SchemaIndex>> {
        match ast_from_item {
            apllodb_ast::FromItem::TableNameVariant { .. } => Ok(vec![]),
            apllodb_ast::FromItem::JoinVariant {
                left, right, on, ..
            } => {
                let expression = AstTranslator::expression_in_select(
                    on.expression.clone(),
                    &Self::ast_from_item_into_correlation_references(ast_from_item)?,
                )?;

                let mut idxs = expression.to_schema_indexes();
                idxs.append(&mut Self::ast_from_item_into_indexes(left.as_ref())?);
                idxs.append(&mut Self::ast_from_item_into_indexes(right.as_ref())?);
                Ok(idxs)
            }
        }
    }

    fn join_variant_into_join_op(
        join_type: &apllodb_ast::JoinType,
        on: &apllodb_ast::Condition,
        joined_schema: RecordSchema,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<BinaryPlanOperation> {
        assert!(
            matches!(join_type, apllodb_ast::JoinType::InnerJoin,),
            "only InnerJoin is supported currently"
        );

        match &on.expression {
            apllodb_ast::Expression::BinaryOperatorVariant(bin_op, left, right) => {
                let left =
                    AstTranslator::expression_in_select(*left.clone(), from_item_correlations)?;
                let right =
                    AstTranslator::expression_in_select(*right.clone(), from_item_correlations)?;

                match (bin_op, left, right) {
                    (
                        apllodb_ast::BinaryOperator::Equal,
                        Expression::SchemaIndexVariant(left_field),
                        Expression::SchemaIndexVariant(right_field),
                    ) => Ok(BinaryPlanOperation::HashJoin {
                        left_field,
                        right_field,
                        joined_schema,
                    }),
                    _ => Err(ApllodbError::feature_not_supported(
                        "only `ON a = b` JOIN condition is supported currently",
                    )),
                }
            }
            _ => Err(ApllodbError::feature_not_supported(
                "only `ON a = b` JOIN condition is supported currently",
            )),
        }
    }
}
