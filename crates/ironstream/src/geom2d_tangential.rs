// FILE: rust/ironstream/crates/ironstream/src/geom2d_tangential.rs
//! Tangential arc / circumcircle solvers for 2-D geometry.
//!
//! Ports three OpenCascade concepts to zero-dependency Rust:
//!
//! | OCCT type                    | IronStream type          |
//! |------------------------------|--------------------------|
//! | `Geom2dGcc_Circ2d3Tan` result| [`Geom2dTanArcResult`]   |
//! | `Geom2dGcc_Circ2d3Tan`       | [`Geom2dCirc3Tan`]       |
//! | `GCE2d_MakeArcOfCircle`      | [`Geom2dArcBuilder`]     |

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Geom2dTanArcResult
// ---------------------------------------------------------------------------

// occt: Geom2dGcc_Circ2d3Tan // result — circle tangent to constraints
/// A single solution produced by a 3-constraint tangency solver.
///
/// Stores the solution circle (center + radius) and the three parameter values
/// that record the tangency points on each constraint.
///
/// Mirrors one result entry of `Geom2dGcc_Circ2d3Tan` in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geom2dTanArcResult {
    center: [f64; 2],
    radius: f64,
    param1: f64,
    param2: f64,
    param3: f64,
    is_done: bool,
}

impl Geom2dTanArcResult {
    /// Create a result entry with the given center, radius and tangency
    /// parameters.
    pub fn new(
        center: [f64; 2],
        radius: f64,
        param1: f64,
        param2: f64,
        param3: f64,
    ) -> Self {
        Self {
            center,
            radius,
            param1,
            param2,
            param3,
            is_done: true,
        }
    }

    /// Return the center of the solution circle.
    pub fn center(&self) -> [f64; 2] {
        self.center
    }

    /// Return the radius of the solution circle.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Return the three tangency parameters `(param1, param2, param3)`.
    pub fn params(&self) -> (f64, f64, f64) {
        (self.param1, self.param2, self.param3)
    }

    /// Return `true` when the result was successfully computed.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

// ---------------------------------------------------------------------------
// Geom2dCirc3Tan
// ---------------------------------------------------------------------------

// occt: Geom2dGcc_Circ2d3Tan // — find circle tangent to 3 points (Apollonius problem stub)
/// Solver that finds a circle passing through (tangent to) three 2-D points.
///
/// The implemented case is the **circumscribed circle** of three non-collinear
/// points: the unique circle whose boundary passes through all three input
/// points.  The center is located at the intersection of the perpendicular
/// bisectors of the two chords `p1p2` and `p2p3`.
///
/// This corresponds to the special case of `Geom2dGcc_Circ2d3Tan` where all
/// three constraints are point constraints.
///
/// # Usage
/// ```
/// use ironstream::geom2d_tangential::Geom2dCirc3Tan;
/// let mut solver = Geom2dCirc3Tan::new(1e-10);
/// solver.solve_from_points([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]);
/// assert!(solver.is_done());
/// assert_eq!(solver.nb_solutions(), 1);
/// ```
///
/// Mirrors `Geom2dGcc_Circ2d3Tan` in OpenCascade.
#[derive(Debug, Clone)]
pub struct Geom2dCirc3Tan {
    solutions: Vec<Geom2dTanArcResult>,
    is_done: bool,
    tolerance: f64,
}

impl Geom2dCirc3Tan {
    /// Create a new, not-yet-solved instance with the given numerical
    /// tolerance.  The tolerance is used to detect degenerate (collinear)
    /// configurations.
    pub fn new(tolerance: f64) -> Self {
        Self {
            solutions: Vec::new(),
            is_done: false,
            tolerance,
        }
    }

