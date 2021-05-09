use serde::{Deserialize, Serialize};

/// All the possible errors in apllodb workspace.
///
/// Subset of SQL standard errors, whose SQLSTATE starts from 0-4, are borrowed from PostgreSQL:
/// <https://github.com/postgres/postgres/blob/master/src/backend/utils/errcodes.txt>
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ApllodbErrorKind {
}
