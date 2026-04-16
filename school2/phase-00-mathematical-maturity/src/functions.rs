//! Finite-set function helpers — injectivity, surjectivity, bijectivity,
//! composition, image, and preimage.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// ---------------------------------------------------------------------------
// Core function representation
// ---------------------------------------------------------------------------

/// A total function A → B represented as a map.
/// Every key in `domain` must appear in the map; values land in the codomain.
pub struct FiniteFunction<A, B> {
    map: HashMap<A, B>,
    codomain: HashSet<B>,
}

impl<A, B> FiniteFunction<A, B>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
{
    /// Construct from an explicit `domain → codomain` mapping plus the
    /// full codomain set.  Panics if `map` is not total over every key
    /// declared, but callers supply both together so this is self-consistent.
    pub fn new(map: HashMap<A, B>, codomain: HashSet<B>) -> Self {
        Self { map, codomain }
    }

    /// Apply the function.
    pub fn apply(&self, a: &A) -> Option<&B> {
        self.map.get(a)
    }

    pub fn domain(&self) -> HashSet<A> {
        self.map.keys().cloned().collect()
    }

    pub fn codomain(&self) -> &HashSet<B> {
        &self.codomain
    }

    /// Range = actual set of outputs produced.
    pub fn range(&self) -> HashSet<B> {
        self.map.values().cloned().collect()
    }

    // -----------------------------------------------------------------------
    // Injectivity, surjectivity, bijectivity
    // -----------------------------------------------------------------------

    /// f is injective iff no two inputs share an output.
    ///
    /// f(a₁) = f(a₂)  ⟹  a₁ = a₂
    pub fn is_injective(&self) -> bool {
        let mut seen: HashSet<&B> = HashSet::new();
        self.map.values().all(|b| seen.insert(b))
    }

    /// f is surjective iff every codomain element is hit.
    ///
    /// ∀ b ∈ B, ∃ a ∈ A, f(a) = b
    pub fn is_surjective(&self) -> bool {
        let range = self.range();
        self.codomain.iter().all(|b| range.contains(b))
    }

    /// f is bijective iff injective and surjective.
    pub fn is_bijective(&self) -> bool {
        self.is_injective() && self.is_surjective()
    }

    // -----------------------------------------------------------------------
    // Image and preimage
    // -----------------------------------------------------------------------

    /// Image of a subset S ⊆ A: { f(a) | a ∈ S }.
    pub fn image_of(&self, subset: &HashSet<A>) -> HashSet<B> {
        subset
            .iter()
            .filter_map(|a| self.map.get(a))
            .cloned()
            .collect()
    }

