pub(crate) mod sql_compare_result;
pub(crate) mod sql_value_hash_key;

use std::hash::Hash;

use crate::{
    data_structure::{
        data_type::{data_type_kind::DataTypeKind, DataType},
        expression::{constant::Constant, Expression},
    },
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
    CharacterConstant, NumericConstant,
};
use serde::{Deserialize, Serialize};

use self::sql_compare_result::SqlCompareResult;

pub const SQL_VALUE_NULL: Option<i16> = None;

/// SQL-typed value that is efficiently compressed.
///
/// # Comparing SqlValues
///
/// An SqlValue implements Into<[Constant](crate::Constant)>.
/// Constant is in hierarchical structure.
/// If top-level variant (e.g. [NumericConstantVariant](crate::Constant::NumericConstantVariant)) are the same among two values,
/// these two are **comparable**, meaning equality comparison to them is valid.
/// Also, ordered comparison is valid for values within some top-level variant of Constant.
/// Such variants and values within one are called **ordered**.
/// **Ordered** is stronger property than **comparable**.
///
/// ## Failures on comparison
///
/// Comparing non-**comparable** values and ordered comparison to non-**ordered** values cause [ApllodbErrorKind::DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch).
///
/// ## Comparison with NULL
///
/// Any SqlValue can calculate equality- and ordered- comparison with NULL value.
///
/// Equality-comparison and ordered-comparison with NULL is evaluated to NULL.
/// NULL is always evaluated as FALSE in boolean context (, therefore all of `x = NULL`, `x != NULL`, `x < NULL`, `x > NULL` are evaluated to FALSE in boolean context).
///
/// # Hashing SqlValues
///
/// Hashed values are sometimes used in query execution (e.g. hash-join, hash-aggregation).
/// SqlValue implements `Hash` but does not `Eq` so SqlValue cannot be used as hash key of `HashMap` and `HashSet`.
///
/// Use [SqlValueHashKey](self::sql_value_hash_key::SqlValueHashKey) for that purpose.
#[derive(Clone, Serialize, Deserialize)]
pub struct SqlValue {
    data_type: DataType,
    raw: Vec<u8>,
}
impl PartialEq for SqlValue {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.sql_compare(other), Ok(SqlCompareResult::Eq))
    }
}
impl Hash for SqlValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match Constant::from(self) {
            Constant::Null => {
                // Generating different hash value for each NULL value to avoid collision in hash table.
                let v = fastrand::u64(..);
                v.hash(state)
            }
            Constant::NumericConstantVariant(nc) => match nc {
                NumericConstant::IntegerConstantVariant(ic) => {
                    let v = ic.as_i64();
                    v.hash(state)
                }
            },
            Constant::CharacterConstantVariant(cc) => match cc {
                CharacterConstant::TextConstantVariant(tc) => {
                    let v = tc.as_str();
                    v.hash(state)
                }
            },
        }
    }
}
impl std::fmt::Debug for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
SqlValue {{
    data_type: {:?},
    raw(into Expression): {:?}
}}",
            self.data_type,
            Expression::from(self)
        )
    }
}

impl SqlValue {
    /// Convert rust_value into `SqlValue`
    ///
    /// # Failures
    ///
    /// - Errors from `T::pack()`.
    pub fn pack<T>(into_type: &DataType, rust_value: &T) -> ApllodbResult<Self>
    where
        T: SqlConvertible,
    {
        let raw = T::pack(into_type, rust_value)?;
        Ok(Self {
            data_type: into_type.clone(),
            raw,
        })
    }

    /// Makes NULL SqlValue
    pub fn null() -> Self {
        Self::pack(
            &DataType::new(DataTypeKind::SmallInt, true),
            &SQL_VALUE_NULL,
        )
        .expect("creating NULL should always succeed")
    }

