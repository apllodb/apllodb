//! apllodb-SQL's AST.
//!
//! This module provides the root node ([ApllodbAst](struct.ApllodbAst.html)) and other intermediate nodes.
//!
//! Intermediate nodes have 1-to-1 relationship with terms defined in [apllodb-SQL syntax](https://github.com/darwin-education/apllodb/tree/master/apllodb-sql-parser/src/pest_grammar/apllodb_sql.pest).

#![allow(missing_docs)]

#[deny(missing_docs)]
pub mod types;

pub use types::NonEmptyVec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The AST root of apllodb-SQL.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApllodbAst(pub Command);

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Constant {
    NumericConstantVariant(NumericConstant),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NumericConstant {
    IntegerConstantVariant(IntegerConstant),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntegerConstant(pub String);

/*
 * ================================================================================================
 * Identifier:
 * ================================================================================================
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier(pub String);

/*
 * ================================================================================================
 * Value Expressions:
 * ================================================================================================
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Condition {
    pub expression: Expression,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DataType {
    IntegerTypeVariant(IntegerType),
}

/*
 * ----------------------------------------------------------------------------
 * Integer Types
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Command {
    CreateDatabaseCommandVariant(CreateDatabaseCommand),
    UseDatabaseCommandVariant(UseDatabaseCommand),

    BeginTransactionCommandVariant,
    CommitTransactionCommandVariant,

    AlterTableCommandVariant(AlterTableCommand),
    CreateTableCommandVariant(CreateTableCommand),
    DropTableCommandVariant(DropTableCommand),

    SelectCommandVariant(SelectCommand),
    InsertCommandVariant(InsertCommand),
    UpdateCommandVariant(UpdateCommand),
    DeleteCommandVariant(DeleteCommand),
}

/*
 * ----------------------------------------------------------------------------
 * ALTER TABLE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AlterTableCommand {
    pub table_name: TableName,
    pub actions: NonEmptyVec<Action>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Action {
    AddColumnVariant(AddColumn),
    DropColumnVariant(DropColumn),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AddColumn {
    pub column_definition: ColumnDefinition,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DropColumn {
    pub column_name: ColumnName,
}

/*
 * ----------------------------------------------------------------------------
 * CREATE DATABASE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateDatabaseCommand {
    pub database_name: DatabaseName,
}

/*
 * ----------------------------------------------------------------------------
 * USE DATABASE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UseDatabaseCommand {
    pub database_name: DatabaseName,
}

/*
 * ----------------------------------------------------------------------------
 * CREATE TABLE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateTableCommand {
    pub table_name: TableName,
    pub table_elements: NonEmptyVec<TableElement>,
}

/*
 * ----------------------------------------------------------------------------
 * DELETE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeleteCommand {
    pub table_name: TableName,
    pub alias: Option<Alias>,
    pub where_condition: Option<Condition>,
}

/*
 * ----------------------------------------------------------------------------
 * DROP TABLE
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DropTableCommand {
    pub table_name: TableName,
}

/*
 * ----------------------------------------------------------------------------
 * INSERT
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectCommand {
    pub select_fields: NonEmptyVec<SelectField>,
    pub from_items: NonEmptyVec<FromItem>,
    pub where_condition: Option<Condition>,
    pub grouping_elements: Option<NonEmptyVec<GroupingElement>>,
    pub having_conditions: Option<NonEmptyVec<Condition>>,
    pub order_bys: Option<NonEmptyVec<OrderBy>>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectField {
    pub expression: Expression,
    pub alias: Option<Alias>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FromItem {
    pub table_name: TableName,
    pub alias: Option<Alias>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GroupingElement {
    ExpressionVariant(Expression),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderBy {
    pub expression: Expression,
    pub ordering: Option<Ordering>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DatabaseName(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableName(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColumnName(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alias(pub Identifier);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Correlation {
    TableNameVariant(TableName),
    AliasVariant(Alias),
}

/*
 * ----------------------------------------------------------------------------
 * Table Elements
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TableElement {
    ColumnDefinitionVariant(ColumnDefinition),
    TableConstraintVariant(TableConstraint),
}

/*
 * ----------------------------------------------------------------------------
 * Constraints
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColumnConstraint {
    NotNullVariant,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TableConstraint {
    PrimaryKeyVariant(NonEmptyVec<ColumnName>),
}

/*
 * ----------------------------------------------------------------------------
 * Column Definitions
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColumnDefinition {
    pub column_name: ColumnName,
    pub data_type: DataType,
    pub column_constraints: Vec<ColumnConstraint>,
}
