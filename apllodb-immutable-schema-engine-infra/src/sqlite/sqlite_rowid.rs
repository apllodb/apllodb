/// See: https://www.sqlite.org/lang_createtable.html#rowid
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite) struct SqliteRowid(pub i64);
