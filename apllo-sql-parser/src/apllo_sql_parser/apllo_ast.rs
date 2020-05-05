//! APLLO SQL's AST.
//!
//! This module provides the root node ([AplloAst](struct.AplloAst.html)) and other intermediate nodes.
//!
//! Intermediate nodes have 1-to-1 relationship with terms defined in [APLLO SQL syntax](https://github.com/darwin-education/apllo/tree/master/apllo-sql-parser/src/pest_grammar/apllo_sql.pest).

#![allow(missing_docs)]

pub mod types;

pub use types::NonEmptyVec;

/// The AST root of APLLO SQL.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AplloAst(pub Command);

// TODO: Auto generation from .pest file?

/*
 * ================================================================================================
 * Lexical Structure:
 * ================================================================================================
 */

/*
 * ----------------------------------------------------------------------------
 * Constants
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Constant {
    NumericConstantVariant(NumericConstant),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum NumericConstant {
    IntegerConstantVariant(IntegerConstant),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct IntegerConstant(pub String);

/*
 * ================================================================================================
 * Identifier:
 * ================================================================================================
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Identifier(pub String);

/*
 * ================================================================================================
 * Value Expressions:
 * ================================================================================================
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Condition {
    pub expression: Expression,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Expression {
    ConstantVariant(Constant),
    ColumnReferenceVariant(ColumnReference),
}

/*
 * ----------------------------------------------------------------------------
 * Column References
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColumnReference {
    pub correlation: Option<Correlation>,
    pub column_name: ColumnName,
}

/*
 * ================================================================================================
 * Data Types:
 * ================================================================================================
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum DataType {
    IntegerTypeVariant(IntegerType),
}

/*
 * ----------------------------------------------------------------------------
 * Integer Types
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum IntegerType {
    SmallIntVariant,
    IntegerVariant,
    BigIntVariant,
}

/*
 * ================================================================================================
 * Commands:
 * ================================================================================================
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Command {
    AlterTableCommandVariant(AlterTableCommand),
    CreateTableCommandVariant(CreateTableCommand),
    DropTableCommandVariant(DropTableCommand),
    InsertCommandVariant(InsertCommand),
    SelectCommandVariant(SelectCommand),
    UpdateCommandVariant(UpdateCommand),
}

/*
 * ----------------------------------------------------------------------------
 * ALTER TABLE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlterTableCommand {
    pub table_name: TableName,
    pub actions: NonEmptyVec<Action>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Action {
    AddColumnVariant(AddColumn),
    DropColumnVariant(DropColumn),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddColumn {
    pub column_name: ColumnName,
    pub data_type: DataType,
    pub column_constraints: Vec<ColumnConstraint>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct DropColumn {
    pub column_name: ColumnName,
}

/*
 * ----------------------------------------------------------------------------
 * CREATE TABLE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CreateTableCommand {
    pub table_name: TableName,
    pub create_table_column_definitions: NonEmptyVec<CreateTableColumnDefinition>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CreateTableColumnDefinition {
    pub column_name: ColumnName,
    pub data_type: DataType,
    pub column_constraints: Vec<ColumnConstraint>,
}

/*
 * ----------------------------------------------------------------------------
 * DROP TABLE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct DropTableCommand {
    pub table_name: TableName,
}

/*
 * ----------------------------------------------------------------------------
 * INSERT
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct InsertCommand {
    pub table_name: TableName,
    pub alias: Option<Alias>,
    pub column_names: NonEmptyVec<ColumnName>,
    pub expressions: NonEmptyVec<Expression>,
}

/*
 * ----------------------------------------------------------------------------
 * SELECT
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectCommand {
    pub select_fields: NonEmptyVec<SelectField>,
    pub from_items: NonEmptyVec<FromItem>,
    pub where_condition: Option<Condition>,
    pub grouping_elements: Option<NonEmptyVec<GroupingElement>>,
    pub having_conditions: Option<NonEmptyVec<Condition>>,
    pub order_bys: Option<NonEmptyVec<OrderBy>>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectField {
    pub expression: Expression,
    pub alias: Option<Alias>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FromItem {
    pub table_name: TableName,
    pub alias: Option<Alias>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GroupingElement {
    ExpressionVariant(Expression),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct OrderBy {
    pub expression: Expression,
    pub ordering: Option<Ordering>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Ordering {
    AscVariant,
    DescVariant,
}

/*
 * ----------------------------------------------------------------------------
 * SELECT
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct UpdateCommand {
    pub table_name: TableName,
    pub alias: Option<Alias>,
    pub column_name: ColumnName,
    pub expression: Expression,
    pub where_condition: Option<Condition>,
}

/*
 * ================================================================================================
 * Misc:
 * ================================================================================================
 */

/*
 * ----------------------------------------------------------------------------
 * Names
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TableName(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColumnName(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Alias(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Correlation {
    TableNameVariant(TableName),
    AliasVariant(Alias),
}

/*
 * ----------------------------------------------------------------------------
 * Constraints
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ColumnConstraint {
    NotNullVariant,
}
