pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;

/// Processes SQL.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct SQLProcessor {
    // TODO コネクションオブジェクトみたいなのがないと、既に開いている tx を参照などできない。
}
