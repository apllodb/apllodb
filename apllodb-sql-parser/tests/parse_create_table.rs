use apllodb_sql_parser::{
    apllodb_ast::{
        ColumnConstraint, ColumnDefinition, Command, CreateTableCommand, DataType, TableConstraint,
        TableElement,
    },
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_create_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, CreateTableCommand)> = vec![
        (
            "CREATE TABLE t (id INTEGER)",
            CreateTableCommand::factory(
                "t",
                vec![TableElement::factory_coldef(ColumnDefinition::factory(
                    "id",
                    DataType::integer(),
                    vec![],
                ))],
            ),
        ),
        (
            "CREATE TABLE t (id INTEGER NOT NULL, c1 INTEGER)",
            CreateTableCommand::factory(
                "t",
                vec![
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "id",
                        DataType::integer(),
                        vec![ColumnConstraint::NotNullVariant],
                    )),
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "c1",
                        DataType::integer(),
                        vec![],
                    )),
                ],
            ),
        ),
        (
            "CREATE TABLE t (c TEXT)",
            CreateTableCommand::factory(
                "t",
                vec![TableElement::factory_coldef(ColumnDefinition::factory(
                    "c",
                    DataType::text(),
                    vec![],
                ))],
            ),
        ),
        (
            "CREATE TABLE t (id INTEGER, c1 INTEGER, PRIMARY KEY (id, c1))",
            CreateTableCommand::factory(
                "t",
                vec![
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "id",
                        DataType::integer(),
                        vec![],
                    )),
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "c1",
                        DataType::integer(),
                        vec![],
                    )),
                    TableElement::factory_tc(TableConstraint::factory_pk(vec!["id", "c1"])),
                ],
            ),
        ),
        (
            "CREATE TABLE t (id INTEGER, PRIMARY KEY (id), c1 INTEGER)",
            CreateTableCommand::factory(
                "t",
                vec![
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "id",
                        DataType::integer(),
                        vec![],
                    )),
                    TableElement::factory_tc(TableConstraint::factory_pk(vec!["id"])),
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "c1",
                        DataType::integer(),
                        vec![],
                    )),
                ],
            ),
        ),
        // https://github.com/eukarya-inc/apllodb/issues/253
        (
            "CREATE TABLE t (id integer, c text)",
            CreateTableCommand::factory(
                "t",
                vec![
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "id",
                        DataType::integer(),
                        vec![],
                    )),
                    TableElement::factory_coldef(ColumnDefinition::factory(
                        "c",
                        DataType::text(),
                        vec![],
                    )),
                ],
            ),
        ),
    ];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::CreateTableCommandVariant(create_table_command))) => {
                assert_eq!(create_table_command, expected_ast);
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
        "CREATE TABLE t c1 INTEGER NOT NULL",
        // `NOT NULL` is a keyword, only a space is allowed.
        "CREATE TABLE t (c1 INTEGER NOT  NULL)",
    ];

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
