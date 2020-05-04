#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    pub fn new(v: Vec<T>) -> Self {
        assert!(!v.is_empty());
        Self(v)
    }

    pub fn get(&self) -> &Vec<T> {
        &self.0
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
