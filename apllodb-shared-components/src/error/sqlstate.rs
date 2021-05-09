mod sqlstate_detail;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use self::sqlstate_detail::{SqlStateCategory, SqlStateClass, SqlStateDetail};

/// SQLSTATE.
///
/// Errors for specific SQL part (`SQL/JRT`, for example) are omitted.
///
/// See: <https://en.wikipedia.org/wiki/SQLSTATE>
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub enum SqlState {
    SuccessfulCompletion,
    Warning,
    WarningCursorOperationConflict,
    WarningDisconnectError,
    WarningNullValueEliminatedInSetFunction,
    WarningStringDataRightTruncation,
    WarningInsufficientItemDescriptorAreas,
    WarningPrivilegeNotRevoked,
    WarningPrivilegeNotGranted,
    WarningSearchConditionTooLongForInformationSchema,
    WarningQueryExpressionTooLongForInformationSchema,
    WarningDefaultValueTooLongForInformationSchema,
    WarningResultSetsReturned,
    WarningAdditionalResultSetsReturned,
    WarningAttemptToReturnTooManyResultSets,
    WarningStatementTooLongForInformationSchema,
    WarningInvalidNumberOfConditions,
    WarningArrayDataRightTruncation,
    NoData,
    NoDataNoAdditionalResultSetsReturned,
    DynamicSQLError,
    DynamicSQLErrorUsingClauseDoesNotMatchDynamicParameterSpecifications,
    DynamicSQLErrorUsingClauseDoesNotMatchTargetSpecifications,
    DynamicSQLErrorCursorSpecificationCannotBeExecuted,
    DynamicSQLErrorUsingClauseRequiredForDynamicParameters,
    DynamicSQLErrorPreparedStatementNotACursorSpecification,
    DynamicSQLErrorRestrictedDataTypeAttributeViolation,
    DynamicSQLErrorUsingClauseRequiredForResultFields,
    DynamicSQLErrorInvalidDescriptorCount,
    DynamicSQLErrorInvalidDescriptorIndex,
    DynamicSQLErrorDataTypeTransformFunctionViolation,
    DynamicSQLErrorUndefinedDATAValue,
    DynamicSQLErrorInvalidDATATarget,
    DynamicSQLErrorInvalidLEVELValue,
    DynamicSQLErrorInvalidDatetimeIntervalCode,
    ConnectionException,
    ConnectionExceptionSQLclientUnableToEstablishSQLconnection,
    ConnectionExceptionConnectionNameInUse,
    ConnectionExceptionConnectionDoesNotExist,
    ConnectionExceptionSQLserverRejectedEstablishmentOfSQLconnection,
    ConnectionExceptionConnectionFailure,
    ConnectionExceptionTransactionResolutionUnknown,
    ConnectionExceptionDatabaseNotOpen,
    ConnectionExceptionDatabaseAlreadyOpen,
    TriggeredActionException,
    FeatureNotSupported,
    FeatureNotSupportedMultipleServerTransactions,
    InvalidTargetTypeSpecification,
    InvalidSchemaNameListSpecification,
    LocatorException,
    LocatorExceptionInvalidSpecification,
    InvalidGrantor,
    InvalidSQLinvokedProcedureReference,
    InvalidRoleSpecification,
    InvalidTransformGroupNameSpecification,
    TargetTableDisagreesWithCursorSpecification,
    AttemptToAssignToNonupdatableColumn,
    AttemptToAssignToOrderingColumn,
    ProhibitedStatementEncounteredDuringTriggerExecution,
    ProhibitedStatementEncounteredDuringTriggerExecutionModifyTableModifiedByDataChangeDeltaTable,
    DiagnosticsException,
    DiagnosticsExceptionMaximumNumberOfStackedDiagnosticsAreasExceeded,
    CardinalityViolation,
    DataException,
    DataExceptionStringDataRightTruncation,
    DataExceptionNullValueNoIndicatorParameter,
    DataExceptionNumericValueOutOfRange,
    DataExceptionNullValueNotAllowed,
    DataExceptionErrorInAssignment,
    DataExceptionInvalidIntervalFormat,
    DataExceptionInvalidDatetimeFormat,
    DataExceptionDatetimeFieldOverflow,
    DataExceptionInvalidTimeZoneDisplacementValue,
    DataExceptionEscapeCharacterConflict,
    DataExceptionInvalidUseOfEscapeCharacter,
    DataExceptionInvalidEscapeOctet,
    DataExceptionNullValueInArrayTarget,
    DataExceptionZerolengthCharacterString,
    DataExceptionMostSpecificTypeMismatch,
    DataExceptionSequenceGeneratorLimitExceeded,
    DataExceptionIntervalValueOutOfRange,
    DataExceptionMultisetValueOverflow,
    DataExceptionInvalidIndicatorParameterValue,
    DataExceptionSubstringError,
    DataExceptionDivisionByZero,
    DataExceptionInvalidPrecedingOrFollowingSizeInWindowFunction,
    DataExceptionInvalidArgumentForNTILEFunction,
    DataExceptionIntervalFieldOverflow,
    DataExceptionInvalidArgumentForNthValueFunction,
    DataExceptionInvalidCharacterValueForCast,
    DataExceptionInvalidEscapeCharacter,
    DataExceptionInvalidRegularExpression,
    DataExceptionNullRowNotPermittedInTable,
    DataExceptionInvalidArgumentForNaturalLogarithm,
    DataExceptionInvalidArgumentForPowerFunction,
    DataExceptionInvalidArgumentForWidthBucketFunction,
    DataExceptionInvalidRowVersion,
    DataExceptionInvalidQueryRegularExpression,
    DataExceptionInvalidQueryOptionFlag,
    DataExceptionAttemptToReplaceAZerolengthString,
    DataExceptionInvalidQueryReplacementString,
    DataExceptionInvalidRowCountInFetchFirstClause,
    DataExceptionInvalidRowCountInResultOffsetClause,
    DataExceptionCharacterNotInRepertoire,
    DataExceptionIndicatorOverflow,
    DataExceptionInvalidParameterValue,
    DataExceptionUnterminatedCString,
    DataExceptionInvalidEscapeSequence,
    DataExceptionStringDataLengthMismatch,
    DataExceptionTrimError,
    DataExceptionNoncharacterInUCSString,
    DataExceptionNullValueSubstitutedForMutatorSubjectParameter,
    DataExceptionArrayElementError,
    DataExceptionArrayDataRightTruncation,
    DataExceptionInvalidRepeatArgumentInASampleClause,
    DataExceptionInvalidSampleSize,
    DataExceptionIllegalConversion,
    DataExceptionIllegalComparison,
    DataExceptionIllegalOperation,
    IntegrityConstraintViolation,
    IntegrityConstraintViolationRestrictViolation,
    IntegrityConstraintNotNullViolation,
    IntegrityConstraintUniqueViolation,
    InvalidCursorState,
    InvalidTransactionState,
    InvalidTransactionStateActiveSQLtransaction,
    InvalidTransactionStateBranchTransactionAlreadyActive,
    InvalidTransactionStateInappropriateAccessModeForBranchTransaction,
    InvalidTransactionStateInappropriateIsolationLevelForBranchTransaction,
    InvalidTransactionStateNoActiveSQLtransactionForBranchTransaction,
    InvalidTransactionStateReadonlySQLtransaction,
    InvalidTransactionStateSchemaAndDataStatementMixingNotSupported,
    InvalidTransactionStateHeldCursorRequiresSameIsolationLevel,
    InvalidSQLStatementName,
    TriggeredDataChangeViolation,
    TriggeredDataChangeViolationModifyTableModifiedByDataChangeDeltaTable,
    InvalidAuthorizationSpecification,
    DependentPrivilegeDescriptorsStillExist,
    InvalidCharacterSetName,
    InvalidTransactionTermination,
    InvalidConnectionName,
    SQLRoutineException,
    SQLRoutineExceptionModifyingSQLdataNotPermitted,
    SQLRoutineExceptionProhibitedSQLstatementAttempted,
    SQLRoutineExceptionReadingSQLdataNotPermitted,
    SQLRoutineExceptionFunctionExecutedNoReturnStatement,
    InvalidCollationName,
    InvalidSQLStatementIdentifier,
    InvalidSQLDescriptorName,
    InvalidCursorName,
    InvalidConditionNumber,
    CursorSensitivityException,
    CursorSensitivityExceptionRequestRejected,
    CursorSensitivityExceptionRequestFailed,
    ExternalRoutineException,
    ExternalRoutineExceptionContainingSQLNotPermitted,
    ExternalRoutineExceptionModifyingSQLdataNotPermitted,
    ExternalRoutineExceptionProhibitedSQLstatementAttempted,
    ExternalRoutineExceptionReadingSQLdataNotPermitted,
    ExternalRoutineInvocationException,
    ExternalRoutineInvocationExceptionNullValueNotAllowed,
    SavepointException,
    SavepointExceptionInvalidSpecification,
    SavepointExceptionTooMany,
    AmbiguousCursorName,
    InvalidCatalogName,
    InvalidSchemaName,
    TransactionRollback,
    TransactionRollbackSerializationFailure,
    TransactionRollbackIntegrityConstraintViolation,
    TransactionRollbackStatementCompletionUnknown,
    TransactionRollbackTriggeredActionException,
    TransactionRollbackDeadlock,
    SyntaxErrorOrAccessRuleViolation,
    SyntaxErrorOrAccessRuleViolationSyntaxError,
    WithCheckOptionViolation,
    ReservedForISO9579,
    IoError,
    NameError,
    NameErrorNotFound,
    NameErrorAmbiguous,
    NameErrorDuplicate,
    NameErrorTooLong,
    DdlError,
    SystemError,
}

