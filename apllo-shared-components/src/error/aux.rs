use super::sqlstate::SqlState;

#[derive(Clone, Debug)]
pub(super) struct AplloErrorAux {
    pub(super) sqlstate: SqlState,
    pub(super) errcode: String,
}
