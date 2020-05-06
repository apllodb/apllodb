mod constraint;

use apllo_shared_components::ColumnName;

use constraint::VersionSetConstraint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VersionSetColumnDefinition {
    // このデータ型は無効。バージョンセットはテーブル制約（複数カラムにまたがるUNIQUEなど）を持つので。VersionSetColumnConstraintならわかる。
    column: ColumnName,
    constraints: Vec<VersionSetConstraint>,
}
