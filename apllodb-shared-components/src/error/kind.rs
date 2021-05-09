use serde::{Deserialize, Serialize};

/// All the possible errors in apllodb workspace.
///
/// Subset of SQL standard errors, whose SQLSTATE starts from 0-4, are borrowed from PostgreSQL:
/// <https://github.com/postgres/postgres/blob/master/src/backend/utils/errcodes.txt>
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ApllodbErrorKind {
    // Class 00 - successful completion

    // Class 01 - warning

    // Class 02 - no data
    NoData,
    NoAdditionalDynamicResultSetsReturned,

    // Class 07 - dynamic SQL error

    // Class 08 - connection exception
    ConnectionException,
    ConnectionDoesNotExist,
    ConnectionFailure,
    SqlclientUnableToEstablishSqlconnection,
    SqlserverRejectedEstablishmentOfSqlconnection,
    TransactionResolutionUnknown,
    ProtocolViolation,

    // Class 09 - triggered action exception
    TriggeredActionException,

    // Class 0A - feature not supported
    FeatureNotSupported,

    // Class 0D - invalid target typ specification

    // Class 0D - invalid target type specification

    // Class 0E - invalid schema name list specification

    // Class 0F - locator exception
    LocatorException,
    InvalidLocatorSpecification,

    // Class 0K - resignal when handler not active

    // Class 0L - invalid grantor
    InvalidGrantor,
    InvalidGrantOperation,

    // Class 0M - invalid SQL-invoked procedure reference

    // Class 0N - SQL/XML mapping error

    // Class 0P - invalid role specification
    InvalidRoleSpecification,

    // Class 0S - invalid transform group name specification

    // Class 0T - target table disagrees with cursor specification

    // Class 0U - attempt to assign to non-updatable column

    // Class 0V - attempt to assign to ordering column

    // Class 0W - prohibited statement encountered during trigger execution

    // Class 0X - invalid foreign server specification

    // Class 0Y - pass-through specific condition

    // Class 0Z - diagnostics exception
    DiagnosticsException,
    StackedDiagnosticsAccessedWithoutActiveHandler,

    // Class 10 - XQuery error

    // Class 20 - case not found for case statement
    CaseNotFound,

    // Class 21 - cardinality violation
    CardinalityViolation,

    // Class 22 - data exception
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

    // Class 23 - integrity constraint violation
    IntegrityConstraintViolation,
    RestrictViolation,
    NotNullViolation,
    ForeignKeyViolation,
    UniqueViolation,
    CheckViolation,
    ExclusionViolation,

    // Class 24 - invalid cursor state
    InvalidCursorState,

    // Class 25 - invalid transaction ttate
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

    // Class 26 - invalid SQL statement name
    InvalidSqlStatementName,

    // Class 27 - triggered data change violation
    TriggeredDataChangeViolation,

    // Class 28 - invalid authorization specification
    InvalidAuthorizationSpecification,
    InvalidPassword,

    // Class 2B - dependent privilege descriptors still exist
    DependentPrivilegeDescriptorsStillExist,
    DependentObjectsStillExist,

    // Class 2C - invalid character set name

    // Class 2D - invalid transaction termination
    InvalidTransactionTermination,

    // Class 2E - invalid connection name

    // Class 2F - SQL routine exception
    SqlRoutineException,
    SqlFunctionExecutedNoReturnStatement,
    SqlModifyingSqlDataNotPermitted,
    SqlProhibitedSqlStatementAttempted,
    SqlReadingSqlDataNotPermitted,

    // Class 2H - invalid collation name

    // Class 30 - invalid SQL statement identifier

    // Class 33 - invalid SQL descriptor name

    // Class 34 - invalid cursor name
    InvalidCursorName,

    // Class 35 - invalid condition number

    // Class 36 - cursor sensitivity exception

    // Class 38 - external routine exception
    ExternalRoutineException,
    ExternalContainingSqlNotPermitted,
    ExternalModifyingSqlDataNotPermitted,
    ExternalProhibitedSqlStatementAttempted,
    ExternalReadingSqlDataNotPermitted,

    // Class 39 - external routine invocation exception
    ExternalRoutineInvocationException,
    ExternalInvalidSqlstateReturned,
    ExternalNullValueNotAllowed,
    ExternalTriggerProtocolViolated,
    ExternalSrfProtocolViolated,
    ExternalEventTriggerProtocolViolated,

    // Class 3B - savepoint exception
    SavepointException,
    InvalidSavepointSpecification,

    // Class 3C - ambiguous cursor name

    // Class 3D - invalid catalog name
    InvalidCatalogName,

    // Class 3F - invalid schema name
    InvalidSchemaName,

    // Class 40 - transaction rollback
    TransactionRollback,
    TransactionIntegrityConstraintViolation,
    SerializationFailure,
    StatementCompletionUnknown,
    DeadlockDetected,

    // Class 42 - syntax error or access rule violation
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

    // Class 44 - with check option violation
    WithCheckOptionViolation,

    // Class 45 - unhandled user-defined exception

    // Class 46 - OLB-specific error, Java DDL

    // Class HW - datalink exception

    // Class HV - FDW-specific condition

    // Class HY - CLI-specific condition

    // Class HZ - Reserved for ISO9579 (RDA)

    // Class 58 (non-standard; partially same as PostgreSWL) - System Errors
    SystemError,
    IoError,
    DeserializationError,
    SerializationError,
    UndefinedPrimaryKey,
    InvalidVersion,
}
