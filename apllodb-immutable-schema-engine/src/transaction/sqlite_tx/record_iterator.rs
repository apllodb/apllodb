use apllodb_shared_components::data_structure::Record;

pub(crate) struct SqliteRecordIterator;

impl Iterator for SqliteRecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
