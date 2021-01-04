use apllodb_shared_components::SqlType;
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn data_type(ast_data_type: apllodb_ast::DataType) -> SqlType {
        match ast_data_type {
            apllodb_ast::DataType::IntegerTypeVariant(i) => match i {
                apllodb_ast::IntegerType::SmallIntVariant => SqlType::small_int(),
                apllodb_ast::IntegerType::IntegerVariant => SqlType::integer(),
                apllodb_ast::IntegerType::BigIntVariant => SqlType::big_int(),
            },
        }
    }
}
