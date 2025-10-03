//! Lattice-theoretic primitives.
//!
//! This module provides the foundational traits for join/meet semi-lattices
//! and lattices. The traits are deliberately minimal: they describe the
//! operations required by the algebraic structure and leave verification of
//! laws (associativity, commutativity, absorption, distributivity, etc.) to
//! implementors and tests.
//!
//! Notation:
//! - ∨: join (least upper bound)
//! - ∧: meet (greatest lower bound)
//! - ⊥: least element (bottom)
//! - ⊤: greatest element (top)
use crate::{LowerBounded, Set, UpperBounded};

/// A join-semilattice is a partially ordered set in which any two elements
/// have a least upper bound (join, ∨).
///
/// # Mathematical Definition
/// For a poset (P, ≤), a binary operation ∨ is a join if for all a, b ∈ P:
///
/// 1. a ≤ a ∨ b and b ≤ a ∨ b (upper bound)
/// 2. if a ≤ c and b ≤ c then a ∨ b ≤ c (least such upper bound)
///
/// # Properties
/// - Commutative: a ∨ b = b ∨ a
/// - Associative: (a ∨ b) ∨ c = a ∨ (b ∨ c)
/// - Idempotent: a ∨ a = a
pub trait JoinSemiLattice: Set + PartialOrd {
    /// Compute the join (least upper bound) of `self` and `other`.
    fn join(&self, other: &Self) -> Self;
}

/// A meet-semilattice is a partially ordered set in which any two elements
/// have a greatest lower bound (meet, ∧).
///
/// # Mathematical Definition
/// For a poset (P, ≤), a binary operation ∧ is a meet if for all a, b ∈ P:
///
/// 1. a ∧ b ≤ a and a ∧ b ≤ b (lower bound)
/// 2. if c ≤ a and c ≤ b then c ≤ a ∧ b (greatest such lower bound)
///
/// # Properties
/// - Commutative: a ∧ b = b ∧ a
/// - Associative: (a ∧ b) ∧ c = a ∧ (b ∧ c)
/// - Idempotent: a ∧ a = a
pub trait MeetSemiLattice: Set + PartialOrd {
    /// Compute the meet (greatest lower bound) of `self` and `other`.
    fn meet(&self, other: &Self) -> Self;
}

/// A lattice is a structure that is both a join- and meet-semilattice.
///
/// # Mathematical Definition
/// A type T is a lattice if it implements both a join and a meet satisfying
/// the usual lattice laws (absorption, associativity, commutativity, idempotence).
///
/// # Usage
/// The blanket implementation is provided for any type that implements both
/// `JoinSemiLattice` and `MeetSemiLattice`.
pub trait Lattice: JoinSemiLattice + MeetSemiLattice {}
impl<T: JoinSemiLattice + MeetSemiLattice> Lattice for T {}

/// A distributive lattice is a lattice in which meet and join distribute over each other.
///
/// # Mathematical Definition
/// A lattice (L, ∨, ∧) is distributive if for all a, b, c ∈ L:
///
/// a ∨ (b ∧ c) = (a ∨ b) ∧ (a ∨ c)
/// a ∧ (b ∨ c) = (a ∧ b) ∨ (a ∧ c)
///
/// # Properties
/// - Distributivity implies the lattice is well-behaved with respect to expansions
///   and factorizations of expressions involving ∨ and ∧.
/// - Many familiar lattices (e.g., powerset lattices ordered by ⊆) are distributive.
pub trait DistributiveLattice: Lattice + Ord {}
impl<T: Lattice + Ord> DistributiveLattice for T {}

/// Boolean algebras are distributive lattices with a complement and bounds.
///
/// # Mathematical Definition
/// A Boolean algebra is a distributive lattice (L, ∨, ∧, ⊥, ⊤) equipped with a
/// complement operation ¬ such that for all a ∈ L:
///
/// a ∨ ¬a = ⊤ and a ∧ ¬a = ⊥
///
/// # Properties
/// - Every Boolean algebra is a complemented distributive lattice.
/// - In the powerset example, complement corresponds to set-theoretic complement
///   with respect to the universal set, ⊤.
///
/// The trait requires `LowerBounded` and `UpperBounded` to ensure the presence
/// of ⊥ and ⊤ respectively.
pub trait BooleanAlgebra: DistributiveLattice + LowerBounded + UpperBounded {
    /// Return the complement of `self` (logical negation / set complement).
    fn complement(&self) -> Self;
}
