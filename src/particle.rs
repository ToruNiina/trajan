//! A module to handle any kind of format in the same way.
//!
//! Different format contains the trajectory data in a different way.
//! It is not realistic implementing the same operations (like, calculating
//! RMSD) for all the file format. However, simultaneously, it is neither
//! realistic converting all the format into the same data container that can
//! have any kind of data because the data type may become too large.
//!
//! Here, a trait that all the `SomeParticle` struct must implement. Through
//! this, all the data format can be accessible in the uniform way.

/// An enum to have a value that might be contained in a file.
///
/// A file can have any kind of values (like, residue name, charge, spin, or
/// other kind of parameters ... ). To get some additional parameters, we need
/// this kind of struct. `std::any::Any` might be enough for this purpose, but
/// to improve the efficiency, it is implemented in enum.
#[derive(Debug)]
pub enum Attribute {
    Float(f64),
    Integer(i64),
    String(std::string::String),
    Vector(nalgebra::Vector3<f64>),
    Other(std::boxed::Box<std::any::Any + std::marker::Send + std::marker::Sync>),
}

/// A trait that should be implemented for all the `Particle` classes to provide
/// the same interface for any kind of file format
pub trait Particle<T: nalgebra::Scalar> {
    type Value;
    fn mass(&self) -> Option<T>;
    fn pos(&self) -> Option<nalgebra::Vector3<T>>;
    fn vel(&self) -> Option<nalgebra::Vector3<T>>;
    fn force(&self) -> Option<nalgebra::Vector3<T>>;
    fn attribute(&self, name: &str) -> Option<Attribute>;
}
