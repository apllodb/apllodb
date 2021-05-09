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
