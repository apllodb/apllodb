use crate::ActiveVersion;

/// Collection of [ActiveVersion](x.html) sorted from latest to oldest.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ActiveVersions(Vec<ActiveVersion>);

impl<I: IntoIterator<Item = ActiveVersion>> From<I> for ActiveVersions {
    /// Construct sorted collection.
    /// `i` need not to be sorted.
    fn from(i: I) -> Self {
        let mut v: Vec<ActiveVersion> = i.into_iter().collect();
        v.sort_by(|a, b| b.cmp(a));
        Self(v)
    }
}

impl ActiveVersions {
    pub fn as_sorted_slice(&self) -> &[ActiveVersion] {
        &self.0
    }
}
