use crate::error::{ErrorKind, Error, Result};
use crate::particle::{Attribute, Particle};
use crate::coordkind::{CoordKind, Coordinate};
use std::io::BufRead; // to use read_line

#[derive(Debug, PartialEq)]
pub struct XYZParticle<T> {
    pub name : std::string::String,
    pub xyz  : Coordinate<T>,
}

impl<T> XYZParticle<T>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    pub fn new(name: std::string::String, xyz: Coordinate<T>) -> Self {
        XYZParticle{name: name, xyz: xyz}
    }

    // "H 1.00 1.00 1.00" -> XYZParticle
    fn from_line(line: &str, kind: CoordKind) -> Result<Self> {
        let elems: std::vec::Vec<&str> = line.split_whitespace().collect();

        if elems.len() != 4 {
            return Err(Error::new(failure::Context::new(ErrorKind::Format{
                error: format!("invalid XYZ format: {}", line.to_string())
            })));
        }

        let name = elems[0].to_string();
        let x    = elems[1].parse()?;
        let y    = elems[2].parse()?;
        let z    = elems[3].parse()?;

        Ok(XYZParticle::new(name, Coordinate::build(kind, x, y, z)))
    }
}

impl<T: nalgebra::Scalar> Particle<T> for XYZParticle<T> {
    type Value = T;
    fn mass(&self) -> Option<T> {
        None
    }
    fn pos(&self) -> Option<nalgebra::Vector3<T>> {
        return if let Coordinate::Position{x, y, z} = self.xyz {
            Some(nalgebra::Vector3::new(x, y, z))
        } else {
            None
        }
    }
    fn vel(&self) -> Option<nalgebra::Vector3<T>> {
        return if let Coordinate::Velocity{x, y, z} = self.xyz {
            Some(nalgebra::Vector3::new(x, y, z))
        } else {
            None
        }
    }
    fn force(&self) -> Option<nalgebra::Vector3<T>> {
        return if let Coordinate::Force{x, y, z} = self.xyz {
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
        XYZSnapshot{comment: comment, particles: particles}
    }

    pub fn which(&self) -> std::option::Option<CoordKind> {
        self.particles.first().map(|p| p.xyz.which())
    }
}

pub struct XYZReader<T, R> {
    pub kind: CoordKind,
    bufreader: std::io::BufReader<R>,
    _marker: std::marker::PhantomData<T>,
}

impl<T, R> XYZReader<T, R>
where
    R: std::io::Read,
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    pub fn new(kind: CoordKind, inner: R) -> Self {
        XYZReader::<T, R>{
            kind: kind,
            bufreader: std::io::BufReader::new(inner),
            _marker: std::marker::PhantomData
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
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    type Item = XYZSnapshot<T>;
    fn next(&mut self) -> std::option::Option<Self::Item> {
        self.read_snapshot().ok()
    }
}

pub fn read<T>(kind: CoordKind, fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(kind, file))
}

pub fn read_pos<T>(fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(CoordKind::Position, file))
}

pub fn read_vel<T>(fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(CoordKind::Velocity, file))
}

pub fn read_force<T>(fname: &str) -> Result<XYZReader<T, std::fs::File>>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    let file = std::fs::File::open(fname)?;
    Ok(XYZReader::new(CoordKind::Force, file))
}
