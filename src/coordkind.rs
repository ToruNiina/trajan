#[derive(Debug, PartialEq)]
pub enum Coordinate<T> {
    Position{x:T, y:T, z:T},
    Velocity{x:T, y:T, z:T},
    Force{x:T, y:T, z:T},
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoordKind {
    Position,
    Velocity,
    Force,
}

impl<T> Coordinate<T> {
    pub fn build(kind: CoordKind, x: T, y: T, z: T) -> Self {
        match kind {
            CoordKind::Position => Coordinate::Position{x: x, y: y, z: z},
            CoordKind::Velocity => Coordinate::Velocity{x: x, y: y, z: z},
            CoordKind::Force    => Coordinate::Force{x: x, y: y, z: z},
        }
    }

    pub fn which(&self) -> CoordKind {
        match self {
            Coordinate::Position{x:_,y:_,z:_} => CoordKind::Position,
            Coordinate::Velocity{x:_,y:_,z:_} => CoordKind::Velocity,
            Coordinate::Force{x:_,y:_,z:_}    => CoordKind::Force,
        }
    }
}

impl<T> Into<nalgebra::Vector3<T>> for Coordinate<T>
where
    T: nalgebra::Scalar
{
    fn into(self) -> nalgebra::Vector3<T> {
        match self {
            Coordinate::Position{x, y, z} => nalgebra::Vector3::new(x, y, z),
            Coordinate::Velocity{x, y, z} => nalgebra::Vector3::new(x, y, z),
            Coordinate::Force{x, y, z}    => nalgebra::Vector3::new(x, y, z),
        }
    }
}
