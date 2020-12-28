use apllodb_shared_components::TableName;

use super::row::StubRowIterator;

#[derive(Clone, Eq, PartialEq, Debug, new)]
pub(crate) struct StubData {
    pub(crate) tables: Vec<StubTable>,
}

#[derive(Clone, Eq, PartialEq, Debug, new)]
pub(crate) struct StubTable {
    pub(crate) table_name: TableName,
    pub(crate) rows: StubRowIterator,
}
