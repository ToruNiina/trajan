use nalgebra::base::Scalar;
use std::string::String;
use std::option::Option;

/// Particle can have any kind of attributes that has different type each other,
/// like atom_name: String, charge: f64, and residue_id: i64. We can make no
/// assumption about a type of an attribute, so here we use enum to get an
/// attribute value.
pub enum Attribute {
    Integer(i64),
    Float(f64),
    String(String),
}

pub trait Particle<T: Scalar> {
    fn mass(self) -> Option<T>;
    fn pos (self) -> Option<nalgebra::Vector3<T>>;
    fn vel (self) -> Option<nalgebra::Vector3<T>>;
    fn acc (self) -> Option<nalgebra::Vector3<T>>;
    fn attr(self, name: &str) -> Option<Attribute>;
}
