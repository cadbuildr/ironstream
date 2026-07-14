// FILE: brep_sweep.rs

use std::f64::consts::PI;

/// Errors returned by sweep constructors.
// occt-note: BRepSweep_Error (approximated)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepError {
    EmptyShape,
    InvalidDirection,
    ZeroAngle,
    ZeroDistance,
}

/// Linear sweep (prism) of a shape along a direction vector.
// occt-ref: BRepPrimAPI_MakePrism
pub struct BRepSweepPrism {
    direction: [f64; 3],
    length: f64,
    is_infinite: bool,
    done: bool,
}

fn normalize3(v: [f64; 3]) -> Option<[f64; 3]> {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-12 {
        None
    } else {
        Some([v[0] / len, v[1] / len, v[2] / len])
    }
}

impl BRepSweepPrism {
    /// Finite prism: extrude `direction` (normalized) by `length`.
    pub fn new(direction: [f64; 3], length: f64) -> Result<Self, SweepError> {
        if length <= 0.0 {
            return Err(SweepError::ZeroDistance);
        }
        let dir = normalize3(direction).ok_or(SweepError::InvalidDirection)?;
        Ok(Self {
            direction: dir,
            length,
            is_infinite: false,
            done: true,
        })
    }

    /// Infinite prism along `direction` (no length bound).
    pub fn infinite(direction: [f64; 3]) -> Result<Self, SweepError> {
        let dir = normalize3(direction).ok_or(SweepError::InvalidDirection)?;
        Ok(Self {
            direction: dir,
            length: f64::INFINITY,
            is_infinite: true,
            done: true,
        })
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn is_infinite(&self) -> bool {
        self.is_infinite
    }
}

/// Rotational sweep (revolution) of a shape around an axis.
// occt-ref: BRepPrimAPI_MakeRevol
pub struct BRepSweepRevolution {
    /// [origin, direction] of the revolution axis.
    axis: [[f64; 3]; 2],
    /// Sweep angle in radians; 2π for a full revolution.
    angle: f64,
    is_full: bool,
    done: bool,
}

impl BRepSweepRevolution {
    /// Partial or full revolution through `angle` radians around the axis
    /// defined by `origin` and `direction`.
    pub fn new(
        origin: [f64; 3],
        direction: [f64; 3],
        angle: f64,
    ) -> Result<Self, SweepError> {
        if angle.abs() < 1e-12 {
            return Err(SweepError::ZeroAngle);
        }
        let dir = normalize3(direction).ok_or(SweepError::InvalidDirection)?;
        let is_full = (angle.abs() - 2.0 * PI).abs() < 1e-9 || angle.abs() >= 2.0 * PI;
        Ok(Self {
            axis: [origin, dir],
            angle,
            is_full,
            done: true,
        })
    }

    /// Convenience constructor for a complete 360° revolution.
    pub fn full_revolution(
        origin: [f64; 3],
        direction: [f64; 3],
    ) -> Result<Self, SweepError> {
        Self::new(origin, direction, 2.0 * PI)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn axis_origin(&self) -> [f64; 3] {
        self.axis[0]
    }

    pub fn axis_direction(&self) -> [f64; 3] {
        self.axis[1]
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prism_z_axis_stores_normalized_direction() {
        let p = BRepSweepPrism::new([0.0, 0.0, 5.0], 3.0).unwrap();
        let d = p.direction();
        assert!((d[2] - 1.0).abs() < 1e-12);
        assert!(d[0].abs() < 1e-12 && d[1].abs() < 1e-12);
        assert_eq!(p.length(), 3.0);
        assert!(p.is_done());
        assert!(!p.is_infinite());
    }

    #[test]
    fn prism_zero_length_errors() {
        assert!(matches!(
            BRepSweepPrism::new([0.0, 0.0, 1.0], 0.0),
            Err(SweepError::ZeroDistance)
        ));
        assert!(matches!(
            BRepSweepPrism::new([0.0, 0.0, 1.0], -1.0),
            Err(SweepError::ZeroDistance)
        ));
    }

    #[test]
    fn prism_zero_direction_errors() {
        assert!(matches!(
            BRepSweepPrism::new([0.0, 0.0, 0.0], 1.0),
            Err(SweepError::InvalidDirection)
        ));
    }

    #[test]
    fn prism_infinite_flag() {
        let p = BRepSweepPrism::infinite([1.0, 0.0, 0.0]).unwrap();
        assert!(p.is_infinite());
        assert!(p.length().is_infinite());
        assert!(p.is_done());
    }

    #[test]
    fn prism_infinite_zero_direction_errors() {
        assert!(matches!(
            BRepSweepPrism::infinite([0.0, 0.0, 0.0]),
            Err(SweepError::InvalidDirection)
        ));
    }

    #[test]
    fn revolution_full_around_y() {
        let r = BRepSweepRevolution::full_revolution([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
        assert!(r.is_done());
        assert!(r.is_full());
        assert!((r.angle() - 2.0 * PI).abs() < 1e-9);
        let d = r.axis_direction();
        assert!((d[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn revolution_partial_not_full() {
        let r = BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 2.0).unwrap();
        assert!(!r.is_full());
        assert!((r.angle() - PI / 2.0).abs() < 1e-12);
    }

    #[test]
    fn revolution_zero_angle_errors() {
        assert!(matches!(
            BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
            Err(SweepError::ZeroAngle)
        ));
    }

    #[test]
    fn revolution_zero_direction_errors() {
        assert!(matches!(
            BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], PI),
            Err(SweepError::InvalidDirection)
        ));
    }

    #[test]
    fn revolution_axis_origin_stored() {
        let origin = [1.0, 2.0, 3.0];
        let r = BRepSweepRevolution::new(origin, [0.0, 0.0, 1.0], PI).unwrap();
        let o = r.axis_origin();
        assert!((o[0] - 1.0).abs() < 1e-12);
        assert!((o[1] - 2.0).abs() < 1e-12);
        assert!((o[2] - 3.0).abs() < 1e-12);
    }
}