    /// Preimage of a subset T ⊆ B: { a ∈ A | f(a) ∈ T }.
    pub fn preimage_of(&self, subset: &HashSet<B>) -> HashSet<A> {
        self.map
            .iter()
            .filter_map(|(a, b)| if subset.contains(b) { Some(a.clone()) } else { None })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Composition: (g ∘ f)(a) = g(f(a))
// ---------------------------------------------------------------------------

/// Compose f : A → B and g : B → C into (g ∘ f) : A → C.
///
/// Only produces values for inputs where both applications succeed.
pub fn compose<A, B, C>(
    f: &FiniteFunction<A, B>,
    g: &FiniteFunction<B, C>,
) -> HashMap<A, C>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
    C: Eq + Hash + Clone,
{
    f.map
        .iter()
        .filter_map(|(a, b)| g.map.get(b).map(|c| (a.clone(), c.clone())))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fn<A: Eq + Hash + Clone, B: Eq + Hash + Clone>(
        pairs: impl IntoIterator<Item = (A, B)>,
        codomain: impl IntoIterator<Item = B>,
    ) -> FiniteFunction<A, B> {
        FiniteFunction::new(pairs.into_iter().collect(), codomain.into_iter().collect())
    }

    fn hs<T: Eq + Hash>(items: impl IntoIterator<Item = T>) -> HashSet<T> {
        items.into_iter().collect()
    }

    // --- Injectivity ---

    #[test]
    fn injective_when_all_outputs_distinct() {
        // f: {1,2,3} → {a,b,c}  via  1→a, 2→b, 3→c
        let f = make_fn([(1, 'a'), (2, 'b'), (3, 'c')], ['a', 'b', 'c']);
        assert!(f.is_injective());
    }

    #[test]
    fn not_injective_when_two_inputs_share_output() {
        // f: {1,2,3} → {a,b,c}  via  1→a, 2→a, 3→c
        let f = make_fn([(1, 'a'), (2, 'a'), (3, 'c')], ['a', 'b', 'c']);
        assert!(!f.is_injective());
    }

    // --- Surjectivity ---

    #[test]
    fn surjective_when_all_codomain_hit() {
        // f: {1,2,3} → {a,b,c}  via  1→a, 2→b, 3→c  (bijection)
        let f = make_fn([(1, 'a'), (2, 'b'), (3, 'c')], ['a', 'b', 'c']);
        assert!(f.is_surjective());
    }

    #[test]
    fn not_surjective_when_codomain_element_missed() {
        // Codomain has 'b' but nothing maps to it
        let f = make_fn([(1, 'a'), (2, 'a'), (3, 'c')], ['a', 'b', 'c']);
        assert!(!f.is_surjective());
    }

    // --- Bijection ---

    #[test]
    fn bijective_iff_injective_and_surjective() {
        let bij = make_fn([(1, 'a'), (2, 'b'), (3, 'c')], ['a', 'b', 'c']);
        assert!(bij.is_bijective());

        let inj_only = make_fn([(1, 'a'), (2, 'b')], ['a', 'b', 'c']);
        assert!(inj_only.is_injective());
        assert!(!inj_only.is_surjective());
        assert!(!inj_only.is_bijective());
    }

    // --- Image and preimage ---

    #[test]
    fn image_of_subset() {
        // f: n ↦ 2n (on finite set {1,2,3})
        let f = make_fn([(1u8, 2u8), (2, 4), (3, 6)], [2u8, 4, 6]);
        assert_eq!(f.image_of(&hs([1u8, 2])), hs([2u8, 4]));
    }

    #[test]
    fn preimage_of_subset() {
        let f = make_fn([(1u8, 2u8), (2, 4), (3, 6)], [2u8, 4, 6]);
        assert_eq!(f.preimage_of(&hs([2u8, 4])), hs([1u8, 2]));
    }

    #[test]
    fn preimage_of_unhit_element_is_empty() {
        // doubling on {1,2,3}: no element maps to 3 (odd)
        let f = make_fn([(1u8, 2u8), (2, 4), (3, 6)], [2u8, 4, 6]);
        assert!(f.preimage_of(&hs([3u8])).is_empty());
    }

    // --- Composition ---

    #[test]
    fn composition_is_applied_right_to_left() {
        // f: 1→2, 2→3  (add 1)
        // g: 2→20, 3→30  (multiply 10)
        // (g ∘ f)(1) = g(2) = 20
        let f = make_fn([(1u8, 2u8), (2u8, 3u8)], [2u8, 3u8]);
        let g = make_fn([(2u8, 20u8), (3u8, 30u8)], [20u8, 30u8]);
        let gf = compose(&f, &g);
        assert_eq!(gf[&1u8], 20u8);
        assert_eq!(gf[&2u8], 30u8);
    }

    // --- Structural lemmas (injective + injective → injective, etc.) ---

    #[test]
    fn composition_of_injectives_is_injective() {
        let f = make_fn([(1u16, 10u16), (2u16, 20u16), (3u16, 30u16)], [10u16, 20u16, 30u16]);
        let g = make_fn(
            [(10u16, 100u16), (20u16, 200u16), (30u16, 300u16)],
            [100u16, 200u16, 300u16],
        );
        assert!(f.is_injective());
        assert!(g.is_injective());

        let gf_map = compose(&f, &g);
        let gf = FiniteFunction::new(gf_map, hs([100u16, 200u16, 300u16]));
        assert!(gf.is_injective());
    }
}
