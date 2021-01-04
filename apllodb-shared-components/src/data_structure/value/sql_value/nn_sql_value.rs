use std::{any::type_name, fmt::Display, hash::Hash};

use crate::{
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
};
use serde::{Deserialize, Serialize};

use super::sql_compare_result::SqlCompareResult;

use crate::data_structure::value::sql_type::{
    I64LooseType, NumericComparableType, SqlType, StringComparableLoseType,
};

/// NOT NULL value.
#[derive(Clone, Serialize, Deserialize)]
pub struct NNSqlValue {
    sql_type: SqlType,
    raw: Vec<u8>,
}

/// Although function is better to use,
///
/// ```
/// fn for_all_loose_types<R, FnNull, FnI64, FnString>(
///     &self,
///     f_i64: FnI64,
///     f_string: FnString,
/// ) -> R
/// where
///     FnI64: FnOnce(i64) -> R,
///     FnString: FnOnce(String) -> R,
/// ```
///
/// does not work properly with closures which capture &mut environments.
macro_rules! for_all_loose_types {
    ( $nn_sql_value:expr, $closure_i64:expr, $closure_string:expr ) => {{
        match &$nn_sql_value.sql_type {
            SqlType::NumericComparable(n) => match n {
                NumericComparableType::I64Loose(_) => {
                    let v = $nn_sql_value.to_i64_loosely().unwrap();
                    $closure_i64(v)
                }
            },
            SqlType::StringComparableLoose(s) => match s {
                StringComparableLoseType::Text => {
                    let v = $nn_sql_value.to_string_loosely().unwrap();
                    $closure_string(v)
                }
            },
        }
    }};
}

impl PartialEq for NNSqlValue {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.sql_compare(other), Ok(SqlCompareResult::Eq))
    }
}

impl Hash for NNSqlValue {
    /// Although raw format are different between two NNSqlValue, this hash function must return the same value if loosely typed values are the same.
    /// E.g. `42 SMALLINT`'s hash value must be equal to that of `42 INTEGER`.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for_all_loose_types!(
            self,
            |i: i64| {
                i.hash(state);
            },
            |s: String| {
                s.hash(state);
            }
        )
    }
}

impl std::fmt::Debug for NNSqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NNSqlValue({})", self)
    }
}

impl Display for NNSqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_in_s: String = for_all_loose_types!(self, |i: i64| i.to_string(), |s: String| s);
        write!(f, "{}", raw_in_s)
    }
}

impl NNSqlValue {
    /// Convert rust_value into `NNSqlValue`
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - Any value of `T` cannot be typed as `into_type`'s NNSqlType (E.g. `T = i64`, `into_type = SmallInt`).
    /// - Errors from [T::pack()](crate::SqlConvertible::pack).
    pub fn pack<T>(into_type: SqlType, rust_value: &T) -> ApllodbResult<Self>
    where
        T: SqlConvertible,
    {
        if T::to_sql_types().contains(&into_type) {
            let raw = T::pack(rust_value)?;
            Ok(Self {
                sql_type: into_type,
                raw,
            })
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "cannot convert Rust value `{:?}: {}` into SQL type `{:?}`",
                    rust_value,
                    type_name::<T>(),
                    into_type
                ),
                None,
            ))
        }
    }

    /// Retrieve Rust value
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - Any value of `T` cannot be typed as this SqlValue's SqlType (E.g. `T = i64`, `SqlType = SmallInt`).
    /// - Errors from [T::unpack()](crate::SqlConvertible::unpack).
    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        if T::from_sql_types().contains(&self.sql_type) {
            T::unpack(&self.raw)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "cannot convert data from SQL type `{:?}` into Rust type `{}`",
                    self.sql_type,
                    type_name::<Self>()
                ),
                None,
            ))
        }
    }

    /// SqlType of this value
    pub fn sql_type(&self) -> &SqlType {
        &self.sql_type
    }

    pub(super) fn sql_compare(&self, other: &Self) -> ApllodbResult<SqlCompareResult> {
        match (&self.sql_type, &other.sql_type) {
            (SqlType::NumericComparable(self_n), SqlType::NumericComparable(other_n)) => {
                match (self_n, other_n) {
                    (NumericComparableType::I64Loose(_), NumericComparableType::I64Loose(_)) => {
                        let (self_i64, other_i64) =
                            (self.to_i64_loosely()?, other.to_i64_loosely()?);
                        Ok(SqlCompareResult::from(self_i64.cmp(&other_i64)))
                    }
                }
            }
            (SqlType::StringComparableLoose(self_s), SqlType::StringComparableLoose(other_s)) => {
                match (self_s, other_s) {
                    (StringComparableLoseType::Text, StringComparableLoseType::Text) => {
                        let (self_string, other_string) =
                            (self.to_string_loosely()?, other.to_string_loosely()?);
                        Ok(SqlCompareResult::from(self_string.cmp(&other_string)))
                    }
                }
            }
            (_, _) => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "`self` and `other` are not in comparable type - self: {:?}, other: {:?}",
                    self, other
                ),
                None,
            )),
        }
    }

    fn to_i64_loosely(&self) -> ApllodbResult<i64> {
        match &self.sql_type {
            SqlType::NumericComparable(n) => match n {
                NumericComparableType::I64Loose(i) => {
                    let v: i64 = match i {
                        I64LooseType::SmallInt => self.unpack::<i16>().unwrap() as i64,
                        I64LooseType::Integer => self.unpack::<i32>().unwrap() as i64,
                        I64LooseType::BigInt => self.unpack::<i64>().unwrap() as i64,
                    };
                    Ok(v)
                }
            },
            _ => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!("`{:?}` cannot be loosely typed as i64", self.sql_type),
                None,
            )),
        }
    }

    fn to_string_loosely(&self) -> ApllodbResult<String> {
        match &self.sql_type {
            SqlType::StringComparableLoose(s) => match s {
                StringComparableLoseType::Text => {
                    let v = self.unpack::<String>().unwrap();
                    Ok(v)
                }
            },
            _ => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!("`{:?}` cannot be loosely typed as i64", self.sql_type),
                None,
            )),
        }
    }
}