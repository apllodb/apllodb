use crate::Table;
use serde::{Deserialize, Serialize};

/// Instantiated in [SimpleStorage](foobar.html)'s memory.
/// Never be copied.
/// Created only via [SimpleStorage::load_table()](foobar.html) by a transaction who has [TableRwToken](foobar.html).
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::transaction::simple_tx) struct TableObj(Table);

impl TableObj {
    pub(in crate::transaction::simple_tx) fn update_by(&mut self, table: Table) {
        self.0 = table;
    }

    pub(in crate::transaction::simple_tx) fn as_table(&self) -> &Table {
        &self.0
    }
}
