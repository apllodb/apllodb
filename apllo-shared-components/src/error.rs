//! Provides Result type and Error type commonly used in apllo workspace.

mod aux;
mod dummy;
mod kind;
mod sqlstate;

use aux::AplloErrorAux;
use dummy::DummyError;
pub use kind::AplloErrorKind;
use sqlstate::SqlState;
use std::{error::Error, fmt::Display};

/// Result type commonly used in apllo workspace.
pub type AplloResult<T> = Result<T, AplloError>;

/// Error type commonly used in apllo workspace.
#[derive(Debug)]
pub struct AplloError {
    kind: AplloErrorKind,

    // FIXME: Better to wrap by Option but then I don't know how to return `Option<&(dyn Error + 'static)>`
    // in [source()](method.source.html).
    //
    // [DummyError](dummy/struct.DummyError.html) is being used instead to represent no-root-cause case.
    source: Box<dyn Error + Sync + Send + 'static>,
}

impl AplloError {
    /// Constructor.
    ///
    /// Pass `Some(SourceError)` if you have one.
    pub fn new(
        kind: AplloErrorKind,
        source: Option<Box<dyn Error + Sync + Send + 'static>>,
    ) -> Self {
        Self {
            kind,
            source: match source {
                None => Box::new(DummyError),
                Some(e) => e,
            },
        }
    }
}

impl Error for AplloError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source.downcast_ref::<DummyError>() {
            Some(_) => None,
            _ => Some(self.source.as_ref()),
        }
    }
}

impl Display for AplloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) caused by: {}",
            self.errcode(),
            self.sqlstate(),
            self.source()
                .map_or_else(|| "none".to_string(), |e| format!("{}", e))
        )
    }
}

impl AplloError {
    /// Use this for error handling with pattern match.
    pub fn kind(&self) -> &AplloErrorKind {
        &self.kind
    }

    /// [SQLSTATE](https://www.postgresql.org/docs/12/errcodes-appendix.html).
    ///
    /// Although [kind()](struct.AplloError.html#method.kind) is considered to be a better way for error handling,
    /// this might be helpful for coordinating with some libraries your client depends on.
    pub fn sqlstate(&self) -> SqlState {
        self.aux().sqlstate
    }

    fn errcode(&self) -> String {
        self.aux().errcode
    }

