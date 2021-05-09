//! Provides Result type and Error type commonly used in apllodb workspace.

mod from;
pub(crate) mod session_error;
pub(crate) mod sqlstate;

use sqlstate::SqlState;
use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

/// Result type commonly used in apllodb workspace.
pub type ApllodbResult<T> = Result<T, ApllodbError>;

/// Error type commonly used in apllodb workspace.
///
/// Note that `source` parameter is always serialized into `None`, so that, for example, a client cannot know what's the cause of a server's error.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApllodbError {
    /// Machine-readable error type.
    kind: SqlState,

    /// Human-readable description of each error instance.
    desc: String,

    /// Source of this error if any.
    /// `impl From<FooError> for ApllodbError` is supposed to set this as `Some(foo_err)`
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    source: Option<Box<dyn Error + Sync + Send + 'static>>,
}

impl ApllodbError {
    /// General constructor.
    ///
    /// Pass `Some(SourceError)` if you have one.
    fn new(
        kind: SqlState,
        desc: impl ToString,
        source: Option<Box<dyn Error + Sync + Send + 'static>>,
    ) -> Self {
        Self {
            kind,
            desc: desc.to_string(),
            source,
        }
    }

    /// Constructor of [SqlState::FeatureNotSupported](crate::SqlState::FeatureNotSupported).
    pub fn feature_not_supported(desc: impl ToString) -> Self {
        Self::new(SqlState::FeatureNotSupported, desc, None)
    }

    /// Constructor of [SqlState::ConnectionExceptionDatabaseNotOpen](crate::SqlState::ConnectionExceptionDatabaseNotOpen).
    pub fn connection_exception_database_not_open(desc: impl ToString) -> Self {
        Self::new(SqlState::ConnectionExceptionDatabaseNotOpen, desc, None)
    }

    /// Constructor of [SqlState::DataException](crate::SqlState::DataException).
    pub fn data_exception(desc: impl ToString) -> Self {
        Self::new(SqlState::DataException, desc, None)
    }

    /// Constructor of [SqlState::DataExceptionIllegalConversion](crate::SqlState::DataExceptionIllegalConversion).
    pub fn data_exception_illegal_conversion(desc: impl ToString) -> Self {
        Self::new(SqlState::DataExceptionIllegalConversion, desc, None)
    }

    /// Constructor of [SqlState::DataExceptionIllegalComparison](crate::SqlState::DataExceptionIllegalComparison).
    pub fn data_exception_illegal_comparison(desc: impl ToString) -> Self {
        Self::new(SqlState::DataExceptionIllegalComparison, desc, None)
    }

    /// Constructor of [SqlState::DataExceptionIllegalOperation](crate::SqlState::DataExceptionIllegalOperation).
    pub fn data_exception_illegal_operation(desc: impl ToString) -> Self {
        Self::new(SqlState::DataExceptionIllegalOperation, desc, None)
    }

    /// Constructor of [SqlState::IntegrityConstraintNotNullViolation](crate::SqlState::IntegrityConstraintNotNullViolation).
    pub fn integrity_constraint_not_null_violation(desc: impl ToString) -> Self {
        Self::new(SqlState::IntegrityConstraintNotNullViolation, desc, None)
    }

    /// Constructor of [SqlState::IntegrityConstraintUniqueViolation](crate::SqlState::IntegrityConstraintUniqueViolation).
    pub fn integrity_constraint_unique_violation(desc: impl ToString) -> Self {
        Self::new(SqlState::IntegrityConstraintUniqueViolation, desc, None)
    }

    /// Constructor of [SqlState::NameErrorNotFound](crate::SqlState::NameErrorNotFound).
    pub fn name_error_not_found(desc: impl ToString) -> Self {
        Self::new(SqlState::NameErrorNotFound, desc, None)
    }

    /// Constructor of [SqlState::NameErrorAmbiguous](crate::SqlState::NameErrorAmbiguous).
    pub fn name_error_ambiguous(desc: impl ToString) -> Self {
        Self::new(SqlState::NameErrorAmbiguous, desc, None)
    }

    /// Constructor of [SqlState::NameErrorDuplicate](crate::SqlState::NameErrorDuplicate).
    pub fn name_error_duplicate(desc: impl ToString) -> Self {
        Self::new(SqlState::NameErrorDuplicate, desc, None)
    }

    /// Constructor of [SqlState::NameErrorTooLong](crate::SqlState::NameErrorTooLong).
    pub fn name_error_too_long(desc: impl ToString) -> Self {
        Self::new(SqlState::NameErrorTooLong, desc, None)
    }

    /// Constructor of [SqlState::TransactionRollbackDeadlock](crate::SqlState::TransactionRollbackDeadlock).
    pub fn transaction_rollback_deadlock(desc: impl ToString) -> Self {
        Self::new(SqlState::TransactionRollbackDeadlock, desc, None)
    }

    /// Constructor of [SqlState::DdlError](crate::SqlState::DdlError).
    pub fn ddl_error(desc: impl ToString) -> Self {
        Self::new(SqlState::DdlError, desc, None)
    }

    /// Constructor of [SqlState::SystemError](crate::SqlState::SystemError).
    pub fn system_error(
        desc: impl ToString,
        source: Box<dyn Error + Sync + Send + 'static>,
    ) -> Self {
        Self::new(SqlState::SystemError, desc, Some(source))
    }
}

impl Error for ApllodbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // FIXME `self.source.as_ref().map(|s| s.as_ref())` produces compile error

        #[allow(clippy::manual_map)]
        match &self.source {
            Some(s) => Some(s.as_ref()),
            None => None,
        }
    }
}

impl Display for ApllodbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{description} ({sqlstate}) `{}` ; caused by: `{source}`"#,
            description = self.desc,
            sqlstate = self.kind(),
            source = self
                .source()
                .map_or_else(|| "none".to_string(), |e| format!("{}", e))
        )
    }
}

impl ApllodbError {
    /// Use this for error handling with pattern match.
    pub fn kind(&self) -> &SqlState {
        &self.kind
    }

    /// Human-readable error description.
    pub fn desc(&self) -> &str {
        &self.desc
    }
}
