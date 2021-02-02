use apllodb_shared_components::{ApllodbResult, FieldIndex, FullFieldReference, Record};

pub(crate) fn r_projection(r: Record, fields: Vec<FullFieldReference>) -> ApllodbResult<Record> {
    r.projection(
        &fields
            .into_iter()
            .map(FieldIndex::InFullFieldReference)
            .collect(),
    )
}
