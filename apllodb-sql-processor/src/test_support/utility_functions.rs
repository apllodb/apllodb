use apllodb_shared_components::{ApllodbResult, ColumnReference, FieldIndex, Record};

pub(crate) fn _r_projection(r: Record, fields: Vec<ColumnReference>) -> ApllodbResult<Record> {
    r.projection(
        &fields
            .into_iter()
            .map(FieldIndex::InColumnReference)
            .collect(),
    )
}
