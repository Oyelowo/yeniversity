//! Basic set operations on `HashSet<T>`.

use std::collections::HashSet;
use std::hash::Hash;

pub fn union<T: Eq + Hash + Clone>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.union(b).cloned().collect()
}

pub fn intersection<T: Eq + Hash + Clone>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.intersection(b).cloned().collect()
}

pub fn difference<T: Eq + Hash + Clone>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.difference(b).cloned().collect()
}

/// Power set of a finite set (2^n elements).
pub fn power_set<T: Eq + Hash + Clone>(s: &HashSet<T>) -> Vec<HashSet<T>> {
    let elements: Vec<&T> = s.iter().collect();
    let n = elements.len();
    (0..(1u64 << n))
        .map(|mask| {
            elements
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, &e)| e.clone())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_set_size() {
        let s: HashSet<u8> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(power_set(&s).len(), 8); // 2^3
    }
}
