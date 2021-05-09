mod boolean;
mod int;
mod text;

use crate::{
    error::{sqlstate::SqlState, ApllodbError, ApllodbResult},
    NnSqlValue,
};
use std::any::type_name;

/// Rust values which can have bidirectional mapping to/from SQL [NnSqlValue](crate::NnSqlValue).
pub trait SqlConvertible: Sized {
    /// Convert Rust type into strictly-matching SQL type.
    fn into_sql_value(self) -> NnSqlValue;

    /// # Failures
    ///
    /// - [DataExceptionIllegalConversion](crate::SqlState::DataExceptionIllegalConversion) when:
    ///   - the type implementing SqlConvertible is not convertible from i16
    fn try_from_i16(_: &i16) -> ApllodbResult<Self> {
        Self::default_err("i16")
    }

    /// # Failures
    ///
    /// - [DataExceptionIllegalConversion](crate::SqlState::DataExceptionIllegalConversion) when:
    ///   - the type implementing SqlConvertible is not convertible from i32
    fn try_from_i32(_: &i32) -> ApllodbResult<Self> {
        Self::default_err("i32")
    }

    /// # Failures
    ///
    /// - [DataExceptionIllegalConversion](crate::SqlState::DataExceptionIllegalConversion) when:
    ///   - the type implementing SqlConvertible is not convertible from i64
    fn try_from_i64(_: &i64) -> ApllodbResult<Self> {
        Self::default_err("i64")
    }

    /// # Failures
    ///
    /// - [DataExceptionIllegalConversion](crate::SqlState::DataExceptionIllegalConversion) when:
    ///   - the type implementing SqlConvertible is not convertible from String
    fn try_from_string(_: &str) -> ApllodbResult<Self> {
        Self::default_err("String")
    }

    /// # Failures
    ///
    /// - [DataExceptionIllegalConversion](crate::SqlState::DataExceptionIllegalConversion) when:
    ///   - the type implementing SqlConvertible is not convertible from bool
    fn try_from_bool(_: &bool) -> ApllodbResult<Self> {
        Self::default_err("bool")
    }

    #[doc(hidden)]
    fn default_err(from_type: &str) -> ApllodbResult<Self> {
        Err(ApllodbError::data_exception_illegal_conversion(format!(
            "cannot convert {} -> {}",
            from_type,
            type_name::<Self>()
        )))
    }
}
