use super::sqlstate::SqlState;

#[derive(Clone, Debug)]
pub(super) struct ApllodbErrorAux {
    pub(super) sqlstate: SqlState,
    pub(super) errcode: String,
}