    /// Retrieve Rust value
    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        T::unpack(&self.data_type, &self.raw)
    }

    /// DataType of this value
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Compares two SqlValues.
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - `self` and `other` have different top-level variant of [Constant](crate::Constant).
    pub fn sql_compare(&self, other: &Self) -> ApllodbResult<SqlCompareResult> {
        match (Constant::from(self), Constant::from(other)) {
            (Constant::Null, _) | (_, Constant::Null) => Ok(SqlCompareResult::Null),
            (
                Constant::NumericConstantVariant(self_nc),
                Constant::NumericConstantVariant(other_nc),
            ) => match (self_nc, other_nc) {
                (
                    NumericConstant::IntegerConstantVariant(self_ic),
                    NumericConstant::IntegerConstantVariant(other_ic),
                ) => {
                    let (self_i64, other_i64) = (self_ic.as_i64(), other_ic.as_i64());
                    if self_i64 == other_i64 {
                        Ok(SqlCompareResult::Eq)
                    } else if self_i64 < other_i64 {
                        Ok(SqlCompareResult::LessThan)
                    } else {
                        Ok(SqlCompareResult::GreaterThan)
                    }
                }
            },
            (
                Constant::CharacterConstantVariant(self_cc),
                Constant::CharacterConstantVariant(other_cc),
            ) => match (self_cc, other_cc) {
                (
                    CharacterConstant::TextConstantVariant(self_tc),
                    CharacterConstant::TextConstantVariant(other_tc),
                ) => {
                    let (self_str, other_str) = (self_tc.as_str(), other_tc.as_str());
                    if self_str == other_str {
                        Ok(SqlCompareResult::Eq)
                    } else if self_str < other_str {
                        Ok(SqlCompareResult::LessThan)
                    } else {
                        Ok(SqlCompareResult::GreaterThan)
                    }
                }
            },
            _ => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "`self` and `other` are not in comparable type - self: {:?}, other: {:?}",
                    self, other
                ),
                None,
            )),
        }
    }

    /// Construct from Expression. DataType must be passed explicitly.
    pub fn try_from(expr: &Expression, data_type: &DataType) -> ApllodbResult<Self> {
        match expr {
            Expression::ConstantVariant(v) => match v {
                Constant::Null => {
                    if data_type.nullable() {
                        SqlValue::pack(&data_type, &None::<i16>)
                    } else {
                        Err(ApllodbError::new(
                        ApllodbErrorKind::NullValueNotAllowed,
                        format!("NULL expression is passed but requested to interpret as non-nullable type: {:?}",  data_type),
                        None
                    ))
                    }
                }
                Constant::NumericConstantVariant(nv) => match nv {
                    NumericConstant::IntegerConstantVariant(iv) =>
                        match data_type.kind() {
                            DataTypeKind::SmallInt => {
                                let i = iv.as_i64() as i16;
                                SqlValue::pack(&data_type, &i)
                            }
                            DataTypeKind::Integer => {
                                let i = iv.as_i64() as i32;
                                SqlValue::pack(&data_type, &i)
                            }
                            DataTypeKind::BigInt => {
                                let i = iv.as_i64();
                                SqlValue::pack(&data_type, &i)
                            }
                            data_type_kind  => {
                                Err(ApllodbError::new(
                                    ApllodbErrorKind::DatatypeMismatch,
                                    format!("expression `{:?}` is integer constant but requested to interpret as {:?}", v, data_type_kind),
                                    None
                                ))
                            }
                        }

                },
                Constant::CharacterConstantVariant(c) => match c {
                    CharacterConstant::TextConstantVariant(tx) => {
                        match data_type.kind() {
                            DataTypeKind::Text => {
                                SqlValue::pack(&data_type, &tx.as_str().to_string())
                            }
                            data_type_kind  => {
                                Err(ApllodbError::new(
                                    ApllodbErrorKind::DatatypeMismatch,
                                    format!("expression `{:?}` is character constant but requested to interpret as {:?}", v, data_type_kind),
                                    None
                                ))
                            }
                        }
                    }
                },
            },
            Expression::ColumnNameVariant(column_name) => Err(ApllodbError::new(
                ApllodbErrorKind::DataException,
                format!(
                    "cannot construct SqlValue from column reference: {:?}",
                    column_name
                ),
                None,
            )),
            Expression::BooleanExpressionVariant(b) => Err(ApllodbError::new(
                ApllodbErrorKind::FeatureNotSupported,
                format!(
                    "currently constructing SqlValue from BooleanExpression is not supported: {:?}",
                    b
                ),
                None,
            )),
        }
    }
}
