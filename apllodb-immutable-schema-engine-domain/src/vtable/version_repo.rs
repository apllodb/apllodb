use crate::{ActiveVersion, InactiveVersion};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(crate) struct VersionRepo {
    active_versions: Vec<ActiveVersion>,
    inactive_versions: Vec<InactiveVersion>,
}

impl Default for VersionRepo {
    fn default() -> Self {
        Self {
            active_versions: vec![],
            inactive_versions: vec![],
        }
    }
}

impl VersionRepo {
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - No version is active (table must be already DROPped).
    pub(crate) fn current_version(&self) -> ApllodbResult<&ActiveVersion> {
        self.active_versions.last().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                "no active version found",
                None,
            )
        })
    }

    pub(super) fn add_active_version(&mut self, version: ActiveVersion) {
        self.active_versions.push(version);
    }
}
