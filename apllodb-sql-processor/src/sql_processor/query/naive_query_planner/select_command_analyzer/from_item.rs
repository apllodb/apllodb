use apllodb_shared_components::{ApllodbResult, AstTranslator, CorrelationReference};

use super::SelectCommandAnalyzer;

impl SelectCommandAnalyzer {
    pub(in super::super) fn from_item_correlation_references(
        &self,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        let ast_from_item = self.select_command.from_item.as_ref();

        if let Some(ast_from_item) = ast_from_item {
            AstTranslator::from_item(ast_from_item.clone())
        } else {
            Ok(vec![])
        }
    }
}
