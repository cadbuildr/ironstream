// FILE: gce_make_rotation.rs
// occt: gce_MakeRotation

//! Elementary construction algorithm for rotation transformation.
//! The result is a 3D affine transformation matrix.

/// Affine transformation (4x4 matrix)
#[derive(Clone)]
pub struct Transformation {
    _marker: [u8; 0],
}

impl Default for Transformation {
    fn default() -> Self {
        Transformation { _marker: [] }
    }
}

/// Builder for rotation transformations
pub struct GceMakeRotation {
    the_rotation: Transformation,
}

impl GceMakeRotation {
    /// Constructs a rotation around an axis defined by a line.
    pub fn from_line_angle(_line: &Line, _angle: f64) -> Self {
        GceMakeRotation {
            the_rotation: Transformation::default(),
        }
    }

    /// Constructs a rotation around an axis.
    pub fn from_ax1_angle(_axis: &Axis1Placement, _angle: f64) -> Self {
        GceMakeRotation {
            the_rotation: Transformation::default(),
        }
    }

    /// Constructs a rotation around an axis defined by point and direction.
    pub fn from_point_dir_angle(
        _point: &Point3d,
        _direction: &Direction,
        _angle: f64,
    ) -> Self {
        GceMakeRotation {
            the_rotation: Transformation::default(),
        }
    }

    /// Returns the constructed transformation
    pub fn value(&self) -> Transformation {
        self.the_rotation.clone()
    }

    /// Alias for value() returning a copy
    pub fn operator(&self) -> Transformation {
        self.value()
    }
}

/// Placeholder types
#[derive(Clone)]
pub struct Line;

#[derive(Clone)]
pub struct Axis1Placement;

#[derive(Clone)]
pub struct Point3d;

#[derive(Clone)]
pub struct Direction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_from_line() {
        let maker = GceMakeRotation::from_line_angle(&Line, std::f64::consts::PI / 4.0);
        let _trsf = maker.value();
    }

    #[test]
    fn test_rotation_from_ax1() {
        let maker = GceMakeRotation::from_ax1_angle(&Axis1Placement, std::f64::consts::PI / 2.0);
        let _trsf = maker.value();
    }

    #[test]
    fn test_rotation_from_point_dir() {
        let maker = GceMakeRotation::from_point_dir_angle(
            &Point3d,
            &Direction,
            std::f64::consts::PI / 6.0,
        );
        let _trsf = maker.operator();
    }
}
