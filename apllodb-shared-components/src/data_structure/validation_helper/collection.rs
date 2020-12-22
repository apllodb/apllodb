use std::{collections::HashSet, hash::Hash};

/// Find a first element that is equal to another element in `s`.
#[allow(dead_code)]
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

/// Find a first element that is equal to another element in `s`.
/// It uses O(n^2) nested loop algorithm. Use [find_dup()](method.find_dup.html) if `T` implements `Hash` instead.
pub(in crate::data_structure) fn find_dup_slow<T>(iter: T) -> Option<T::Item>
where
    T: Iterator,
    T::Item: Eq + Clone,
{
    let copy: Vec<T::Item> = iter.collect();

    for i in 0..copy.len() {
        for j in (i + 1)..copy.len() {
            if copy.get(i) == copy.get(j) {
                return Some(copy[i].clone());
            }
        }
    }
    None
}
