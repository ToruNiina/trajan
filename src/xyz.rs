use crate::error::{Error, Result};
use crate::particle::{Attribute, Particle};
use crate::coordinate::{CoordKind, Coordinate};
use std::io::{BufRead, Write}; // to use read_line

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
            return Err(Error::invalid_format(
                format!("invalid XYZ format: {}", line)
            ));
        }

        let name = elems[0].to_string();
        let x    = elems[1].parse()?;
        let y    = elems[2].parse()?;
        let z    = elems[3].parse()?;

        Ok(XYZParticle::new(name, Coordinate::build(kind, x, y, z)))
    }
}

impl<T> std::str::FromStr for XYZParticle<T>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    type Err = Error;
    fn from_str(line: &str) -> Result<Self> {
         Self::from_line(line, CoordKind::Position)
    }
}

impl<T:std::fmt::Display> std::fmt::Display for XYZParticle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:8} {:16} {:16} {:16}",
               self.name, self.xyz[0], self.xyz[1], self.xyz[2])
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

impl<T> XYZReader<T, std::fs::File>
where
    T: std::str::FromStr,
    Error: std::convert::From<<T as std::str::FromStr>::Err>
{
    pub fn open<P>(kind: CoordKind, path: P) -> Result<Self>
    where
        P: std::convert::AsRef<std::path::Path>
    {
        let f = std::fs::File::open(path)?;
        Ok(XYZReader::<T, std::fs::File>{
            kind: kind,
            bufreader: std::io::BufReader::new(f),
            _marker: std::marker::PhantomData
        })
    }

    pub fn open_pos<P>(path: P) -> Result<Self>
    where
        P: std::convert::AsRef<std::path::Path>
    {
        let f = std::fs::File::open(path)?;
        Ok(XYZReader::<T, std::fs::File>{
            kind: CoordKind::Position,
            bufreader: std::io::BufReader::new(f),
            _marker: std::marker::PhantomData
        })
    }
    pub fn open_vel<P>(path: P) -> Result<Self>
    where
        P: std::convert::AsRef<std::path::Path>
    {
        let f = std::fs::File::open(path)?;
        Ok(XYZReader::<T, std::fs::File>{
            kind: CoordKind::Velocity,
            bufreader: std::io::BufReader::new(f),
            _marker: std::marker::PhantomData
        })
    }
    pub fn open_force<P>(path: P) -> Result<Self>
    where
        P: std::convert::AsRef<std::path::Path>
    {
        let f = std::fs::File::open(path)?;
        Ok(XYZReader::<T, std::fs::File>{
            kind: CoordKind::Force,
            bufreader: std::io::BufReader::new(f),
            _marker: std::marker::PhantomData
        })
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

pub struct XYZWriter<W: std::io::Write> {
    bufwriter: std::io::BufWriter<W>,
}

impl<W: std::io::Write> XYZWriter<W> {
    pub fn new(inner: W) -> Self {
        XYZWriter{
            bufwriter: std::io::BufWriter::new(inner),
        }
    }
    pub fn write_snapshot<T>(&mut self, ss: XYZSnapshot<T>) -> Result<()>
    where
        T: std::fmt::Display
    {
        self.bufwriter.write(ss.particles.len().to_string().as_bytes())?;
        self.bufwriter.write(b"\n")?;
        self.bufwriter.write(ss.comment.as_bytes())?;
        self.bufwriter.write(b"\n")?;
        for particle in ss.particles {
            self.bufwriter.write(particle.to_string().as_bytes())?;
            self.bufwriter.write(b"\n")?;
        }
        Ok(())
    }
}
