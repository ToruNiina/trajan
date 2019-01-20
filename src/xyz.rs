use crate::error::{ErrorKind, Error, Result};
use crate::particle::{Attribute, Particle};
use crate::coordkind::{FileKind, CoordKind};
use std::io::BufRead; // to use read_line

#[derive(Debug, PartialEq)]
pub struct XYZParticle<T> {
    pub name : std::string::String,
    pub xyz  : CoordKind<T>,
}

impl<T> XYZParticle<T>
where
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    pub fn new(name: std::string::String, xyz: CoordKind<T>) -> Self {
        XYZParticle::<T>{name: name, xyz: xyz}
    }

    // "H 1.00 1.00 1.00" -> XYZParticle
    fn from_line(line: &str, kind: FileKind) -> Result<Self> {
        let elems: std::vec::Vec<&str> = line.split_whitespace().collect();

        if elems.len() != 4 {
            return Err(Error::new(failure::Context::new(ErrorKind::Format{
                error: format!("invalid XYZ format: {}", line.to_string())
            })));
        }

        let name = elems[0].to_string();
        let x    = elems[1].parse::<T>()?;
        let y    = elems[2].parse::<T>()?;
        let z    = elems[3].parse::<T>()?;

        Ok(XYZParticle::<T>::new(name, match kind {
            FileKind::Position => CoordKind::Position::<T>{x:x, y:y, z:z},
            FileKind::Velocity => CoordKind::Velocity::<T>{x:x, y:y, z:z},
            FileKind::Force    => CoordKind::Force::<T>{x:x, y:y, z:z},
        }))
    }
}

impl<T: nalgebra::Scalar> Particle<T> for XYZParticle<T> {
    type Value = T;
    fn mass(&self) -> Option<T> {
        None
    }
    fn pos(&self) -> Option<nalgebra::Vector3<T>> {
        return if let CoordKind::Position::<T>{x, y, z} = self.xyz {
            Some(nalgebra::Vector3::new(x, y, z))
        } else {
            None
        }
    }
    fn vel(&self) -> Option<nalgebra::Vector3<T>> {
        return if let CoordKind::Velocity::<T>{x, y, z} = self.xyz {
            Some(nalgebra::Vector3::new(x, y, z))
        } else {
            None
        }
    }
    fn frc(&self) -> Option<nalgebra::Vector3<T>> {
        return if let CoordKind::Force::<T>{x, y, z} = self.xyz {
            Some(nalgebra::Vector3::new(x, y, z))
        } else {
            None
        }
    }
    fn attribute(&self, name: std::string::String) -> Option<Attribute> {
        return match name.as_str() {
            "name" => Some(Attribute::String(self.name.clone())),
            "elem" => Some(Attribute::String(self.name.clone())),
            _ => None,
        }
    }
}

pub struct XYZSnapshot<T> {
    pub comment:   std::string::String,
    pub particles: std::vec::Vec<XYZParticle<T>>,
}

impl<T> XYZSnapshot<T> {
    pub fn new(comment: std::string::String,
               particles: std::vec::Vec<XYZParticle<T>>) -> Self {
        XYZSnapshot::<T>{
            comment: comment,
            particles: particles,
        }
    }
}

pub struct XYZReader<T, R> {
    pub kind: FileKind,
    bufreader: std::io::BufReader<R>,
    _marker: std::marker::PhantomData<T>,
}

impl<T, R> XYZReader<T, R>
where
    R: std::io::Read,
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    pub fn new(kind: FileKind, inner: R) -> Self {
        XYZReader::<T, R>{
            kind: kind,
            bufreader: std::io::BufReader::new(inner),
            _marker: std::marker::PhantomData::<T>
        }
    }

    pub fn read_snapshot(&mut self) -> Result<XYZSnapshot<T>> {
        let mut line = std::string::String::new();

        self.bufreader.read_line(&mut line)?;
        let num = line.trim().parse::<usize>()?;
        line.clear();

        // comment line
        self.bufreader.read_line(&mut line)?;
        let comment = line.clone();
        line.clear();

        let mut particles = std::vec::Vec::with_capacity(num);
        for _ in 0 .. num {
            self.bufreader.read_line(&mut line)?;
            particles.push(XYZParticle::from_line(line.as_str(), self.kind)?);
            line.clear();
        }
        Ok(XYZSnapshot::new(comment, particles))
    }
}

impl<T, R> std::iter::Iterator for XYZReader<T, R>
where
    R: std::io::Read,
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    type Item = XYZSnapshot<T>;
    fn next(&mut self) -> std::option::Option<Self::Item> {
        self.read_snapshot().ok()
    }
}

pub fn open<T>(kind: FileKind, fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(kind, file))
}

pub fn open_pos<T>(fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(FileKind::Position, file))
}

pub fn open_vel<T>(fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(FileKind::Velocity, file))
}

pub fn open_force<T>(fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr<Err = std::num::ParseFloatError>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(FileKind::Force, file))
}
