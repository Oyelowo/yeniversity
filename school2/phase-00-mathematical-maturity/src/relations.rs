//! Finite binary relation helpers.

use std::collections::HashSet;
use std::hash::Hash;

pub type Relation<T> = HashSet<(T, T)>;

pub fn is_reflexive<T: Eq + Hash + Clone>(universe: &HashSet<T>, relation: &Relation<T>) -> bool {
    universe
        .iter()
        .all(|value| relation.contains(&(value.clone(), value.clone())))
}

pub fn is_symmetric<T: Eq + Hash + Clone>(relation: &Relation<T>) -> bool {
    relation
        .iter()
        .all(|(left, right)| relation.contains(&(right.clone(), left.clone())))
}

pub fn is_antisymmetric<T: Eq + Hash + Clone>(relation: &Relation<T>) -> bool {
    relation.iter().all(|(left, right)| {
        left == right || !relation.contains(&(right.clone(), left.clone()))
    })
}

pub fn is_transitive<T: Eq + Hash + Clone>(relation: &Relation<T>) -> bool {
    relation.iter().all(|(left, middle)| {
        relation.iter().all(|(other_middle, right)| {
            other_middle != middle || relation.contains(&(left.clone(), right.clone()))
        })
    })
}

pub fn is_equivalence_relation<T: Eq + Hash + Clone>(
    universe: &HashSet<T>,
    relation: &Relation<T>,
) -> bool {
    is_reflexive(universe, relation) && is_symmetric(relation) && is_transitive(relation)
}

pub fn is_partial_order<T: Eq + Hash + Clone>(
    universe: &HashSet<T>,
    relation: &Relation<T>,
) -> bool {
    is_reflexive(universe, relation) && is_antisymmetric(relation) && is_transitive(relation)
}

pub fn equivalence_class<T: Eq + Hash + Clone>(
    universe: &HashSet<T>,
    relation: &Relation<T>,
    value: &T,
) -> HashSet<T> {
    universe
        .iter()
        .filter(|other| relation.contains(&(value.clone(), (*other).clone())))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hs<T: Eq + Hash>(values: impl IntoIterator<Item = T>) -> HashSet<T> {
        values.into_iter().collect()
    }

    #[test]
    fn detects_equivalence_relation() {
        let universe = hs([0_u8, 1, 2, 3]);
        let relation = hs([
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (0, 2),
            (2, 0),
            (1, 3),
            (3, 1),
        ]);

        assert!(is_equivalence_relation(&universe, &relation));
        assert_eq!(equivalence_class(&universe, &relation, &0), hs([0, 2]));
    }

    #[test]
    fn detects_partial_order() {
        let universe = hs([1_u8, 2, 4]);
        let relation = hs([(1, 1), (2, 2), (4, 4), (1, 2), (1, 4), (2, 4)]);

        assert!(is_partial_order(&universe, &relation));
        assert!(!is_symmetric(&relation));
    }
}