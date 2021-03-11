use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast;

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::QueryPlanNode;
use super::QueryPlanner;

impl QueryPlanner {
    /// outputs plan sub-tree to:
    ///
    /// - fetches records from tables in storage engines
    /// - calculate records from sub-query
    /// - merge (JOIN/UNION) these records
    pub(super) fn from_item(_ast_from_item: &apllodb_ast::FromItem) -> ApllodbResult<QueryPlanNode> {
        // 各テーブルのプロジェクション情報も必要だ
        // これはSELECT の select field からはわからない。
        // where の中だけで参照する（が最終的には捨てられる）カラムがあるので


        todo!()
    }
}
