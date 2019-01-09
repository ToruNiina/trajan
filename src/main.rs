// #[macro_use] extern crate failure;

/* ----------- error type ----------- */

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "Format error")]
    Format,
}

/* ----------- failure boilerplate ----------- */

use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

/* ----------- conversion between errors ----------- */

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Error {
        Error {
            inner: error.context(ErrorKind::Format),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Error {
        Error {
            inner: error.context(ErrorKind::Format),
        }
    }
}

/* ----------- attribute ----------- */

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
    Vector(nalgebra::Vector3<f64>)
}

pub trait Particle {
    fn mass(self) -> Option<f64>;
    fn pos(self)  -> Option<nalgebra::Vector3<f64>>;
    fn attr(self, name: &str) -> Option<Attribute>;
}

/* ----------- xyz format ----------- */

use std::io::BufRead; // for BufReader.lines
use std::str::FromStr;

#[macro_use]
extern crate soa_derive;

#[derive(Debug, PartialEq, StructOfArray)]
#[soa_derive = "Debug, PartialEq"]
pub struct XYZParticle {
    pub name : std::string::String,
    pub pos  : nalgebra::Vector3<f64>,
}

impl FromStr for XYZParticle {
    type Err = Error;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let elems: std::vec::Vec<&str> = line.split_whitespace().collect();

        if elems.len() != 4 {
            return Err(Error::new(Context::new(ErrorKind::Format)));
        }
        let name = elems[0];
        let x    = elems[1].parse::<f64>()?;
        let y    = elems[2].parse::<f64>()?;
        let z    = elems[3].parse::<f64>()?;
        Ok(XYZParticle{name: name.to_string(), pos: nalgebra::Vector3::new(x,y,z)})
    }
}

impl Particle for XYZParticle {
    fn mass(self) -> Option<f64> {
        None
    }
    fn pos (self) -> Option<nalgebra::Vector3<f64>> {
        Some(self.pos)
    }
    fn attr(self, name: &str) -> Option<Attribute> {
        if name == "name" {
            Some(Attribute::String(self.name))
        } else {
            None
        }
    }
}

fn read_xyz_snapshot(filename: &str) -> std::result::Result<XYZParticleVec, Error> {
    let mut fbuf = std::io::BufReader::new(std::fs::File::open(filename)?);

    let mut line = std::string::String::new();
    fbuf.read_line(&mut line)?;

    let number_of_particles = &line.trim().parse::<usize>()?;
    line.clear();

    fbuf.read_line(&mut line)?; // comment line
    line.clear();

    let mut snapshot = XYZParticleVec::with_capacity(*number_of_particles);
    for _ in 0 .. *number_of_particles {
        fbuf.read_line(&mut line)?;
        let particle = line.parse::<XYZParticle>()?;
        snapshot.push(particle);
        line.clear();
    }
    Ok(snapshot)
}

fn main() {
    let xyz_snapshot = read_xyz_snapshot("example.xyz").expect("read xyz file");

    println!("found {} particles", xyz_snapshot.len());
    for particle in xyz_snapshot.iter() {
        println!("{:?}", particle);
    }
}