    /// Compute the circumscribed circle of the three points.
    ///
    /// The center is found by intersecting the perpendicular bisectors of
    /// `p1p2` and `p2p3`.  If the three points are (nearly) collinear the
    /// solver records no solution but still marks itself as done.
    ///
    /// The three `param` fields of the result are set to the angles (radians,
    /// measured from the positive X axis through the center) of the respective
    /// tangency points on the solution circle.
    pub fn solve_from_points(
        &mut self,
        p1: [f64; 2],
        p2: [f64; 2],
        p3: [f64; 2],
    ) {
        self.solutions.clear();
        self.is_done = true;

        // Mid-points of the two chords.
        let mid12 = [(p1[0] + p2[0]) * 0.5, (p1[1] + p2[1]) * 0.5];
        let mid23 = [(p2[0] + p3[0]) * 0.5, (p2[1] + p3[1]) * 0.5];

        // Direction vectors of the two chords.
        let d12 = [p2[0] - p1[0], p2[1] - p1[1]];
        let d23 = [p3[0] - p2[0], p3[1] - p2[1]];

        // Perpendicular bisectors of chord 12: passes through mid12 with
        // direction perpendicular to d12, i.e. (-d12[1], d12[0]).
        // Perpendicular bisectors of chord 23: passes through mid23 with
        // direction perpendicular to d23, i.e. (-d23[1], d23[0]).
        //
        // Parametric form:
        //   bisector12(s) = mid12 + s * n12   where n12 = [-d12[1], d12[0]]
        //   bisector23(t) = mid23 + t * n23   where n23 = [-d23[1], d23[0]]
        //
        // Solve for s such that bisector12(s) = bisector23(t):
        //   mid12 + s*n12 = mid23 + t*n23
        //   s*n12 - t*n23 = mid23 - mid12
        //
        // In matrix form [ n12 | -n23 ] [s; t] = (mid23 - mid12)
        // det = n12[0]*(-n23[1]) - n12[1]*(-n23[0])
        //     = -n12[0]*n23[1] + n12[1]*n23[0]

        let n12 = [-d12[1], d12[0]];
        let n23 = [-d23[1], d23[0]];

        // det of coefficient matrix [[n12[0], -n23[0]], [n12[1], -n23[1]]]
        let det = n12[0] * (-n23[1]) - (-n23[0]) * n12[1];

        if det.abs() < self.tolerance {
            // Collinear points — no circumcircle exists.
            return;
        }

        let rhs = [mid23[0] - mid12[0], mid23[1] - mid12[1]];
        // Cramer's rule for s (first unknown):
        // s = det([[rhs[0], -n23[0]], [rhs[1], -n23[1]]]) / det
        let s = (rhs[0] * (-n23[1]) - (-n23[0]) * rhs[1]) / det;

        let cx = mid12[0] + s * n12[0];
        let cy = mid12[1] + s * n12[1];
        let center = [cx, cy];

        let radius = {
            let dx = p1[0] - cx;
            let dy = p1[1] - cy;
            (dx * dx + dy * dy).sqrt()
        };

        // Compute tangency parameters as angles from center to each point.
        let angle_of = |p: [f64; 2]| -> f64 {
            let a = (p[1] - cy).atan2(p[0] - cx);
            if a < 0.0 { a + 2.0 * PI } else { a }
        };

        let param1 = angle_of(p1);
        let param2 = angle_of(p2);
        let param3 = angle_of(p3);

        self.solutions.push(Geom2dTanArcResult::new(
            center, radius, param1, param2, param3,
        ));
    }

    /// Return `true` once the solver has been run.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of solution circles found (0 or 1).
    pub fn nb_solutions(&self) -> usize {
        self.solutions.len()
    }

    /// Access the `i`-th solution (0-based index).
    ///
    /// # Panics
    /// Panics if `i >= nb_solutions()`.
    pub fn solution(&self, i: usize) -> &Geom2dTanArcResult {
        &self.solutions[i]
    }
}

// ---------------------------------------------------------------------------
// Geom2dArcBuilder
// ---------------------------------------------------------------------------

// occt-ref: GCE2d_MakeArcOfCircle // — build 2D arc from center/radius/angles
/// Builder that constructs a 2-D circular arc from a center, radius and two
/// bounding angles.
///
/// Angles are measured in radians, counter-clockwise from the positive X axis.
/// `start_angle` may be greater than `end_angle`; in that case the arc wraps
/// through 0 (i.e. it crosses the positive X axis).
///
/// Mirrors `GCE2d_MakeArcOfCircle` in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geom2dArcBuilder {
    center: [f64; 2],
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    is_done: bool,
}

impl Geom2dArcBuilder {
    /// Create a new arc builder.  Call [`build`](Self::build) to finalise.
    pub fn new(
        center: [f64; 2],
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
            is_done: false,
        }
    }

    /// Finalise the arc.  Sets `is_done` to `true`.
    pub fn build(&mut self) {
        self.is_done = true;
    }

    /// Return `true` after [`build`](Self::build) has been called.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the start point of the arc on the circle boundary.
    pub fn start_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.radius * self.start_angle.cos(),
            self.center[1] + self.radius * self.start_angle.sin(),
        ]
    }

    /// Return the end point of the arc on the circle boundary.
    pub fn end_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.radius * self.end_angle.cos(),
            self.center[1] + self.radius * self.end_angle.sin(),
        ]
    }

    /// Return the mid-point of the arc.
    ///
    /// The mid-point lies at the angle halfway between `start_angle` and
    /// `end_angle` going counter-clockwise.  When `end_angle < start_angle`
    /// the arc wraps through 0, so the swept angle is computed modulo 2π.
    pub fn mid_point(&self) -> [f64; 2] {
        let sweep = self.sweep_angle();
        let mid_angle = self.start_angle + sweep * 0.5;
        [
            self.center[0] + self.radius * mid_angle.cos(),
            self.center[1] + self.radius * mid_angle.sin(),
        ]
    }

    /// Return the arc length (`radius * swept_angle`).
    pub fn arc_length(&self) -> f64 {
        self.radius * self.sweep_angle()
    }

    // ------------------------------------------------------------------
    // private helpers
    // ------------------------------------------------------------------

    /// Counter-clockwise swept angle in [0, 2π].
    fn sweep_angle(&self) -> f64 {
        let mut sweep = self.end_angle - self.start_angle;
        if sweep < 0.0 {
            sweep += 2.0 * PI;
        }
        sweep
    }
}
