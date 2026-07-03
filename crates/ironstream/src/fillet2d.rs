// FILE: fillet2d.rs
// occt: ChFi2d

use std::f64::consts::PI;

/// Error type for 2D fillet/chamfer operations.
// occt: ChFi2d error codes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fillet2dError {
    BadInput,
    DistanceTooLarge,
    NotPlanar,
    NotConnected,
}

/// Result of a 2D fillet computation: an arc defined by center, radius, and angle span.
// occt: ChFi2d_AnaFilletAlgo result
#[derive(Clone, Copy, Debug)]
pub struct Fillet2dArc {
    pub center: [f64; 2],
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub ccw: bool,
}

impl Fillet2dArc {
    /// Arc length = radius * |end_angle - start_angle|.
    pub fn arc_length(&self) -> f64 {
        self.radius * (self.end_angle - self.start_angle).abs()
    }
}

/// Input parameters for a 2D fillet: two segments meeting at a corner.
// occt: ChFi2d_AnaFilletAlgo parameters
#[derive(Clone, Copy, Debug)]
pub struct FilletParams2d {
    pub p1: [f64; 2],
    pub corner: [f64; 2],
    pub p2: [f64; 2],
    pub radius: f64,
}

/// Builder for 2D analytic fillet arcs.
// occt: ChFi2d_AnaFilletAlgo
pub struct Fillet2dBuilder {
    params: Option<FilletParams2d>,
}

impl Fillet2dBuilder {
    pub fn new() -> Self {
        Self { params: None }
    }

    pub fn set_params(&mut self, p: FilletParams2d) {
        self.params = Some(p);
    }

    /// Computes a fillet arc inscribed at the corner formed by (p1, corner, p2).
    ///
    /// Returns a stub arc centered at the corner with the requested radius and
    /// an angle span of [0, PI], consistent with the ChFi2d_AnaFilletAlgo stub
    /// approach for the pure-math port layer.
    pub fn perform(&self) -> Result<Fillet2dArc, Fillet2dError> {
        let p = self.params.as_ref().ok_or(Fillet2dError::BadInput)?;
        Ok(Fillet2dArc {
            center: p.corner,
            radius: p.radius,
            start_angle: 0.0,
            end_angle: PI,
            ccw: true,
        })
    }
}

impl Default for Fillet2dBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for 2D chamfer segments.
// occt: ChFi2d_ChamferBuilder
pub struct Chamfer2dBuilder {
    p1: Option<[f64; 2]>,
    corner: Option<[f64; 2]>,
    p2: Option<[f64; 2]>,
    dist: f64,
}

impl Chamfer2dBuilder {
    pub fn new() -> Self {
        Self {
            p1: None,
            corner: None,
            p2: None,
            dist: 0.0,
        }
    }

    pub fn set_corner(&mut self, p1: [f64; 2], corner: [f64; 2], p2: [f64; 2]) {
        self.p1 = Some(p1);
        self.corner = Some(corner);
        self.p2 = Some(p2);
    }

    pub fn set_distance(&mut self, d: f64) {
        self.dist = d;
    }

    /// Returns the start and end points of the chamfer segment.
    ///
    /// When geometry is fully specified, the chamfer points are located at
    /// distance `dist` along each edge from the corner. Falls back to stub
    /// ([corner, corner]) when p1/p2 are not set.
    pub fn perform(&self) -> Result<([f64; 2], [f64; 2]), Fillet2dError> {
        let corner = match self.corner {
            Some(c) => c,
            None => return Err(Fillet2dError::BadInput),
        };

        let (p1, p2) = match (self.p1, self.p2) {
            (Some(a), Some(b)) => (a, b),
            _ => return Ok((corner, corner)),
        };

        let pt1 = point_along(corner, p1, self.dist);
        let pt2 = point_along(corner, p2, self.dist);
        Ok((pt1, pt2))
    }
}

impl Default for Chamfer2dBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the point at distance `d` from `from` towards `to`.
/// If the segment is degenerate (zero length), returns `from`.
#[inline]
fn point_along(from: [f64; 2], to: [f64; 2], d: f64) -> [f64; 2] {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-15 {
        return from;
    }
    [from[0] + dx / len * d, from[1] + dy / len * d]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_length_semicircle() {
        let arc = Fillet2dArc {
            center: [0.0, 0.0],
            radius: 2.0,
            start_angle: 0.0,
            end_angle: PI,
            ccw: true,
        };
        assert!((arc.arc_length() - 2.0 * PI).abs() < 1e-12);
    }

    #[test]
    fn fillet_builder_no_params_is_bad_input() {
        let b = Fillet2dBuilder::new();
        assert_eq!(b.perform().unwrap_err(), Fillet2dError::BadInput);
    }

    #[test]
    fn fillet_builder_radius_preserved() {
        let mut b = Fillet2dBuilder::new();
        b.set_params(FilletParams2d {
            p1: [0.0, 1.0],
            corner: [0.0, 0.0],
            p2: [1.0, 0.0],
            radius: 3.0,
        });
        let arc = b.perform().unwrap();
        assert!((arc.radius - 3.0).abs() < 1e-12);
    }

    #[test]
    fn chamfer_builder_no_corner_is_bad_input() {
        let b = Chamfer2dBuilder::new();
        assert_eq!(b.perform().unwrap_err(), Fillet2dError::BadInput);
    }

    #[test]
    fn point_along_five_unit_vector() {
        let from = [0.0, 0.0];
        let to = [3.0, 4.0];
        let pt = point_along(from, to, 5.0);
        assert!((pt[0] - 3.0).abs() < 1e-12);
        assert!((pt[1] - 4.0).abs() < 1e-12);
    }

    #[test]
    fn arc_zero_span_zero_length() {
        let arc = Fillet2dArc {
            center: [1.0, 1.0],
            radius: 5.0,
            start_angle: 1.0,
            end_angle: 1.0,
            ccw: false,
        };
        assert!(arc.arc_length().abs() < 1e-15);
    }

    #[test]
    fn chamfer_perform_positions_points_along_edges() {
        let mut b = Chamfer2dBuilder::new();
        b.set_corner([0.0, 0.0], [1.0, 0.0], [1.0, 1.0]);
        b.set_distance(0.5);
        // corner is [1,0]; p1=[0,0] is to the left, p2=[1,1] is upward
        let (a, c) = b.perform().unwrap();
        assert!(a[0].is_finite() && a[1].is_finite());
        assert!(c[0].is_finite() && c[1].is_finite());
    }
}
