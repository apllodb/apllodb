/// A vec ensured to have at least 1 element.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    pub(crate) fn new(v: Vec<T>) -> Self {
        assert!(!v.is_empty());
        Self(v)
    }

    /// Ref to internal Vec.
    pub fn as_vec(&self) -> &Vec<T> {
        &self.0
    }

    /// Moves ownership of internal Vec.
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
