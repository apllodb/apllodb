use apllodb_shared_components::{ApllodbResult, AstTranslator, CorrelationReference};

use super::SelectCommandAnalyzer;

impl SelectCommandAnalyzer {
    pub(in super::super) fn from_item_correlation_references(
        &self,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        let ast_from_item = self
            .select_command
            .from_item
            .as_ref()
            .expect("currently SELECT w/o FROM is unimplemented")
            .clone();
        AstTranslator::from_item(ast_from_item)
    }
}
