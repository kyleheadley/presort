//! A crate for permuted vectors.
//!
//! Permuted vectors consist of a vector, together with a permutation of its elements.
//! In particular, `vec.sort()` sorts the permutation, not the vector.
//! This allows the vector to be updated, and
//! if the updates preserve sort order, then the next `vec.sort()`
//! will be O(n) rather than O(n log n).

#[cfg(feature = "serde")]
extern crate serde;
   
#[cfg(feature = "heapsize")]
extern crate heapsize;

mod presort;
pub use presort::*;
pub mod inc_tree;