impl SqlState {
    /// 5 bytes characters called "SQLSTATE".
    pub fn sqlstate(&self) -> String {
        self.detail().sqlstate()
    }

    /// See [SqlStateCategory](sqlstate_detail::SqlStateCategory)
    pub fn category(&self) -> SqlStateCategory {
        self.detail().category()
    }

    #[allow(non_snake_case)]
    fn detail(&self) -> SqlStateDetail {
        use SqlState::*;

        let class00 = SqlStateClass::new("00", "successful completion");
        let class01 = SqlStateClass::new("01", "warning");
        let class02 = SqlStateClass::new("02", "no data");
        let class07 = SqlStateClass::new("07", "dynamic SQL error");
        let class08 = SqlStateClass::new("08", "connection exception");
        let class09 = SqlStateClass::new("09", "triggered action exception");
        let class0A = SqlStateClass::new("0A", "feature not supported");
        let class0D = SqlStateClass::new("0D", "invalid target type specification");
        let class0E = SqlStateClass::new("0E", "invalid schema name list specification");
        let class0F = SqlStateClass::new("0F", "locator exception");
        let class0L = SqlStateClass::new("0L", "invalid grantor");
        let class0M = SqlStateClass::new("0M", "invalid SQL-invoked procedure reference");
        let class0P = SqlStateClass::new("0P", "invalid role specification");
        let class0S = SqlStateClass::new("0S", "invalid transform group name specification");
        let class0T = SqlStateClass::new("0T", "target table disagrees with cursor specification");
        let class0U = SqlStateClass::new("0U", "attempt to assign to non-updatable column");
        let class0V = SqlStateClass::new("0V", "attempt to assign to ordering column");
        let class0W = SqlStateClass::new(
            "0W",
            "prohibited statement encountered during trigger execution",
        );
        let class0Z = SqlStateClass::new("0Z", "diagnostics exception");
        let class21 = SqlStateClass::new("21", "cardinality violation");
        let class22 = SqlStateClass::new("22", "data exception");
        let class23 = SqlStateClass::new("23", "integrity constraint violation");
        let class24 = SqlStateClass::new("24", "invalid cursor state");
        let class25 = SqlStateClass::new("25", "invalid transaction state");
        let class26 = SqlStateClass::new("26", "invalid SQL statement name");
        let class27 = SqlStateClass::new("27", "triggered data change violation");
        let class28 = SqlStateClass::new("28", "invalid authorization specification");
        let class2B = SqlStateClass::new("2B", "dependent privilege descriptors still exist");
        let class2C = SqlStateClass::new("2C", "invalid character set name");
        let class2D = SqlStateClass::new("2D", "invalid transaction termination");
        let class2E = SqlStateClass::new("2E", "invalid connection name");
        let class2F = SqlStateClass::new("2F", "SQL routine exception");
        let class2H = SqlStateClass::new("2H", "invalid collation name");
        let class30 = SqlStateClass::new("30", "invalid SQL statement identifier");
        let class33 = SqlStateClass::new("33", "invalid SQL descriptor name");
        let class34 = SqlStateClass::new("34", "invalid cursor name");
        let class35 = SqlStateClass::new("35", "invalid condition number");
        let class36 = SqlStateClass::new("36", "cursor sensitivity exception");
        let class38 = SqlStateClass::new("38", "external routine exception");
        let class39 = SqlStateClass::new("39", "external routine invocation exception");
        let class3B = SqlStateClass::new("3B", "savepoint exception");
        let class3C = SqlStateClass::new("3C", "ambiguous cursor name");
        let class3D = SqlStateClass::new("3D", "invalid catalog name");
        let class3F = SqlStateClass::new("3F", "invalid schema name");
        let class40 = SqlStateClass::new("40", "transaction rollback");
        let class42 = SqlStateClass::new("42", "syntax error or access rule violation");
        let class44 = SqlStateClass::new("44", "with check option violation");
        let classHZ = SqlStateClass::new("HZ", "Reserved for ISO9579 (RDA)");

        // apllodb's original error class (class must starts from [I-Z])
        let classIO = SqlStateClass::new("IO", "io error");
        let classNM = SqlStateClass::new("NM", "general name error");
        let classSC = SqlStateClass::new("SC", "DDL error");
        let classSY = SqlStateClass::new("SY", "general system error");

        match self {
            SuccessfulCompletion => SqlStateDetail::new(class00, "000", "(no subclass)"),
            Warning => SqlStateDetail::new(class01, "000", "(no subclass)"),
            WarningCursorOperationConflict => {
                SqlStateDetail::new(class01, "001", "cursor operation conflict")
            }
            WarningDisconnectError => SqlStateDetail::new(class01, "002", "disconnect error"),
            WarningNullValueEliminatedInSetFunction => {
                SqlStateDetail::new(class01, "003", "null value eliminated in set function")
            }
            WarningStringDataRightTruncation => {
                SqlStateDetail::new(class01, "004", "string data, right truncation")
            }
            WarningInsufficientItemDescriptorAreas => {
                SqlStateDetail::new(class01, "005", "insufficient item descriptor areas")
            }
            WarningPrivilegeNotRevoked => {
                SqlStateDetail::new(class01, "006", "privilege not revoked")
            }
            WarningPrivilegeNotGranted => {
                SqlStateDetail::new(class01, "007", "privilege not granted")
            }
            WarningSearchConditionTooLongForInformationSchema => SqlStateDetail::new(
                class01,
                "009",
                "search condition too long for information schema",
            ),
            WarningQueryExpressionTooLongForInformationSchema => SqlStateDetail::new(
                class01,
                "00A",
                "query expression too long for information schema",
            ),
            WarningDefaultValueTooLongForInformationSchema => SqlStateDetail::new(
                class01,
                "00B",
                "default value too long for information schema",
            ),
            WarningResultSetsReturned => {
                SqlStateDetail::new(class01, "00C", "result sets returned")
            }
            WarningAdditionalResultSetsReturned => {
                SqlStateDetail::new(class01, "00D", "additional result sets returned")
            }
            WarningAttemptToReturnTooManyResultSets => {
                SqlStateDetail::new(class01, "00E", "attempt to return too many result sets")
            }
            WarningStatementTooLongForInformationSchema => {
                SqlStateDetail::new(class01, "00F", "statement too long for information schema")
            }
            WarningInvalidNumberOfConditions => {
                SqlStateDetail::new(class01, "012", "invalid number of conditions")
            }
            WarningArrayDataRightTruncation => {
                SqlStateDetail::new(class01, "02F", "array data, right truncation")
            }
            NoData => SqlStateDetail::new(class02, "000", "(no subclass)"),
            NoDataNoAdditionalResultSetsReturned => {
                SqlStateDetail::new(class02, "001", "no additional result sets returned")
            }
            DynamicSQLError => SqlStateDetail::new(class07, "000", "(no subclass)"),
            DynamicSQLErrorUsingClauseDoesNotMatchDynamicParameterSpecifications => {
                SqlStateDetail::new(
                    class07,
                    "001",
                    "using clause does not match dynamic parameter specifications",
                )
            }
            DynamicSQLErrorUsingClauseDoesNotMatchTargetSpecifications => SqlStateDetail::new(
                class07,
                "002",
                "using clause does not match target specifications",
            ),
            DynamicSQLErrorCursorSpecificationCannotBeExecuted => {
                SqlStateDetail::new(class07, "003", "cursor specification cannot be executed")
            }
            DynamicSQLErrorUsingClauseRequiredForDynamicParameters => SqlStateDetail::new(
                class07,
                "004",
                "using clause required for dynamic parameters",
            ),
            DynamicSQLErrorPreparedStatementNotACursorSpecification => SqlStateDetail::new(
                class07,
                "005",
                "prepared statement not a cursor specification",
            ),
            DynamicSQLErrorRestrictedDataTypeAttributeViolation => {
                SqlStateDetail::new(class07, "006", "restricted data type attribute violation")
            }
            DynamicSQLErrorUsingClauseRequiredForResultFields => {
                SqlStateDetail::new(class07, "007", "using clause required for result fields")
            }
            DynamicSQLErrorInvalidDescriptorCount => {
                SqlStateDetail::new(class07, "008", "invalid descriptor count")
            }
            DynamicSQLErrorInvalidDescriptorIndex => {
                SqlStateDetail::new(class07, "009", "invalid descriptor index")
            }
            DynamicSQLErrorDataTypeTransformFunctionViolation => {
                SqlStateDetail::new(class07, "00B", "data type transform function violation")
            }
            DynamicSQLErrorUndefinedDATAValue => {
                SqlStateDetail::new(class07, "00C", "undefined DATA value")
            }
            DynamicSQLErrorInvalidDATATarget => {
                SqlStateDetail::new(class07, "00D", "invalid DATA target")
            }
            DynamicSQLErrorInvalidLEVELValue => {
                SqlStateDetail::new(class07, "00E", "invalid LEVEL value")
            }
            DynamicSQLErrorInvalidDatetimeIntervalCode => {
                SqlStateDetail::new(class07, "00F", "invalid DATETIME_INTERVAL_CODE")
            }
            ConnectionException => SqlStateDetail::new(class08, "000", "(no subclass)"),
            ConnectionExceptionSQLclientUnableToEstablishSQLconnection => SqlStateDetail::new(
                class08,
                "001",
                "SQL-client unable to establish SQL-connection",
            ),
            ConnectionExceptionConnectionNameInUse => {
                SqlStateDetail::new(class08, "002", "connection name in use")
            }
            ConnectionExceptionConnectionDoesNotExist => {
                SqlStateDetail::new(class08, "003", "connection does not exist")
            }
            ConnectionExceptionSQLserverRejectedEstablishmentOfSQLconnection => {
                SqlStateDetail::new(
                    class08,
                    "004",
                    "SQL-server rejected establishment of SQL-connection",
                )
            }
            ConnectionExceptionConnectionFailure => {
                SqlStateDetail::new(class08, "006", "connection failure")
            }
            ConnectionExceptionTransactionResolutionUnknown => {
                SqlStateDetail::new(class08, "007", "transaction resolution unknown")
            }
            ConnectionExceptionDatabaseNotOpen => {
                SqlStateDetail::new(class08, "I00", "database not open")
            }
            ConnectionExceptionDatabaseAlreadyOpen => {
                SqlStateDetail::new(class08, "I01", "database already open")
            }
            TriggeredActionException => SqlStateDetail::new(class09, "000", "(no subclass)"),
            FeatureNotSupported => SqlStateDetail::new(class0A, "000", "(no subclass)"),
            FeatureNotSupportedMultipleServerTransactions => {
                SqlStateDetail::new(class0A, "001", "multiple server transactions")
            }
            InvalidTargetTypeSpecification => SqlStateDetail::new(class0D, "000", "(no subclass)"),
            InvalidSchemaNameListSpecification => {
                SqlStateDetail::new(class0E, "000", "(no subclass)")
            }
            LocatorException => SqlStateDetail::new(class0F, "000", "(no subclass)"),
            LocatorExceptionInvalidSpecification => {
                SqlStateDetail::new(class0F, "001", "invalid specification")
            }
            InvalidGrantor => SqlStateDetail::new(class0L, "000", "(no subclass)"),
            InvalidSQLinvokedProcedureReference => {
                SqlStateDetail::new(class0M, "000", "(no subclass)")
            }
            InvalidRoleSpecification => SqlStateDetail::new(class0P, "000", "(no subclass)"),
            InvalidTransformGroupNameSpecification => {
                SqlStateDetail::new(class0S, "000", "(no subclass)")
            }
            TargetTableDisagreesWithCursorSpecification => {
                SqlStateDetail::new(class0T, "000", "(no subclass)")
            }
            AttemptToAssignToNonupdatableColumn => {
                SqlStateDetail::new(class0U, "000", "(no subclass)")
            }
            AttemptToAssignToOrderingColumn => SqlStateDetail::new(class0V, "000", "(no subclass)"),
            ProhibitedStatementEncounteredDuringTriggerExecution => {
                SqlStateDetail::new(class0W, "000", "(no subclass)")
            }
            ProhibitedStatementEncounteredDuringTriggerExecutionModifyTableModifiedByDataChangeDeltaTable => {
                SqlStateDetail::new(
                    class0W,
                    "001",
                    "modify table modified by data change delta table",
                )
            }
            DiagnosticsException => SqlStateDetail::new(class0Z, "000", "(no subclass)"),
            DiagnosticsExceptionMaximumNumberOfStackedDiagnosticsAreasExceeded => {
                SqlStateDetail::new(
                    class0Z,
                    "001",
                    "maximum number of stacked diagnostics areas exceeded",
                )
            }
            CardinalityViolation => SqlStateDetail::new(class21, "000", "(no subclass)"),
            DataException => SqlStateDetail::new(class22, "000", "(no subclass)"),
            DataExceptionStringDataRightTruncation => {
                SqlStateDetail::new(class22, "001", "string data, right truncation")
            }
            DataExceptionNullValueNoIndicatorParameter => {
                SqlStateDetail::new(class22, "002", "null value, no indicator parameter")
            }
            DataExceptionNumericValueOutOfRange => {
                SqlStateDetail::new(class22, "003", "numeric value out of range")
            }
            DataExceptionNullValueNotAllowed => {
                SqlStateDetail::new(class22, "004", "null value not allowed")
            }
            DataExceptionErrorInAssignment => {
                SqlStateDetail::new(class22, "005", "error in assignment")
            }
            DataExceptionInvalidIntervalFormat => {
                SqlStateDetail::new(class22, "006", "invalid interval format")
            }
            DataExceptionInvalidDatetimeFormat => {
                SqlStateDetail::new(class22, "007", "invalid datetime format")
            }
            DataExceptionDatetimeFieldOverflow => {
                SqlStateDetail::new(class22, "008", "datetime field overflow")
            }
            DataExceptionInvalidTimeZoneDisplacementValue => {
                SqlStateDetail::new(class22, "009", "invalid time zone displacement value")
            }
            DataExceptionEscapeCharacterConflict => {
                SqlStateDetail::new(class22, "00B", "escape character conflict")
            }
            DataExceptionInvalidUseOfEscapeCharacter => {
                SqlStateDetail::new(class22, "00C", "invalid use of escape character")
            }
            DataExceptionInvalidEscapeOctet => {
                SqlStateDetail::new(class22, "00D", "invalid escape octet")
            }
            DataExceptionNullValueInArrayTarget => {
                SqlStateDetail::new(class22, "00E", "null value in array target")
            }
            DataExceptionZerolengthCharacterString => {
                SqlStateDetail::new(class22, "00F", "zero-length character string")
            }
            DataExceptionMostSpecificTypeMismatch => {
                SqlStateDetail::new(class22, "00G", "most specific type mismatch")
            }
            DataExceptionSequenceGeneratorLimitExceeded => {
                SqlStateDetail::new(class22, "00H", "sequence generator limit exceeded")
            }
            DataExceptionIntervalValueOutOfRange => {
                SqlStateDetail::new(class22, "00P", "interval value out of range")
            }
            DataExceptionMultisetValueOverflow => {
                SqlStateDetail::new(class22, "00Q", "multiset value overflow")
            }
            DataExceptionInvalidIndicatorParameterValue => {
                SqlStateDetail::new(class22, "010", "invalid indicator parameter value")
            }
            DataExceptionSubstringError => SqlStateDetail::new(class22, "011", "substring error"),
            DataExceptionDivisionByZero => SqlStateDetail::new(class22, "012", "division by zero"),
            DataExceptionInvalidPrecedingOrFollowingSizeInWindowFunction => SqlStateDetail::new(
                class22,
                "013",
                "invalid preceding or following size in window function",
            ),
            DataExceptionInvalidArgumentForNTILEFunction => {
                SqlStateDetail::new(class22, "014", "invalid argument for NTILE function")
            }
            DataExceptionIntervalFieldOverflow => {
                SqlStateDetail::new(class22, "015", "interval field overflow")
            }
            DataExceptionInvalidArgumentForNthValueFunction => {
                SqlStateDetail::new(class22, "016", "invalid argument for NTH_VALUE function")
            }
            DataExceptionInvalidCharacterValueForCast => {
                SqlStateDetail::new(class22, "018", "invalid character value for cast")
            }
            DataExceptionInvalidEscapeCharacter => {
                SqlStateDetail::new(class22, "019", "invalid escape character")
            }
            DataExceptionInvalidRegularExpression => {
                SqlStateDetail::new(class22, "01B", "invalid regular expression")
            }
            DataExceptionNullRowNotPermittedInTable => {
                SqlStateDetail::new(class22, "01C", "null row not permitted in table")
            }
            DataExceptionInvalidArgumentForNaturalLogarithm => {
                SqlStateDetail::new(class22, "01E", "invalid argument for natural logarithm")
            }
            DataExceptionInvalidArgumentForPowerFunction => {
                SqlStateDetail::new(class22, "01F", "invalid argument for power function")
            }
            DataExceptionInvalidArgumentForWidthBucketFunction => {
                SqlStateDetail::new(class22, "01G", "invalid argument for width bucket function")
            }
            DataExceptionInvalidRowVersion => {
                SqlStateDetail::new(class22, "01H", "invalid row version")
            }
            DataExceptionInvalidQueryRegularExpression => {
                SqlStateDetail::new(class22, "01S", "invalid Query regular expression")
            }
            DataExceptionInvalidQueryOptionFlag => {
                SqlStateDetail::new(class22, "01T", "invalid Query option flag")
            }
            DataExceptionAttemptToReplaceAZerolengthString => {
                SqlStateDetail::new(class22, "01U", "attempt to replace a zero-length string")
            }
            DataExceptionInvalidQueryReplacementString => {
                SqlStateDetail::new(class22, "01V", "invalid Query replacement string")
            }
            DataExceptionInvalidRowCountInFetchFirstClause => {
                SqlStateDetail::new(class22, "01W", "invalid row count in fetch first clause")
            }
            DataExceptionInvalidRowCountInResultOffsetClause => {
                SqlStateDetail::new(class22, "01X", "invalid row count in result offset clause")
            }
            DataExceptionCharacterNotInRepertoire => {
                SqlStateDetail::new(class22, "021", "character not in repertoire")
            }
            DataExceptionIndicatorOverflow => {
                SqlStateDetail::new(class22, "022", "indicator overflow")
            }
            DataExceptionInvalidParameterValue => {
                SqlStateDetail::new(class22, "023", "invalid parameter value")
            }
            DataExceptionUnterminatedCString => {
                SqlStateDetail::new(class22, "024", "unterminated C string")
            }
            DataExceptionInvalidEscapeSequence => {
                SqlStateDetail::new(class22, "025", "invalid escape sequence")
            }
            DataExceptionStringDataLengthMismatch => {
                SqlStateDetail::new(class22, "026", "string data, length mismatch")
            }
            DataExceptionTrimError => SqlStateDetail::new(class22, "027", "trim error"),
            DataExceptionNoncharacterInUCSString => {
                SqlStateDetail::new(class22, "029", "noncharacter in UCS string")
            }
            DataExceptionNullValueSubstitutedForMutatorSubjectParameter => SqlStateDetail::new(
                class22,
                "02D",
                "null value substituted for mutator subject parameter",
            ),
            DataExceptionArrayElementError => {
                SqlStateDetail::new(class22, "02E", "array element error")
            }
            DataExceptionArrayDataRightTruncation => {
                SqlStateDetail::new(class22, "02F", "array data, right truncation")
            }
            DataExceptionInvalidRepeatArgumentInASampleClause => {
                SqlStateDetail::new(class22, "02G", "invalid repeat argument in a sample clause")
            }
            DataExceptionInvalidSampleSize => {
                SqlStateDetail::new(class22, "02H", "invalid sample size")
            }
            DataExceptionIllegalConversion => {
                SqlStateDetail::new(class22, "I00", "illegal data conversion")
            }
            DataExceptionIllegalComparison => {
                SqlStateDetail::new(class22, "I01", "illegal data comparison")
            }
            DataExceptionIllegalOperation => {
                SqlStateDetail::new(class22, "I02", "illegal operation to data")
            }
            IntegrityConstraintViolation => SqlStateDetail::new(class23, "000", "(no subclass)"),
            IntegrityConstraintViolationRestrictViolation => {
                SqlStateDetail::new(class23, "001", "restrict violation")
            }
            IntegrityConstraintNotNullViolation => {
                SqlStateDetail::new(class23, "I00", "not null violation")
            }
            IntegrityConstraintUniqueViolation => {
                SqlStateDetail::new(class23, "I01", "unique violation")
            }
            InvalidCursorState => SqlStateDetail::new(class24, "000", "(no subclass)"),
            InvalidTransactionState => SqlStateDetail::new(class25, "000", "(no subclass)"),
            InvalidTransactionStateActiveSQLtransaction => {
                SqlStateDetail::new(class25, "001", "active SQL-transaction")
            }
            InvalidTransactionStateBranchTransactionAlreadyActive => {
                SqlStateDetail::new(class25, "002", "branch transaction already active")
            }
            InvalidTransactionStateInappropriateAccessModeForBranchTransaction => {
                SqlStateDetail::new(
                    class25,
                    "003",
                    "inappropriate access mode for branch transaction",
                )
            }
            InvalidTransactionStateInappropriateIsolationLevelForBranchTransaction => {
                SqlStateDetail::new(
                    class25,
                    "004",
                    "inappropriate isolation level for branch transaction",
                )
            }
            InvalidTransactionStateNoActiveSQLtransactionForBranchTransaction => {
                SqlStateDetail::new(
                    class25,
                    "005",
                    "no active SQL-transaction for branch transaction",
                )
            }
            InvalidTransactionStateReadonlySQLtransaction => {
                SqlStateDetail::new(class25, "006", "read-only SQL-transaction")
            }
            InvalidTransactionStateSchemaAndDataStatementMixingNotSupported => SqlStateDetail::new(
                class25,
                "007",
                "schema and data statement mixing not supported",
            ),
            InvalidTransactionStateHeldCursorRequiresSameIsolationLevel => {
                SqlStateDetail::new(class25, "008", "held cursor requires same isolation level")
            }
            InvalidSQLStatementName => SqlStateDetail::new(class26, "000", "(no subclass)"),
            TriggeredDataChangeViolation => SqlStateDetail::new(class27, "000", "(no subclass)"),
            TriggeredDataChangeViolationModifyTableModifiedByDataChangeDeltaTable => {
                SqlStateDetail::new(
                    class27,
                    "001",
                    "modify table modified by data change delta table",
                )
            }
            InvalidAuthorizationSpecification => {
                SqlStateDetail::new(class28, "000", "(no subclass)")
            }
            DependentPrivilegeDescriptorsStillExist => {
                SqlStateDetail::new(class2B, "000", "(no subclass)")
            }
            InvalidCharacterSetName => SqlStateDetail::new(class2C, "000", "(no subclass)"),
            InvalidTransactionTermination => SqlStateDetail::new(class2D, "000", "(no subclass)"),
            InvalidConnectionName => SqlStateDetail::new(class2E, "000", "(no subclass)"),
            SQLRoutineException => SqlStateDetail::new(class2F, "000", "(no subclass)"),
            SQLRoutineExceptionModifyingSQLdataNotPermitted => {
                SqlStateDetail::new(class2F, "002", "modifying SQL-data not permitted")
            }
            SQLRoutineExceptionProhibitedSQLstatementAttempted => {
                SqlStateDetail::new(class2F, "003", "prohibited SQL-statement attempted")
            }
            SQLRoutineExceptionReadingSQLdataNotPermitted => {
                SqlStateDetail::new(class2F, "004", "reading SQL-data not permitted")
            }
            SQLRoutineExceptionFunctionExecutedNoReturnStatement => {
                SqlStateDetail::new(class2F, "005", "function executed no return statement")
            }
            InvalidCollationName => SqlStateDetail::new(class2H, "000", "(no subclass)"),
            InvalidSQLStatementIdentifier => SqlStateDetail::new(class30, "000", "(no subclass)"),
            InvalidSQLDescriptorName => SqlStateDetail::new(class33, "000", "(no subclass)"),
            InvalidCursorName => SqlStateDetail::new(class34, "000", "(no subclass)"),
            InvalidConditionNumber => SqlStateDetail::new(class35, "000", "(no subclass)"),
            CursorSensitivityException => SqlStateDetail::new(class36, "000", "(no subclass)"),
            CursorSensitivityExceptionRequestRejected => {
                SqlStateDetail::new(class36, "001", "request rejected")
            }
            CursorSensitivityExceptionRequestFailed => {
                SqlStateDetail::new(class36, "002", "request failed")
            }
            ExternalRoutineException => SqlStateDetail::new(class38, "000", "(no subclass)"),
            ExternalRoutineExceptionContainingSQLNotPermitted => {
                SqlStateDetail::new(class38, "001", "containing SQL not permitted")
            }
            ExternalRoutineExceptionModifyingSQLdataNotPermitted => {
                SqlStateDetail::new(class38, "002", "modifying SQL-data not permitted")
            }
            ExternalRoutineExceptionProhibitedSQLstatementAttempted => {
                SqlStateDetail::new(class38, "003", "prohibited SQL-statement attempted")
            }
            ExternalRoutineExceptionReadingSQLdataNotPermitted => {
                SqlStateDetail::new(class38, "004", "reading SQL-data not permitted")
            }
            ExternalRoutineInvocationException => {
                SqlStateDetail::new(class39, "000", "(no subclass)")
            }
            ExternalRoutineInvocationExceptionNullValueNotAllowed => {
                SqlStateDetail::new(class39, "004", "null value not allowed")
            }
            SavepointException => SqlStateDetail::new(class3B, "000", "(no subclass)"),
            SavepointExceptionInvalidSpecification => {
                SqlStateDetail::new(class3B, "001", "invalid specification")
            }
            SavepointExceptionTooMany => SqlStateDetail::new(class3B, "002", "too many"),
            AmbiguousCursorName => SqlStateDetail::new(class3C, "000", "(no subclass)"),
            InvalidCatalogName => SqlStateDetail::new(class3D, "000", "(no subclass)"),
            InvalidSchemaName => SqlStateDetail::new(class3F, "000", "(no subclass)"),
            TransactionRollback => SqlStateDetail::new(class40, "000", "(no subclass)"),
            TransactionRollbackSerializationFailure => {
                SqlStateDetail::new(class40, "001", "serialization failure")
            }
            TransactionRollbackIntegrityConstraintViolation => {
                SqlStateDetail::new(class40, "002", "integrity constraint violation")
            }
            TransactionRollbackStatementCompletionUnknown => {
                SqlStateDetail::new(class40, "003", "statement completion unknown")
            }
            TransactionRollbackTriggeredActionException => {
                SqlStateDetail::new(class40, "004", "triggered action exception")
            }
            TransactionRollbackDeadlock => SqlStateDetail::new(class40, "I00", "deadlock detected"),
            SyntaxErrorOrAccessRuleViolation => {
                SqlStateDetail::new(class42, "000", "(no subclass)")
            }
            SyntaxErrorOrAccessRuleViolationSyntaxError => {
                SqlStateDetail::new(class42, "I00", "syntax error")
            }
            WithCheckOptionViolation => SqlStateDetail::new(class44, "000", "(no subclass)"),
            ReservedForISO9579 => SqlStateDetail::new(classHZ, "???", ""),
            IoError => SqlStateDetail::new(classIO, "000", "(no subclass)"),
            NameError => SqlStateDetail::new(classNM, "000", "(no subclass)"),
            NameErrorNotFound => SqlStateDetail::new(classNM, "001", "not found by name"),
            NameErrorAmbiguous => SqlStateDetail::new(classNM, "002", "ambiguous name"),
            NameErrorDuplicate => SqlStateDetail::new(classNM, "003", "duplicate name"),
            NameErrorTooLong => SqlStateDetail::new(classNM, "004", "too long name"),
            DdlError => SqlStateDetail::new(classSC, "000", "(no subclass)"),
            SystemError => SqlStateDetail::new(classSY, "000", "(no subclass)"),
        }
    }
}

impl Display for SqlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let detail = self.detail();

        write!(
            f,
            r#"SQLSTATE: "{}", Category: "{}", ClassText: "{}", SubclassText: "{}""#,
            detail.sqlstate(),
            detail.category(),
            &detail.class.class_text,
            &detail.subclass_text,
        )
    }
}
