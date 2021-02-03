use apllodb_shared_components::{ApllodbResult, FieldIndex, Record};

pub(crate) fn r_projection(r: Record, fields: Vec<FieldIndex>) -> ApllodbResult<Record> {
    r.projection(&fields.into_iter().collect())
}
