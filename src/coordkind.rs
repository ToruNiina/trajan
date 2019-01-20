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
