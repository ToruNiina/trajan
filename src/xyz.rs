/* ----------- xyz format ----------- */

use crate::error::*;
use std::io::BufRead; // for BufReader.lines
use std::str::FromStr;
use failure::Context;

#[derive(Debug, PartialEq)]
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

pub fn read_xyz_snapshot(filename: &str) -> std::result::Result<std::vec::Vec<XYZParticle>, Error> {
    let mut fbuf = std::io::BufReader::new(std::fs::File::open(filename)?);

    let mut line = std::string::String::new();
    fbuf.read_line(&mut line)?;

    let number_of_particles = &line.trim().parse::<usize>()?;
    line.clear();

    fbuf.read_line(&mut line)?; // comment line
    line.clear();

    let mut snapshot = std::vec::Vec::with_capacity(*number_of_particles);
    for _ in 0 .. *number_of_particles {
        fbuf.read_line(&mut line)?;
        let particle = line.parse::<XYZParticle>()?;
        snapshot.push(particle);
        line.clear();
    }
    Ok(snapshot)
}

