//! Provides Result type and Error type commonly used in apllodb workspace.

mod aux;
mod dummy;
mod from;
mod kind;
mod sqlstate;

use aux::ApllodbErrorAux;
use dummy::DummyError;
pub use kind::ApllodbErrorKind;
use sqlstate::SqlState;
use std::{error::Error, fmt::Display};

/// Result type commonly used in apllodb workspace.
pub type ApllodbResult<T> = Result<T, ApllodbError>;

/// Error type commonly used in apllodb workspace.
#[derive(Debug)]
pub struct ApllodbError {
    kind: ApllodbErrorKind,

    /// Human-readable description of each error instance.
    desc: String,

    /// Source of this error if any.
    ///
    /// FIXME: Better to wrap by Option but then I don't know how to return `Option<&(dyn Error + 'static)>`
    /// in [source()](method.source.html).
    ///
    /// [DummyError](dummy/struct.DummyError.html) is being used instead to represent no-root-cause case.
    source: Box<dyn Error + Sync + Send + 'static>,
}

impl ApllodbError {
    /// Constructor.
    ///
    /// Pass `Some(SourceError)` if you have one.
    pub fn new<S: Into<String>>(
        kind: ApllodbErrorKind,
        desc: S,
        source: Option<Box<dyn Error + Sync + Send + 'static>>,
    ) -> Self {
        Self {
            kind,
            desc: desc.into(),
            source: match source {
                None => Box::new(DummyError),
                Some(e) => e,
            },
        }
    }
}

impl Error for ApllodbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source.downcast_ref::<DummyError>() {
            Some(_) => None,
            _ => Some(self.source.as_ref()),
        }
    }
}

impl Display for ApllodbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) `{}` ; caused by: `{}`",
            self.errcode(),
            self.sqlstate(),
            self.desc,
            self.source()
                .map_or_else(|| "none".to_string(), |e| format!("{}", e))
        )
    }
}

impl ApllodbError {
    /// Use this for error handling with pattern match.
    pub fn kind(&self) -> &ApllodbErrorKind {
        &self.kind
    }

    /// [SQLSTATE](https://www.postgresql.org/docs/12/errcodes-appendix.html).
    ///
    /// Although [kind()](struct.ApllodbError.html#method.kind) is considered to be a better way for error handling,
    /// this might be helpful for coordinating with some libraries your client depends on.
    pub fn sqlstate(&self) -> SqlState {
        self.aux().sqlstate
    }

    fn errcode(&self) -> String {
        self.aux().errcode
    }

