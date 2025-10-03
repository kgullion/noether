use noether::lattice::{BooleanAlgebra, JoinSemiLattice, MeetSemiLattice};
use noether::{LowerBounded, SymmetricDifference, UpperBounded};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};

/// Simple powerset implemented as a bitmask.
///
/// This example demonstrates how a small finite powerset forms a lattice
/// under the subset order (⊆) with join = union and meet = intersection.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord)]
pub struct Powerset<const N: usize> {
    mask: u64,
}

impl<const N: usize> Iterator for Powerset<N> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            None
        } else {
            let tz = self.mask.trailing_zeros() as usize;
            self.mask &= !(1 << tz);
            Some(tz)
        }
    }
}

impl<const N: usize> ExactSizeIterator for Powerset<N> {
    fn len(&self) -> usize {
        self.mask.count_ones() as usize
    }
}

impl<const N: usize> FromIterator<usize> for Powerset<N> {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self {
            mask: iter.into_iter().map(|i| 1 << i).fold(0, |acc, x| acc | x),
        }
    }
}

impl<const N: usize> Display for Powerset<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl<const N: usize> PartialOrd for Powerset<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let meet = self.mask & other.mask;
        match (self.mask == meet, other.mask == meet) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
        }
    }
}

impl<const N: usize> JoinSemiLattice for Powerset<N> {
    fn join(&self, other: &Self) -> Self {
        Self {
            mask: self.mask | other.mask,
        }
    }
}

impl<const N: usize> MeetSemiLattice for Powerset<N> {
    fn meet(&self, other: &Self) -> Self {
        Self {
            mask: self.mask & other.mask,
        }
    }
}

impl<const N: usize> LowerBounded for Powerset<N> {
    fn infimum() -> Self {
        Self { mask: 0 }
    }
}

impl<const N: usize> UpperBounded for Powerset<N> {
    fn supremum() -> Self {
        Self {
            mask: !0 >> (64 - N),
        }
    }
}

impl<const N: usize> BooleanAlgebra for Powerset<N> {
    fn complement(&self) -> Self {
        Self {
            mask: !self.mask & Self::supremum().mask,
        }
    }
}

impl<const N: usize> SymmetricDifference for Powerset<N> {
    fn sym_diff(&self, other: &Self) -> Self {
        Self {
            mask: self.mask ^ other.mask,
        }
    }
}

fn main() {
    // Example in the powerset of 5 elements
    type P5 = Powerset<5>;

    let a: P5 = [0, 2].into_iter().collect();
    let b = [1, 2, 4].into_iter().collect();

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a ∪ b = {}", a.join(&b));
    println!("a ∩ b = {}", a.meet(&b));
    println!(
        "a Δ b (via join+meet/complements): {}",
        a.join(&b)
            .meet(&a.complement().join(&b.complement()).complement())
    );

    // Boolean algebra laws
    assert_eq!(a.join(&a.complement()), P5::supremum());
    assert_eq!(a.meet(&a.complement()), P5::infimum());

    // Distributive lattice law check (one direction)
    let c: P5 = [0, 1].into_iter().collect();
    let lhs = a.join(&b.meet(&c));
    let rhs = a.join(&b).meet(&a.join(&c));
    assert_eq!(lhs, rhs);

    // Type-level assertions (compile-time checks by trait bounds)
    fn _assert_boolean_algebra<T: BooleanAlgebra>() {}
    _assert_boolean_algebra::<P5>();

    println!("All lattice examples succeeded.");
}
