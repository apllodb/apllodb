pub(crate) mod query_plan_tree;

use std::convert::TryFrom;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, AstTranslator, ColumnName, FieldIndex, FullFieldReference,
    Ordering,
};
use apllodb_sql_parser::apllodb_ast::{self, FromItem, SelectCommand, SelectField};
use apllodb_storage_engine_interface::{AliasDef, ProjectionQuery};

use self::query_plan_tree::{
    query_plan_node::{
        LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf, QueryPlanNodeUnary, UnaryPlanOperation,
    },
    QueryPlanTree,
};

/// Query plan from which an executor can do its work deterministically.
#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct QueryPlan {
    pub(crate) plan_tree: QueryPlanTree,
    // TODO evaluated cost, etc...
    // See PostgreSQL's plan structure: <https://github.com/postgres/postgres/blob/master/src/include/nodes/plannodes.h#L110>
}

impl TryFrom<SelectCommand> for QueryPlan {
    type Error = ApllodbError;

    fn try_from(sc: SelectCommand) -> ApllodbResult<Self> {
        if sc.grouping_elements.is_some() {
            unimplemented!();
        }
        if sc.having_conditions.is_some() {
            unimplemented!();
        }

        let from_items = Self::select_command_into_from_items(sc.clone());

        let from_item = if from_items.len() != 1 {
            unimplemented!()
        } else {
            from_items.first().unwrap().clone()
        };

        let select_fields = sc.select_fields.into_vec();
        let ffrs: Vec<FullFieldReference> =
            Self::select_fields_into_ffrs(&select_fields, &from_items)?;
        // この時点でschemaつくればええやん。AliasDefなんて作らずに
        // -> んー、ProjectionQueryとRecordRefSchemaは両立は不要だな。
        // あと、ここでschema作る問題は、2テーブル以上にまたがったときにどうやってschemaをsplitするか。
        //
        // ProjectionQuery (特に * の対応) を廃止し、ここで「ストレージエンジンへのカタログ問い合わせ」を行ってFFR解決まで行うことも視野に入れつつ、
        //
        //
        // > 2テーブル以上にまたがったときにどうやってschemaをsplitするか。
        // FFRにCorrelationReferenceでfilterする関数を設ける？その方針だったら Schema::joined は廃止して、最終的なRecordのSchemaから分割していく作りになる

        let column_names: Vec<ColumnName> = ffrs
            .iter()
            .map(|ffr| ffr.as_column_name())
            .cloned()
            .collect();

        let alias_def = AliasDef::from(ffrs);

        let leaf_node = QueryPlanNode::Leaf(QueryPlanNodeLeaf {
            op: LeafPlanOperation::SeqScan {
                table_name: AstTranslator::table_name(from_item.table_name)?, // correlation alias情報が消えている
                projection: ProjectionQuery::ColumnNames(column_names),
                alias_def,
            },
        });

        let node1 = if let Some(condition) = sc.where_condition {
            let selection_op = UnaryPlanOperation::Selection {
                condition: AstTranslator::condition_in_select(condition, from_items.clone())?,
            };
            QueryPlanNode::Unary(QueryPlanNodeUnary {
                op: selection_op,
                left: Box::new(leaf_node),
            })
        } else {
            leaf_node
        };

        let node2 = if let Some(order_byes) = sc.order_bys {
            QueryPlanNode::Unary(QueryPlanNodeUnary {
                op: Self::sort_node(order_byes, &from_items)?,
                left: Box::new(node1),
            })
        } else {
            node1
        };

        let root_node = QueryPlanNode::Unary(QueryPlanNodeUnary {
            op: Self::projection_node(select_fields, &from_items)?,
            left: Box::new(node2),
        });

        Ok(QueryPlan::new(QueryPlanTree::new(root_node)))
    }
}

impl QueryPlan {
    fn select_command_into_from_items(select_command: SelectCommand) -> Vec<FromItem> {
        select_command
            .from_items
            .expect("currently SELECT w/o FROM is unimplemented")
            .into_vec()
    }

    fn select_fields_into_ffrs(
        select_fields: &[SelectField],
        from_items: &[FromItem],
    ) -> ApllodbResult<Vec<FullFieldReference>> {
        select_fields
            .iter()
            .map(|select_field| Self::select_field_into_ffr(select_field, &from_items))
            .collect::<ApllodbResult<_>>()
    }

    fn select_field_into_ffr(
        select_field: &SelectField,
        from_items: &[FromItem],
    ) -> ApllodbResult<FullFieldReference> {
        match &select_field.expression {
            apllodb_ast::Expression::ConstantVariant(_) => {
                unimplemented!();
            }
            apllodb_ast::Expression::ColumnReferenceVariant(_) => {
                AstTranslator::select_field_column_reference(
                    select_field.clone(),
                    from_items.to_vec(),
                )
            }
            apllodb_ast::Expression::UnaryOperatorVariant(_, _)
            | apllodb_ast::Expression::BinaryOperatorVariant(_, _, _) => {
                // TODO このレイヤーで計算しちゃいたい
                unimplemented!();
            }
        }
    }

    fn projection_node(
        select_fields: Vec<SelectField>,
        from_items: &[FromItem],
    ) -> ApllodbResult<UnaryPlanOperation> {
        let node = UnaryPlanOperation::Projection {
            fields: select_fields
                .into_iter()
                .map(|select_field| {
                    AstTranslator::select_field_column_reference(select_field, from_items.to_vec())
                        .map(FieldIndex::from)
                })
                .collect::<ApllodbResult<_>>()?,
        };
        Ok(node)
    }

    fn sort_node(
        ast_order_byes: apllodb_ast::NonEmptyVec<apllodb_ast::OrderBy>,
        from_items: &[FromItem],
    ) -> ApllodbResult<UnaryPlanOperation> {
        let order_byes = ast_order_byes.into_vec();

        let field_orderings: Vec<(FieldIndex, Ordering)> = order_byes
            .into_iter()
            .map(|order_by| {
                let expression =
                    AstTranslator::expression_in_select(order_by.expression, from_items.to_vec())?;
                let ffr = match expression {
                    apllodb_shared_components::Expression::FullFieldReferenceVariant(ffr) => ffr,
                    apllodb_shared_components::Expression::ConstantVariant(_)
                    | apllodb_shared_components::Expression::UnaryOperatorVariant(_, _)
                    | apllodb_shared_components::Expression::BooleanExpressionVariant(_) => {
                        unimplemented!()
                    }
                };
                let index = FieldIndex::from(ffr);
                let ordering = AstTranslator::ordering(order_by.ordering);
                Ok((index, ordering))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(UnaryPlanOperation::Sort { field_orderings })
    }
}
