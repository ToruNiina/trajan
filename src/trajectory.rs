//! A module to handle trajectory contained as any kind of format in the same way.
//!
//! It requires that Trajectory should be an indexable and the Output of Index
//! implementes trajan::snapshot::Snapshot trait.
//!
//! Through this, all the `SomeSnapshot` can be used in the same way.
use crate::particle::Particle;
use crate::snapshot::Snapshot;

/// A trait to provide the same accessibility to any kind of snapshots.
pub trait Trajectory<T>: std::ops::Index<usize>
where
    T: nalgebra::Scalar,
    <Self as std::ops::Index<usize>>::Output: Snapshot<T>,
    <<Self as std::ops::Index<usize>>::Output as std::ops::Index<usize>>::Output: Particle<T>,
{
    /// precision of the value (e.g. f32 or f64).
    type Value;

    /// returns how many snapshots are contained in the trajectory.
    fn len(&self) -> usize;
}
