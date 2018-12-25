use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;
use nalgebra::base::Scalar;

use super::attributes::Attribute;
use super::attributes::Particle;

#[derive(Debug, PartialEq)]
pub struct XYZParticle<T: Scalar+FromStr> {
    pub name  : std::string::String,
    pub coord : nalgebra::Vector3<T>,
}

impl<T:Scalar+FromStr> FromStr for XYZParticle<T> {
    type Err = std::io::Error;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let elems: std::vec::Vec<&str> = line.split_whitespace().collect();
        if elems.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid line"));
        }
        let name = elems[0];
        let x    = elems[1].parse::<T>().map_err(|_| Error::new(ErrorKind::InvalidInput, "ParseNumError"))?;
        let y    = elems[2].parse::<T>().map_err(|_| Error::new(ErrorKind::InvalidInput, "ParseNumError"))?;
        let z    = elems[3].parse::<T>().map_err(|_| Error::new(ErrorKind::InvalidInput, "ParseNumError"))?;
        Ok(XYZParticle::<T>{name: name.to_string(), coord: nalgebra::Vector3::new(x, y, z)})
    }
}

impl<T:Scalar+FromStr> Particle<T> for XYZParticle<T> {
    fn mass(self) -> Option<T> {
        None
    }
    fn pos (self) -> Option<nalgebra::Vector3<T>> {
        Some(self.coord)
    }
    fn vel (self) -> Option<nalgebra::Vector3<T>> {
        None
    }
    fn acc (self) -> Option<nalgebra::Vector3<T>> {
        None
    }
    fn attr(self, name: &str) -> Option<Attribute> {
        if name == "name" {
            Some(Attribute::String(self.name))
        } else {
            None
        }
    }
}
