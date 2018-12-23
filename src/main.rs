use std::io::BufRead; // for BufReader.lines
use std::str::FromStr;
#[macro_use]
extern crate soa_derive;

#[derive(Debug, PartialEq, StructOfArray)]
#[soa_derive = "Debug, PartialEq"]
pub struct XYZParticle {
    pub name  : std::string::String,
    pub coord : nalgebra::Vector3<f64>,
}
// to implement the following, we need a number of of trait boundaries and
// tons of boilerplates for error handlings.
// pub struct XYZParticle<T: nalgebra::base::Scalar> {
//     pub name  : std::string::String,
//     pub coord : nalgebra::Vector3<T>,
// }

impl FromStr for XYZParticle {
    type Err = std::num::ParseFloatError;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let elems: std::vec::Vec<&str> = line.split_whitespace().collect();
        let name = elems[0];
        let x    = elems[1].parse::<f64>()?;
        let y    = elems[2].parse::<f64>()?;
        let z    = elems[3].parse::<f64>()?;
        Ok(XYZParticle{ name: name.to_string(), coord: nalgebra::Vector3::new(x, y, z)})
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
