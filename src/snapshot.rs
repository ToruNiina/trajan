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
