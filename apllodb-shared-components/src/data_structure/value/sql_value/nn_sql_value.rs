use std::{fmt::Display, hash::Hash};

use crate::{
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
};
use serde::{Deserialize, Serialize};

use super::sql_compare_result::SqlCompareResult;

use crate::data_structure::value::sql_type::{
    NumericComparableType, SqlType, StringComparableLoseType,
};

/// NOT NULL value.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NNSqlValue {
    /// SMALLINT
    SmallInt(i16),
    /// INTEGER
    Integer(i32),
    /// BIGINT
    BigInt(i64),

    /// TEXT
    Text(String),
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
        match &$nn_sql_value {
            NNSqlValue::SmallInt(_) | NNSqlValue::Integer(_) | NNSqlValue::BigInt(_) => {
                let v = $nn_sql_value.unpack::<i64>().unwrap();
                $closure_i64(v)
            }
            NNSqlValue::Text(s) => $closure_string(s.to_string()),
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

impl Display for NNSqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = for_all_loose_types!(
            self,
            |i: i64| i.to_string(),
            |s: String| format!(r#""{}""#, s)
        );
        write!(f, "{}", s)
    }
}

impl NNSqlValue {
    /// Retrieve Rust value.
    ///
    /// Allows "loosely-get", which captures a value into a looser type.
    /// E.g. unpack() `NNSqlValue::SmallInt(1)` into `i32`.
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - Any value of `T` cannot be typed as this SqlValue's SqlType (E.g. `T = i64`, `SqlType = SmallInt`).
    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        match self {
            NNSqlValue::SmallInt(i16_) => T::try_from_i16(i16_),
            NNSqlValue::Integer(i32_) => T::try_from_i32(i32_),
            NNSqlValue::BigInt(i64_) => T::try_from_i64(i64_),
            NNSqlValue::Text(string) => T::try_from_string(string),
        }
    }

    /// SqlType of this value
    pub fn sql_type(&self) -> SqlType {
        match self {
            NNSqlValue::SmallInt(_) => SqlType::small_int(),
            NNSqlValue::Integer(_) => SqlType::integer(),
            NNSqlValue::BigInt(_) => SqlType::big_int(),
            NNSqlValue::Text(_) => SqlType::text(),
        }
    }

    pub(super) fn sql_compare(&self, other: &Self) -> ApllodbResult<SqlCompareResult> {
        match (self.sql_type(), other.sql_type()) {
            (SqlType::NumericComparable(self_n), SqlType::NumericComparable(other_n)) => {
                match (self_n, other_n) {
                    (NumericComparableType::I64Loose(_), NumericComparableType::I64Loose(_)) => {
                        let (self_i64, other_i64) = (self.unpack::<i64>()?, other.unpack::<i64>()?);
                        Ok(SqlCompareResult::from(self_i64.cmp(&other_i64)))
                    }
                }
            }
            (SqlType::StringComparableLoose(self_s), SqlType::StringComparableLoose(other_s)) => {
                match (self_s, other_s) {
                    (StringComparableLoseType::Text, StringComparableLoseType::Text) => {
                        let (self_string, other_string) =
                            (self.unpack::<String>()?, other.unpack::<String>()?);
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

    /// # Failures
    ///
    /// - [InvalidParameterValue](apllodb_shared_components::ApllodbErrorKind::InvalidParameterValue) when:
    ///   - inner value cannot negate
    pub(crate) fn negate(self) -> ApllodbResult<Self> {
        match self {
            NNSqlValue::SmallInt(v) => Ok(Self::SmallInt(-v)),
            NNSqlValue::Integer(v) => Ok(Self::Integer(-v)),
            NNSqlValue::BigInt(v) => Ok(Self::BigInt(-v)),
            NNSqlValue::Text(_) => Err(ApllodbError::new(
                ApllodbErrorKind::InvalidParameterValue,
                "String cannot negate",
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApllodbErrorKind, ApllodbResult, NNSqlValue};

    #[test]
    fn test_unpack_loosely() -> ApllodbResult<()> {
        assert_eq!(NNSqlValue::SmallInt(-1).unpack::<i16>()?, -1);
        assert_eq!(NNSqlValue::SmallInt(-1).unpack::<i32>()?, -1);
        assert_eq!(NNSqlValue::SmallInt(-1).unpack::<i64>()?, -1);

        assert_eq!(
            NNSqlValue::Integer(-1).unpack::<i16>().unwrap_err().kind(),
            &ApllodbErrorKind::DatatypeMismatch
        );
        assert_eq!(NNSqlValue::Integer(-1).unpack::<i32>()?, -1);
        assert_eq!(NNSqlValue::Integer(-1).unpack::<i64>()?, -1);

        assert_eq!(
            NNSqlValue::BigInt(-1).unpack::<i16>().unwrap_err().kind(),
            &ApllodbErrorKind::DatatypeMismatch
        );
        assert_eq!(
            NNSqlValue::BigInt(-1).unpack::<i32>().unwrap_err().kind(),
            &ApllodbErrorKind::DatatypeMismatch
        );
        assert_eq!(NNSqlValue::BigInt(-1).unpack::<i64>()?, -1);

        assert_eq!(
            NNSqlValue::Text("🚔".to_string()).unpack::<String>()?,
            "🚔".to_string()
        );

        Ok(())
    }
}
