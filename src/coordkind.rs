#[derive(Debug, PartialEq)]
pub enum CoordKind<T> {
    Position{x:T, y:T, z:T},
    Velocity{x:T, y:T, z:T},
    Force{x:T, y:T, z:T},
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileKind {
    Position,
    Velocity,
    Force,
}

impl<T> CoordKind<T> {
    pub fn build(kind: FileKind, x: T, y: T, z: T) -> Self {
        match kind {
            FileKind::Position => CoordKind::Position{x: x, y: y, z: z},
            FileKind::Velocity => CoordKind::Velocity{x: x, y: y, z: z},
            FileKind::Force    => CoordKind::Force{x: x, y: y, z: z},
        }
    }

    pub fn which(&self) -> FileKind {
        match self {
            CoordKind::Position{x:_,y:_,z:_} => FileKind::Position,
            CoordKind::Velocity{x:_,y:_,z:_} => FileKind::Velocity,
            CoordKind::Force{x:_,y:_,z:_}    => FileKind::Force,
        }
    }
}