    fn aux(&self) -> ApllodbErrorAux {
        match self.kind {
            // One-liner:
            // $ egrep '^[0-4][0-9A-Z][0-9A-Z][0-9A-Z][0-9A-Z]' errcodes.txt |perl -pe 's/(?:^|_)([a-z])/\u\1/g' |awk '{print $4 " => ApllodbErrorAux { sqlstate: SqlState::new(___" $1 "___.into()), errcode: ___" $3 "___.into()},"}' |perl -pe 's/(^[a-z])/ApllodbErrorKind::\u\1/g' |perl -pe 's/___/"/g' |grep "^Apllodb" |pbcopy
            ApllodbErrorKind::NoData => ApllodbErrorAux {
                sqlstate: SqlState::new("02000".into()),
                errcode: "ERRCODE_NO_DATA".into(),
            },
            ApllodbErrorKind::NoAdditionalDynamicResultSetsReturned => ApllodbErrorAux {
                sqlstate: SqlState::new("02001".into()),
                errcode: "ERRCODE_NO_ADDITIONAL_DYNAMIC_RESULT_SETS_RETURNED".into(),
            },
            ApllodbErrorKind::SqlStatementNotYetComplete => ApllodbErrorAux {
                sqlstate: SqlState::new("03000".into()),
                errcode: "ERRCODE_SQL_STATEMENT_NOT_YET_COMPLETE".into(),
            },
            ApllodbErrorKind::ConnectionException => ApllodbErrorAux {
                sqlstate: SqlState::new("08000".into()),
                errcode: "ERRCODE_CONNECTION_EXCEPTION".into(),
            },
            ApllodbErrorKind::ConnectionDoesNotExist => ApllodbErrorAux {
                sqlstate: SqlState::new("08003".into()),
                errcode: "ERRCODE_CONNECTION_DOES_NOT_EXIST".into(),
            },
            ApllodbErrorKind::ConnectionFailure => ApllodbErrorAux {
                sqlstate: SqlState::new("08006".into()),
                errcode: "ERRCODE_CONNECTION_FAILURE".into(),
            },
            ApllodbErrorKind::SqlclientUnableToEstablishSqlconnection => ApllodbErrorAux {
                sqlstate: SqlState::new("08001".into()),
                errcode: "ERRCODE_SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION".into(),
            },
            ApllodbErrorKind::SqlserverRejectedEstablishmentOfSqlconnection => ApllodbErrorAux {
                sqlstate: SqlState::new("08004".into()),
                errcode: "ERRCODE_SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION".into(),
            },
            ApllodbErrorKind::TransactionResolutionUnknown => ApllodbErrorAux {
                sqlstate: SqlState::new("08007".into()),
                errcode: "ERRCODE_TRANSACTION_RESOLUTION_UNKNOWN".into(),
            },
            ApllodbErrorKind::ProtocolViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("08P01".into()),
                errcode: "ERRCODE_PROTOCOL_VIOLATION".into(),
            },
            ApllodbErrorKind::TriggeredActionException => ApllodbErrorAux {
                sqlstate: SqlState::new("09000".into()),
                errcode: "ERRCODE_TRIGGERED_ACTION_EXCEPTION".into(),
            },
            ApllodbErrorKind::FeatureNotSupported => ApllodbErrorAux {
                sqlstate: SqlState::new("0A000".into()),
                errcode: "ERRCODE_FEATURE_NOT_SUPPORTED".into(),
            },
            ApllodbErrorKind::InvalidTransactionInitiation => ApllodbErrorAux {
                sqlstate: SqlState::new("0B000".into()),
                errcode: "ERRCODE_INVALID_TRANSACTION_INITIATION".into(),
            },
            ApllodbErrorKind::LocatorException => ApllodbErrorAux {
                sqlstate: SqlState::new("0F000".into()),
                errcode: "ERRCODE_LOCATOR_EXCEPTION".into(),
            },
            ApllodbErrorKind::InvalidLocatorSpecification => ApllodbErrorAux {
                sqlstate: SqlState::new("0F001".into()),
                errcode: "ERRCODE_L_E_INVALID_SPECIFICATION".into(),
            },
            ApllodbErrorKind::InvalidGrantor => ApllodbErrorAux {
                sqlstate: SqlState::new("0L000".into()),
                errcode: "ERRCODE_INVALID_GRANTOR".into(),
            },
            ApllodbErrorKind::InvalidGrantOperation => ApllodbErrorAux {
                sqlstate: SqlState::new("0LP01".into()),
                errcode: "ERRCODE_INVALID_GRANT_OPERATION".into(),
            },
            ApllodbErrorKind::InvalidRoleSpecification => ApllodbErrorAux {
                sqlstate: SqlState::new("0P000".into()),
                errcode: "ERRCODE_INVALID_ROLE_SPECIFICATION".into(),
            },
            ApllodbErrorKind::DiagnosticsException => ApllodbErrorAux {
                sqlstate: SqlState::new("0Z000".into()),
                errcode: "ERRCODE_DIAGNOSTICS_EXCEPTION".into(),
            },
            ApllodbErrorKind::StackedDiagnosticsAccessedWithoutActiveHandler => ApllodbErrorAux {
                sqlstate: SqlState::new("0Z002".into()),
                errcode: "ERRCODE_STACKED_DIAGNOSTICS_ACCESSED_WITHOUT_ACTIVE_HANDLER".into(),
            },
            ApllodbErrorKind::CaseNotFound => ApllodbErrorAux {
                sqlstate: SqlState::new("20000".into()),
                errcode: "ERRCODE_CASE_NOT_FOUND".into(),
            },
            ApllodbErrorKind::CardinalityViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("21000".into()),
                errcode: "ERRCODE_CARDINALITY_VIOLATION".into(),
            },
            ApllodbErrorKind::DataException => ApllodbErrorAux {
                sqlstate: SqlState::new("22000".into()),
                errcode: "ERRCODE_DATA_EXCEPTION".into(),
            },
            ApllodbErrorKind::ArraySubscriptError => ApllodbErrorAux {
                sqlstate: SqlState::new("2202E".into()),
                errcode: "ERRCODE_ARRAY_SUBSCRIPT_ERROR".into(),
            },
            ApllodbErrorKind::CharacterNotInRepertoire => ApllodbErrorAux {
                sqlstate: SqlState::new("22021".into()),
                errcode: "ERRCODE_CHARACTER_NOT_IN_REPERTOIRE".into(),
            },
            ApllodbErrorKind::DatetimeFieldOverflow => ApllodbErrorAux {
                sqlstate: SqlState::new("22008".into()),
                errcode: "ERRCODE_DATETIME_FIELD_OVERFLOW".into(),
            },
            ApllodbErrorKind::DivisionByZero => ApllodbErrorAux {
                sqlstate: SqlState::new("22012".into()),
                errcode: "ERRCODE_DIVISION_BY_ZERO".into(),
            },
            ApllodbErrorKind::ErrorInAssignment => ApllodbErrorAux {
                sqlstate: SqlState::new("22005".into()),
                errcode: "ERRCODE_ERROR_IN_ASSIGNMENT".into(),
            },
            ApllodbErrorKind::EscapeCharacterConflict => ApllodbErrorAux {
                sqlstate: SqlState::new("2200B".into()),
                errcode: "ERRCODE_ESCAPE_CHARACTER_CONFLICT".into(),
            },
            ApllodbErrorKind::IndicatorOverflow => ApllodbErrorAux {
                sqlstate: SqlState::new("22022".into()),
                errcode: "ERRCODE_INDICATOR_OVERFLOW".into(),
            },
            ApllodbErrorKind::IntervalFieldOverflow => ApllodbErrorAux {
                sqlstate: SqlState::new("22015".into()),
                errcode: "ERRCODE_INTERVAL_FIELD_OVERFLOW".into(),
            },
            ApllodbErrorKind::InvalidArgumentForLogarithm => ApllodbErrorAux {
                sqlstate: SqlState::new("2201E".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_LOG".into(),
            },
            ApllodbErrorKind::InvalidArgumentForNtileFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("22014".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_NTILE".into(),
            },
            ApllodbErrorKind::InvalidArgumentForNthValueFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("22016".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_NTH_VALUE".into(),
            },
            ApllodbErrorKind::InvalidArgumentForPowerFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("2201F".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_POWER_FUNCTION".into(),
            },
            ApllodbErrorKind::InvalidArgumentForWidthBucketFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("2201G".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_WIDTH_BUCKET_FUNCTION".into(),
            },
            ApllodbErrorKind::InvalidCharacterValueForCast => ApllodbErrorAux {
                sqlstate: SqlState::new("22018".into()),
                errcode: "ERRCODE_INVALID_CHARACTER_VALUE_FOR_CAST".into(),
            },
            ApllodbErrorKind::InvalidDatetimeFormat => ApllodbErrorAux {
                sqlstate: SqlState::new("22007".into()),
                errcode: "ERRCODE_INVALID_DATETIME_FORMAT".into(),
            },
            ApllodbErrorKind::InvalidEscapeCharacter => ApllodbErrorAux {
                sqlstate: SqlState::new("22019".into()),
                errcode: "ERRCODE_INVALID_ESCAPE_CHARACTER".into(),
            },
            ApllodbErrorKind::InvalidEscapeOctet => ApllodbErrorAux {
                sqlstate: SqlState::new("2200D".into()),
                errcode: "ERRCODE_INVALID_ESCAPE_OCTET".into(),
            },
            ApllodbErrorKind::InvalidEscapeSequence => ApllodbErrorAux {
                sqlstate: SqlState::new("22025".into()),
                errcode: "ERRCODE_INVALID_ESCAPE_SEQUENCE".into(),
            },
            ApllodbErrorKind::NonstandardUseOfEscapeCharacter => ApllodbErrorAux {
                sqlstate: SqlState::new("22P06".into()),
                errcode: "ERRCODE_NONSTANDARD_USE_OF_ESCAPE_CHARACTER".into(),
            },
            ApllodbErrorKind::InvalidIndicatorParameterValue => ApllodbErrorAux {
                sqlstate: SqlState::new("22010".into()),
                errcode: "ERRCODE_INVALID_INDICATOR_PARAMETER_VALUE".into(),
            },
            ApllodbErrorKind::InvalidParameterValue => ApllodbErrorAux {
                sqlstate: SqlState::new("22023".into()),
                errcode: "ERRCODE_INVALID_PARAMETER_VALUE".into(),
            },
            ApllodbErrorKind::InvalidPrecedingOrFollowingSize => ApllodbErrorAux {
                sqlstate: SqlState::new("22013".into()),
                errcode: "ERRCODE_INVALID_PRECEDING_OR_FOLLOWING_SIZE".into(),
            },
            ApllodbErrorKind::InvalidRegularExpression => ApllodbErrorAux {
                sqlstate: SqlState::new("2201B".into()),
                errcode: "ERRCODE_INVALID_REGULAR_EXPRESSION".into(),
            },
            ApllodbErrorKind::InvalidRowCountInLimitClause => ApllodbErrorAux {
                sqlstate: SqlState::new("2201W".into()),
                errcode: "ERRCODE_INVALID_ROW_COUNT_IN_LIMIT_CLAUSE".into(),
            },
            ApllodbErrorKind::InvalidRowCountInResultOffsetClause => ApllodbErrorAux {
                sqlstate: SqlState::new("2201X".into()),
                errcode: "ERRCODE_INVALID_ROW_COUNT_IN_RESULT_OFFSET_CLAUSE".into(),
            },
            ApllodbErrorKind::InvalidTablesampleArgument => ApllodbErrorAux {
                sqlstate: SqlState::new("2202H".into()),
                errcode: "ERRCODE_INVALID_TABLESAMPLE_ARGUMENT".into(),
            },
            ApllodbErrorKind::InvalidTablesampleRepeat => ApllodbErrorAux {
                sqlstate: SqlState::new("2202G".into()),
                errcode: "ERRCODE_INVALID_TABLESAMPLE_REPEAT".into(),
            },
            ApllodbErrorKind::InvalidTimeZoneDisplacementValue => ApllodbErrorAux {
                sqlstate: SqlState::new("22009".into()),
                errcode: "ERRCODE_INVALID_TIME_ZONE_DISPLACEMENT_VALUE".into(),
            },
            ApllodbErrorKind::InvalidUseOfEscapeCharacter => ApllodbErrorAux {
                sqlstate: SqlState::new("2200C".into()),
                errcode: "ERRCODE_INVALID_USE_OF_ESCAPE_CHARACTER".into(),
            },
            ApllodbErrorKind::MostSpecificTypeMismatch => ApllodbErrorAux {
                sqlstate: SqlState::new("2200G".into()),
                errcode: "ERRCODE_MOST_SPECIFIC_TYPE_MISMATCH".into(),
            },
            ApllodbErrorKind::NullValueNotAllowed => ApllodbErrorAux {
                sqlstate: SqlState::new("22004".into()),
                errcode: "ERRCODE_NULL_VALUE_NOT_ALLOWED".into(),
            },
            ApllodbErrorKind::NullValueNoIndicatorParameter => ApllodbErrorAux {
                sqlstate: SqlState::new("22002".into()),
                errcode: "ERRCODE_NULL_VALUE_NO_INDICATOR_PARAMETER".into(),
            },
            ApllodbErrorKind::NumericValueOutOfRange => ApllodbErrorAux {
                sqlstate: SqlState::new("22003".into()),
                errcode: "ERRCODE_NUMERIC_VALUE_OUT_OF_RANGE".into(),
            },
            ApllodbErrorKind::SequenceGeneratorLimitExceeded => ApllodbErrorAux {
                sqlstate: SqlState::new("2200H".into()),
                errcode: "ERRCODE_SEQUENCE_GENERATOR_LIMIT_EXCEEDED".into(),
            },
            ApllodbErrorKind::StringDataLengthMismatch => ApllodbErrorAux {
                sqlstate: SqlState::new("22026".into()),
                errcode: "ERRCODE_STRING_DATA_LENGTH_MISMATCH".into(),
            },
            ApllodbErrorKind::StringDataRightTruncation => ApllodbErrorAux {
                sqlstate: SqlState::new("22001".into()),
                errcode: "ERRCODE_STRING_DATA_RIGHT_TRUNCATION".into(),
            },
            ApllodbErrorKind::SubstringError => ApllodbErrorAux {
                sqlstate: SqlState::new("22011".into()),
                errcode: "ERRCODE_SUBSTRING_ERROR".into(),
            },
            ApllodbErrorKind::TrimError => ApllodbErrorAux {
                sqlstate: SqlState::new("22027".into()),
                errcode: "ERRCODE_TRIM_ERROR".into(),
            },
            ApllodbErrorKind::UnterminatedCString => ApllodbErrorAux {
                sqlstate: SqlState::new("22024".into()),
                errcode: "ERRCODE_UNTERMINATED_C_STRING".into(),
            },
            ApllodbErrorKind::ZeroLengthCharacterString => ApllodbErrorAux {
                sqlstate: SqlState::new("2200F".into()),
                errcode: "ERRCODE_ZERO_LENGTH_CHARACTER_STRING".into(),
            },
            ApllodbErrorKind::FloatingPointException => ApllodbErrorAux {
                sqlstate: SqlState::new("22P01".into()),
                errcode: "ERRCODE_FLOATING_POINT_EXCEPTION".into(),
            },
            ApllodbErrorKind::InvalidTextRepresentation => ApllodbErrorAux {
                sqlstate: SqlState::new("22P02".into()),
                errcode: "ERRCODE_INVALID_TEXT_REPRESENTATION".into(),
            },
            ApllodbErrorKind::InvalidBinaryRepresentation => ApllodbErrorAux {
                sqlstate: SqlState::new("22P03".into()),
                errcode: "ERRCODE_INVALID_BINARY_REPRESENTATION".into(),
            },
            ApllodbErrorKind::BadCopyFileFormat => ApllodbErrorAux {
                sqlstate: SqlState::new("22P04".into()),
                errcode: "ERRCODE_BAD_COPY_FILE_FORMAT".into(),
            },
            ApllodbErrorKind::UntranslatableCharacter => ApllodbErrorAux {
                sqlstate: SqlState::new("22P05".into()),
                errcode: "ERRCODE_UNTRANSLATABLE_CHARACTER".into(),
            },
            ApllodbErrorKind::NotAnXmlDocument => ApllodbErrorAux {
                sqlstate: SqlState::new("2200L".into()),
                errcode: "ERRCODE_NOT_AN_XML_DOCUMENT".into(),
            },
            ApllodbErrorKind::InvalidXmlDocument => ApllodbErrorAux {
                sqlstate: SqlState::new("2200M".into()),
                errcode: "ERRCODE_INVALID_XML_DOCUMENT".into(),
            },
            ApllodbErrorKind::InvalidXmlContent => ApllodbErrorAux {
                sqlstate: SqlState::new("2200N".into()),
                errcode: "ERRCODE_INVALID_XML_CONTENT".into(),
            },
            ApllodbErrorKind::InvalidXmlComment => ApllodbErrorAux {
                sqlstate: SqlState::new("2200S".into()),
                errcode: "ERRCODE_INVALID_XML_COMMENT".into(),
            },
            ApllodbErrorKind::InvalidXmlProcessingInstruction => ApllodbErrorAux {
                sqlstate: SqlState::new("2200T".into()),
                errcode: "ERRCODE_INVALID_XML_PROCESSING_INSTRUCTION".into(),
            },
            ApllodbErrorKind::DuplicateJsonObjectKeyValue => ApllodbErrorAux {
                sqlstate: SqlState::new("22030".into()),
                errcode: "ERRCODE_DUPLICATE_JSON_OBJECT_KEY_VALUE".into(),
            },
            ApllodbErrorKind::InvalidArgumentForJsonDatetimeFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("22031".into()),
                errcode: "ERRCODE_INVALID_ARGUMENT_FOR_JSON_DATETIME_FUNCTION".into(),
            },
            ApllodbErrorKind::InvalidJsonText => ApllodbErrorAux {
                sqlstate: SqlState::new("22032".into()),
                errcode: "ERRCODE_INVALID_JSON_TEXT".into(),
            },
            ApllodbErrorKind::InvalidSqlJsonSubscript => ApllodbErrorAux {
                sqlstate: SqlState::new("22033".into()),
                errcode: "ERRCODE_INVALID_SQL_JSON_SUBSCRIPT".into(),
            },
            ApllodbErrorKind::MoreThanOneSqlJsonItem => ApllodbErrorAux {
                sqlstate: SqlState::new("22034".into()),
                errcode: "ERRCODE_MORE_THAN_ONE_SQL_JSON_ITEM".into(),
            },
            ApllodbErrorKind::NoSqlJsonItem => ApllodbErrorAux {
                sqlstate: SqlState::new("22035".into()),
                errcode: "ERRCODE_NO_SQL_JSON_ITEM".into(),
            },
            ApllodbErrorKind::NonNumericSqlJsonItem => ApllodbErrorAux {
                sqlstate: SqlState::new("22036".into()),
                errcode: "ERRCODE_NON_NUMERIC_SQL_JSON_ITEM".into(),
            },
            ApllodbErrorKind::NonUniqueKeysInAJsonObject => ApllodbErrorAux {
                sqlstate: SqlState::new("22037".into()),
                errcode: "ERRCODE_NON_UNIQUE_KEYS_IN_A_JSON_OBJECT".into(),
            },
            ApllodbErrorKind::SingletonSqlJsonItemRequired => ApllodbErrorAux {
                sqlstate: SqlState::new("22038".into()),
                errcode: "ERRCODE_SINGLETON_SQL_JSON_ITEM_REQUIRED".into(),
            },
            ApllodbErrorKind::SqlJsonArrayNotFound => ApllodbErrorAux {
                sqlstate: SqlState::new("22039".into()),
                errcode: "ERRCODE_SQL_JSON_ARRAY_NOT_FOUND".into(),
            },
            ApllodbErrorKind::SqlJsonMemberNotFound => ApllodbErrorAux {
                sqlstate: SqlState::new("2203A".into()),
                errcode: "ERRCODE_SQL_JSON_MEMBER_NOT_FOUND".into(),
            },
            ApllodbErrorKind::SqlJsonNumberNotFound => ApllodbErrorAux {
                sqlstate: SqlState::new("2203B".into()),
                errcode: "ERRCODE_SQL_JSON_NUMBER_NOT_FOUND".into(),
            },
            ApllodbErrorKind::SqlJsonObjectNotFound => ApllodbErrorAux {
                sqlstate: SqlState::new("2203C".into()),
                errcode: "ERRCODE_SQL_JSON_OBJECT_NOT_FOUND".into(),
            },
            ApllodbErrorKind::TooManyJsonArrayElements => ApllodbErrorAux {
                sqlstate: SqlState::new("2203D".into()),
                errcode: "ERRCODE_TOO_MANY_JSON_ARRAY_ELEMENTS".into(),
            },
            ApllodbErrorKind::TooManyJsonObjectMembers => ApllodbErrorAux {
                sqlstate: SqlState::new("2203E".into()),
                errcode: "ERRCODE_TOO_MANY_JSON_OBJECT_MEMBERS".into(),
            },
            ApllodbErrorKind::SqlJsonScalarRequired => ApllodbErrorAux {
                sqlstate: SqlState::new("2203F".into()),
                errcode: "ERRCODE_SQL_JSON_SCALAR_REQUIRED".into(),
            },
            ApllodbErrorKind::IntegrityConstraintViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23000".into()),
                errcode: "ERRCODE_INTEGRITY_CONSTRAINT_VIOLATION".into(),
            },
            ApllodbErrorKind::RestrictViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23001".into()),
                errcode: "ERRCODE_RESTRICT_VIOLATION".into(),
            },
            ApllodbErrorKind::NotNullViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23502".into()),
                errcode: "ERRCODE_NOT_NULL_VIOLATION".into(),
            },
            ApllodbErrorKind::ForeignKeyViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23503".into()),
                errcode: "ERRCODE_FOREIGN_KEY_VIOLATION".into(),
            },
            ApllodbErrorKind::UniqueViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23505".into()),
                errcode: "ERRCODE_UNIQUE_VIOLATION".into(),
            },
            ApllodbErrorKind::CheckViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23514".into()),
                errcode: "ERRCODE_CHECK_VIOLATION".into(),
            },
            ApllodbErrorKind::ExclusionViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("23P01".into()),
                errcode: "ERRCODE_EXCLUSION_VIOLATION".into(),
            },
            ApllodbErrorKind::InvalidCursorState => ApllodbErrorAux {
                sqlstate: SqlState::new("24000".into()),
                errcode: "ERRCODE_INVALID_CURSOR_STATE".into(),
            },
            ApllodbErrorKind::InvalidTransactionState => ApllodbErrorAux {
                sqlstate: SqlState::new("25000".into()),
                errcode: "ERRCODE_INVALID_TRANSACTION_STATE".into(),
            },
            ApllodbErrorKind::ActiveSqlTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25001".into()),
                errcode: "ERRCODE_ACTIVE_SQL_TRANSACTION".into(),
            },
            ApllodbErrorKind::BranchTransactionAlreadyActive => ApllodbErrorAux {
                sqlstate: SqlState::new("25002".into()),
                errcode: "ERRCODE_BRANCH_TRANSACTION_ALREADY_ACTIVE".into(),
            },
            ApllodbErrorKind::HeldCursorRequiresSameIsolationLevel => ApllodbErrorAux {
                sqlstate: SqlState::new("25008".into()),
                errcode: "ERRCODE_HELD_CURSOR_REQUIRES_SAME_ISOLATION_LEVEL".into(),
            },
            ApllodbErrorKind::InappropriateAccessModeForBranchTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25003".into()),
                errcode: "ERRCODE_INAPPROPRIATE_ACCESS_MODE_FOR_BRANCH_TRANSACTION".into(),
            },
            ApllodbErrorKind::InappropriateIsolationLevelForBranchTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25004".into()),
                errcode: "ERRCODE_INAPPROPRIATE_ISOLATION_LEVEL_FOR_BRANCH_TRANSACTION".into(),
            },
            ApllodbErrorKind::NoActiveSqlTransactionForBranchTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25005".into()),
                errcode: "ERRCODE_NO_ACTIVE_SQL_TRANSACTION_FOR_BRANCH_TRANSACTION".into(),
            },
            ApllodbErrorKind::ReadOnlySqlTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25006".into()),
                errcode: "ERRCODE_READ_ONLY_SQL_TRANSACTION".into(),
            },
            ApllodbErrorKind::SchemaAndDataStatementMixingNotSupported => ApllodbErrorAux {
                sqlstate: SqlState::new("25007".into()),
                errcode: "ERRCODE_SCHEMA_AND_DATA_STATEMENT_MIXING_NOT_SUPPORTED".into(),
            },
            ApllodbErrorKind::NoActiveSqlTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25P01".into()),
                errcode: "ERRCODE_NO_ACTIVE_SQL_TRANSACTION".into(),
            },
            ApllodbErrorKind::InFailedSqlTransaction => ApllodbErrorAux {
                sqlstate: SqlState::new("25P02".into()),
                errcode: "ERRCODE_IN_FAILED_SQL_TRANSACTION".into(),
            },
            ApllodbErrorKind::IdleInTransactionSessionTimeout => ApllodbErrorAux {
                sqlstate: SqlState::new("25P03".into()),
                errcode: "ERRCODE_IDLE_IN_TRANSACTION_SESSION_TIMEOUT".into(),
            },
            ApllodbErrorKind::InvalidSqlStatementName => ApllodbErrorAux {
                sqlstate: SqlState::new("26000".into()),
                errcode: "ERRCODE_INVALID_SQL_STATEMENT_NAME".into(),
            },
            ApllodbErrorKind::TriggeredDataChangeViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("27000".into()),
                errcode: "ERRCODE_TRIGGERED_DATA_CHANGE_VIOLATION".into(),
            },
            ApllodbErrorKind::InvalidAuthorizationSpecification => ApllodbErrorAux {
                sqlstate: SqlState::new("28000".into()),
                errcode: "ERRCODE_INVALID_AUTHORIZATION_SPECIFICATION".into(),
            },
            ApllodbErrorKind::InvalidPassword => ApllodbErrorAux {
                sqlstate: SqlState::new("28P01".into()),
                errcode: "ERRCODE_INVALID_PASSWORD".into(),
            },
            ApllodbErrorKind::DependentPrivilegeDescriptorsStillExist => ApllodbErrorAux {
                sqlstate: SqlState::new("2B000".into()),
                errcode: "ERRCODE_DEPENDENT_PRIVILEGE_DESCRIPTORS_STILL_EXIST".into(),
            },
            ApllodbErrorKind::DependentObjectsStillExist => ApllodbErrorAux {
                sqlstate: SqlState::new("2BP01".into()),
                errcode: "ERRCODE_DEPENDENT_OBJECTS_STILL_EXIST".into(),
            },
            ApllodbErrorKind::InvalidTransactionTermination => ApllodbErrorAux {
                sqlstate: SqlState::new("2D000".into()),
                errcode: "ERRCODE_INVALID_TRANSACTION_TERMINATION".into(),
            },
            ApllodbErrorKind::SqlRoutineException => ApllodbErrorAux {
                sqlstate: SqlState::new("2F000".into()),
                errcode: "ERRCODE_SQL_ROUTINE_EXCEPTION".into(),
            },
            ApllodbErrorKind::SqlFunctionExecutedNoReturnStatement => ApllodbErrorAux {
                sqlstate: SqlState::new("2F005".into()),
                errcode: "ERRCODE_S_R_E_FUNCTION_EXECUTED_NO_RETURN_STATEMENT".into(),
            },
            ApllodbErrorKind::SqlModifyingSqlDataNotPermitted => ApllodbErrorAux {
                sqlstate: SqlState::new("2F002".into()),
                errcode: "ERRCODE_S_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED".into(),
            },
            ApllodbErrorKind::SqlProhibitedSqlStatementAttempted => ApllodbErrorAux {
                sqlstate: SqlState::new("2F003".into()),
                errcode: "ERRCODE_S_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED".into(),
            },
            ApllodbErrorKind::SqlReadingSqlDataNotPermitted => ApllodbErrorAux {
                sqlstate: SqlState::new("2F004".into()),
                errcode: "ERRCODE_S_R_E_READING_SQL_DATA_NOT_PERMITTED".into(),
            },
            ApllodbErrorKind::InvalidCursorName => ApllodbErrorAux {
                sqlstate: SqlState::new("34000".into()),
                errcode: "ERRCODE_INVALID_CURSOR_NAME".into(),
            },
            ApllodbErrorKind::ExternalRoutineException => ApllodbErrorAux {
                sqlstate: SqlState::new("38000".into()),
                errcode: "ERRCODE_EXTERNAL_ROUTINE_EXCEPTION".into(),
            },
            ApllodbErrorKind::ExternalContainingSqlNotPermitted => ApllodbErrorAux {
                sqlstate: SqlState::new("38001".into()),
                errcode: "ERRCODE_E_R_E_CONTAINING_SQL_NOT_PERMITTED".into(),
            },
            ApllodbErrorKind::ExternalModifyingSqlDataNotPermitted => ApllodbErrorAux {
                sqlstate: SqlState::new("38002".into()),
                errcode: "ERRCODE_E_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED".into(),
            },
            ApllodbErrorKind::ExternalProhibitedSqlStatementAttempted => ApllodbErrorAux {
                sqlstate: SqlState::new("38003".into()),
                errcode: "ERRCODE_E_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED".into(),
            },
            ApllodbErrorKind::ExternalReadingSqlDataNotPermitted => ApllodbErrorAux {
                sqlstate: SqlState::new("38004".into()),
                errcode: "ERRCODE_E_R_E_READING_SQL_DATA_NOT_PERMITTED".into(),
            },
            ApllodbErrorKind::ExternalRoutineInvocationException => ApllodbErrorAux {
                sqlstate: SqlState::new("39000".into()),
                errcode: "ERRCODE_EXTERNAL_ROUTINE_INVOCATION_EXCEPTION".into(),
            },
            ApllodbErrorKind::ExternalInvalidSqlstateReturned => ApllodbErrorAux {
                sqlstate: SqlState::new("39001".into()),
                errcode: "ERRCODE_E_R_I_E_INVALID_SQLSTATE_RETURNED".into(),
            },
            ApllodbErrorKind::ExternalNullValueNotAllowed => ApllodbErrorAux {
                sqlstate: SqlState::new("39004".into()),
                errcode: "ERRCODE_E_R_I_E_NULL_VALUE_NOT_ALLOWED".into(),
            },
            ApllodbErrorKind::ExternalTriggerProtocolViolated => ApllodbErrorAux {
                sqlstate: SqlState::new("39P01".into()),
                errcode: "ERRCODE_E_R_I_E_TRIGGER_PROTOCOL_VIOLATED".into(),
            },
            ApllodbErrorKind::ExternalSrfProtocolViolated => ApllodbErrorAux {
                sqlstate: SqlState::new("39P02".into()),
                errcode: "ERRCODE_E_R_I_E_SRF_PROTOCOL_VIOLATED".into(),
            },
            ApllodbErrorKind::ExternalEventTriggerProtocolViolated => ApllodbErrorAux {
                sqlstate: SqlState::new("39P03".into()),
                errcode: "ERRCODE_E_R_I_E_EVENT_TRIGGER_PROTOCOL_VIOLATED".into(),
            },
            ApllodbErrorKind::SavepointException => ApllodbErrorAux {
                sqlstate: SqlState::new("3B000".into()),
                errcode: "ERRCODE_SAVEPOINT_EXCEPTION".into(),
            },
            ApllodbErrorKind::InvalidSavepointSpecification => ApllodbErrorAux {
                sqlstate: SqlState::new("3B001".into()),
                errcode: "ERRCODE_S_E_INVALID_SPECIFICATION".into(),
            },
            ApllodbErrorKind::InvalidCatalogName => ApllodbErrorAux {
                sqlstate: SqlState::new("3D000".into()),
                errcode: "ERRCODE_INVALID_CATALOG_NAME".into(),
            },
            ApllodbErrorKind::InvalidSchemaName => ApllodbErrorAux {
                sqlstate: SqlState::new("3F000".into()),
                errcode: "ERRCODE_INVALID_SCHEMA_NAME".into(),
            },
            ApllodbErrorKind::TransactionRollback => ApllodbErrorAux {
                sqlstate: SqlState::new("40000".into()),
                errcode: "ERRCODE_TRANSACTION_ROLLBACK".into(),
            },
            ApllodbErrorKind::TransactionIntegrityConstraintViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("40002".into()),
                errcode: "ERRCODE_T_R_INTEGRITY_CONSTRAINT_VIOLATION".into(),
            },
            ApllodbErrorKind::SerializationFailure => ApllodbErrorAux {
                sqlstate: SqlState::new("40001".into()),
                errcode: "ERRCODE_T_R_SERIALIZATION_FAILURE".into(),
            },
            ApllodbErrorKind::StatementCompletionUnknown => ApllodbErrorAux {
                sqlstate: SqlState::new("40003".into()),
                errcode: "ERRCODE_T_R_STATEMENT_COMPLETION_UNKNOWN".into(),
            },
            ApllodbErrorKind::DeadlockDetected => ApllodbErrorAux {
                sqlstate: SqlState::new("40P01".into()),
                errcode: "ERRCODE_T_R_DEADLOCK_DETECTED".into(),
            },
            ApllodbErrorKind::SyntaxErrorOrAccessRuleViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("42000".into()),
                errcode: "ERRCODE_SYNTAX_ERROR_OR_ACCESS_RULE_VIOLATION".into(),
            },
            ApllodbErrorKind::SyntaxError => ApllodbErrorAux {
                sqlstate: SqlState::new("42601".into()),
                errcode: "ERRCODE_SYNTAX_ERROR".into(),
            },
            ApllodbErrorKind::InsufficientPrivilege => ApllodbErrorAux {
                sqlstate: SqlState::new("42501".into()),
                errcode: "ERRCODE_INSUFFICIENT_PRIVILEGE".into(),
            },
            ApllodbErrorKind::CannotCoerce => ApllodbErrorAux {
                sqlstate: SqlState::new("42846".into()),
                errcode: "ERRCODE_CANNOT_COERCE".into(),
            },
            ApllodbErrorKind::GroupingError => ApllodbErrorAux {
                sqlstate: SqlState::new("42803".into()),
                errcode: "ERRCODE_GROUPING_ERROR".into(),
            },
            ApllodbErrorKind::WindowingError => ApllodbErrorAux {
                sqlstate: SqlState::new("42P20".into()),
                errcode: "ERRCODE_WINDOWING_ERROR".into(),
            },
            ApllodbErrorKind::InvalidRecursion => ApllodbErrorAux {
                sqlstate: SqlState::new("42P19".into()),
                errcode: "ERRCODE_INVALID_RECURSION".into(),
            },
            ApllodbErrorKind::InvalidForeignKey => ApllodbErrorAux {
                sqlstate: SqlState::new("42830".into()),
                errcode: "ERRCODE_INVALID_FOREIGN_KEY".into(),
            },
            ApllodbErrorKind::InvalidName => ApllodbErrorAux {
                sqlstate: SqlState::new("42602".into()),
                errcode: "ERRCODE_INVALID_NAME".into(),
            },
            ApllodbErrorKind::NameTooLong => ApllodbErrorAux {
                sqlstate: SqlState::new("42622".into()),
                errcode: "ERRCODE_NAME_TOO_LONG".into(),
            },
            ApllodbErrorKind::ReservedName => ApllodbErrorAux {
                sqlstate: SqlState::new("42939".into()),
                errcode: "ERRCODE_RESERVED_NAME".into(),
            },
            ApllodbErrorKind::DatatypeMismatch => ApllodbErrorAux {
                sqlstate: SqlState::new("42804".into()),
                errcode: "ERRCODE_DATATYPE_MISMATCH".into(),
            },
            ApllodbErrorKind::IndeterminateDatatype => ApllodbErrorAux {
                sqlstate: SqlState::new("42P18".into()),
                errcode: "ERRCODE_INDETERMINATE_DATATYPE".into(),
            },
            ApllodbErrorKind::CollationMismatch => ApllodbErrorAux {
                sqlstate: SqlState::new("42P21".into()),
                errcode: "ERRCODE_COLLATION_MISMATCH".into(),
            },
            ApllodbErrorKind::IndeterminateCollation => ApllodbErrorAux {
                sqlstate: SqlState::new("42P22".into()),
                errcode: "ERRCODE_INDETERMINATE_COLLATION".into(),
            },
            ApllodbErrorKind::WrongObjectType => ApllodbErrorAux {
                sqlstate: SqlState::new("42809".into()),
                errcode: "ERRCODE_WRONG_OBJECT_TYPE".into(),
            },
            ApllodbErrorKind::GeneratedAlways => ApllodbErrorAux {
                sqlstate: SqlState::new("428C9".into()),
                errcode: "ERRCODE_GENERATED_ALWAYS".into(),
            },
            ApllodbErrorKind::UndefinedColumn => ApllodbErrorAux {
                sqlstate: SqlState::new("42703".into()),
                errcode: "ERRCODE_UNDEFINED_COLUMN".into(),
            },
            ApllodbErrorKind::UndefinedFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("42883".into()),
                errcode: "ERRCODE_UNDEFINED_FUNCTION".into(),
            },
            ApllodbErrorKind::UndefinedTable => ApllodbErrorAux {
                sqlstate: SqlState::new("42P01".into()),
                errcode: "ERRCODE_UNDEFINED_TABLE".into(),
            },
            ApllodbErrorKind::UndefinedParameter => ApllodbErrorAux {
                sqlstate: SqlState::new("42P02".into()),
                errcode: "ERRCODE_UNDEFINED_PARAMETER".into(),
            },
            ApllodbErrorKind::UndefinedObject => ApllodbErrorAux {
                sqlstate: SqlState::new("42704".into()),
                errcode: "ERRCODE_UNDEFINED_OBJECT".into(),
            },
            ApllodbErrorKind::DuplicateColumn => ApllodbErrorAux {
                sqlstate: SqlState::new("42701".into()),
                errcode: "ERRCODE_DUPLICATE_COLUMN".into(),
            },
            ApllodbErrorKind::DuplicateCursor => ApllodbErrorAux {
                sqlstate: SqlState::new("42P03".into()),
                errcode: "ERRCODE_DUPLICATE_CURSOR".into(),
            },
            ApllodbErrorKind::DuplicateDatabase => ApllodbErrorAux {
                sqlstate: SqlState::new("42P04".into()),
                errcode: "ERRCODE_DUPLICATE_DATABASE".into(),
            },
            ApllodbErrorKind::DuplicateFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("42723".into()),
                errcode: "ERRCODE_DUPLICATE_FUNCTION".into(),
            },
            ApllodbErrorKind::DuplicatePreparedStatement => ApllodbErrorAux {
                sqlstate: SqlState::new("42P05".into()),
                errcode: "ERRCODE_DUPLICATE_PSTATEMENT".into(),
            },
            ApllodbErrorKind::DuplicateSchema => ApllodbErrorAux {
                sqlstate: SqlState::new("42P06".into()),
                errcode: "ERRCODE_DUPLICATE_SCHEMA".into(),
            },
            ApllodbErrorKind::DuplicateTable => ApllodbErrorAux {
                sqlstate: SqlState::new("42P07".into()),
                errcode: "ERRCODE_DUPLICATE_TABLE".into(),
            },
            ApllodbErrorKind::DuplicateAlias => ApllodbErrorAux {
                sqlstate: SqlState::new("42712".into()),
                errcode: "ERRCODE_DUPLICATE_ALIAS".into(),
            },
            ApllodbErrorKind::DuplicateObject => ApllodbErrorAux {
                sqlstate: SqlState::new("42710".into()),
                errcode: "ERRCODE_DUPLICATE_OBJECT".into(),
            },
            ApllodbErrorKind::AmbiguousColumn => ApllodbErrorAux {
                sqlstate: SqlState::new("42702".into()),
                errcode: "ERRCODE_AMBIGUOUS_COLUMN".into(),
            },
            ApllodbErrorKind::AmbiguousFunction => ApllodbErrorAux {
                sqlstate: SqlState::new("42725".into()),
                errcode: "ERRCODE_AMBIGUOUS_FUNCTION".into(),
            },
            ApllodbErrorKind::AmbiguousParameter => ApllodbErrorAux {
                sqlstate: SqlState::new("42P08".into()),
                errcode: "ERRCODE_AMBIGUOUS_PARAMETER".into(),
            },
            ApllodbErrorKind::AmbiguousAlias => ApllodbErrorAux {
                sqlstate: SqlState::new("42P09".into()),
                errcode: "ERRCODE_AMBIGUOUS_ALIAS".into(),
            },
            ApllodbErrorKind::InvalidColumnReference => ApllodbErrorAux {
                sqlstate: SqlState::new("42P10".into()),
                errcode: "ERRCODE_INVALID_COLUMN_REFERENCE".into(),
            },
            ApllodbErrorKind::InvalidColumnDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42611".into()),
                errcode: "ERRCODE_INVALID_COLUMN_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidCursorDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P11".into()),
                errcode: "ERRCODE_INVALID_CURSOR_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidDatabaseDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P12".into()),
                errcode: "ERRCODE_INVALID_DATABASE_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidFunctionDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P13".into()),
                errcode: "ERRCODE_INVALID_FUNCTION_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidPreparedStatementDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P14".into()),
                errcode: "ERRCODE_INVALID_PSTATEMENT_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidSchemaDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P15".into()),
                errcode: "ERRCODE_INVALID_SCHEMA_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidTableDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P16".into()),
                errcode: "ERRCODE_INVALID_TABLE_DEFINITION".into(),
            },
            ApllodbErrorKind::InvalidObjectDefinition => ApllodbErrorAux {
                sqlstate: SqlState::new("42P17".into()),
                errcode: "ERRCODE_INVALID_OBJECT_DEFINITION".into(),
            },
            ApllodbErrorKind::WithCheckOptionViolation => ApllodbErrorAux {
                sqlstate: SqlState::new("44000".into()),
                errcode: "ERRCODE_WITH_CHECK_OPTION_VIOLATION".into(),
            },
            ApllodbErrorKind::SystemError => ApllodbErrorAux {
                sqlstate: SqlState::new("58000".into()),
                errcode: "ERRCODE_SYSTEM_ERROR".into(),
            },
            ApllodbErrorKind::IoError => ApllodbErrorAux {
                sqlstate: SqlState::new("58030".into()),
                errcode: "ERRCODE_IO_ERROR".into(),
            },
            ApllodbErrorKind::DeserializationError => ApllodbErrorAux {
                sqlstate: SqlState::new("58100".into()),
                errcode: "ERRCODE_DESERIALIZATION_ERROR".into(),
            },
            ApllodbErrorKind::SerializationError => ApllodbErrorAux {
                sqlstate: SqlState::new("58110".into()),
                errcode: "ERRCODE_SERIALIZATION_ERROR".into(),
            },
            ApllodbErrorKind::UndefinedPrimaryKey => ApllodbErrorAux {
                sqlstate: SqlState::new("58200".into()),
                errcode: "ERRCODE_UNDEFINED_PRIMARY_KEY_ERROR".into(),
            },
        }
    }
}
