use crate::ColumnConstraintKind;
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn column_constraint(
        ast_column_constraint: apllodb_ast::ColumnConstraint,
    ) -> Option<ColumnConstraintKind> {
        match ast_column_constraint {
            apllodb_ast::ColumnConstraint::NotNullVariant => {
                None // nullability is not held as ColumnConstraintKind
            }
        }
    }

    pub fn nullable(ast_column_constraints: &[apllodb_ast::ColumnConstraint]) -> bool {
        !ast_column_constraints
            .iter()
            .any(|cc| matches!(cc, apllodb_ast::ColumnConstraint::NotNullVariant))
    }
}
