use super::query::query_plan::query_plan_tree::query_plan_node::operation::SeqScanOperation;

struct SelectCommandAnalyzer {}

impl SelectCommandAnalyzer {
    fn seq_scan_operations(&self) -> Vec<SeqScanOperation> {}
}