    fn aux(&self) -> AplloErrorAux {
        match self.kind {
            // One-liner:
            // $ egrep '^[0-4][0-9A-Z][0-9A-Z][0-9A-Z][0-9A-Z]' errcodes.txt |perl -pe 's/(?:^|_)([a-z])/\u\1/g' |awk '{print $4 " => AplloErrorAux { sqlstate: SqlState::new(___" $1 "___.into()), errcode: ___" $3 "___.into()},"}' |perl -pe 's/(^[a-z])/AplloErrorKind::\u\1/g' |perl -pe 's/___/"/g' |grep "^Apllo" |pbcopy
            AplloErrorKind::NoData => AplloErrorAux {
                sqlstate: SqlState::new("02000".into()),
                errcode: "ERRCODE_NO_DATA".into(),
            },
            AplloErrorKind::NoAdditionalDynamicResultSetsReturned => AplloErrorAux {
                sqlstate: SqlState::new("02001".into()),
                errcode: "ERRCODE_NO_ADDITIONAL_DYNAMIC_RESULT_SETS_RETURNED".into(),
            },
            AplloErrorKind::SqlStatementNotYetComplete => AplloErrorAux {
                sqlstate: SqlState::new("03000".into()),
                errcode: "ERRCODE_SQL_STATEMENT_NOT_YET_COMPLETE".into(),
            },
            AplloErrorKind::ConnectionException => AplloErrorAux {
                sqlstate: SqlState::new("08000".into()),
                errcode: "ERRCODE_CONNECTION_EXCEPTION".into(),
            },
            AplloErrorKind::ConnectionDoesNotExist => AplloErrorAux {
                sqlstate: SqlState::new("08003".into()),
                errcode: "ERRCODE_CONNECTION_DOES_NOT_EXIST".into(),
            },
            AplloErrorKind::ConnectionFailure => AplloErrorAux {
                sqlstate: SqlState::new("08006".into()),
                errcode: "ERRCODE_CONNECTION_FAILURE".into(),
            },
            AplloErrorKind::SqlclientUnableToEstablishSqlconnection => AplloErrorAux {
                sqlstate: SqlState::new("08001".into()),
                errcode: "ERRCODE_SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION".into(),
            },
            AplloErrorKind::SqlserverRejectedEstablishmentOfSqlconnection => AplloErrorAux {
                sqlstate: SqlState::new("08004".into()),
                errcode: "ERRCODE_SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION".into(),
            },
            AplloErrorKind::TransactionResolutionUnknown => AplloErrorAux {
                sqlstate: SqlState::new("08007".into()),
                errcode: "ERRCODE_TRANSACTION_RESOLUTION_UNKNOWN".into(),
            },
            AplloErrorKind::ProtocolViolation => AplloErrorAux {
                sqlstate: SqlState::new("08P01".into()),
                errcode: "ERRCODE_PROTOCOL_VIOLATION".into(),
            },
            AplloErrorKind::TriggeredActionException => AplloErrorAux {
                sqlstate: SqlState::new("09000".into()),
                errcode: "ERRCODE_TRIGGERED_ACTION_EXCEPTION".into(),
            },
            AplloErrorKind::FeatureNotSupported => AplloErrorAux {
                sqlstate: SqlState::new("0A000".into()),
                errcode: "ERRCODE_FEATURE_NOT_SUPPORTED".into(),
            },
            AplloErrorKind::InvalidTransactionInitiation => AplloErrorAux {
                sqlstate: SqlState::new("0B000".into()),
                errcode: "ERRCODE_INVALID_TRANSACTION_INITIATION".into(),
            },
            AplloErrorKind::LocatorException => AplloErrorAux {
                sqlstate: SqlState::new("0F000".into()),
                errcode: "ERRCODE_LOCATOR_EXCEPTION".into(),
            },
            AplloErrorKind::InvalidLocatorSpecification => AplloErrorAux {
                sqlstate: SqlState::new("0F001".into()),
                errcode: "ERRCODE_L_E_INVALID_SPECIFICATION".into(),
            },
            AplloErrorKind::InvalidGrantor => AplloErrorAux {
                sqlstate: SqlState::new("0L000".into()),
                errcode: "ERRCODE_INVALID_GRANTOR".into(),
            },
            AplloErrorKind::InvalidGrantOperation => AplloErrorAux {
                sqlstate: SqlState::new("0LP01".into()),
                errcode: "ERRCODE_INVALID_GRANT_OPERATION".into(),
            },
            AplloErrorKind::InvalidRoleSpecification => AplloErrorAux {
                sqlstate: SqlState::new("0P000".into()),
                errcode: "ERRCODE_INVALID_ROLE_SPECIFICATION".into(),
            },
            AplloErrorKind::DiagnosticsException => AplloErrorAux {
                sqlstate: SqlState::new("0Z000".into()),
                errcode: "ERRCODE_DIAGNOSTICS_EXCEPTION".into(),
            },
            AplloErrorKind::StackedDiagnosticsAccessedWithoutActiveHandler => AplloErrorAux {
                sqlstate: SqlState::new("0Z002".into()),
                errcode: "ERRCODE_STACKED_DIAGNOSTICS_ACCESSED_WITHOUT_ACTIVE_HANDLER".into(),
            },
            AplloErrorKind::CaseNotFound => AplloErrorAux {
                sqlstate: SqlState::new("20000".into()),
                errcode: "ERRCODE_CASE_NOT_FOUND".into(),
            },
            AplloErrorKind::CardinalityViolation => AplloErrorAux {
                sqlstate: SqlState::new("21000".into()),
                errcode: "ERRCODE_CARDINALITY_VIOLATION".into(),
            },
            AplloErrorKind::DataException => AplloErrorAux {
                sqlstate: SqlState::new("22000".into()),
                errcode: "ERRCODE_DATA_EXCEPTION".into(),
            },
            AplloErrorKind::ArraySubscriptError => AplloErrorAux {
                sqlstate: SqlState::new("2202E".into()),
                errcode: "ERRCODE_ARRAY_SUBSCRIPT_ERROR".into(),
            },
            AplloErrorKind::CharacterNotInRepertoire => AplloErrorAux {
                sqlstate: SqlState::new("22021".into()),
                errcode: "ERRCODE_CHARACTER_NOT_IN_REPERTOIRE".into(),
            },
            AplloErrorKind::DatetimeFieldOverflow => AplloErrorAux {
                sqlstate: SqlState::new("22008".into()),
                errcode: "ERRCODE_DATETIME_FIELD_OVERFLOW".into(),
            },
            AplloErrorKind::DivisionByZero => AplloErrorAux {
                sqlstate: SqlState::new("22012".into()),
                errcode: "ERRCODE_DIVISION_BY_ZERO".into(),
            },
            AplloErrorKind::ErrorInAssignment => AplloErrorAux {
                sqlstate: SqlState::new("22005".into()),
                errcode: "ERRCODE_ERROR_IN_ASSIGNMENT".into(),
            },
            AplloErrorKind::EscapeCharacterConflict => AplloErrorAux {
                sqlstate: SqlState::new("2200B".into()),
                errcode: "ERRCODE_ESCAPE_CHARACTER_CONFLICT".into(),
            },
            AplloErrorKind::IndicatorOverflow => AplloErrorAux {
                sqlstate: SqlState::new("22022".into()),
                errcode: "ERRCODE_INDICATOR_OVERFLOW".into(),
            },
            AplloErrorKind::IntervalFieldOverflow => AplloErrorAux {
                sqlstate: SqlState::new("22015".into()),
                errcode: "ERRCODE_INTERVAL_FIELD_OVERFLOW".into(),
            },
            AplloErrorKind::InvalidArgumentForLogarithm => AplloErrorAux {
                sqlstate: SqlState::new("2201E".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_LOG".into(),
            },
            AplloErrorKind::InvalidArgumentForNtileFunction => AplloErrorAux {
                sqlstate: SqlState::new("22014".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_NTILE".into(),
            },
            AplloErrorKind::InvalidArgumentForNthValueFunction => AplloErrorAux {
                sqlstate: SqlState::new("22016".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_NTH_VALUE".into(),
            },
            AplloErrorKind::InvalidArgumentForPowerFunction => AplloErrorAux {
                sqlstate: SqlState::new("2201F".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_POWER_FUNCTION".into(),
            },
            AplloErrorKind::InvalidArgumentForWidthBucketFunction => AplloErrorAux {
                sqlstate: SqlState::new("2201G".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_WIDTH_BUCKET_FUNCTION".into(),
            },
            AplloErrorKind::InvalidCharacterValueForCast => AplloErrorAux {
                sqlstate: SqlState::new("22018".into()),
                errcode: "ERRCODE_INVALID_CHARACTER_VALUE_FOR_CAST".into(),
            },
            AplloErrorKind::InvalidDatetimeFormat => AplloErrorAux {
                sqlstate: SqlState::new("22007".into()),
                errcode: "ERRCODE_INVALID_DATETIME_FORMAT".into(),
            },
            AplloErrorKind::InvalidEscapeCharacter => AplloErrorAux {
                sqlstate: SqlState::new("22019".into()),
                errcode: "ERRCODE_INVALID_ESCAPE_CHARACTER".into(),
            },
            AplloErrorKind::InvalidEscapeOctet => AplloErrorAux {
                sqlstate: SqlState::new("2200D".into()),
                errcode: "ERRCODE_INVALID_ESCAPE_OCTET".into(),
            },
            AplloErrorKind::InvalidEscapeSequence => AplloErrorAux {
                sqlstate: SqlState::new("22025".into()),
                errcode: "ERRCODE_INVALID_ESCAPE_SEQUENCE".into(),
            },
            AplloErrorKind::NonstandardUseOfEscapeCharacter => AplloErrorAux {
                sqlstate: SqlState::new("22P06".into()),
                errcode: "ERRCODE_NONSTANDARD_USE_OF_ESCAPE_CHARACTER".into(),
            },
            AplloErrorKind::InvalidIndicatorParameterValue => AplloErrorAux {
                sqlstate: SqlState::new("22010".into()),
                errcode: "ERRCODE_INVALID_INDICATOR_PARAMETER_VALUE".into(),
            },
            AplloErrorKind::InvalidParameterValue => AplloErrorAux {
                sqlstate: SqlState::new("22023".into()),
                errcode: "ERRCODE_INVALID_PARAMETER_VALUE".into(),
            },
            AplloErrorKind::InvalidPrecedingOrFollowingSize => AplloErrorAux {
                sqlstate: SqlState::new("22013".into()),
                errcode: "ERRCODE_INVALID_PRECEDING_OR_FOLLOWING_SIZE".into(),
            },
            AplloErrorKind::InvalidRegularExpression => AplloErrorAux {
                sqlstate: SqlState::new("2201B".into()),
                errcode: "ERRCODE_INVALID_REGULAR_EXPRESSION".into(),
            },
            AplloErrorKind::InvalidRowCountInLimitClause => AplloErrorAux {
                sqlstate: SqlState::new("2201W".into()),
                errcode: "ERRCODE_INVALID_ROW_COUNT_IN_LIMIT_CLAUSE".into(),
            },
            AplloErrorKind::InvalidRowCountInResultOffsetClause => AplloErrorAux {
                sqlstate: SqlState::new("2201X".into()),
                errcode: "ERRCODE_INVALID_ROW_COUNT_IN_RESULT_OFFSET_CLAUSE".into(),
            },
            AplloErrorKind::InvalidTablesampleArgument => AplloErrorAux {
                sqlstate: SqlState::new("2202H".into()),
                errcode: "ERRCODE_INVALID_TABLESAMPLE_ARGUMENT".into(),
            },
            AplloErrorKind::InvalidTablesampleRepeat => AplloErrorAux {
                sqlstate: SqlState::new("2202G".into()),
                errcode: "ERRCODE_INVALID_TABLESAMPLE_REPEAT".into(),
            },
            AplloErrorKind::InvalidTimeZoneDisplacementValue => AplloErrorAux {
                sqlstate: SqlState::new("22009".into()),
                errcode: "ERRCODE_INVALID_TIME_ZONE_DISPLACEMENT_VALUE".into(),
            },
            AplloErrorKind::InvalidUseOfEscapeCharacter => AplloErrorAux {
                sqlstate: SqlState::new("2200C".into()),
                errcode: "ERRCODE_INVALID_USE_OF_ESCAPE_CHARACTER".into(),
            },
            AplloErrorKind::MostSpecificTypeMismatch => AplloErrorAux {
                sqlstate: SqlState::new("2200G".into()),
                errcode: "ERRCODE_MOST_SPECIFIC_TYPE_MISMATCH".into(),
            },
            AplloErrorKind::NullValueNotAllowed => AplloErrorAux {
                sqlstate: SqlState::new("22004".into()),
                errcode: "ERRCODE_NULL_VALUE_NOT_ALLOWED".into(),
            },
            AplloErrorKind::NullValueNoIndicatorParameter => AplloErrorAux {
                sqlstate: SqlState::new("22002".into()),
                errcode: "ERRCODE_NULL_VALUE_NO_INDICATOR_PARAMETER".into(),
            },
            AplloErrorKind::NumericValueOutOfRange => AplloErrorAux {
                sqlstate: SqlState::new("22003".into()),
                errcode: "ERRCODE_NUMERIC_VALUE_OUT_OF_RANGE".into(),
            },
            AplloErrorKind::SequenceGeneratorLimitExceeded => AplloErrorAux {
                sqlstate: SqlState::new("2200H".into()),
                errcode: "ERRCODE_SEQUENCE_GENERATOR_LIMIT_EXCEEDED".into(),
            },
            AplloErrorKind::StringDataLengthMismatch => AplloErrorAux {
                sqlstate: SqlState::new("22026".into()),
                errcode: "ERRCODE_STRING_DATA_LENGTH_MISMATCH".into(),
            },
            AplloErrorKind::StringDataRightTruncation => AplloErrorAux {
                sqlstate: SqlState::new("22001".into()),
                errcode: "ERRCODE_STRING_DATA_RIGHT_TRUNCATION".into(),
            },
            AplloErrorKind::SubstringError => AplloErrorAux {
                sqlstate: SqlState::new("22011".into()),
                errcode: "ERRCODE_SUBSTRING_ERROR".into(),
            },
            AplloErrorKind::TrimError => AplloErrorAux {
                sqlstate: SqlState::new("22027".into()),
                errcode: "ERRCODE_TRIM_ERROR".into(),
            },
            AplloErrorKind::UnterminatedCString => AplloErrorAux {
                sqlstate: SqlState::new("22024".into()),
                errcode: "ERRCODE_UNTERMINATED_C_STRING".into(),
            },
            AplloErrorKind::ZeroLengthCharacterString => AplloErrorAux {
                sqlstate: SqlState::new("2200F".into()),
                errcode: "ERRCODE_ZERO_LENGTH_CHARACTER_STRING".into(),
            },
            AplloErrorKind::FloatingPointException => AplloErrorAux {
                sqlstate: SqlState::new("22P01".into()),
                errcode: "ERRCODE_FLOATING_POINT_EXCEPTION".into(),
            },
            AplloErrorKind::InvalidTextRepresentation => AplloErrorAux {
                sqlstate: SqlState::new("22P02".into()),
                errcode: "ERRCODE_INVALID_TEXT_REPRESENTATION".into(),
            },
            AplloErrorKind::InvalidBinaryRepresentation => AplloErrorAux {
                sqlstate: SqlState::new("22P03".into()),
                errcode: "ERRCODE_INVALID_BINARY_REPRESENTATION".into(),
            },
            AplloErrorKind::BadCopyFileFormat => AplloErrorAux {
                sqlstate: SqlState::new("22P04".into()),
                errcode: "ERRCODE_BAD_COPY_FILE_FORMAT".into(),
            },
            AplloErrorKind::UntranslatableCharacter => AplloErrorAux {
                sqlstate: SqlState::new("22P05".into()),
                errcode: "ERRCODE_UNTRANSLATABLE_CHARACTER".into(),
            },
            AplloErrorKind::NotAnXmlDocument => AplloErrorAux {
                sqlstate: SqlState::new("2200L".into()),
                errcode: "ERRCODE_NOT_AN_XML_DOCUMENT".into(),
            },
            AplloErrorKind::InvalidXmlDocument => AplloErrorAux {
                sqlstate: SqlState::new("2200M".into()),
                errcode: "ERRCODE_INVALID_XML_DOCUMENT".into(),
            },
            AplloErrorKind::InvalidXmlContent => AplloErrorAux {
                sqlstate: SqlState::new("2200N".into()),
                errcode: "ERRCODE_INVALID_XML_CONTENT".into(),
            },
            AplloErrorKind::InvalidXmlComment => AplloErrorAux {
                sqlstate: SqlState::new("2200S".into()),
                errcode: "ERRCODE_INVALID_XML_COMMENT".into(),
            },
            AplloErrorKind::InvalidXmlProcessingInstruction => AplloErrorAux {
                sqlstate: SqlState::new("2200T".into()),
                errcode: "ERRCODE_INVALID_XML_PROCESSING_INSTRUCTION".into(),
            },
            AplloErrorKind::DuplicateJsonObjectKeyValue => AplloErrorAux {
                sqlstate: SqlState::new("22030".into()),
                errcode: "ERRCODE_DUPLICATE_JSON_OBJECT_KEY_VALUE".into(),
            },
            AplloErrorKind::InvalidArgumentForJsonDatetimeFunction => AplloErrorAux {
                sqlstate: SqlState::new("22031".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_JSON_DATETIME_FUNCTION".into(),
            },
            AplloErrorKind::InvalidJsonText => AplloErrorAux {
                sqlstate: SqlState::new("22032".into()),
                errcode: "ERRCODE_INVALID_JSON_TEXT".into(),
            },
            AplloErrorKind::InvalidSqlJsonSubscript => AplloErrorAux {
                sqlstate: SqlState::new("22033".into()),
                errcode: "ERRCODE_INVALID_SQL_JSON_SUBSCRIPT".into(),
            },
            AplloErrorKind::MoreThanOneSqlJsonItem => AplloErrorAux {
                sqlstate: SqlState::new("22034".into()),
                errcode: "ERRCODE_MORE_THAN_ONE_SQL_JSON_ITEM".into(),
            },
            AplloErrorKind::NoSqlJsonItem => AplloErrorAux {
                sqlstate: SqlState::new("22035".into()),
                errcode: "ERRCODE_NO_SQL_JSON_ITEM".into(),
            },
            AplloErrorKind::NonNumericSqlJsonItem => AplloErrorAux {
                sqlstate: SqlState::new("22036".into()),
                errcode: "ERRCODE_NON_NUMERIC_SQL_JSON_ITEM".into(),
            },
            AplloErrorKind::NonUniqueKeysInAJsonObject => AplloErrorAux {
                sqlstate: SqlState::new("22037".into()),
                errcode: "ERRCODE_NON_UNIQUE_KEYS_IN_A_JSON_OBJECT".into(),
            },
            AplloErrorKind::SingletonSqlJsonItemRequired => AplloErrorAux {
                sqlstate: SqlState::new("22038".into()),
                errcode: "ERRCODE_SINGLETON_SQL_JSON_ITEM_REQUIRED".into(),
            },
            AplloErrorKind::SqlJsonArrayNotFound => AplloErrorAux {
                sqlstate: SqlState::new("22039".into()),
                errcode: "ERRCODE_SQL_JSON_ARRAY_NOT_FOUND".into(),
            },
            AplloErrorKind::SqlJsonMemberNotFound => AplloErrorAux {
                sqlstate: SqlState::new("2203A".into()),
                errcode: "ERRCODE_SQL_JSON_MEMBER_NOT_FOUND".into(),
            },
            AplloErrorKind::SqlJsonNumberNotFound => AplloErrorAux {
                sqlstate: SqlState::new("2203B".into()),
                errcode: "ERRCODE_SQL_JSON_NUMBER_NOT_FOUND".into(),
            },
            AplloErrorKind::SqlJsonObjectNotFound => AplloErrorAux {
                sqlstate: SqlState::new("2203C".into()),
                errcode: "ERRCODE_SQL_JSON_OBJECT_NOT_FOUND".into(),
            },
            AplloErrorKind::TooManyJsonArrayElements => AplloErrorAux {
                sqlstate: SqlState::new("2203D".into()),
                errcode: "ERRCODE_TOO_MANY_JSON_ARRAY_ELEMENTS".into(),
            },
            AplloErrorKind::TooManyJsonObjectMembers => AplloErrorAux {
                sqlstate: SqlState::new("2203E".into()),
                errcode: "ERRCODE_TOO_MANY_JSON_OBJECT_MEMBERS".into(),
            },
            AplloErrorKind::SqlJsonScalarRequired => AplloErrorAux {
                sqlstate: SqlState::new("2203F".into()),
                errcode: "ERRCODE_SQL_JSON_SCALAR_REQUIRED".into(),
            },
            AplloErrorKind::IntegrityConstraintViolation => AplloErrorAux {
                sqlstate: SqlState::new("23000".into()),
                errcode: "ERRCODE_INTEGRITY_CONSTRAINT_VIOLATION".into(),
            },
            AplloErrorKind::RestrictViolation => AplloErrorAux {
                sqlstate: SqlState::new("23001".into()),
                errcode: "ERRCODE_RESTRICT_VIOLATION".into(),
            },
            AplloErrorKind::NotNullViolation => AplloErrorAux {
                sqlstate: SqlState::new("23502".into()),
                errcode: "ERRCODE_NOT_NULL_VIOLATION".into(),
            },
            AplloErrorKind::ForeignKeyViolation => AplloErrorAux {
                sqlstate: SqlState::new("23503".into()),
                errcode: "ERRCODE_FOREIGN_KEY_VIOLATION".into(),
            },
            AplloErrorKind::UniqueViolation => AplloErrorAux {
                sqlstate: SqlState::new("23505".into()),
                errcode: "ERRCODE_UNIQUE_VIOLATION".into(),
            },
            AplloErrorKind::CheckViolation => AplloErrorAux {
                sqlstate: SqlState::new("23514".into()),
                errcode: "ERRCODE_CHECK_VIOLATION".into(),
            },
            AplloErrorKind::ExclusionViolation => AplloErrorAux {
                sqlstate: SqlState::new("23P01".into()),
                errcode: "ERRCODE_EXCLUSION_VIOLATION".into(),
            },
            AplloErrorKind::InvalidCursorState => AplloErrorAux {
                sqlstate: SqlState::new("24000".into()),
                errcode: "ERRCODE_INVALID_CURSOR_STATE".into(),
            },
            AplloErrorKind::InvalidTransactionState => AplloErrorAux {
                sqlstate: SqlState::new("25000".into()),
                errcode: "ERRCODE_INVALID_TRANSACTION_STATE".into(),
            },
            AplloErrorKind::ActiveSqlTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25001".into()),
                errcode: "ERRCODE_ACTIVE_SQL_TRANSACTION".into(),
            },
            AplloErrorKind::BranchTransactionAlreadyActive => AplloErrorAux {
                sqlstate: SqlState::new("25002".into()),
                errcode: "ERRCODE_BRANCH_TRANSACTION_ALREADY_ACTIVE".into(),
            },
            AplloErrorKind::HeldCursorRequiresSameIsolationLevel => AplloErrorAux {
                sqlstate: SqlState::new("25008".into()),
                errcode: "ERRCODE_HELD_CURSOR_REQUIRES_SAME_ISOLATION_LEVEL".into(),
            },
            AplloErrorKind::InappropriateAccessModeForBranchTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25003".into()),
                errcode: "ERRCODE_INAPPROPRIATE_ACCESS_MODE_FOR_BRANCH_TRANSACTION".into(),
            },
            AplloErrorKind::InappropriateIsolationLevelForBranchTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25004".into()),
                errcode: "ERRCODE_INAPPROPRIATE_ISOLATION_LEVEL_FOR_BRANCH_TRANSACTION".into(),
            },
            AplloErrorKind::NoActiveSqlTransactionForBranchTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25005".into()),
                errcode: "ERRCODE_NO_ACTIVE_SQL_TRANSACTION_FOR_BRANCH_TRANSACTION".into(),
            },
            AplloErrorKind::ReadOnlySqlTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25006".into()),
                errcode: "ERRCODE_READ_ONLY_SQL_TRANSACTION".into(),
            },
            AplloErrorKind::SchemaAndDataStatementMixingNotSupported => AplloErrorAux {
                sqlstate: SqlState::new("25007".into()),
                errcode: "ERRCODE_SCHEMA_AND_DATA_STATEMENT_MIXING_NOT_SUPPORTED".into(),
            },
            AplloErrorKind::NoActiveSqlTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25P01".into()),
                errcode: "ERRCODE_NO_ACTIVE_SQL_TRANSACTION".into(),
            },
            AplloErrorKind::InFailedSqlTransaction => AplloErrorAux {
                sqlstate: SqlState::new("25P02".into()),
                errcode: "ERRCODE_IN_FAILED_SQL_TRANSACTION".into(),
            },
            AplloErrorKind::IdleInTransactionSessionTimeout => AplloErrorAux {
                sqlstate: SqlState::new("25P03".into()),
                errcode: "ERRCODE_IDLE_IN_TRANSACTION_SESSION_TIMEOUT".into(),
            },
            AplloErrorKind::InvalidSqlStatementName => AplloErrorAux {
                sqlstate: SqlState::new("26000".into()),
                errcode: "ERRCODE_INVALID_SQL_STATEMENT_NAME".into(),
            },
            AplloErrorKind::TriggeredDataChangeViolation => AplloErrorAux {
                sqlstate: SqlState::new("27000".into()),
                errcode: "ERRCODE_TRIGGERED_DATA_CHANGE_VIOLATION".into(),
            },
            AplloErrorKind::InvalidAuthorizationSpecification => AplloErrorAux {
                sqlstate: SqlState::new("28000".into()),
                errcode: "ERRCODE_INVALID_AUTHORIZATION_SPECIFICATION".into(),
            },
            AplloErrorKind::InvalidPassword => AplloErrorAux {
                sqlstate: SqlState::new("28P01".into()),
                errcode: "ERRCODE_INVALID_PASSWORD".into(),
            },
            AplloErrorKind::DependentPrivilegeDescriptorsStillExist => AplloErrorAux {
                sqlstate: SqlState::new("2B000".into()),
                errcode: "ERRCODE_DEPENDENT_PRIVILEGE_DESCRIPTORS_STILL_EXIST".into(),
            },
            AplloErrorKind::DependentObjectsStillExist => AplloErrorAux {
                sqlstate: SqlState::new("2BP01".into()),
                errcode: "ERRCODE_DEPENDENT_OBJECTS_STILL_EXIST".into(),
            },
            AplloErrorKind::InvalidTransactionTermination => AplloErrorAux {
                sqlstate: SqlState::new("2D000".into()),
                errcode: "ERRCODE_INVALID_TRANSACTION_TERMINATION".into(),
            },
            AplloErrorKind::SqlRoutineException => AplloErrorAux {
                sqlstate: SqlState::new("2F000".into()),
                errcode: "ERRCODE_SQL_ROUTINE_EXCEPTION".into(),
            },
            AplloErrorKind::SqlFunctionExecutedNoReturnStatement => AplloErrorAux {
                sqlstate: SqlState::new("2F005".into()),
                errcode: "ERRCODE_S_R_E_FUNCTION_EXECUTED_NO_RETURN_STATEMENT".into(),
            },
            AplloErrorKind::SqlModifyingSqlDataNotPermitted => AplloErrorAux {
                sqlstate: SqlState::new("2F002".into()),
                errcode: "ERRCODE_S_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED".into(),
            },
            AplloErrorKind::SqlProhibitedSqlStatementAttempted => AplloErrorAux {
                sqlstate: SqlState::new("2F003".into()),
                errcode: "ERRCODE_S_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED".into(),
            },
            AplloErrorKind::SqlReadingSqlDataNotPermitted => AplloErrorAux {
                sqlstate: SqlState::new("2F004".into()),
                errcode: "ERRCODE_S_R_E_READING_SQL_DATA_NOT_PERMITTED".into(),
            },
            AplloErrorKind::InvalidCursorName => AplloErrorAux {
                sqlstate: SqlState::new("34000".into()),
                errcode: "ERRCODE_INVALID_CURSOR_NAME".into(),
            },
            AplloErrorKind::ExternalRoutineException => AplloErrorAux {
                sqlstate: SqlState::new("38000".into()),
                errcode: "ERRCODE_EXTERNAL_ROUTINE_EXCEPTION".into(),
            },
            AplloErrorKind::ExternalContainingSqlNotPermitted => AplloErrorAux {
                sqlstate: SqlState::new("38001".into()),
                errcode: "ERRCODE_E_R_E_CONTAINING_SQL_NOT_PERMITTED".into(),
            },
            AplloErrorKind::ExternalModifyingSqlDataNotPermitted => AplloErrorAux {
                sqlstate: SqlState::new("38002".into()),
                errcode: "ERRCODE_E_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED".into(),
            },
            AplloErrorKind::ExternalProhibitedSqlStatementAttempted => AplloErrorAux {
                sqlstate: SqlState::new("38003".into()),
                errcode: "ERRCODE_E_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED".into(),
            },
            AplloErrorKind::ExternalReadingSqlDataNotPermitted => AplloErrorAux {
                sqlstate: SqlState::new("38004".into()),
                errcode: "ERRCODE_E_R_E_READING_SQL_DATA_NOT_PERMITTED".into(),
            },
            AplloErrorKind::ExternalRoutineInvocationException => AplloErrorAux {
                sqlstate: SqlState::new("39000".into()),
                errcode: "ERRCODE_EXTERNAL_ROUTINE_INVOCATION_EXCEPTION".into(),
            },
            AplloErrorKind::ExternalInvalidSqlstateReturned => AplloErrorAux {
                sqlstate: SqlState::new("39001".into()),
                errcode: "ERRCODE_E_R_I_E_INVALID_SQLSTATE_RETURNED".into(),
            },
            AplloErrorKind::ExternalNullValueNotAllowed => AplloErrorAux {
                sqlstate: SqlState::new("39004".into()),
                errcode: "ERRCODE_E_R_I_E_NULL_VALUE_NOT_ALLOWED".into(),
            },
            AplloErrorKind::ExternalTriggerProtocolViolated => AplloErrorAux {
                sqlstate: SqlState::new("39P01".into()),
                errcode: "ERRCODE_E_R_I_E_TRIGGER_PROTOCOL_VIOLATED".into(),
            },
            AplloErrorKind::ExternalSrfProtocolViolated => AplloErrorAux {
                sqlstate: SqlState::new("39P02".into()),
                errcode: "ERRCODE_E_R_I_E_SRF_PROTOCOL_VIOLATED".into(),
            },
            AplloErrorKind::ExternalEventTriggerProtocolViolated => AplloErrorAux {
                sqlstate: SqlState::new("39P03".into()),
                errcode: "ERRCODE_E_R_I_E_EVENT_TRIGGER_PROTOCOL_VIOLATED".into(),
            },
            AplloErrorKind::SavepointException => AplloErrorAux {
                sqlstate: SqlState::new("3B000".into()),
                errcode: "ERRCODE_SAVEPOINT_EXCEPTION".into(),
            },
            AplloErrorKind::InvalidSavepointSpecification => AplloErrorAux {
                sqlstate: SqlState::new("3B001".into()),
                errcode: "ERRCODE_S_E_INVALID_SPECIFICATION".into(),
            },
            AplloErrorKind::InvalidCatalogName => AplloErrorAux {
                sqlstate: SqlState::new("3D000".into()),
                errcode: "ERRCODE_INVALID_CATALOG_NAME".into(),
            },
            AplloErrorKind::InvalidSchemaName => AplloErrorAux {
                sqlstate: SqlState::new("3F000".into()),
                errcode: "ERRCODE_INVALID_SCHEMA_NAME".into(),
            },
            AplloErrorKind::TransactionRollback => AplloErrorAux {
                sqlstate: SqlState::new("40000".into()),
                errcode: "ERRCODE_TRANSACTION_ROLLBACK".into(),
            },
            AplloErrorKind::TransactionIntegrityConstraintViolation => AplloErrorAux {
                sqlstate: SqlState::new("40002".into()),
                errcode: "ERRCODE_T_R_INTEGRITY_CONSTRAINT_VIOLATION".into(),
            },
            AplloErrorKind::SerializationFailure => AplloErrorAux {
                sqlstate: SqlState::new("40001".into()),
                errcode: "ERRCODE_T_R_SERIALIZATION_FAILURE".into(),
            },
            AplloErrorKind::StatementCompletionUnknown => AplloErrorAux {
                sqlstate: SqlState::new("40003".into()),
                errcode: "ERRCODE_T_R_STATEMENT_COMPLETION_UNKNOWN".into(),
            },
            AplloErrorKind::DeadlockDetected => AplloErrorAux {
                sqlstate: SqlState::new("40P01".into()),
                errcode: "ERRCODE_T_R_DEADLOCK_DETECTED".into(),
            },
            AplloErrorKind::SyntaxErrorOrAccessRuleViolation => AplloErrorAux {
                sqlstate: SqlState::new("42000".into()),
                errcode: "ERRCODE_SYNTAX_ERROR_OR_ACCESS_RULE_VIOLATION".into(),
            },
            AplloErrorKind::SyntaxError => AplloErrorAux {
                sqlstate: SqlState::new("42601".into()),
                errcode: "ERRCODE_SYNTAX_ERROR".into(),
            },
            AplloErrorKind::InsufficientPrivilege => AplloErrorAux {
                sqlstate: SqlState::new("42501".into()),
                errcode: "ERRCODE_INSUFFICIENT_PRIVILEGE".into(),
            },
            AplloErrorKind::CannotCoerce => AplloErrorAux {
                sqlstate: SqlState::new("42846".into()),
                errcode: "ERRCODE_CANNOT_COERCE".into(),
            },
            AplloErrorKind::GroupingError => AplloErrorAux {
                sqlstate: SqlState::new("42803".into()),
                errcode: "ERRCODE_GROUPING_ERROR".into(),
            },
            AplloErrorKind::WindowingError => AplloErrorAux {
                sqlstate: SqlState::new("42P20".into()),
                errcode: "ERRCODE_WINDOWING_ERROR".into(),
            },
            AplloErrorKind::InvalidRecursion => AplloErrorAux {
                sqlstate: SqlState::new("42P19".into()),
                errcode: "ERRCODE_INVALID_RECURSION".into(),
            },
            AplloErrorKind::InvalidForeignKey => AplloErrorAux {
                sqlstate: SqlState::new("42830".into()),
                errcode: "ERRCODE_INVALID_FOREIGN_KEY".into(),
            },
            AplloErrorKind::InvalidName => AplloErrorAux {
                sqlstate: SqlState::new("42602".into()),
                errcode: "ERRCODE_INVALID_NAME".into(),
            },
            AplloErrorKind::NameTooLong => AplloErrorAux {
                sqlstate: SqlState::new("42622".into()),
                errcode: "ERRCODE_NAME_TOO_LONG".into(),
            },
            AplloErrorKind::ReservedName => AplloErrorAux {
                sqlstate: SqlState::new("42939".into()),
                errcode: "ERRCODE_RESERVED_NAME".into(),
            },
            AplloErrorKind::DatatypeMismatch => AplloErrorAux {
                sqlstate: SqlState::new("42804".into()),
                errcode: "ERRCODE_DATATYPE_MISMATCH".into(),
            },
            AplloErrorKind::IndeterminateDatatype => AplloErrorAux {
                sqlstate: SqlState::new("42P18".into()),
                errcode: "ERRCODE_INDETERMINATE_DATATYPE".into(),
            },
            AplloErrorKind::CollationMismatch => AplloErrorAux {
                sqlstate: SqlState::new("42P21".into()),
                errcode: "ERRCODE_COLLATION_MISMATCH".into(),
            },
            AplloErrorKind::IndeterminateCollation => AplloErrorAux {
                sqlstate: SqlState::new("42P22".into()),
                errcode: "ERRCODE_INDETERMINATE_COLLATION".into(),
            },
            AplloErrorKind::WrongObjectType => AplloErrorAux {
                sqlstate: SqlState::new("42809".into()),
                errcode: "ERRCODE_WRONG_OBJECT_TYPE".into(),
            },
            AplloErrorKind::GeneratedAlways => AplloErrorAux {
                sqlstate: SqlState::new("428C9".into()),
                errcode: "ERRCODE_GENERATED_ALWAYS".into(),
            },
            AplloErrorKind::UndefinedColumn => AplloErrorAux {
                sqlstate: SqlState::new("42703".into()),
                errcode: "ERRCODE_UNDEFINED_COLUMN".into(),
            },
            AplloErrorKind::UndefinedFunction => AplloErrorAux {
                sqlstate: SqlState::new("42883".into()),
                errcode: "ERRCODE_UNDEFINED_FUNCTION".into(),
            },
            AplloErrorKind::UndefinedTable => AplloErrorAux {
                sqlstate: SqlState::new("42P01".into()),
                errcode: "ERRCODE_UNDEFINED_TABLE".into(),
            },
            AplloErrorKind::UndefinedParameter => AplloErrorAux {
                sqlstate: SqlState::new("42P02".into()),
                errcode: "ERRCODE_UNDEFINED_PARAMETER".into(),
            },
            AplloErrorKind::UndefinedObject => AplloErrorAux {
                sqlstate: SqlState::new("42704".into()),
                errcode: "ERRCODE_UNDEFINED_OBJECT".into(),
            },
            AplloErrorKind::DuplicateColumn => AplloErrorAux {
                sqlstate: SqlState::new("42701".into()),
                errcode: "ERRCODE_DUPLICATE_COLUMN".into(),
            },
            AplloErrorKind::DuplicateCursor => AplloErrorAux {
                sqlstate: SqlState::new("42P03".into()),
                errcode: "ERRCODE_DUPLICATE_CURSOR".into(),
            },
            AplloErrorKind::DuplicateDatabase => AplloErrorAux {
                sqlstate: SqlState::new("42P04".into()),
                errcode: "ERRCODE_DUPLICATE_DATABASE".into(),
            },
            AplloErrorKind::DuplicateFunction => AplloErrorAux {
                sqlstate: SqlState::new("42723".into()),
                errcode: "ERRCODE_DUPLICATE_FUNCTION".into(),
            },
            AplloErrorKind::DuplicatePreparedStatement => AplloErrorAux {
                sqlstate: SqlState::new("42P05".into()),
                errcode: "ERRCODE_DUPLICATE_PSTATEMENT".into(),
            },
            AplloErrorKind::DuplicateSchema => AplloErrorAux {
                sqlstate: SqlState::new("42P06".into()),
                errcode: "ERRCODE_DUPLICATE_SCHEMA".into(),
            },
            AplloErrorKind::DuplicateTable => AplloErrorAux {
                sqlstate: SqlState::new("42P07".into()),
                errcode: "ERRCODE_DUPLICATE_TABLE".into(),
            },
            AplloErrorKind::DuplicateAlias => AplloErrorAux {
                sqlstate: SqlState::new("42712".into()),
                errcode: "ERRCODE_DUPLICATE_ALIAS".into(),
            },
            AplloErrorKind::DuplicateObject => AplloErrorAux {
                sqlstate: SqlState::new("42710".into()),
                errcode: "ERRCODE_DUPLICATE_OBJECT".into(),
            },
            AplloErrorKind::AmbiguousColumn => AplloErrorAux {
                sqlstate: SqlState::new("42702".into()),
                errcode: "ERRCODE_AMBIGUOUS_COLUMN".into(),
            },
            AplloErrorKind::AmbiguousFunction => AplloErrorAux {
                sqlstate: SqlState::new("42725".into()),
                errcode: "ERRCODE_AMBIGUOUS_FUNCTION".into(),
            },
            AplloErrorKind::AmbiguousParameter => AplloErrorAux {
                sqlstate: SqlState::new("42P08".into()),
                errcode: "ERRCODE_AMBIGUOUS_PARAMETER".into(),
            },
            AplloErrorKind::AmbiguousAlias => AplloErrorAux {
                sqlstate: SqlState::new("42P09".into()),
                errcode: "ERRCODE_AMBIGUOUS_ALIAS".into(),
            },
            AplloErrorKind::InvalidColumnReference => AplloErrorAux {
                sqlstate: SqlState::new("42P10".into()),
                errcode: "ERRCODE_INVALID_COLUMN_REFERENCE".into(),
            },
            AplloErrorKind::InvalidColumnDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42611".into()),
                errcode: "ERRCODE_INVALID_COLUMN_DEFINITION".into(),
            },
            AplloErrorKind::InvalidCursorDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P11".into()),
                errcode: "ERRCODE_INVALID_CURSOR_DEFINITION".into(),
            },
            AplloErrorKind::InvalidDatabaseDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P12".into()),
                errcode: "ERRCODE_INVALID_DATABASE_DEFINITION".into(),
            },
            AplloErrorKind::InvalidFunctionDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P13".into()),
                errcode: "ERRCODE_INVALID_FUNCTION_DEFINITION".into(),
            },
            AplloErrorKind::InvalidPreparedStatementDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P14".into()),
                errcode: "ERRCODE_INVALID_PSTATEMENT_DEFINITION".into(),
            },
            AplloErrorKind::InvalidSchemaDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P15".into()),
                errcode: "ERRCODE_INVALID_SCHEMA_DEFINITION".into(),
            },
            AplloErrorKind::InvalidTableDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P16".into()),
                errcode: "ERRCODE_INVALID_TABLE_DEFINITION".into(),
            },
            AplloErrorKind::InvalidObjectDefinition => AplloErrorAux {
                sqlstate: SqlState::new("42P17".into()),
                errcode: "ERRCODE_INVALID_OBJECT_DEFINITION".into(),
            },
            AplloErrorKind::WithCheckOptionViolation => AplloErrorAux {
                sqlstate: SqlState::new("44000".into()),
                errcode: "ERRCODE_WITH_CHECK_OPTION_VIOLATION".into(),
            },
        }
    }
}
