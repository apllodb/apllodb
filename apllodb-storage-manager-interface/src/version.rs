mod action;
mod active_version;
mod column;
mod constraint;
mod inactive_version;

pub use active_version::ActiveVersion;
pub use inactive_version::InactiveVersion;

use column::ColumnDataType;
use constraint::VersionConstraint;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Version.
///
/// A version belongs to a [Table](struct.Table.html).
/// A version directly has subset of records in the Table.
///
/// - The version `v_1` is created by apllodb CREATE TABLE command.
/// - Version `v_(current+1)` is created by apllodb ALTER TABLE command.
/// - Some of `v_1` ~ `v_current` are inactivated by apllodb ALTER TABLE command
///   if all the records in `v_i` can be migrated to `v_(current+1)` (auto upgrade).
/// - All of `v_1` ~ `v_current` are inactivated by apllodb DROP TABLE command.
///
/// Each version is purely immutable.
///
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
struct Version {
    number: u64,
    column_data_types: Vec<ColumnDataType>,
    constraints: Vec<VersionConstraint>, // TODO make VersionConstraints type and validation like TableConstraints.
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
