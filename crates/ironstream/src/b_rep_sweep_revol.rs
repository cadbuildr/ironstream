// FILE: b_rep_sweep_revol.rs
// occt: BRepSweep_Revol

use std::f64::consts::PI;

const PRECISION_ANGULAR: f64 = 1e-9;
const PRECISION_CONFUSION: f64 = 1e-7;

/// Provides natural constructors to build BRepSweep rotated swept Primitives.
/// A revol (revolution) is a shape obtained by rotating a meridian shape around an axis.
pub struct BRepSweepRevol {
    axis: GpAx1,
    angle: f64,
    copy: bool,
}

impl BRepSweepRevol {
    /// Builds the Revol of meridian S axis A and angle D.
    /// If C is true S is copied.
    pub fn new_with_angle(
        _s: &TopoDS_Shape,
        a: &GpAx1,
        d: f64,
        c: bool,
    ) -> Self {
        let angle = Self::compute_angle(d);
        if angle.abs() <= PRECISION_ANGULAR {
            panic!("BRepSweepRevol: angle is too small");
        }

        BRepSweepRevol {
            axis: a.clone(),
            angle,
            copy: c,
        }
    }

    /// Builds the Revol of meridian S axis A and angle 2*Pi.
    /// If C is true S is copied.
    pub fn new_full_revolution(
        _s: &TopoDS_Shape,
        a: &GpAx1,
        c: bool,
    ) -> Self {
        BRepSweepRevol {
            axis: a.clone(),
            angle: 2.0 * PI,
            copy: c,
        }
    }

    /// Returns the TopoDS Shape attached to the Revol.
    pub fn shape(&self) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape generated with aGenS (subShape of the generating shape).
    pub fn shape_from_gen(&self, _a_gen_s: &TopoDS_Shape) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the first shape of the revol (coinciding with the generating shape).
    pub fn first_shape(&self) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the first shape of the revol generated with aGenS.
    pub fn first_shape_from_gen(&self, _a_gen_s: &TopoDS_Shape) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape of the top of the revol.
    pub fn last_shape(&self) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape of the top of the revol generated with aGenS.
    pub fn last_shape_from_gen(&self, _a_gen_s: &TopoDS_Shape) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the axis
    pub fn axe(&self) -> GpAx1 {
        self.axis.clone()
    }

    /// Returns the angle.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Returns true if the aGenS is used in resulting Shape
    pub fn is_used(&self, _a_gen_s: &TopoDS_Shape) -> bool {
        false
    }

    fn compute_angle(d: f64) -> f64 {
        // Normalize angle to [0, 2*pi)
        let mut angle = d % (2.0 * PI);
        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        angle
    }
}

// Stub structures for type safety
#[derive(Clone)]
pub struct TopoDS_Shape {
    // Placeholder
}

impl TopoDS_Shape {
    pub fn new() -> Self {
        TopoDS_Shape {}
    }
}

/// Minimal GpAx1 stub (axis: point + direction)
#[derive(Clone, Debug)]
pub struct GpAx1 {
    origin: (f64, f64, f64),
    direction: (f64, f64, f64),
}

impl GpAx1 {
    pub fn new(origin: (f64, f64, f64), direction: (f64, f64, f64)) -> Self {
        let (dx, dy, dz) = direction;
        let mag = (dx * dx + dy * dy + dz * dz).sqrt();
        if mag.abs() < PRECISION_CONFUSION {
            panic!("GpAx1: direction magnitude is near zero");
        }
        GpAx1 {
            origin,
            direction: (dx / mag, dy / mag, dz / mag),
        }
    }

    pub fn origin(&self) -> (f64, f64, f64) {
        self.origin
    }

    pub fn direction(&self) -> (f64, f64, f64) {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revol_with_angle() {
        let origin = (0.0, 0.0, 0.0);
        let direction = (0.0, 0.0, 1.0);
        let axis = GpAx1::new(origin, direction);
        let shape = TopoDS_Shape::new();

        let revol = BRepSweepRevol::new_with_angle(&shape, &axis, PI / 2.0, false);
        assert!((revol.angle() - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_revol_full_revolution() {
        let origin = (0.0, 0.0, 0.0);
        let direction = (0.0, 0.0, 1.0);
        let axis = GpAx1::new(origin, direction);
        let shape = TopoDS_Shape::new();

        let revol = BRepSweepRevol::new_full_revolution(&shape, &axis, true);
        assert!((revol.angle() - 2.0 * PI).abs() < 1e-10);
        assert_eq!(revol.copy, true);
    }

    #[test]
    fn test_revol_axis_retrieval() {
        let origin = (1.0, 2.0, 3.0);
        let direction = (1.0, 0.0, 0.0);
        let axis = GpAx1::new(origin, direction);
        let shape = TopoDS_Shape::new();

        let revol = BRepSweepRevol::new_with_angle(&shape, &axis, PI, false);
        let retrieved_axis = revol.axe();
        let (ox, oy, oz) = retrieved_axis.origin();
        assert!((ox - 1.0).abs() < 1e-10);
        assert!((oy - 2.0).abs() < 1e-10);
        assert!((oz - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_revol_shape_methods() {
        let origin = (0.0, 0.0, 0.0);
        let direction = (0.0, 0.0, 1.0);
        let axis = GpAx1::new(origin, direction);
        let shape = TopoDS_Shape::new();

        let revol = BRepSweepRevol::new_with_angle(&shape, &axis, PI, false);

        let _ = revol.shape();
        let _ = revol.first_shape();
        let _ = revol.last_shape();
        let _ = revol.is_used(&shape);
    }

    #[test]
    #[should_panic(expected = "angle is too small")]
    fn test_revol_zero_angle() {
        let origin = (0.0, 0.0, 0.0);
        let direction = (0.0, 0.0, 1.0);
        let axis = GpAx1::new(origin, direction);
        let shape = TopoDS_Shape::new();
        let _ = BRepSweepRevol::new_with_angle(&shape, &axis, 0.0, false);
    }
}
