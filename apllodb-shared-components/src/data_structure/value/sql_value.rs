use super::SqlConvertible;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct SqlValue<T: SqlConvertible>(T);
