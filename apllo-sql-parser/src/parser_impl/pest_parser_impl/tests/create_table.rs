use super::super::PestParserImpl;
use crate::apllo_ast::{
    ColumnConstraintDefinition, ColumnDefinition, DataType, EmbeddedSqlStatement, Identifier,
    SqlExecutableStatement, SqlSchemaDefinitionStatement, SqlSchemaStatement,
    StatementOrDeclaration, TableContentsSource, TableDefinition, TableElement, TableElementList,
};
use crate::parser_interface::ParserLike;
use crate::AplloAst;

#[derive(Clone, Eq, PartialEq, Debug)]
struct CreateTableParams {
    table_name: String,
    column_definitions: Vec<ColumnDefinition>,
}
impl CreateTableParams {
    fn new(table_name: &str, coldefs: Vec<ColumnDefinition>) -> Self {
        Self {
            table_name: table_name.into(),
            column_definitions: coldefs,
        }
    }
}

macro_rules! coldef {
    ($column_name: expr, $data_type_variant: expr, $column_constraint_definitions: expr) => {
        ColumnDefinition {
            column_name: Identifier($column_name.to_string()),
            data_type: $data_type_variant,
            column_constraint_definitions: $column_constraint_definitions,
        }
    };
}

#[test]
fn test_create_table_accepted() {
    let sql_vs_expected_params: Vec<(&str, CreateTableParams)> = vec![
        (
            "CREATE TABLE t (id INT)",
            CreateTableParams::new("t", vec![coldef!("id", DataType::IntVariant, vec![])]),
        ),
        (
            "CREATE TABLE t (id INT NOT NULL, c1 INT)",
            CreateTableParams::new(
                "t",
                vec![
                    coldef!(
                        "id",
                        DataType::IntVariant,
                        vec![ColumnConstraintDefinition::NotNullVariant]
                    ),
                    coldef!("c1", DataType::IntVariant, vec![]),
                ],
            ),
        ),
    ];

    let parser = PestParserImpl::new();

    for (sql, expected_params) in sql_vs_expected_params {
        match parser.parse(sql) {
            Ok(AplloAst(EmbeddedSqlStatement {
                statement_or_declaration:
                    StatementOrDeclaration::SqlExecutableStatementVariant(
                        SqlExecutableStatement::SqlSchemaStatementVariant(
                            SqlSchemaStatement::SqlSchemaDefinitionStatementVariant(
                                SqlSchemaDefinitionStatement::TableDefinitionVariant(
                                    TableDefinition {
                                        table_name: Identifier(table_name),
                                        table_contents_source:
                                            TableContentsSource::TableElementListVariant(
                                                TableElementList {
                                                    head_table_element,
                                                    tail_table_elements,
                                                },
                                            ),
                                    },
                                ),
                            ),
                        ),
                    ),
            })) => {
                assert_eq!(table_name, expected_params.table_name);

                let parsed_coldefs: Vec<ColumnDefinition> = vec![head_table_element]
                    .into_iter()
                    .chain(tail_table_elements.into_iter())
                    .map(|te| match te {
                        TableElement::ColumnDefinitionVariant(cd) => cd,
                    })
                    .collect();
                assert_eq!(parsed_coldefs, expected_params.column_definitions);
            }

            Ok(ast) => panic!(
                "'{}' should be parsed as CREATE TABLE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_create_table_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack data-type.
        "CREATE TABLE t (c1)",
        // Lack data-type again.
        "CREATE TABLE t (c1 NOT NULL)",
        // Should be parenthesized.
        "CREATE TABLE t c1 INT NOT NULL",
        // `NOT NULL` is a keyword, only a space is allowed.
        "CREATE TABLE t (c1 INT NOT  NULL)",
    ];

    let parser = PestParserImpl::new();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
