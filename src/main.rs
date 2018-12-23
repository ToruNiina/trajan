use std::io::BufRead; // for BufReader.lines
use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;
use nalgebra::base::Scalar;
// #[macro_use]
// extern crate soa_derive;

// #[derive(Debug, PartialEq, StructOfArray)]
// #[soa_derive = "Debug, PartialEq"]
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

fn read_xyz_snapshot<T:Scalar+FromStr>(filename: &str) -> std::io::Result<std::vec::Vec<XYZParticle<T>>> {
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

    let mut snapshot = std::vec::Vec::with_capacity(*number_of_particles);
    for _ in 0..*number_of_particles {
        fbuf.read_line(&mut line)?;
        snapshot.push(line.parse::<XYZParticle<T>>().map_err(
            |_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "failed to read xyz particle"
            ))?);
        line.clear();
    }
    Ok(snapshot)
}

fn main() {
    let xyz_snapshot = read_xyz_snapshot::<f64>("example.xyz").expect("read xyz file");

    println!("found {} particles", xyz_snapshot.len());
    for particle in xyz_snapshot.iter() {
        println!("{:?}", particle);
    }
}
