mod sqlstate_detail;

use std::{fmt::Display, sync::Arc};

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

    fn detail(&self) -> SqlStateDetail {
        use SqlState::*;

        let class00 = Arc::new(SqlStateClass::new("00", "successful completion"));
        let class01 = Arc::new(SqlStateClass::new("01", "warning"));
        let class02 = Arc::new(SqlStateClass::new("02", "no data"));
        let class07 = Arc::new(SqlStateClass::new("07", "dynamic SQL error"));
        let class08 = Arc::new(SqlStateClass::new("08", "connection exception"));
        let class09 = Arc::new(SqlStateClass::new("09", "triggered action exception"));
        let class0A = Arc::new(SqlStateClass::new("0A", "feature not supported"));
        let class0D = Arc::new(SqlStateClass::new(
            "0D",
            "invalid target type specification",
        ));
        let class0E = Arc::new(SqlStateClass::new(
            "0E",
            "invalid schema name list specification",
        ));
        let class0F = Arc::new(SqlStateClass::new("0F", "locator exception"));
        let class0L = Arc::new(SqlStateClass::new("0L", "invalid grantor"));
        let class0M = Arc::new(SqlStateClass::new(
            "0M",
            "invalid SQL-invoked procedure reference",
        ));
        let class0P = Arc::new(SqlStateClass::new("0P", "invalid role specification"));
        let class0S = Arc::new(SqlStateClass::new(
            "0S",
            "invalid transform group name specification",
        ));
        let class0T = Arc::new(SqlStateClass::new(
            "0T",
            "target table disagrees with cursor specification",
        ));
        let class0U = Arc::new(SqlStateClass::new(
            "0U",
            "attempt to assign to non-updatable column",
        ));
        let class0V = Arc::new(SqlStateClass::new(
            "0V",
            "attempt to assign to ordering column",
        ));
        let class0W = Arc::new(SqlStateClass::new(
            "0W",
            "prohibited statement encountered during trigger execution",
        ));
        let class0Z = Arc::new(SqlStateClass::new("0Z", "diagnostics exception"));
        let class21 = Arc::new(SqlStateClass::new("21", "cardinality violation"));
        let class22 = Arc::new(SqlStateClass::new("22", "data exception"));
        let class23 = Arc::new(SqlStateClass::new("23", "integrity constraint violation"));
        let class24 = Arc::new(SqlStateClass::new("24", "invalid cursor state"));
        let class25 = Arc::new(SqlStateClass::new("25", "invalid transaction state"));
        let class26 = Arc::new(SqlStateClass::new("26", "invalid SQL statement name"));
        let class27 = Arc::new(SqlStateClass::new("27", "triggered data change violation"));
        let class28 = Arc::new(SqlStateClass::new(
            "28",
            "invalid authorization specification",
        ));
        let class2B = Arc::new(SqlStateClass::new(
            "2B",
            "dependent privilege descriptors still exist",
        ));
        let class2C = Arc::new(SqlStateClass::new("2C", "invalid character set name"));
        let class2D = Arc::new(SqlStateClass::new("2D", "invalid transaction termination"));
        let class2E = Arc::new(SqlStateClass::new("2E", "invalid connection name"));
        let class2F = Arc::new(SqlStateClass::new("2F", "SQL routine exception"));
        let class2H = Arc::new(SqlStateClass::new("2H", "invalid collation name"));
        let class30 = Arc::new(SqlStateClass::new("30", "invalid SQL statement identifier"));
        let class33 = Arc::new(SqlStateClass::new("33", "invalid SQL descriptor name"));
        let class34 = Arc::new(SqlStateClass::new("34", "invalid cursor name"));
        let class35 = Arc::new(SqlStateClass::new("35", "invalid condition number"));
        let class36 = Arc::new(SqlStateClass::new("36", "cursor sensitivity exception"));
        let class38 = Arc::new(SqlStateClass::new("38", "external routine exception"));
        let class39 = Arc::new(SqlStateClass::new(
            "39",
            "external routine invocation exception",
        ));
        let class3B = Arc::new(SqlStateClass::new("3B", "savepoint exception"));
        let class3C = Arc::new(SqlStateClass::new("3C", "ambiguous cursor name"));
        let class3D = Arc::new(SqlStateClass::new("3D", "invalid catalog name"));
        let class3F = Arc::new(SqlStateClass::new("3F", "invalid schema name"));
        let class40 = Arc::new(SqlStateClass::new("40", "transaction rollback"));
        let class42 = Arc::new(SqlStateClass::new(
            "42",
            "syntax error or access rule violation",
        ));
        let class44 = Arc::new(SqlStateClass::new("44", "with check option violation"));
        let classHZ = Arc::new(SqlStateClass::new("HZ", "Reserved for ISO9579 (RDA)"));

        // apllodb's original error class (class must starts from [I-Z])
        let classIO = Arc::new(SqlStateClass::new("IO", "io error"));
        let classNM = Arc::new(SqlStateClass::new("NM", "general name error"));
        let classSC = Arc::new(SqlStateClass::new("SC", "DDL error"));
        let classSY = Arc::new(SqlStateClass::new("SY", "general system error"));

        match self {
            SuccessfulCompletion => SqlStateDetail::new(class00.clone(), "000", "(no subclass)"),
            Warning => SqlStateDetail::new(class01.clone(), "000", "(no subclass)"),
            WarningCursorOperationConflict => {
                SqlStateDetail::new(class01.clone(), "001", "cursor operation conflict")
            }
            WarningDisconnectError => {
                SqlStateDetail::new(class01.clone(), "002", "disconnect error")
            }
            WarningNullValueEliminatedInSetFunction => SqlStateDetail::new(
                class01.clone(),
                "003",
                "null value eliminated in set function",
            ),
            WarningStringDataRightTruncation => {
                SqlStateDetail::new(class01.clone(), "004", "string data, right truncation")
            }
            WarningInsufficientItemDescriptorAreas => {
                SqlStateDetail::new(class01.clone(), "005", "insufficient item descriptor areas")
            }
            WarningPrivilegeNotRevoked => {
                SqlStateDetail::new(class01.clone(), "006", "privilege not revoked")
            }
            WarningPrivilegeNotGranted => {
                SqlStateDetail::new(class01.clone(), "007", "privilege not granted")
            }
            WarningSearchConditionTooLongForInformationSchema => SqlStateDetail::new(
                class01.clone(),
                "009",
                "search condition too long for information schema",
            ),
            WarningQueryExpressionTooLongForInformationSchema => SqlStateDetail::new(
                class01.clone(),
                "00A",
                "query expression too long for information schema",
            ),
            WarningDefaultValueTooLongForInformationSchema => SqlStateDetail::new(
                class01.clone(),
                "00B",
                "default value too long for information schema",
            ),
            WarningResultSetsReturned => {
                SqlStateDetail::new(class01.clone(), "00C", "result sets returned")
            }
            WarningAdditionalResultSetsReturned => {
                SqlStateDetail::new(class01.clone(), "00D", "additional result sets returned")
            }
            WarningAttemptToReturnTooManyResultSets => SqlStateDetail::new(
                class01.clone(),
                "00E",
                "attempt to return too many result sets",
            ),
            WarningStatementTooLongForInformationSchema => SqlStateDetail::new(
                class01.clone(),
                "00F",
                "statement too long for information schema",
            ),
            WarningInvalidNumberOfConditions => {
                SqlStateDetail::new(class01.clone(), "012", "invalid number of conditions")
            }
            WarningArrayDataRightTruncation => {
                SqlStateDetail::new(class01.clone(), "02F", "array data, right truncation")
            }
            NoData => SqlStateDetail::new(class02.clone(), "000", "(no subclass)"),
            NoDataNoAdditionalResultSetsReturned => {
                SqlStateDetail::new(class02.clone(), "001", "no additional result sets returned")
            }
            DynamicSQLError => SqlStateDetail::new(class07.clone(), "000", "(no subclass)"),
            DynamicSQLErrorUsingClauseDoesNotMatchDynamicParameterSpecifications => {
                SqlStateDetail::new(
                    class07.clone(),
                    "001",
                    "using clause does not match dynamic parameter specifications",
                )
            }
            DynamicSQLErrorUsingClauseDoesNotMatchTargetSpecifications => SqlStateDetail::new(
                class07.clone(),
                "002",
                "using clause does not match target specifications",
            ),
            DynamicSQLErrorCursorSpecificationCannotBeExecuted => SqlStateDetail::new(
                class07.clone(),
                "003",
                "cursor specification cannot be executed",
            ),
            DynamicSQLErrorUsingClauseRequiredForDynamicParameters => SqlStateDetail::new(
                class07.clone(),
                "004",
                "using clause required for dynamic parameters",
            ),
            DynamicSQLErrorPreparedStatementNotACursorSpecification => SqlStateDetail::new(
                class07.clone(),
                "005",
                "prepared statement not a cursor specification",
            ),
            DynamicSQLErrorRestrictedDataTypeAttributeViolation => SqlStateDetail::new(
                class07.clone(),
                "006",
                "restricted data type attribute violation",
            ),
            DynamicSQLErrorUsingClauseRequiredForResultFields => SqlStateDetail::new(
                class07.clone(),
                "007",
                "using clause required for result fields",
            ),
            DynamicSQLErrorInvalidDescriptorCount => {
                SqlStateDetail::new(class07.clone(), "008", "invalid descriptor count")
            }
            DynamicSQLErrorInvalidDescriptorIndex => {
                SqlStateDetail::new(class07.clone(), "009", "invalid descriptor index")
            }
            DynamicSQLErrorDataTypeTransformFunctionViolation => SqlStateDetail::new(
                class07.clone(),
                "00B",
                "data type transform function violation",
            ),
            DynamicSQLErrorUndefinedDATAValue => {
                SqlStateDetail::new(class07.clone(), "00C", "undefined DATA value")
            }
            DynamicSQLErrorInvalidDATATarget => {
                SqlStateDetail::new(class07.clone(), "00D", "invalid DATA target")
            }
            DynamicSQLErrorInvalidLEVELValue => {
                SqlStateDetail::new(class07.clone(), "00E", "invalid LEVEL value")
            }
            DynamicSQLErrorInvalidDatetimeIntervalCode => {
                SqlStateDetail::new(class07.clone(), "00F", "invalid DATETIME_INTERVAL_CODE")
            }
            ConnectionException => SqlStateDetail::new(class08.clone(), "000", "(no subclass)"),
            ConnectionExceptionSQLclientUnableToEstablishSQLconnection => SqlStateDetail::new(
                class08.clone(),
                "001",
                "SQL-client unable to establish SQL-connection",
            ),
            ConnectionExceptionConnectionNameInUse => {
                SqlStateDetail::new(class08.clone(), "002", "connection name in use")
            }
            ConnectionExceptionConnectionDoesNotExist => {
                SqlStateDetail::new(class08.clone(), "003", "connection does not exist")
            }
            ConnectionExceptionSQLserverRejectedEstablishmentOfSQLconnection => {
                SqlStateDetail::new(
                    class08.clone(),
                    "004",
                    "SQL-server rejected establishment of SQL-connection",
                )
            }
            ConnectionExceptionConnectionFailure => {
                SqlStateDetail::new(class08.clone(), "006", "connection failure")
            }
            ConnectionExceptionTransactionResolutionUnknown => {
                SqlStateDetail::new(class08.clone(), "007", "transaction resolution unknown")
            }
            TriggeredActionException => {
                SqlStateDetail::new(class09.clone(), "000", "(no subclass)")
            }
            FeatureNotSupported => SqlStateDetail::new(class0A.clone(), "000", "(no subclass)"),
            FeatureNotSupportedMultipleServerTransactions => {
                SqlStateDetail::new(class0A.clone(), "001", "multiple server transactions")
            }
            InvalidTargetTypeSpecification => {
                SqlStateDetail::new(class0D.clone(), "000", "(no subclass)")
            }
            InvalidSchemaNameListSpecification => {
                SqlStateDetail::new(class0E.clone(), "000", "(no subclass)")
            }
            LocatorException => SqlStateDetail::new(class0F.clone(), "000", "(no subclass)"),
            LocatorExceptionInvalidSpecification => {
                SqlStateDetail::new(class0F.clone(), "001", "invalid specification")
            }
            InvalidGrantor => SqlStateDetail::new(class0L.clone(), "000", "(no subclass)"),
            InvalidSQLinvokedProcedureReference => {
                SqlStateDetail::new(class0M.clone(), "000", "(no subclass)")
            }
            InvalidRoleSpecification => {
                SqlStateDetail::new(class0P.clone(), "000", "(no subclass)")
            }
            InvalidTransformGroupNameSpecification => {
                SqlStateDetail::new(class0S.clone(), "000", "(no subclass)")
            }
            TargetTableDisagreesWithCursorSpecification => {
                SqlStateDetail::new(class0T.clone(), "000", "(no subclass)")
            }
            AttemptToAssignToNonupdatableColumn => {
                SqlStateDetail::new(class0U.clone(), "000", "(no subclass)")
            }
            AttemptToAssignToOrderingColumn => {
                SqlStateDetail::new(class0V.clone(), "000", "(no subclass)")
            }
            ProhibitedStatementEncounteredDuringTriggerExecution => {
                SqlStateDetail::new(class0W.clone(), "000", "(no subclass)")
            }
            ProhibitedStatementEncounteredDuringTriggerExecutionModifyTableModifiedByDataChangeDeltaTable => {
                SqlStateDetail::new(
                    class0W.clone(),
                    "001",
                    "modify table modified by data change delta table",
                )
            }
            DiagnosticsException => SqlStateDetail::new(class0Z.clone(), "000", "(no subclass)"),
            DiagnosticsExceptionMaximumNumberOfStackedDiagnosticsAreasExceeded => {
                SqlStateDetail::new(
                    class0Z.clone(),
                    "001",
                    "maximum number of stacked diagnostics areas exceeded",
                )
            }
            CardinalityViolation => SqlStateDetail::new(class21.clone(), "000", "(no subclass)"),
            DataException => SqlStateDetail::new(class22.clone(), "000", "(no subclass)"),
            DataExceptionStringDataRightTruncation => {
                SqlStateDetail::new(class22.clone(), "001", "string data, right truncation")
            }
            DataExceptionNullValueNoIndicatorParameter => {
                SqlStateDetail::new(class22.clone(), "002", "null value, no indicator parameter")
            }
            DataExceptionNumericValueOutOfRange => {
                SqlStateDetail::new(class22.clone(), "003", "numeric value out of range")
            }
            DataExceptionNullValueNotAllowed => {
                SqlStateDetail::new(class22.clone(), "004", "null value not allowed")
            }
            DataExceptionErrorInAssignment => {
                SqlStateDetail::new(class22.clone(), "005", "error in assignment")
            }
            DataExceptionInvalidIntervalFormat => {
                SqlStateDetail::new(class22.clone(), "006", "invalid interval format")
            }
            DataExceptionInvalidDatetimeFormat => {
                SqlStateDetail::new(class22.clone(), "007", "invalid datetime format")
            }
            DataExceptionDatetimeFieldOverflow => {
                SqlStateDetail::new(class22.clone(), "008", "datetime field overflow")
            }
            DataExceptionInvalidTimeZoneDisplacementValue => SqlStateDetail::new(
                class22.clone(),
                "009",
                "invalid time zone displacement value",
            ),
            DataExceptionEscapeCharacterConflict => {
                SqlStateDetail::new(class22.clone(), "00B", "escape character conflict")
            }
            DataExceptionInvalidUseOfEscapeCharacter => {
                SqlStateDetail::new(class22.clone(), "00C", "invalid use of escape character")
            }
            DataExceptionInvalidEscapeOctet => {
                SqlStateDetail::new(class22.clone(), "00D", "invalid escape octet")
            }
            DataExceptionNullValueInArrayTarget => {
                SqlStateDetail::new(class22.clone(), "00E", "null value in array target")
            }
            DataExceptionZerolengthCharacterString => {
                SqlStateDetail::new(class22.clone(), "00F", "zero-length character string")
            }
            DataExceptionMostSpecificTypeMismatch => {
                SqlStateDetail::new(class22.clone(), "00G", "most specific type mismatch")
            }
            DataExceptionSequenceGeneratorLimitExceeded => {
                SqlStateDetail::new(class22.clone(), "00H", "sequence generator limit exceeded")
            }
            DataExceptionIntervalValueOutOfRange => {
                SqlStateDetail::new(class22.clone(), "00P", "interval value out of range")
            }
            DataExceptionMultisetValueOverflow => {
                SqlStateDetail::new(class22.clone(), "00Q", "multiset value overflow")
            }
            DataExceptionInvalidIndicatorParameterValue => {
                SqlStateDetail::new(class22.clone(), "010", "invalid indicator parameter value")
            }
            DataExceptionSubstringError => {
                SqlStateDetail::new(class22.clone(), "011", "substring error")
            }
            DataExceptionDivisionByZero => {
                SqlStateDetail::new(class22.clone(), "012", "division by zero")
            }
            DataExceptionInvalidPrecedingOrFollowingSizeInWindowFunction => SqlStateDetail::new(
                class22.clone(),
                "013",
                "invalid preceding or following size in window function",
            ),
            DataExceptionInvalidArgumentForNTILEFunction => SqlStateDetail::new(
                class22.clone(),
                "014",
                "invalid argument for NTILE function",
            ),
            DataExceptionIntervalFieldOverflow => {
                SqlStateDetail::new(class22.clone(), "015", "interval field overflow")
            }
            DataExceptionInvalidArgumentForNthValueFunction => SqlStateDetail::new(
                class22.clone(),
                "016",
                "invalid argument for NTH_VALUE function",
            ),
            DataExceptionInvalidCharacterValueForCast => {
                SqlStateDetail::new(class22.clone(), "018", "invalid character value for cast")
            }
            DataExceptionInvalidEscapeCharacter => {
                SqlStateDetail::new(class22.clone(), "019", "invalid escape character")
            }
            DataExceptionInvalidRegularExpression => {
                SqlStateDetail::new(class22.clone(), "01B", "invalid regular expression")
            }
            DataExceptionNullRowNotPermittedInTable => {
                SqlStateDetail::new(class22.clone(), "01C", "null row not permitted in table")
            }
            DataExceptionInvalidArgumentForNaturalLogarithm => SqlStateDetail::new(
                class22.clone(),
                "01E",
                "invalid argument for natural logarithm",
            ),
            DataExceptionInvalidArgumentForPowerFunction => SqlStateDetail::new(
                class22.clone(),
                "01F",
                "invalid argument for power function",
            ),
            DataExceptionInvalidArgumentForWidthBucketFunction => SqlStateDetail::new(
                class22.clone(),
                "01G",
                "invalid argument for width bucket function",
            ),
            DataExceptionInvalidRowVersion => {
                SqlStateDetail::new(class22.clone(), "01H", "invalid row version")
            }
            DataExceptionInvalidQueryRegularExpression => {
                SqlStateDetail::new(class22.clone(), "01S", "invalid Query regular expression")
            }
            DataExceptionInvalidQueryOptionFlag => {
                SqlStateDetail::new(class22.clone(), "01T", "invalid Query option flag")
            }
            DataExceptionAttemptToReplaceAZerolengthString => SqlStateDetail::new(
                class22.clone(),
                "01U",
                "attempt to replace a zero-length string",
            ),
            DataExceptionInvalidQueryReplacementString => {
                SqlStateDetail::new(class22.clone(), "01V", "invalid Query replacement string")
            }
            DataExceptionInvalidRowCountInFetchFirstClause => SqlStateDetail::new(
                class22.clone(),
                "01W",
                "invalid row count in fetch first clause",
            ),
            DataExceptionInvalidRowCountInResultOffsetClause => SqlStateDetail::new(
                class22.clone(),
                "01X",
                "invalid row count in result offset clause",
            ),
            DataExceptionCharacterNotInRepertoire => {
                SqlStateDetail::new(class22.clone(), "021", "character not in repertoire")
            }
            DataExceptionIndicatorOverflow => {
                SqlStateDetail::new(class22.clone(), "022", "indicator overflow")
            }
            DataExceptionInvalidParameterValue => {
                SqlStateDetail::new(class22.clone(), "023", "invalid parameter value")
            }
            DataExceptionUnterminatedCString => {
                SqlStateDetail::new(class22.clone(), "024", "unterminated C string")
            }
            DataExceptionInvalidEscapeSequence => {
                SqlStateDetail::new(class22.clone(), "025", "invalid escape sequence")
            }
            DataExceptionStringDataLengthMismatch => {
                SqlStateDetail::new(class22.clone(), "026", "string data, length mismatch")
            }
            DataExceptionTrimError => SqlStateDetail::new(class22.clone(), "027", "trim error"),
            DataExceptionNoncharacterInUCSString => {
                SqlStateDetail::new(class22.clone(), "029", "noncharacter in UCS string")
            }
            DataExceptionNullValueSubstitutedForMutatorSubjectParameter => SqlStateDetail::new(
                class22.clone(),
                "02D",
                "null value substituted for mutator subject parameter",
            ),
            DataExceptionArrayElementError => {
                SqlStateDetail::new(class22.clone(), "02E", "array element error")
            }
            DataExceptionArrayDataRightTruncation => {
                SqlStateDetail::new(class22.clone(), "02F", "array data, right truncation")
            }
            DataExceptionInvalidRepeatArgumentInASampleClause => SqlStateDetail::new(
                class22.clone(),
                "02G",
                "invalid repeat argument in a sample clause",
            ),
            DataExceptionInvalidSampleSize => {
                SqlStateDetail::new(class22.clone(), "02H", "invalid sample size")
            }
            DataExceptionIllegalConversion => {
                SqlStateDetail::new(class22.clone(), "I00", "illegal data conversion")
            }
            DataExceptionIllegalComparison => {
                SqlStateDetail::new(class22.clone(), "I01", "illegal data comparison")
            }
            DataExceptionIllegalOperation => {
                SqlStateDetail::new(class22.clone(), "I02", "illegal operation to data")
            }
            IntegrityConstraintViolation => {
                SqlStateDetail::new(class23.clone(), "000", "(no subclass)")
            }
            IntegrityConstraintViolationRestrictViolation => {
                SqlStateDetail::new(class23.clone(), "001", "restrict violation")
            }
            IntegrityConstraintNotNullViolation => {
                SqlStateDetail::new(class23.clone(), "I00", "not null violation")
            }
            IntegrityConstraintUniqueViolation => {
                SqlStateDetail::new(class23.clone(), "I01", "unique violation")
            }
            InvalidCursorState => SqlStateDetail::new(class24.clone(), "000", "(no subclass)"),
            InvalidTransactionState => SqlStateDetail::new(class25.clone(), "000", "(no subclass)"),
            InvalidTransactionStateActiveSQLtransaction => {
                SqlStateDetail::new(class25.clone(), "001", "active SQL-transaction")
            }
            InvalidTransactionStateBranchTransactionAlreadyActive => {
                SqlStateDetail::new(class25.clone(), "002", "branch transaction already active")
            }
            InvalidTransactionStateInappropriateAccessModeForBranchTransaction => {
                SqlStateDetail::new(
                    class25.clone(),
                    "003",
                    "inappropriate access mode for branch transaction",
                )
            }
            InvalidTransactionStateInappropriateIsolationLevelForBranchTransaction => {
                SqlStateDetail::new(
                    class25.clone(),
                    "004",
                    "inappropriate isolation level for branch transaction",
                )
            }
            InvalidTransactionStateNoActiveSQLtransactionForBranchTransaction => {
                SqlStateDetail::new(
                    class25.clone(),
                    "005",
                    "no active SQL-transaction for branch transaction",
                )
            }
            InvalidTransactionStateReadonlySQLtransaction => {
                SqlStateDetail::new(class25.clone(), "006", "read-only SQL-transaction")
            }
            InvalidTransactionStateSchemaAndDataStatementMixingNotSupported => SqlStateDetail::new(
                class25.clone(),
                "007",
                "schema and data statement mixing not supported",
            ),
            InvalidTransactionStateHeldCursorRequiresSameIsolationLevel => SqlStateDetail::new(
                class25.clone(),
                "008",
                "held cursor requires same isolation level",
            ),
            InvalidSQLStatementName => SqlStateDetail::new(class26.clone(), "000", "(no subclass)"),
            TriggeredDataChangeViolation => {
                SqlStateDetail::new(class27.clone(), "000", "(no subclass)")
            }
            TriggeredDataChangeViolationModifyTableModifiedByDataChangeDeltaTable => {
                SqlStateDetail::new(
                    class27.clone(),
                    "001",
                    "modify table modified by data change delta table",
                )
            }
            InvalidAuthorizationSpecification => {
                SqlStateDetail::new(class28.clone(), "000", "(no subclass)")
            }
            DependentPrivilegeDescriptorsStillExist => {
                SqlStateDetail::new(class2B.clone(), "000", "(no subclass)")
            }
            InvalidCharacterSetName => SqlStateDetail::new(class2C.clone(), "000", "(no subclass)"),
            InvalidTransactionTermination => {
                SqlStateDetail::new(class2D.clone(), "000", "(no subclass)")
            }
            InvalidConnectionName => SqlStateDetail::new(class2E.clone(), "000", "(no subclass)"),
            SQLRoutineException => SqlStateDetail::new(class2F.clone(), "000", "(no subclass)"),
            SQLRoutineExceptionModifyingSQLdataNotPermitted => {
                SqlStateDetail::new(class2F.clone(), "002", "modifying SQL-data not permitted")
            }
            SQLRoutineExceptionProhibitedSQLstatementAttempted => {
                SqlStateDetail::new(class2F.clone(), "003", "prohibited SQL-statement attempted")
            }
            SQLRoutineExceptionReadingSQLdataNotPermitted => {
                SqlStateDetail::new(class2F.clone(), "004", "reading SQL-data not permitted")
            }
            SQLRoutineExceptionFunctionExecutedNoReturnStatement => SqlStateDetail::new(
                class2F.clone(),
                "005",
                "function executed no return statement",
            ),
            InvalidCollationName => SqlStateDetail::new(class2H.clone(), "000", "(no subclass)"),
            InvalidSQLStatementIdentifier => {
                SqlStateDetail::new(class30.clone(), "000", "(no subclass)")
            }
            InvalidSQLDescriptorName => {
                SqlStateDetail::new(class33.clone(), "000", "(no subclass)")
            }
            InvalidCursorName => SqlStateDetail::new(class34.clone(), "000", "(no subclass)"),
            InvalidConditionNumber => SqlStateDetail::new(class35.clone(), "000", "(no subclass)"),
            CursorSensitivityException => {
                SqlStateDetail::new(class36.clone(), "000", "(no subclass)")
            }
            CursorSensitivityExceptionRequestRejected => {
                SqlStateDetail::new(class36.clone(), "001", "request rejected")
            }
            CursorSensitivityExceptionRequestFailed => {
                SqlStateDetail::new(class36.clone(), "002", "request failed")
            }
            ExternalRoutineException => {
                SqlStateDetail::new(class38.clone(), "000", "(no subclass)")
            }
            ExternalRoutineExceptionContainingSQLNotPermitted => {
                SqlStateDetail::new(class38.clone(), "001", "containing SQL not permitted")
            }
            ExternalRoutineExceptionModifyingSQLdataNotPermitted => {
                SqlStateDetail::new(class38.clone(), "002", "modifying SQL-data not permitted")
            }
            ExternalRoutineExceptionProhibitedSQLstatementAttempted => {
                SqlStateDetail::new(class38.clone(), "003", "prohibited SQL-statement attempted")
            }
            ExternalRoutineExceptionReadingSQLdataNotPermitted => {
                SqlStateDetail::new(class38.clone(), "004", "reading SQL-data not permitted")
            }
            ExternalRoutineInvocationException => {
                SqlStateDetail::new(class39.clone(), "000", "(no subclass)")
            }
            ExternalRoutineInvocationExceptionNullValueNotAllowed => {
                SqlStateDetail::new(class39.clone(), "004", "null value not allowed")
            }
            SavepointException => SqlStateDetail::new(class3B.clone(), "000", "(no subclass)"),
            SavepointExceptionInvalidSpecification => {
                SqlStateDetail::new(class3B.clone(), "001", "invalid specification")
            }
            SavepointExceptionTooMany => SqlStateDetail::new(class3B.clone(), "002", "too many"),
            AmbiguousCursorName => SqlStateDetail::new(class3C.clone(), "000", "(no subclass)"),
            InvalidCatalogName => SqlStateDetail::new(class3D.clone(), "000", "(no subclass)"),
            InvalidSchemaName => SqlStateDetail::new(class3F.clone(), "000", "(no subclass)"),
            TransactionRollback => SqlStateDetail::new(class40.clone(), "000", "(no subclass)"),
            TransactionRollbackSerializationFailure => {
                SqlStateDetail::new(class40.clone(), "001", "serialization failure")
            }
            TransactionRollbackIntegrityConstraintViolation => {
                SqlStateDetail::new(class40.clone(), "002", "integrity constraint violation")
            }
            TransactionRollbackStatementCompletionUnknown => {
                SqlStateDetail::new(class40.clone(), "003", "statement completion unknown")
            }
            TransactionRollbackTriggeredActionException => {
                SqlStateDetail::new(class40.clone(), "004", "triggered action exception")
            }
            TransactionRollbackDeadlock => {
                SqlStateDetail::new(class40.clone(), "I00", "deadlock detected")
            }
            SyntaxErrorOrAccessRuleViolation => {
                SqlStateDetail::new(class42.clone(), "000", "(no subclass)")
            }
            WithCheckOptionViolation => {
                SqlStateDetail::new(class44.clone(), "000", "(no subclass)")
            }
            ReservedForISO9579 => SqlStateDetail::new(classHZ.clone(), "???", ""),
            IoError => SqlStateDetail::new(classIO.clone(), "000", "(no subclass)"),
            NameError => SqlStateDetail::new(classNM.clone(), "000", "(no subclass)"),
            NameErrorNotFound => SqlStateDetail::new(classNM.clone(), "001", "not found by name"),
            NameErrorAmbiguous => SqlStateDetail::new(classNM.clone(), "002", "ambiguous name"),
            NameErrorDuplicate => SqlStateDetail::new(classNM.clone(), "003", "duplicate name"),
            NameErrorTooLong => SqlStateDetail::new(classNM.clone(), "004", "too long name"),
            DdlError => SqlStateDetail::new(classSC.clone(), "000", "(no subclass)"),
            SystemError => SqlStateDetail::new(classSY.clone(), "000", "(no subclass)"),
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
