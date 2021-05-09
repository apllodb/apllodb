use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// SQLSTATE.
///
/// Errors for specific SQL part (`SQL/JRT`, for example) are omitted.
///
/// See: <https://en.wikipedia.org/wiki/SQLSTATE>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub enum SqlState {
    // Class 00 - successful completion
    SuccessfulCompletion,

    // Class 01 - warning
    Warning,
    CursorOperationConflict,
    DisconnectError,
    NullValueEliminatedInSetFunction,
    StringData,
    RightTruncation,
    InsufficientItemDescriptorAreas,
    PrivilegeNotRevoked,
    PrivilegeNotGranted,
    SearchConditionTooLongForInformationSchema,
    QueryExpressionTooLongForInformationSchema,
    DefaultValueTooLongForInformationSchema,
    ResultSetsReturned,
    AdditionalResultSetsReturned,
    AttemptToReturnTooManyResultSets,
    StatementTooLongForInformationSchema,
    ColumnCannotBeMapped,
    InvalidNumberOfConditions,
    ArrayDataRightTruncationWarning,

    // Class 02 - no data
    NoData,
    NoAdditionalResultSetsReturned,

    // Class 07 - dynamic SQL error
    DynamicSQLError,
    UsingClauseDoesNotMatchDynamicParameterSpecifications,
    UsingClauseDoesNotMatchTargetSpecifications,
    CursorSpecificationCannotBeExecuted,
    UsingClauseRequiredForDynamicParameters,
    PreparedStatementNotACursorSpecification,
    RestrictedDataTypeAttributeViolation,
    UsingClauseRequiredForResultFields,
    InvalidDescriptorCount,
    InvalidDescriptorIndex,
    DataTypeTransformFunctionViolation,
    UndefinedDATAValue,
    InvalidDATATarget,
    InvalidLEVELValue,
    InvalidDatetimeIntervalCode,

    // Class 08 - connection exception
    ConnectionException,
    SQLClientUnableToEstablishSQLConnection,
    ConnectionNameInUse,
    ConnectionDoesNotExist,
    SQLServerRejectedEstablishmentOfSQLConnection,
    ConnectionFailure,
    TransactionResolutionUnknown,

    // Class 09 - triggered action exception
    TriggeredActionException,

    // Class 0A - feature not supported
    FeatureNotSupported,
    MultipleServerTransactions,

    // Class 0D - invalid target type specification
    InvalidTargetTypeSpecification,

    // Class 0E - invalid schema name list specification
    InvalidSchemaNameListSpecification,

    // Class 0F - locator exception
    LocatorException,
    InvalidSpecification,

    // Class 0L - invalid grantor
    InvalidGrantor,

    // Class 0M - invalid SQL-invoked procedure reference
    InvalidSQLInvokedProcedureReference,

    // Class 0P - invalid role specification
    InvalidRoleSpecification,

    // Class 0S - invalid transform group name specification
    InvalidTransformGroupNameSpecification,

    // Class 0T - target table disagrees with cursor specification
    TargetTableDisagreesWithCursorSpecification,

    // Class 0U - attempt to assign to non-updatable column
    AttemptToAssignToNonUpdatableColumn,

    // Class 0V - attempt to assign to ordering column
    AttemptToAssignToOrderingColumn,

    // Class 0W - prohibited statement encountered during trigger execution
    ProhibitedStatementEncounteredDuringTriggerExecution,
    ModifyTableModifiedByDataChangeDeltaTable0W,

    // Class 0Z - diagnostics exception
    DiagnosticsException,
    MaximumNumberOfStackedDiagnosticsAreasExceeded,

    // Class 21 - cardinality violation
    CardinalityViolation,

    // Class 22 - data exception
    DataException,
    StringDataRightTruncation,
    NullValueNoIndicatorParameter,
    NumericValueOutOfRange,
    NullValueNotAllowed,
    ErrorInAssignment,
    InvalidIntervalFormat,
    InvalidDatetimeFormat,
    DatetimeFieldOverflow,
    InvalidTimeZoneDisplacementValue,
    EscapeCharacterConflict,
    InvalidUseOfEscapeCharacter,
    InvalidEscapeOctet,
    NullValueInArrayTarget,
    ZeroLengthCharacterString,
    MostSpecificTypeMismatch,
    SequenceGeneratorLimitExceeded,
    IntervalValueOutOfRange,
    MultisetValueOverflow,
    InvalidIndicatorParameterValue,
    SubstringError,
    DivisionByZero,
    InvalidPrecedingOrFollowingSizeInWindowFunction,
    InvalidArgumentForNTILEFunction,
    IntervalFieldOverflow,
    InvalidArgumentForNthValueFunction,
    InvalidCharacterValueForCast,
    InvalidEscapeCharacter,
    InvalidRegularExpression,
    NullRowNotPermittedInTable,
    InvalidArgumentForNaturalLogarithm,
    InvalidArgumentForPowerFunction,
    InvalidArgumentForWidthBucketFunction,
    InvalidRowVersion,
    InvalidQueryRegularExpression,
    InvalidQueryOptionFlag,
    AttemptToReplaceAZeroLengthString,
    InvalidQueryReplacementString,
    InvalidRowCountInFetchFirstClause,
    InvalidRowCountInResultOffsetClause,
    CharacterNotInRepertoire,
    IndicatorOverflow,
    InvalidParameterValue,
    UnterminatedCString,
    InvalidEscapeSequence,
    StringDataLengthMismatch,
    TrimError,
    NoncharacterInUCSString,
    NullValueSubstitutedForMutatorSubjectParameter,
    ArrayElementError,
    ArrayDataRightTruncation,
    InvalidRepeatArgumentInASampleClause,
    InvalidSampleSize,

    // Class 23 - integrity constraint violation
    IntegrityConstraintViolation,
    RestrictViolation,

    // Class 24 - invalid cursor state
    InvalidCursorState,

    // Class 25 - invalid transaction state
    InvalidTransactionState,
    ActiveSQLTransaction,
    BranchTransactionAlreadyActive,
    InappropriateAccessModeForBranchTransaction,
    InappropriateIsolationLevelForBranchTransaction,
    NoActiveSQLTransactionForBranchTransaction,
    ReadOnlySQLTransaction,
    SchemaAndDataStatementMixingNotSupported,
    HeldCursorRequiresSameIsolationLevel,

    // Class 26 - invalid SQL statement name
    InvalidSqlStatementName,

    // Class 27 - triggered data change violation
    TriggeredDataChangeViolation,
    ModifyTableModifiedByDataChangeDeltaTable,

    // Class 28 - invalid authorization specification
    InvalidAuthorizationSpecification,

    // Class 2B - dependent privilege descriptors still exist
    DependentPrivilegeDescriptorsStillExist,

    // Class 2C - invalid character set name
    InvalidCharacterSetName,

    // Class 2D - invalid transaction termination
    InvalidTransactionTermination,

    // Class 2E - invalid connection name
    InvalidConnectionName,

    // Class 2F - SQL routine exception
    SqlRoutineException,
    ModifyingSQLDataNotPermitted,
    ProhibitedSQLStatementAttempted,
    ReadingSQLDataNotPermitted,
    FunctionExecutedNoReturnStatement,

    // Class 2H - invalid collation name
    InvalidCollationName,

    // Class 30 - invalid SQL statement identifier
    InvalidSQLStatementIdentifier,

    // Class 33 - invalid SQL descriptor name
    InvalidSQLDescriptorName,

    // Class 34 - invalid cursor name
    InvalidCursorName,

    // Class 35 - invalid condition number
    InvalidConditionNumber,

    // Class 36 - cursor sensitivity exception
    CursorSensitivityException,
    RequestRejected,
    RequestFailed,

    // Class 38 - external routine exception
    ExternalRoutineException,
    ContainingSQLNotPermitted,
    ModifyingSQLDataNotPermitted38,
    ProhibitedSQLStatementAttempted38,
    ReadingSQLDataNotPermitted38,

    // Class 39 - external routine invocation exception
    ExternalRoutineInvocationException,
    NullValueNotAllowed39,

    // Class 3B - savepoint exception
    SavepointException,
    InvalidSavepointSpecification,
    TooMany,

    // Class 3C - ambiguous cursor name
    AmbiguousCursorName,

    // Class 3D - invalid catalog name
    InvalidCatalogName,

    // Class 3F - invalid schema name
    InvalidSchemaName,

    // Class 40 - transaction rollback
    TransactionRollback,
    SerializationFailure,
    TransactionIntegrityConstraintViolation,
    StatementCompletionUnknown,
    TransactionTriggeredActionException,

    // Class 42 - syntax error or access rule violation
    SyntaxErrorOrAccessRuleViolation,

    // Class 44 - with check option violation
    WithCheckOptionViolation,

    // Class HZ - Reserved for ISO9579 (RDA)
}

impl Display for SqlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
