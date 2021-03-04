pub(crate) mod nn_sql_value;
pub(crate) mod sql_compare_result;
pub(crate) mod sql_value_hash_key;

use std::{fmt::Display, hash::Hash};

use crate::error::ApllodbResult;
use crate::{ApllodbError, ApllodbErrorKind};
use serde::{Deserialize, Serialize};

use self::{nn_sql_value::NNSqlValue, sql_compare_result::SqlCompareResult};

/// SQL-typed value that is efficiently compressed.
///
/// # Hiding Rust type inside
///
/// It is important feature for SqlValue not to take any type parameter (although some associated methods do).
/// If SqlValue takes any type parameter, collection types holding SqlType have to use impl/dyn trait.
///
/// # Comparing SqlValues
///
/// An SqlValue implements is NULL or NOT NULL.
/// NOT NULL value has its SQL type in [SqlType](crate::SqlType).
/// SqlType forms hierarchical structure and if its comparable top-level variant (e.g. [SqlType::NumericComparable](crate::SqlType::NumericComparable)) are the same among two values,
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
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use apllodb_shared_components::{ApllodbResult, NNSqlValue, SqlType, SqlValue, SqlValueHashKey};
///
/// fn main() -> ApllodbResult<()> {
///     let v_integer = SqlValue::NotNull(NNSqlValue::Integer(42));
///     let v_smallint = SqlValue::NotNull(NNSqlValue::SmallInt(42));
///     let v_bigint = SqlValue::NotNull(NNSqlValue::BigInt(42));
///     let v_null = SqlValue::Null;
///
///     assert_eq!(v_integer, v_integer);
///     assert_eq!(v_smallint, v_bigint, "Comparing SmallInt with BigInt is valid");
///     assert_ne!(v_null, v_null, "NULL != NULL");
///
///     let mut hash_set = HashSet::<SqlValueHashKey>::new();
///     assert_eq!(hash_set.insert(SqlValueHashKey::from(&v_integer)), true);
///     assert_eq!(hash_set.insert(SqlValueHashKey::from(&v_integer)), false, "same value is already inserted");
///     assert_eq!(hash_set.insert(SqlValueHashKey::from(&v_smallint)), false, "same hash values are generated from both SmallInt and Integer");
///     assert_eq!(hash_set.insert(SqlValueHashKey::from(&v_null)), true);
///     assert_eq!(hash_set.insert(SqlValueHashKey::from(&v_null)), true, "two NULL values are different");
///
///     assert_ne!(SqlValueHashKey::from(&v_null), SqlValueHashKey::from(&v_null), "two NULL values generates different Hash value");
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SqlValue {
    /// NULL value.
    Null,
    /// NOT NULL value.
    NotNull(NNSqlValue),
}

impl PartialEq for SqlValue {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.sql_compare(other), Ok(SqlCompareResult::Eq))
    }
}

impl Hash for SqlValue {
    /// Generates different hash value for each NULL value to avoid collision in hash table.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SqlValue::Null => {
                let v = fastrand::u64(..);
                v.hash(state);
            }
            SqlValue::NotNull(nn_sql_value) => nn_sql_value.hash(state),
        }
    }
}

impl Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SqlValue::Null => "NULL".to_string(),
            SqlValue::NotNull(nn) => nn.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl SqlValue {
    /// Compares two SqlValues.
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - `self` and `other` have different top-level variant of [SqlType](crate::SqlType).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult, NNSqlValue, SqlCompareResult, SqlType, SqlValue};
    ///
    /// fn main() -> ApllodbResult<()> {
    ///     let v_integer = SqlValue::NotNull(NNSqlValue::Integer(42));
    ///     let v_smallint = SqlValue::NotNull(NNSqlValue::SmallInt(42));
    ///     let v_bigint = SqlValue::NotNull(NNSqlValue::BigInt(42));
    ///     let v_integer_minus = SqlValue::NotNull(NNSqlValue::Integer(-42));
    ///     let v_text = SqlValue::NotNull(NNSqlValue::Text("abc".to_string()));
    ///     let v_null = SqlValue::Null;
    ///
    ///     matches!(v_integer.sql_compare(&v_integer)?, SqlCompareResult::Eq);
    ///     matches!(v_smallint.sql_compare(&v_bigint)?, SqlCompareResult::Eq);
    ///     matches!(v_integer.sql_compare(&v_integer_minus)?, SqlCompareResult::GreaterThan);
    ///     matches!(v_integer_minus.sql_compare(&v_integer)?, SqlCompareResult::LessThan);
    ///     matches!(v_null.sql_compare(&v_integer)?, SqlCompareResult::Null);
    ///     matches!(v_integer.sql_compare(&v_null)?, SqlCompareResult::Null);
    ///     matches!(v_null.sql_compare(&v_null)?, SqlCompareResult::Null);
    ///
    ///     matches!(
    ///         v_integer.sql_compare(&v_text)
    ///             .expect_err("comparing totally different types").kind(),
    ///         ApllodbErrorKind::DatatypeMismatch
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn sql_compare(&self, other: &Self) -> ApllodbResult<SqlCompareResult> {
        match (self, other) {
            (SqlValue::Null, _) | (_, SqlValue::Null) => Ok(SqlCompareResult::Null),
            (SqlValue::NotNull(nn_self), SqlValue::NotNull(nn_other)) => {
                nn_self.sql_compare(nn_other)
            }
        }
    }

    /// Eval as bool if possible.
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - this SqlValue cannot be evaluated as SQL BOOLEAN
    pub fn to_bool(&self) -> ApllodbResult<bool> {
        match self {
            SqlValue::Null => Ok(false), // NULL is always evaluated as FALSE
            SqlValue::NotNull(nn_sql_value) => match nn_sql_value {
                NNSqlValue::Boolean(b) => Ok(b.clone()),
                _ => Err(ApllodbError::new(
                    ApllodbErrorKind::DatatypeMismatch,
                    format!(
                        "{:?} cannot be evaluated as BOOLEAN",
                        nn_sql_value.sql_type()
                    ),
                    None,
                )),
            },
        }
    }
}
