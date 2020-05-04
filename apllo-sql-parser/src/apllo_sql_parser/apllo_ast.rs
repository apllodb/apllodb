//! APLLO SQL's AST.
//!
//! This module provides the root node ([AplloAst](struct.AplloAst.html)) and other intermediate nodes.
//!
//! Intermediate nodes have 1-to-1 relationship with terms defined in [APLLO SQL syntax](https://github.com/darwin-education/apllo/tree/master/apllo-sql-parser/src/pest_grammar/apllo_sql.pest).

#![allow(missing_docs)]
use types::NonEmptyVec;

pub mod types;

/// The AST root of APLLO SQL.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AplloAst(pub Command);

// TODO: Auto generation from .pest file?

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
    CreateTableCommandVariant(CreateTableCommand),
    DropTableCommandVariant(DropTableCommand),
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
    pub column_constraints: NonEmptyVec<ColumnConstraint>,
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

/*
 * ----------------------------------------------------------------------------
 * Constraints
 * ----------------------------------------------------------------------------
 */

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ColumnConstraint {
    NotNullVariant,
}
