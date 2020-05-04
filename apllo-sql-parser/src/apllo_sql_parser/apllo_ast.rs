#![allow(missing_docs)]

//! APLLO SQL's AST.
//!
//! This module provides the root node ([AplloAst](struct.AplloAst.html)) and other intermediate nodes.

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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TableName(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColumnName(pub Identifier);

/*
 * ----------------------------------------------
 * 6.1 <data type>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum DataType {
    IntVariant,
}

/*
 * ----------------------------------------------
 * 6.3 <value expression primary>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ValueExpressionPrimary {
    ParenthesizedValueExpressionVariant(ParenthesizedValueExpression),
    NonparenthesizedValueExpressionPrimaryVariant(NonparenthesizedValueExpressionPrimary),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParenthesizedValueExpression {
    pub value_expression: Box<ValueExpression>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct NonparenthesizedValueExpressionPrimary {
    pub column_reference: ColumnReference,
}

/*
 * ----------------------------------------------
 * 6.7 <column reference>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColumnReference {
    pub identifier: Identifier,
}

/*
 * ----------------------------------------------
 * 6.28 <value expression>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ValueExpression {
    CommonValueExpressionVariant(CommonValueExpression),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum CommonValueExpression {
    ReferenceValueExpressionVariant(ReferenceValueExpression),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ReferenceValueExpression {
    ValueExpressionPrimaryVariant(ValueExpressionPrimary),
}

/*
 * ----------------------------------------------
 * 7.4 <table expression>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TableExpression {
    pub from_clause: FromClause,
}

/*
 * ----------------------------------------------
 * 7.5 <from clause>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FromClause {
    pub table_name: TableName,
}

/*
 * ----------------------------------------------
 * 7.16 <query specification>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct QuerySpecification {
    pub select_list: SelectList,
    pub table_expression: TableExpression,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectList {
    pub head_select_sublist: SelectSublist,
    pub tail_select_sublists: Vec<SelectSublist>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectSublist {
    pub value_expression: ValueExpression,
    pub as_clause: Option<AsClause>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AsClause {
    pub column_name: ColumnName,
}

/*
 * ----------------------------------------------
 * 7.17 <query expression>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum QueryExpression {
    QueryExpressionBodyVariant(QueryExpressionBody),
}

/*
 * ----------------------------------------------
 * 11.3 <table definition>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TableDefinition {
    pub table_name: TableName,
    pub table_contents_source: TableContentsSource,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TableContentsSource {
    TableElementListVariant(TableElementList),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TableElementList {
    pub head_table_element: TableElement,
    pub tail_table_elements: Vec<TableElement>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TableElement {
    ColumnDefinitionVariant(ColumnDefinition),
}

/*
 * ----------------------------------------------
 * 11.4 <column definition>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColumnDefinition {
    pub column_name: ColumnName,
    pub data_type: DataType,
    pub column_constraint_definitions: Vec<ColumnConstraintDefinition>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ColumnConstraintDefinition {
    NotNullVariant,
}

/*
 * ----------------------------------------------
 * 11.31 <drop table statement>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct DropTableStatement {
    pub table_name: TableName,
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
    SqlSchemaDefinitionStatementVariant(SqlSchemaDefinitionStatement),
    SqlSchemaManipulationStatementVariant(SqlSchemaManipulationStatement),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SqlSchemaDefinitionStatement {
    TableDefinitionVariant(TableDefinition),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SqlSchemaManipulationStatement {
    DropTableStatementVariant(DropTableStatement),
}

/*
 * ----------------------------------------------
 * 14 <with clause> ::=
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum QueryExpressionBody {
    QueryTermVariant(QueryTerm),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum QueryTerm {
    QueryPrimaryVariant(QueryPrimary),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum QueryPrimary {
    SimpleTableVariant(SimpleTable),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SimpleTable {
    QuerySpecificationVariant(QuerySpecification),
}

/*
 * ----------------------------------------------
 * 20.7 <prepare statement>
 * ----------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum PreparableStatement {
    PreparableSqlDataStatementVariant(PreparableSqlDataStatement),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum PreparableSqlDataStatement {
    DynamicSelectStatementVariant(DynamicSelectStatement),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum DynamicSelectStatement {
    QueryExpressionVariant(QueryExpression),
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
