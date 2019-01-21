/// An enum to represent which kind of vector the data means.
///
/// Generally, which kind of vector is contained in a file cannot be deduced by
/// only a file format information.
///
/// Accessible with `operator[]` and `.x()` methods regardless of the kind.
///
/// ```
/// use trajan::coordinate::*;
/// let p = Coordinate::<f64>::build(CoordKind::Position, 1.0, 2.0, 3.0);
/// println!("{} {} {}", p.x(), p.y(), p.z());
/// println!("{} {} {}", p[0],  p[1],  p[2]);
/// ```
///
/// Also, it can be converted into nalgebra::Vector3.
/// ```
/// use trajan::coordinate::*;
/// let p = Coordinate::<f64>::build(CoordKind::Position, 1.0, 2.0, 3.0);
/// let v: nalgebra::Vector3<f64> = Into::into(p);
/// ```
#[derive(Debug, PartialEq)]
pub enum Coordinate<T> {
    Position{x:T, y:T, z:T},
    Velocity{x:T, y:T, z:T},
    Force{x:T, y:T, z:T},
}

/// A flag to represent which kind of vector is contained in a file.
///
/// Generally, which kind of vector is contained in a file cannot be deduced by
/// only a file format information.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoordKind {
    Position,
    Velocity,
    Force,
}

impl<T> Coordinate<T> {
    /// convert (x, y, z) into `Coordinate` with a value of `CoordKind`.
    pub fn build(kind: CoordKind, x: T, y: T, z: T) -> Self {
        match kind {
            CoordKind::Position => Coordinate::Position{x: x, y: y, z: z},
            CoordKind::Velocity => Coordinate::Velocity{x: x, y: y, z: z},
            CoordKind::Force    => Coordinate::Force{x: x, y: y, z: z},
        }
    }

    /// get CoordKind corresponds to the current Coordinate.
    /// If self contains Coordinate::Position, it returns CoordKind::Position.
    pub fn which(&self) -> CoordKind {
        match self {
            Coordinate::Position{..} => CoordKind::Position,
            Coordinate::Velocity{..} => CoordKind::Velocity,
            Coordinate::Force{..}    => CoordKind::Force,
        }
    }

    /// borrow x value regardless of the kind.
    pub fn x(&self) -> &T {
        match self {
            Coordinate::Position{x, ..} => x,
            Coordinate::Velocity{x, ..} => x,
            Coordinate::Force{x, ..}    => x,
        }
    }
    /// borrow y value regardless of the kind.
    pub fn y(&self) -> &T {
        match self {
            Coordinate::Position{y, ..} => y,
            Coordinate::Velocity{y, ..} => y,
            Coordinate::Force{y, ..}    => y,
        }
    }
    /// borrow z value regardless of the kind.
    pub fn z(&self) -> &T {
        match self {
            Coordinate::Position{z, ..} => z,
            Coordinate::Velocity{z, ..} => z,
            Coordinate::Force{z, ..}    => z,
        }
    }

    /// borrow mutable x value regardless of the kind.
    pub fn x_mut<'a>(&'a mut self) -> &'a mut T {
        match self {
            Coordinate::Position{x, ..} => x,
            Coordinate::Velocity{x, ..} => x,
            Coordinate::Force{x, ..}    => x,
        }
    }
    /// borrow mutable y value regardless of the kind.
    pub fn y_mut<'a>(&'a mut self) -> &'a mut T {
        match self {
            Coordinate::Position{y, ..} => y,
            Coordinate::Velocity{y, ..} => y,
            Coordinate::Force{y, ..}    => y,
        }
    }
    /// borrow mutable z value regardless of the kind.
    pub fn z_mut<'a>(&'a mut self) -> &'a mut T {
        match self {
            Coordinate::Position{z, ..} => z,
            Coordinate::Velocity{z, ..} => z,
            Coordinate::Force{z, ..}    => z,
        }
    }
}

