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
    fn pos (self) -> Option<nalgebra::Vector3<f64>>;
    fn attr(self, name: &str) -> Option<Attribute>;
}

use std::io::BufRead; // for BufReader.lines
use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;

#[macro_use]
extern crate soa_derive;

#[derive(Debug, PartialEq, StructOfArray)]
#[soa_derive = "Debug, PartialEq"]
pub struct XYZParticle {
    pub name  : std::string::String,
    pub coord : nalgebra::Vector3<f64>,
}

impl FromStr for XYZParticle {
    type Err = std::io::Error;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let elems: std::vec::Vec<&str> = line.split_whitespace().collect();
        if elems.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid line"));
        }
        let name = elems[0];
        let x    = elems[1].parse::<f64>().map_err(|_| Error::new(ErrorKind::InvalidInput, "ParseNumError"))?;
        let y    = elems[2].parse::<f64>().map_err(|_| Error::new(ErrorKind::InvalidInput, "ParseNumError"))?;
        let z    = elems[3].parse::<f64>().map_err(|_| Error::new(ErrorKind::InvalidInput, "ParseNumError"))?;
        Ok(XYZParticle{name: name.to_string(), coord: nalgebra::Vector3::new(x, y, z)})
    }
}

impl Particle for XYZParticle {
    fn mass(self) -> Option<f64> {
        None
    }
    fn pos (self) -> Option<nalgebra::Vector3<f64>> {
        Some(self.coord)
    }
    fn attr(self, name: &str) -> Option<Attribute> {
        if name == "name" {
            Some(Attribute::String(self.name))
        } else {
            None
        }
    }
}

fn read_xyz_snapshot(filename: &str) -> std::io::Result<XYZParticleVec> {
    let mut fbuf = std::io::BufReader::new(std::fs::File::open(filename)?);

    let mut line = std::string::String::new();
    fbuf.read_line(&mut line)?;
    println!("1st line: {}", line);

    let number_of_particles = &line.trim().parse::<usize>()
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "first line does not contain the number of particles"
            ))?;
    line.clear();

    fbuf.read_line(&mut line)?;
    println!("2nd line: {}", line);
    line.clear();

    let mut snapshot = XYZParticleVec::with_capacity(*number_of_particles);
    for _ in 0..*number_of_particles {
        fbuf.read_line(&mut line)?;
        snapshot.push(line.parse::<XYZParticle>().map_err(
            |_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "failed to read xyz particle"
            ))?);
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
