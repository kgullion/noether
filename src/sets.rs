//! Set theory foundations.
//!
//! This module provides the foundational Set trait which forms the basis
//! of all algebraic structures in Noether.

/// Represents a mathematical set as defined in Zermelo-Fraenkel set theory with Choice (ZFC).
///
/// # Formal Notation
/// - ∅: empty set
/// - ∈: element of
/// - ⊆: subset of
/// - ∪: union
/// - ∩: intersection
/// - \: set difference
/// - Δ: symmetric difference
/// - |A|: cardinality of set A
///
/// # Axioms of ZFC
/// 1. Extensionality: ∀A∀B(∀x(x ∈ A ↔ x ∈ B) → A = B)
/// 2. Empty Set: ∃A∀x(x ∉ A)
/// 3. Pairing: ∀a∀b∃A∀x(x ∈ A ↔ x = a ∨ x = b)
/// 4. Union: ∀F∃A∀x(x ∈ A ↔ ∃B(x ∈ B ∧ B ∈ F))
/// 5. Power Set: ∀A∃P∀x(x ∈ P ↔ x ⊆ A)
/// 6. Infinity: ∃A(∅ ∈ A ∧ ∀x(x ∈ A → x ∪ {x} ∈ A))
/// 7. Separation: ∀A∃B∀x(x ∈ B ↔ x ∈ A ∧ φ(x)) for any formula φ
/// 8. Replacement: ∀A(∀x∀y∀z((x ∈ A ∧ φ(x,y) ∧ φ(x,z)) → y = z) → ∃B∀y(y ∈ B ↔ ∃x(x ∈ A ∧ φ(x,y))))
/// 9. Foundation: ∀A(A ≠ ∅ → ∃x(x ∈ A ∧ x ∩ A = ∅))
/// 10. Choice: ∀A(∅ ∉ A → ∃f:A → ∪A ∀B∈A(f(B) ∈ B))
pub trait Set: Sized + PartialEq {}

// Blanket implementation for any type that satisfies the trait bounds
impl<T: PartialEq> Set for T {}

/// Trait for types that admit a distinguished least element (bottom, ⊥).
///
/// # Mathematical Definition
/// For a partially ordered set (P, ≤), an element ⊥ ∈ P is a least element if:
///
/// ∀ x ∈ P, ⊥ ≤ x
///
/// When the least element exists it is the infimum of the whole set P and is
/// often denoted ⊥ (bottom). This trait expresses that the implementing type
/// provides a canonical least element for the type as a whole.
///
/// # Properties
/// - Uniqueness: a least element (when it exists) is unique.
/// - Lattice usage: in lattices the existence of ⊥ makes the lattice lower-bounded.
/// - Not all posets have a least element -- implement this trait only when such an
///   element is defined for the type.
///
/// # Examples
/// - For the power set P(X) ordered by ⊆, the empty set ∅ is ⊥.
/// - For bounded numeric intervals, the lower endpoint is ⊥ when present.
pub trait LowerBounded: Set + PartialOrd {
    /// Return the distinguished least element (infimum / bottom) for this type.
    fn infimum() -> Self;
}

/// Trait for types that admit a distinguished greatest element (top, ⊤).
///
/// # Mathematical Definition
/// For a partially ordered set (P, ≤), an element ⊤ ∈ P is a greatest element if:
///
/// ∀ x ∈ P, x ≤ ⊤
///
/// When the greatest element exists it is the supremum of the whole set P and
/// is often denoted ⊤ (top). This trait expresses that the implementing type
/// provides a canonical greatest element for the type as a whole.
///
/// # Properties
/// - Uniqueness: a greatest element (when it exists) is unique.
/// - Lattice usage: in lattices the existence of ⊤ makes the lattice upper-bounded.
/// - Not all posets have a greatest element -- implement this trait only when such an
///   element is defined for the type.
///
/// # Examples
/// - For the power set P(X) ordered by ⊆, the universal set X is ⊤.
/// - For bounded numeric intervals, the upper endpoint is ⊤ when present.
pub trait UpperBounded: Set + PartialOrd {
    /// Return the distinguished greatest element (supremum / top) for this type.
    fn supremum() -> Self;
}

/// Trait describing the symmetric difference operation (Δ) between two elements.
///
/// # Mathematical Definition
/// Given two sets A and B, the symmetric difference is defined as:
///
/// A Δ B = (A \\ B) ∪ (B \\ A)
///
/// which is the set of elements that belong to exactly one of A or B.
///
/// # Algebraic Properties
/// - Commutative: A Δ B = B Δ A
/// - Associative: (A Δ B) Δ C = A Δ (B Δ C)
/// - Identity: A Δ ∅ = A (the empty set ∅ acts as the identity)
/// - Self-inverse: A Δ A = ∅ (every element is its own inverse under Δ)
/// - For the power set of X, (P(X), Δ) is an abelian group isomorphic to the
///   vector space (over GF(2)) of indicator functions on X.
///
/// Implement this trait when a type naturally supports a symmetric-difference
/// style binary operation. Implementations should preserve the above algebraic
/// laws wherever they are meaningful for the type.
pub trait SymmetricDifference: Set {
    /// Compute the symmetric difference of `a` and `b`.
    fn sym_diff(&self, b: &Self) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define some test types to validate the Set trait implementation
    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_set_implementation_for_primitives() {
        // Test that primitive types implement Set
        fn assert_is_set<T: Set>(_: &T) {}

        assert_is_set(&42i32);
        assert_is_set(&"hello");
        assert_is_set(&std::f64::consts::PI);
        assert_is_set(&true);
        assert_is_set(&[1, 2, 3]);
    }

    #[test]
    fn test_set_implementation_for_custom_types() {
        // Test that custom types implement Set
        fn assert_is_set<T: Set>(_: &T) {}

        let point = Point { x: 1, y: 2 };
        assert_is_set(&point);
    }

    #[test]
    fn test_set_equality() {
        // Test that Set equality works as expected
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };
        let c = Point { x: 3, y: 4 };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
