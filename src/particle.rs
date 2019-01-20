pub enum Attribute {
    Float(f64),
    Integer(i64),
    String(std::string::String),
    Vector(nalgebra::Vector3<f64>),
}

pub trait Particle<T: nalgebra::Scalar> {
    type Value;
    fn mass(&self) -> Option<T>;
    fn pos(&self) -> Option<nalgebra::Vector3<T>>;
    fn vel(&self) -> Option<nalgebra::Vector3<T>>;
    fn frc(&self) -> Option<nalgebra::Vector3<T>>;
    fn attribute(&self, name: std::string::String) -> Option<Attribute>;
}
