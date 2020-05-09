use std::{collections::HashSet, hash::Hash};

/// Find a first element that is equal to another element in `s`.
pub(in crate::data_structure) fn find_dup<T>(iter: T) -> Option<T::Item>
where
    T: Iterator,
    T::Item: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    for item in iter {
        if !uniq.insert(item.clone()) {
            return Some(item);
        }
    }
    None
}