impl<T> std::ops::Index<usize> for Coordinate<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => panic!("Coordinate: Index out of range"),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Coordinate<T> {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {
        match idx {
            0 => self.x_mut(),
            1 => self.y_mut(),
            2 => self.z_mut(),
            _ => panic!("Coordinate: Index_mut out of range"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn which_coordinate() {
        let p = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
        assert_eq!(p.which(), CoordKind::Position);

        let v = Coordinate::build(CoordKind::Velocity, 1.0, 2.0, 3.0);
        assert_eq!(v.which(), CoordKind::Velocity);

        let f = Coordinate::build(CoordKind::Force, 1.0, 2.0, 3.0);
        assert_eq!(f.which(), CoordKind::Force);
    }

    #[test]
    fn access_element() {
        let p = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
        assert_eq!(*p.x(), 1.0);
        assert_eq!(*p.y(), 2.0);
        assert_eq!(*p.z(), 3.0);

        let v = Coordinate::build(CoordKind::Velocity, 1.0, 2.0, 3.0);
        assert_eq!(*v.x(), 1.0);
        assert_eq!(*v.y(), 2.0);
        assert_eq!(*v.z(), 3.0);

        let f = Coordinate::build(CoordKind::Force, 1.0, 2.0, 3.0);
        assert_eq!(*f.x(), 1.0);
        assert_eq!(*f.y(), 2.0);
        assert_eq!(*f.z(), 3.0);
    }

    #[test]
    fn access_element_mut() {
        let mut p = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
        *p.x_mut() += 100.0;
        *p.y_mut() += 100.0;
        *p.z_mut() += 100.0;
        assert_eq!(*p.x(), 101.0);
        assert_eq!(*p.y(), 102.0);
        assert_eq!(*p.z(), 103.0);

        let mut v = Coordinate::build(CoordKind::Velocity, 1.0, 2.0, 3.0);
        *v.x_mut() += 100.0;
        *v.y_mut() += 100.0;
        *v.z_mut() += 100.0;
        assert_eq!(*v.x(), 101.0);
        assert_eq!(*v.y(), 102.0);
        assert_eq!(*v.z(), 103.0);

        let mut f = Coordinate::build(CoordKind::Force, 1.0, 2.0, 3.0);
        *f.x_mut() += 100.0;
        *f.y_mut() += 100.0;
        *f.z_mut() += 100.0;
        assert_eq!(*f.x(), 101.0);
        assert_eq!(*f.y(), 102.0);
        assert_eq!(*f.z(), 103.0);
    }


    #[test]
    fn access_element_idx() {
        let p = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);
        assert_eq!(p[2], 3.0);

        let v = Coordinate::build(CoordKind::Velocity, 1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        let f = Coordinate::build(CoordKind::Force, 1.0, 2.0, 3.0);
        assert_eq!(f[0], 1.0);
        assert_eq!(f[1], 2.0);
        assert_eq!(f[2], 3.0);
    }
    #[test]
    fn access_element_idx_mut() {
        let mut p = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
        p[0] += 100.0;
        p[1] += 100.0;
        p[2] += 100.0;
        assert_eq!(p[0], 101.0);
        assert_eq!(p[1], 102.0);
        assert_eq!(p[2], 103.0);

        let mut v = Coordinate::build(CoordKind::Velocity, 1.0, 2.0, 3.0);
        v[0] += 100.0;
        v[1] += 100.0;
        v[2] += 100.0;
        assert_eq!(v[0], 101.0);
        assert_eq!(v[1], 102.0);
        assert_eq!(v[2], 103.0);

        let mut f = Coordinate::build(CoordKind::Force, 1.0, 2.0, 3.0);
        f[0] += 100.0;
        f[1] += 100.0;
        f[2] += 100.0;
        assert_eq!(f[0], 101.0);
        assert_eq!(f[1], 102.0);
        assert_eq!(f[2], 103.0);
    }

    #[test]
    #[should_panic]
    fn access_out_of_range() {
        let p = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
        println!("{}", p[3]);
    }

    #[test]
    fn to_nalgebra() {
        {
            let p  = Coordinate::build(CoordKind::Position, 1.0, 2.0, 3.0);
            let na: nalgebra::Vector3<f64> = Into::into(p);

            assert_eq!(na, nalgebra::Vector3::new(1.0, 2.0, 3.0));
        }
        {
            let v  = Coordinate::build(CoordKind::Velocity, 1.0, 2.0, 3.0);
            let na: nalgebra::Vector3<f64> = Into::into(v);

            assert_eq!(na, nalgebra::Vector3::new(1.0, 2.0, 3.0));
        }
        {
            let f  = Coordinate::build(CoordKind::Force, 1.0, 2.0, 3.0);
            let na: nalgebra::Vector3<f64> = Into::into(f);

            assert_eq!(na, nalgebra::Vector3::new(1.0, 2.0, 3.0));
        }
    }
}
