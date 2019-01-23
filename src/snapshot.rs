//! A module to handle trajectory contained as any kind of format in the same way.
//!
//! It requires that Snapshot should be an indexable and the Output of Index
//! implementes trajan::particle::Particle trait.
//!
//! Through this, all the `SomeSnapshot` can be used in the same way.
use crate::particle::{Attribute, Particle};
use std::option::Option;

pub trait Snapshot<T>: std::ops::Index<usize>
where
    T: nalgebra::Scalar,
    <Self as std::ops::Index<usize>>::Output: Particle<T>,
{
    type Value;
    fn len(&self)        -> usize;
    fn mass(&self)       -> Option<std::vec::Vec<T>>;
    fn positions(&self)  -> Option<std::vec::Vec<nalgebra::Vector3<T>>>;
    fn velocities(&self) -> Option<std::vec::Vec<nalgebra::Vector3<T>>>;
    fn forces(&self)     -> Option<std::vec::Vec<nalgebra::Vector3<T>>>;
    fn attributes(&self, name: &str) -> Option<std::vec::Vec<Attribute>>;
}
