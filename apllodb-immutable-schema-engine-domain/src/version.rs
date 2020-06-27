mod active_version;
mod active_versions;
mod constraint_kind;
mod constraints;
mod id;
mod inactive_version;
mod repository;
mod version_number;

pub use active_version::ActiveVersion;
pub use active_versions::ActiveVersions;
pub use id::VersionId;
pub use inactive_version::InactiveVersion;
pub use repository::VersionRepository;
pub use version_number::VersionNumber;

use crate::entity::Entity;
use apllodb_shared_components::data_structure::ColumnDataType;
use constraints::VersionConstraints;
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
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
///
/// Version does not have useful methods because you should access to version via
/// [ActiveVersion](foobar.html) or [InactiveVersion](foobar.html), both of which have different behavior.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(crate) struct Version {
    id: VersionId,

    /// Doesn't include Apparent Primary Key
    column_data_types: Vec<ColumnDataType>,

    constraints: VersionConstraints,
}

impl Entity for Version {
    type Id = VersionId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.version_number.cmp(&other.id.version_number)
    }
}
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
