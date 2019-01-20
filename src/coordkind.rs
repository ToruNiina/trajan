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
