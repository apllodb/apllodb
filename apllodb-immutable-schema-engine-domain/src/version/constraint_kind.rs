use serde::{Deserialize, Serialize};

/// Constraints that each record must satisfy.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(in crate::version) enum VersionConstraintKind {
    Default(/* TODO: Expr */),
    Check(/* TODO: Expr (e.g. c1 + c2 < c3) */),
    ForeignKey(/* TODO: ??? */),
}
