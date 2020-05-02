/// The AST root of APLLO SQL.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AplloAst(pub EmbeddedSqlStatement);

// TODO: 以下の定義は、 .pest から自動生成できるはず。
//   ルールに `|` を含む場合: enum
//   else: struct
//
//   ルールが _{} の場合: そのルールには構造を作らずにskip

/*
 * ----------------------------------------------
 * 5.4 Names and identifiers
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Identifier(pub String);

/*
 * ----------------------------------------------
 * 11.31 <drop table statement>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct DropTableStatement {
    pub table_name: Identifier,
}

/*
 * ----------------------------------------------
 * 13.4 <SQL procedure statement>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SqlExecutableStatement {
    SqlSchemaStatementVariant(SqlSchemaStatement),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SqlSchemaStatement {
    SqlSchemaManipulationStatementVariant(SqlSchemaManipulationStatement),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SqlSchemaManipulationStatement {
    DropTableStatementVariant(DropTableStatement),
}

/*
 * ----------------------------------------------
 * 21.1 <embedded SQL host program>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EmbeddedSqlStatement {
    pub statement_or_declaration: StatementOrDeclaration,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum StatementOrDeclaration {
    SqlExecutableStatementVariant(SqlExecutableStatement),
}
