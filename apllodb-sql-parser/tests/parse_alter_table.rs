use apllodb_sql_parser::{
    apllodb_ast::{
        Action, AlterTableCommand, ColumnConstraint, ColumnDefinition, Command, DataType,
    },
    ApllodbAst, ApllodbSqlParser,
};

use apllodb_test_support::setup::setup_test_logger;

#[ctor::ctor]
fn test_setup() {
    setup_test_logger();
}

#[test]
fn test_alter_table_accepted() {
    let sql_vs_expected_ast: Vec<(&str, AlterTableCommand)> = vec![
        (
            "ALTER TABLE t ADD COLUMN c1 INTEGER",
            AlterTableCommand::factory(
                "t",
                vec![Action::factory_add_col(ColumnDefinition::factory(
                    "c1",
                    DataType::integer(),
                    vec![],
                ))],
            ),
        ),
        (
            "ALTER TABLE t ADD COLUMN c1 INTEGER NOT NULL, DROP COLUMN c2",
            AlterTableCommand::factory(
                "t",
                vec![
                    Action::factory_add_col(ColumnDefinition::factory(
                        "c1",
                        DataType::integer(),
                        vec![ColumnConstraint::NotNullVariant],
                    )),
                    Action::factory_drop_col("c2"),
                ],
            ),
        ),
    ];

    let parser = ApllodbSqlParser::default();

    for (sql, expected_ast) in sql_vs_expected_ast {
        match parser.parse(sql) {
            Ok(ApllodbAst(Command::AlterTableCommandVariant(alter_table_command))) => {
                assert_eq!(alter_table_command, expected_ast);
            }
            Ok(ast) => panic!(
                "'{}' should be parsed as ALTER TABLE but is parsed like: {:?}",
                sql, ast
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

#[test]
fn test_alter_table_rejected() {
    let sqls: Vec<&str> = vec![
        // Lack action.
        "ALTER TABLE t",
        // Lack data-type.
        "ALTER TABLE t ADD COLUMN c1",
        // Should not be parenthesized.
        "ALTER TABLE t (ADD COLUMN c1 INTEGER)",
    ];

    let parser = ApllodbSqlParser::default();

    for sql in sqls {
        assert!(parser.parse(sql).is_err());
    }
}
