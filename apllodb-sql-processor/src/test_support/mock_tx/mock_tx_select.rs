pub(crate) mod mock_select_with_models;

use std::collections::HashSet;

use super::MockTx;
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnReference, FieldIndex, Record,
    RecordIterator, TableName,
};
use apllodb_storage_engine_interface::ProjectionQuery;

#[derive(Clone, PartialEq, Debug)]
struct MockTxDbDatum {
    tables: Vec<MockTxTableDatum>,
}

#[derive(Clone, PartialEq, Debug)]
struct MockTxTableDatum {
    table_name: TableName,
    records: Vec<Record>,
}

fn mock_select(tx: &mut MockTx, data: MockTxDbDatum) {
    tx.expect_select().returning(move |table_name, projection| {
        let table = data
            .tables
            .iter()
            .find(|table| table.table_name == *table_name)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!("table `{:?}` is undefined in MockTx", table_name),
                    None,
                )
            })?;

        let records = table.records.clone();

        match projection {
            ProjectionQuery::All => Ok(RecordIterator::new(records)),
            ProjectionQuery::ColumnNames(column_names) => {
                let fields: HashSet<FieldIndex> = column_names
                    .into_iter()
                    .map(|cn| {
                        FieldIndex::InColumnReference(ColumnReference::new(table_name.clone(), cn))
                    })
                    .collect();

                let projected_records: Vec<Record> = records
                    .into_iter()
                    .map(|record| Ok(record.projection(&fields)?))
                    .collect::<ApllodbResult<_>>()?;

                Ok(RecordIterator::new(projected_records))
            }
        }
    });
}
