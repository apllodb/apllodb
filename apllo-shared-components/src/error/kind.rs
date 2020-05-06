use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum AplloErrorKind {
    // Section: Class 02 - No Data (this is also a warning class per the SQL standard)
    NoData,
    NoAdditionalDynamicResultSetsReturned,

    // Section: Class 03 - SQL Statement Not Yet Complete
    SqlStatementNotYetComplete,

    // Section: Class 08 - Connection Exception
    ConnectionException,
    ConnectionDoesNotExist,
    ConnectionFailure,
    SqlclientUnableToEstablishSqlconnection,
    SqlserverRejectedEstablishmentOfSqlconnection,
    TransactionResolutionUnknown,
    ProtocolViolation,

    // Section: Class 09 - Triggered Action Exception
    TriggeredActionException,

    // Section: Class 0A - Feature Not Supported
    FeatureNotSupported,

    // Section: Class 0B - Invalid Transaction Initiation
    InvalidTransactionInitiation,

    // Section: Class 0F - Locator Exception
    LocatorException,
    InvalidLocatorSpecification,

    // Section: Class 0L - Invalid Grantor
    InvalidGrantor,
    InvalidGrantOperation,

    // Section: Class 0P - Invalid Role Specification
    InvalidRoleSpecification,

    // Section: Class 0Z - Diagnostics Exception
    DiagnosticsException,
    StackedDiagnosticsAccessedWithoutActiveHandler,

    // Section: Class 20 - Case Not Found
    CaseNotFound,

    // Section: Class 21 - Cardinality Violation
    CardinalityViolation,

    // Section: Class 22 - Data Exception
    DataException,
    ArraySubscriptError,
    CharacterNotInRepertoire,
    DatetimeFieldOverflow,
    DivisionByZero,
    ErrorInAssignment,
    EscapeCharacterConflict,
    IndicatorOverflow,
    IntervalFieldOverflow,
    InvalidArgumentForLogarithm,
    InvalidArgumentForNtileFunction,
    InvalidArgumentForNthValueFunction,
    InvalidArgumentForPowerFunction,
    InvalidArgumentForWidthBucketFunction,
    InvalidCharacterValueForCast,
    InvalidDatetimeFormat,
    InvalidEscapeCharacter,
    InvalidEscapeOctet,
    InvalidEscapeSequence,
    NonstandardUseOfEscapeCharacter,
    InvalidIndicatorParameterValue,
    InvalidParameterValue,
    InvalidPrecedingOrFollowingSize,
    InvalidRegularExpression,
    InvalidRowCountInLimitClause,
    InvalidRowCountInResultOffsetClause,
    InvalidTablesampleArgument,
    InvalidTablesampleRepeat,
    InvalidTimeZoneDisplacementValue,
    InvalidUseOfEscapeCharacter,
    MostSpecificTypeMismatch,
    NullValueNotAllowed,
    NullValueNoIndicatorParameter,
    NumericValueOutOfRange,
    SequenceGeneratorLimitExceeded,
    StringDataLengthMismatch,
    StringDataRightTruncation,
    SubstringError,
    TrimError,
    UnterminatedCString,
    ZeroLengthCharacterString,
    FloatingPointException,
    InvalidTextRepresentation,
    InvalidBinaryRepresentation,
    BadCopyFileFormat,
    UntranslatableCharacter,
    NotAnXmlDocument,
    InvalidXmlDocument,
    InvalidXmlContent,
    InvalidXmlComment,
    InvalidXmlProcessingInstruction,
    DuplicateJsonObjectKeyValue,
    InvalidArgumentForJsonDatetimeFunction,
    InvalidJsonText,
    InvalidSqlJsonSubscript,
    MoreThanOneSqlJsonItem,
    NoSqlJsonItem,
    NonNumericSqlJsonItem,
    NonUniqueKeysInAJsonObject,
    SingletonSqlJsonItemRequired,
    SqlJsonArrayNotFound,
    SqlJsonMemberNotFound,
    SqlJsonNumberNotFound,
    SqlJsonObjectNotFound,
    TooManyJsonArrayElements,
    TooManyJsonObjectMembers,
    SqlJsonScalarRequired,

    // Section: Class 23 - Integrity Constraint Violation
    IntegrityConstraintViolation,
    RestrictViolation,
    NotNullViolation,
    ForeignKeyViolation,
    UniqueViolation,
    CheckViolation,
    ExclusionViolation,

    // Section: Class 24 - Invalid Cursor State
    InvalidCursorState,

    // Section: Class 25 - Invalid Transaction State
    InvalidTransactionState,
    ActiveSqlTransaction,
    BranchTransactionAlreadyActive,
    HeldCursorRequiresSameIsolationLevel,
    InappropriateAccessModeForBranchTransaction,
    InappropriateIsolationLevelForBranchTransaction,
    NoActiveSqlTransactionForBranchTransaction,
    ReadOnlySqlTransaction,
    SchemaAndDataStatementMixingNotSupported,
    NoActiveSqlTransaction,
    InFailedSqlTransaction,
    IdleInTransactionSessionTimeout,

    // Section: Class 26 - Invalid SQL Statement Name
    InvalidSqlStatementName,

    // Section: Class 27 - Triggered Data Change Violation
    TriggeredDataChangeViolation,

    // Section: Class 28 - Invalid Authorization Specification
    InvalidAuthorizationSpecification,
    InvalidPassword,

    // Section: Class 2B - Dependent Privilege Descriptors Still Exist
    DependentPrivilegeDescriptorsStillExist,
    DependentObjectsStillExist,

    // Section: Class 2D - Invalid Transaction Termination
    InvalidTransactionTermination,

    // Section: Class 2F - SQL Routine Exception
    SqlRoutineException,
    SqlFunctionExecutedNoReturnStatement,
    SqlModifyingSqlDataNotPermitted,
    SqlProhibitedSqlStatementAttempted,
    SqlReadingSqlDataNotPermitted,

    // Section: Class 34 - Invalid Cursor Name
    InvalidCursorName,

    // Section: Class 38 - External Routine Exception
    ExternalRoutineException,
    ExternalContainingSqlNotPermitted,
    ExternalModifyingSqlDataNotPermitted,
    ExternalProhibitedSqlStatementAttempted,
    ExternalReadingSqlDataNotPermitted,

    // Section: Class 39 - External Routine Invocation Exception
    ExternalRoutineInvocationException,
    ExternalInvalidSqlstateReturned,
    ExternalNullValueNotAllowed,
    ExternalTriggerProtocolViolated,
    ExternalSrfProtocolViolated,
    ExternalEventTriggerProtocolViolated,

    // Section: Class 3B - Savepoint Exception
    SavepointException,
    InvalidSavepointSpecification,

    // Section: Class 3D - Invalid Catalog Name
    InvalidCatalogName,

    // Section: Class 3F - Invalid Schema Name
    InvalidSchemaName,

    // Section: Class 40 - Transaction Rollback
    TransactionRollback,
    TransactionIntegrityConstraintViolation,
    SerializationFailure,
    StatementCompletionUnknown,
    DeadlockDetected,

    // Section: Class 42 - Syntax Error or Access Rule Violation
    /// never use this one;
    SyntaxErrorOrAccessRuleViolation,
    SyntaxError,
    InsufficientPrivilege,
    CannotCoerce,
    GroupingError,
    WindowingError,
    InvalidRecursion,
    InvalidForeignKey,
    InvalidName,
    NameTooLong,
    ReservedName,
    DatatypeMismatch,
    IndeterminateDatatype,
    CollationMismatch,
    IndeterminateCollation,
    WrongObjectType,
    GeneratedAlways,
    UndefinedColumn,
    UndefinedFunction,
    UndefinedTable,
    UndefinedParameter,
    UndefinedObject,
    DuplicateColumn,
    DuplicateCursor,
    DuplicateDatabase,
    DuplicateFunction,
    DuplicatePreparedStatement,
    DuplicateSchema,
    DuplicateTable,
    DuplicateAlias,
    DuplicateObject,
    AmbiguousColumn,
    AmbiguousFunction,
    AmbiguousParameter,
    AmbiguousAlias,
    InvalidColumnReference,
    InvalidColumnDefinition,
    InvalidCursorDefinition,
    InvalidDatabaseDefinition,
    InvalidFunctionDefinition,
    InvalidPreparedStatementDefinition,
    InvalidSchemaDefinition,
    InvalidTableDefinition,
    InvalidObjectDefinition,

    // Section: Class 44 - WITH CHECK OPTION Violation
    WithCheckOptionViolation,
}
