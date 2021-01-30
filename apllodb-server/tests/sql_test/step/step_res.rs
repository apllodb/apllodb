use apllodb_shared_components::{ApllodbErrorKind, RecordIterator};

#[derive(Clone, PartialEq, Debug)]
pub enum StepRes {
    OkQuery(RecordIterator), // TODO レコードまるごとだと比較面倒かも。 |record| ... なクロージャーを取って、assert! を中で書かせるくらいが丁度いい？
    Ok,
    Err(ApllodbErrorKind),
}
