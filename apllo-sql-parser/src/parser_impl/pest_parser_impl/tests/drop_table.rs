use super::super::PestParserImpl;
use crate::apllo_ast::{
    DropTableStatement, EmbeddedSqlStatement, Identifier, SqlExecutableStatement,
    SqlSchemaManipulationStatement, SqlSchemaStatement, StatementOrDeclaration,
};
use crate::parser_interface::ParserLike;
use crate::AplloAst;

struct DropTableParams {
    table_name: String,
}
impl DropTableParams {
    fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.into(),
        }
    }
}

#[test]
fn test_drop_table_accepted() {
    let sql_vs_expected_params: Vec<(&str, DropTableParams)> = vec![
        ("DROP TABLE t", DropTableParams::new("t")),
        ("DROP TABLE t;", DropTableParams::new("t")),
        ("  DROP\tTABLE\nt ", DropTableParams::new("t")),
        ("DROP TABLE 机", DropTableParams::new("机")),
        ("DROP TABLE 🍙", DropTableParams::new("🍙")),
        // Keyword is case-sensitive.
        ("DROP TABLE drop", DropTableParams::new("drop")),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_params) in sql_vs_expected_params {
        match parser.parse(sql) {
            Ok(AplloAst(EmbeddedSqlStatement {
                statement_or_declaration:
                    StatementOrDeclaration::SqlExecutableStatementVariant(
                        SqlExecutableStatement::SqlSchemaStatementVariant(
                            SqlSchemaStatement::SqlSchemaManipulationStatementVariant(
                                SqlSchemaManipulationStatement::DropTableStatementVariant(
                                    DropTableStatement {
                                        table_name: Identifier(table_name),
                                    },
                                ),
                            ),
                        ),
                    ),
            })) => assert_eq!(table_name, expected_params.table_name),

            Ok(ast) => panic!(
                "'{}' should be parsed as DROP TABLE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_drop_table_rejected() {
    let sqls: Vec<&str> = vec![
        // Keyword is case-sensitive.
        "drop table t",
        // Does not accept trailing letter.
        "DROP TABLE t x",
        // Does not accept 2nd statement.
        "DROP TABLE t; DROP TABLE t2;",
        // Does not accept heading letter.
        "x DROP TABLE t",
        // Does not accept illegal white space.
        "DROP　TABLE t",
        // Does not accept keyword as identifier.
        "DROP TABLE CREATE",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
