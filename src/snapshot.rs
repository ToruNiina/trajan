//! A module to handle snapshot contained as any kind of format in the same way.
//!
//! It requires that Snapshot should be an indexable and the Output of Index
//! implementes trajan::particle::Particle trait.
//!
//! Through this, all the `SomeSnapshot` can be used in the same way.
use crate::particle::{Attribute, Particle};
use std::option::Option;

/// A trait to provide the same accessibility to any kind of snapshots.
pub trait Snapshot<T>: std::ops::Index<usize>
where
    T: nalgebra::Scalar,
    <Self as std::ops::Index<usize>>::Output: Particle<T>,
{
    /// precision of the value (e.g. f32 or f64).
    type Value;

    /// returns how many particles are contained in the snapshot.
    fn len(&self)        -> usize;

    /// Collects mass of each particle if it exists.
    fn masses(&self)     -> Option<std::vec::Vec<T>>;

    /// Collects positions of each particle if it exists.
    fn positions(&self)  -> Option<std::vec::Vec<nalgebra::Vector3<T>>>;

    /// Collects velocities of each particle if it exists.
    fn velocities(&self) -> Option<std::vec::Vec<nalgebra::Vector3<T>>>;

    /// Collects forces of each particle if it exists.
    fn forces(&self)     -> Option<std::vec::Vec<nalgebra::Vector3<T>>>;

    /// Collects attributes of each particle if it exists.
    fn attributes(&self, name: &str) -> Option<std::vec::Vec<Attribute>>;
}
