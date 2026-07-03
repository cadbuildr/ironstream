//! `NLPlate_NLPlate` / `NLPlate_HGPPConstraint` — non-linear plate surface
//! fitting, a faithful stub of the OpenCascade plate-surface solver.
//!
//! The NLPlate module in OCCT provides a non-linear thin-plate spline solver
//! that, given a set of point/tangency/curvature constraints in parameter space,
//! produces a smooth surface that satisfies all constraints (to within a
//! tolerance). This module mirrors the public API of the two core classes:
//!
//! - [`NlPtConstraint`] — a single geometric point/plane constraint (OCCT
//!   `NLPlate_HGPPConstraint`). It carries a 2D UV location in the parameter
//!   domain and the target 3D point together with a continuity order.
//! - [`NlPlate`] — the plate solver (OCCT `NLPlate_NLPlate`). It accumulates
//!   constraints and runs an iterative non-linear solver to compute a grid of
//!   poles (control points) such that the resulting surface honours all
//!   constraints.
//!
//! Implementation notes
//! --------------------
//! This is a zero-dependency pure-Rust **stub**. The actual thin-plate spline
//! mathematics (building and solving the large sparse linear system, the
//! iterative non-linear refinement loop, B-spline / Bernstein blending, etc.)
//! are replaced by a bilinear interpolation fallback so that the API compiles,
//! links, and returns geometrically plausible (though approximate) values.  A
//! production implementation would replace `solve_internal` with the full
//! Kirchhoff thin-plate energy minimisation described in OCCT's
//! `NLPlate_NLPlate.cxx`.
//!
//! References
//! ----------
//! - OCCT `src/ModelingAlgorithms/TKFeat/NLPlate/NLPlate_NLPlate.hxx`
//! - OCCT `src/ModelingAlgorithms/TKFeat/NLPlate/NLPlate_HGPPConstraint.hxx`
//! - Floater & Hormann, "Surface Parameterization: a Tutorial and Survey",
//!   *Advances in Multiresolution for Geometric Modelling*, Springer 2005.
//!
//! Uses only `std`. No external crates.

// ---------------------------------------------------------------------------
// ConstraintOrder
// ---------------------------------------------------------------------------

/// Geometric continuity order of a [`NlPtConstraint`].
///
/// Mirrors the `NLPlate_ConstraintOrder` enumeration used by
/// `NLPlate_HGPPConstraint`:
///
/// | Variant | OCCT constant | Meaning                       |
/// |---------|--------------|-------------------------------|
/// | `G0`    | `G0`         | Position only (point)         |
/// | `G1`    | `G1`         | Position + first derivatives  |
/// | `G2`    | `G2`         | Position + up to 2nd derivs   |
// occt: NLPlate_HGPPConstraint
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstraintOrder {
    /// G0 constraint — the surface must pass through the target point.
    G0,
    /// G1 constraint — the surface must match position and tangent plane.
    G1,
    /// G2 constraint — the surface must match position, tangent, and curvature.
    G2,
}

// ---------------------------------------------------------------------------
// NlPtConstraint
// ---------------------------------------------------------------------------

/// A single geometric point/plane/curvature constraint for the plate solver.
///
/// Each constraint is attached to a specific parameter-space location `uv` and
/// specifies the 3D target `point` that the plate surface must (approximately)
/// satisfy. The `order` selects how many derivatives are constrained at that
/// location.
///
/// Mirrors `NLPlate_HGPPConstraint`.
// occt: NLPlate_HGPPConstraint
#[derive(Clone, Debug)]
pub struct NlPtConstraint {
    /// 2D parameter-space location `(u, v)` of the constraint.
    pub uv: [f64; 2],
    /// Target 3D point that the surface must pass through.
    pub point: [f64; 3],
    /// Continuity order required at this constraint location.
    pub order: ConstraintOrder,
}

impl NlPtConstraint {
    /// Construct a G0 point constraint.
    ///
    /// Equivalent to `NLPlate_HGPPConstraint(UV, pt)` with the default
    /// continuity order `G0`.
    ///
    /// # Arguments
    /// * `uv`  — parameter-space location `[u, v]`.
    /// * `pt`  — target 3D position `[x, y, z]`.
    pub fn new(uv: [f64; 2], pt: [f64; 3]) -> Self {
        NlPtConstraint {
            uv,
            point: pt,
            order: ConstraintOrder::G0,
        }
    }

    /// Return a copy of this constraint with a different continuity order.
    ///
    /// Mirrors `NLPlate_HGPPConstraint::SetG0Criterion` / `SetG1Criterion` /
    /// `SetG2Criterion` by providing a builder-style setter.
    ///
    /// # Arguments
    /// * `o` — the desired [`ConstraintOrder`].
    pub fn with_order(mut self, o: ConstraintOrder) -> Self {
        self.order = o;
        self
    }
}

// ---------------------------------------------------------------------------
// NlPlate
// ---------------------------------------------------------------------------

/// Non-linear thin-plate surface solver.
///
/// Accumulates [`NlPtConstraint`]s and computes a tensor-product grid of
/// control poles such that the resulting bi-cubic surface satisfies (within
/// `tol`) all constraints. The computed poles are stored in `poles` as a
/// row-major 2D grid over the U×V parameter domain.
///
/// Mirrors `NLPlate_NLPlate`.
// occt: NLPlate_NLPlate
#[derive(Clone, Debug)]
pub struct NlPlate {
    /// All constraints registered via [`NlPlate::add_constraint`].
    pub constraints: Vec<NlPtConstraint>,
    /// Computed control-point grid after [`NlPlate::solve`] succeeds.
    ///
    /// `poles[i][j]` is the pole at U-index `i`, V-index `j`.
    /// The grid is `n_u × n_v` where both dimensions are derived from the
    /// number and distribution of constraints.
    pub poles: Vec<Vec<[f64; 3]>>,
    /// `true` once [`NlPlate::solve`] has returned `true`.
    pub done: bool,
}

impl NlPlate {
    /// Create an empty plate solver with no constraints.
    ///
    /// Mirrors `NLPlate_NLPlate()`.
    pub fn new() -> Self {
        NlPlate {
            constraints: Vec::new(),
            poles: Vec::new(),
            done: false,
        }
    }

    /// Register a new constraint.
    ///
    /// Mirrors `NLPlate_NLPlate::Load(const Handle(NLPlate_HGPPConstraint)&)`.
    ///
    /// The solver must be re-run (`solve`) after adding constraints.
    ///
    /// # Arguments
    /// * `c` — the constraint to add.
    pub fn add_constraint(&mut self, c: NlPtConstraint) {
        self.done = false;
        self.constraints.push(c);
    }

    /// Run the non-linear plate solver.
    ///
    /// Mirrors `NLPlate_NLPlate::Perform(const Standard_Integer IncrementalOrder,
    ///          const Standard_Integer MaxIterations,
    ///          const Standard_Real Tol)`.
    ///
    /// The stub implements a bilinear scattered-data interpolation as the
    /// base approximation, followed by `iter` smoothing passes that nudge
    /// interior poles towards the constraint residuals.
    ///
    /// # Arguments
    /// * `iter` — maximum number of refinement iterations (analogous to OCCT's
    ///            `MaxIterations`).
    /// * `tol`  — convergence tolerance; iteration stops early when the maximum
    ///            residual at all G0 constraints drops below this value.
    ///
    /// # Returns
    /// `true` when the solver converged within `tol`; `false` otherwise.
    pub fn solve(&mut self, iter: u32, tol: f64) -> bool {
        if self.constraints.is_empty() {
            self.poles = Vec::new();
            self.done = true;
            return true;
        }

        let converged = self.solve_internal(iter, tol);
        self.done = converged;
        converged
    }

    /// Evaluate the solved plate surface at parameter `(u, v)`.
    ///
    /// Mirrors `NLPlate_NLPlate::value(const gp_XY& point2d)`.
    ///
    /// Returns `[0.0, 0.0, 0.0]` when the solver has not been run or failed.
    /// When the pole grid has been computed the method performs bilinear
    /// interpolation within the `[u_min, u_max] × [v_min, v_max]` bounding
    /// box of all constraint UV locations.
    ///
    /// # Arguments
    /// * `u` — U parameter value.
    /// * `v` — V parameter value.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        if !self.done || self.poles.is_empty() {
            return [0.0, 0.0, 0.0];
        }

        let nu = self.poles.len();
        let nv = self.poles[0].len();

        if nu == 0 || nv == 0 {
            return [0.0, 0.0, 0.0];
        }

        // Determine the UV bounding box used to build the pole grid.
        let (u_min, u_max, v_min, v_max) = uv_bounds(&self.constraints);
        let u_range = u_max - u_min;
        let v_range = v_max - v_min;

        // Map (u, v) to normalised grid coordinates [0, nu-1] × [0, nv-1].
        let gu = if u_range < f64::EPSILON {
            0.0
        } else {
            ((u - u_min) / u_range) * (nu as f64 - 1.0)
        };
        let gv = if v_range < f64::EPSILON {
            0.0
        } else {
            ((v - v_min) / v_range) * (nv as f64 - 1.0)
        };

        bilinear_eval(&self.poles, gu, gv)
    }

    /// Returns `true` when a successful [`solve`](NlPlate::solve) has been
    /// completed.
    ///
    /// Mirrors `NLPlate_NLPlate::IsDone()`.
    pub fn is_solved(&self) -> bool {
        self.done
    }

    // -----------------------------------------------------------------------
    // Internal solver
    // -----------------------------------------------------------------------

    /// Core solver — thin-plate spline stub.
    ///
    /// Strategy (stub):
    /// 1. Build an `N × N` uniform grid over the UV bounding box of all
    ///    constraints (N grows with constraint count for accuracy).
    /// 2. Initialise each pole to the bilinearly-interpolated weighted sum of
    ///    all constraint points, weighted by an RBF `exp(-r²/σ²)`.
    /// 3. Perform up to `iter` Gauss-Seidel smoothing sweeps, checking the
    ///    maximum G0 constraint residual after each sweep.
    fn solve_internal(&mut self, iter: u32, tol: f64) -> bool {
        let (u_min, u_max, v_min, v_max) = uv_bounds(&self.constraints);

        // Grid resolution: at least 4 cells in each direction; grows with
        // sqrt(n_constraints) so finer grids are built for denser inputs.
        let n_base = 4usize;
        let nc = self.constraints.len();
        let n_side = (n_base + (nc as f64).sqrt() as usize).max(2);
        let nu = n_side;
        let nv = n_side;

        let u_range = (u_max - u_min).max(f64::EPSILON);
        let v_range = (v_max - v_min).max(f64::EPSILON);

        // Radial basis function bandwidth — one quarter of the domain diagonal.
        let sigma_sq = (u_range * u_range + v_range * v_range) / 16.0;

        // --- Step 1: initialise poles via RBF interpolation ------------------
        let mut poles: Vec<Vec<[f64; 3]>> = (0..nu)
            .map(|i| {
                (0..nv)
                    .map(|j| {
                        let pu = u_min + (i as f64 / (nu as f64 - 1.0)) * u_range;
                        let pv = v_min + (j as f64 / (nv as f64 - 1.0)) * v_range;
                        rbf_interpolate(&self.constraints, pu, pv, sigma_sq)
                    })
                    .collect()
            })
            .collect();

        // --- Step 2: iterative smoothing / constraint projection --------------
        let mut converged = false;
        for _it in 0..iter {
            // Laplacian smoothing of interior poles.
            let prev = poles.clone();
            for i in 1..(nu - 1) {
                for j in 1..(nv - 1) {
                    poles[i][j] = avg4(
                        prev[i - 1][j],
                        prev[i + 1][j],
                        prev[i][j - 1],
                        prev[i][j + 1],
                    );
                }
            }

            // Re-project constraint poles towards target points (G0 correction).
            for c in &self.constraints {
                let (gu, gv) = uv_to_grid(c.uv, u_min, u_range, v_min, v_range, nu, nv);
                let i = gu.floor() as usize;
                let j = gv.floor() as usize;
                let i = i.min(nu - 1);
                let j = j.min(nv - 1);
                // Blend pole towards constraint target (half-step projection).
                let p = &mut poles[i][j];
                p[0] = (p[0] + c.point[0]) * 0.5;
                p[1] = (p[1] + c.point[1]) * 0.5;
                p[2] = (p[2] + c.point[2]) * 0.5;
            }

            // Check convergence: maximum G0 residual over all constraints.
            let max_res = self
                .constraints
                .iter()
                .map(|c| {
                    let (gu, gv) =
                        uv_to_grid(c.uv, u_min, u_range, v_min, v_range, nu, nv);
                    let ev = bilinear_eval(&poles, gu, gv);
                    let dx = ev[0] - c.point[0];
                    let dy = ev[1] - c.point[1];
                    let dz = ev[2] - c.point[2];
                    (dx * dx + dy * dy + dz * dz).sqrt()
                })
                .fold(0.0f64, f64::max);

            if max_res < tol {
                converged = true;
                break;
            }
        }

        self.poles = poles;
        converged
    }
}

impl Default for NlPlate {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Builder function
// ---------------------------------------------------------------------------

/// Convenience constructor: build and solve an [`NlPlate`] from a slice of
/// constraints in one call.
///
/// Uses default solver parameters: up to 64 iterations and tolerance `1e-6`.
/// The caller should check [`NlPlate::is_solved`] on the returned value.
///
/// # Arguments
/// * `constraints` — slice of constraints to load into the plate.
///
/// # Returns
/// A solved (or partially solved) [`NlPlate`].
///
/// # Example
/// ```rust
/// use ironstream::geom_nplate::{NlPtConstraint, build_nl_plate};
///
/// let cs = vec![
///     NlPtConstraint::new([0.0, 0.0], [0.0, 0.0, 0.0]),
///     NlPtConstraint::new([1.0, 0.0], [1.0, 0.0, 0.0]),
///     NlPtConstraint::new([0.0, 1.0], [0.0, 1.0, 0.0]),
///     NlPtConstraint::new([1.0, 1.0], [1.0, 1.0, 0.0]),
/// ];
/// let plate = build_nl_plate(&cs);
/// assert!(plate.is_solved());
/// ```
pub fn build_nl_plate(constraints: &[NlPtConstraint]) -> NlPlate {
    let mut plate = NlPlate::new();
    for c in constraints {
        plate.add_constraint(c.clone());
    }
    plate.solve(64, 1e-6);
    plate
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Compute the axis-aligned bounding box of all constraint UV locations.
///
/// Returns `(u_min, u_max, v_min, v_max)`. Falls back to `[0,1]×[0,1]` for an
/// empty constraint set.
fn uv_bounds(constraints: &[NlPtConstraint]) -> (f64, f64, f64, f64) {
    if constraints.is_empty() {
        return (0.0, 1.0, 0.0, 1.0);
    }
    let mut u_min = f64::MAX;
    let mut u_max = f64::MIN;
    let mut v_min = f64::MAX;
    let mut v_max = f64::MIN;
    for c in constraints {
        if c.uv[0] < u_min {
            u_min = c.uv[0];
        }
        if c.uv[0] > u_max {
            u_max = c.uv[0];
        }
        if c.uv[1] < v_min {
            v_min = c.uv[1];
        }
        if c.uv[1] > v_max {
            v_max = c.uv[1];
        }
    }
    // Expand degenerate domains so the grid is always well-defined.
    if (u_max - u_min).abs() < f64::EPSILON {
        u_max = u_min + 1.0;
    }
    if (v_max - v_min).abs() < f64::EPSILON {
        v_max = v_min + 1.0;
    }
    (u_min, u_max, v_min, v_max)
}

/// Map a UV parameter to fractional grid coordinates `(gu, gv)`.
///
/// `gu` is in `[0, nu-1]` and `gv` in `[0, nv-1]`, clamped at both ends.
#[inline]
fn uv_to_grid(
    uv: [f64; 2],
    u_min: f64,
    u_range: f64,
    v_min: f64,
    v_range: f64,
    nu: usize,
    nv: usize,
) -> (f64, f64) {
    let gu = ((uv[0] - u_min) / u_range) * (nu as f64 - 1.0);
    let gv = ((uv[1] - v_min) / v_range) * (nv as f64 - 1.0);
    let gu = gu.clamp(0.0, nu as f64 - 1.0);
    let gv = gv.clamp(0.0, nv as f64 - 1.0);
    (gu, gv)
}

/// Bilinear interpolation within the pole grid at fractional coordinates
/// `(gu, gv)`.
///
/// Clamps to the grid boundary when `gu` or `gv` is out of range.
fn bilinear_eval(poles: &[Vec<[f64; 3]>], gu: f64, gv: f64) -> [f64; 3] {
    let nu = poles.len();
    if nu == 0 {
        return [0.0, 0.0, 0.0];
    }
    let nv = poles[0].len();
    if nv == 0 {
        return [0.0, 0.0, 0.0];
    }

    let gu = gu.clamp(0.0, nu as f64 - 1.0);
    let gv = gv.clamp(0.0, nv as f64 - 1.0);

    let i0 = (gu.floor() as usize).min(nu - 1);
    let j0 = (gv.floor() as usize).min(nv - 1);
    let i1 = (i0 + 1).min(nu - 1);
    let j1 = (j0 + 1).min(nv - 1);

    let tu = gu - i0 as f64; // fractional part in U, in [0,1]
    let tv = gv - j0 as f64; // fractional part in V, in [0,1]

    let p00 = poles[i0][j0];
    let p10 = poles[i1][j0];
    let p01 = poles[i0][j1];
    let p11 = poles[i1][j1];

    let mut out = [0.0f64; 3];
    for k in 0..3 {
        out[k] = p00[k] * (1.0 - tu) * (1.0 - tv)
            + p10[k] * tu * (1.0 - tv)
            + p01[k] * (1.0 - tu) * tv
            + p11[k] * tu * tv;
    }
    out
}

/// RBF (Gaussian) scattered-data interpolation at `(pu, pv)`.
///
/// Computes `Σ w_i * exp(-‖(pu,pv) - (u_i,v_i)‖² / σ²) * pt_i / Σ w_i`
/// where `(u_i, v_i)` are the constraint UV locations and `pt_i` the target
/// 3D points. Falls back to the centroid of all constraint points when all
/// Gaussian weights underflow to zero.
fn rbf_interpolate(
    constraints: &[NlPtConstraint],
    pu: f64,
    pv: f64,
    sigma_sq: f64,
) -> [f64; 3] {
    let mut num = [0.0f64; 3];
    let mut denom = 0.0f64;

    for c in constraints {
        let du = pu - c.uv[0];
        let dv = pv - c.uv[1];
        let r2 = du * du + dv * dv;
        let w = (-r2 / sigma_sq).exp();
        denom += w;
        num[0] += w * c.point[0];
        num[1] += w * c.point[1];
        num[2] += w * c.point[2];
    }

    if denom < f64::EPSILON {
        // All constraints are far away — use unweighted centroid.
        let n = constraints.len() as f64;
        let mut centroid = [0.0f64; 3];
        for c in constraints {
            centroid[0] += c.point[0];
            centroid[1] += c.point[1];
            centroid[2] += c.point[2];
        }
        [centroid[0] / n, centroid[1] / n, centroid[2] / n]
    } else {
        [num[0] / denom, num[1] / denom, num[2] / denom]
    }
}

/// Average of four 3D points (used in the Laplacian smoothing step).
#[inline]
fn avg4(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> [f64; 3] {
    [
        (a[0] + b[0] + c[0] + d[0]) * 0.25,
        (a[1] + b[1] + c[1] + d[1]) * 0.25,
        (a[2] + b[2] + c[2] + d[2]) * 0.25,
    ]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// A flat four-corner constraint set over the unit square should produce a
    /// plate whose mid-point evaluates close to `[0.5, 0.5, 0.0]`.
    #[test]
    fn test_flat_plate_midpoint() {
        let cs = vec![
            NlPtConstraint::new([0.0, 0.0], [0.0, 0.0, 0.0]),
            NlPtConstraint::new([1.0, 0.0], [1.0, 0.0, 0.0]),
            NlPtConstraint::new([0.0, 1.0], [0.0, 1.0, 0.0]),
            NlPtConstraint::new([1.0, 1.0], [1.0, 1.0, 0.0]),
        ];
        let plate = build_nl_plate(&cs);
        assert!(plate.is_solved());
        let mid = plate.evaluate(0.5, 0.5);
        assert!(
            (mid[0] - 0.5).abs() < 0.15,
            "mid[0] = {}",
            mid[0]
        );
        assert!(
            (mid[1] - 0.5).abs() < 0.15,
            "mid[1] = {}",
            mid[1]
        );
        assert!(mid[2].abs() < 0.15, "mid[2] = {}", mid[2]);
    }

    #[test]
    fn test_constraint_order_builder() {
        let c = NlPtConstraint::new([0.5, 0.5], [1.0, 2.0, 3.0])
            .with_order(ConstraintOrder::G1);
        assert_eq!(c.order, ConstraintOrder::G1);
        assert_eq!(c.uv, [0.5, 0.5]);
        assert_eq!(c.point, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_empty_plate() {
        let mut plate = NlPlate::new();
        assert!(!plate.is_solved());
        let ok = plate.solve(10, 1e-6);
        assert!(ok);
        assert!(plate.is_solved());
        let ev = plate.evaluate(0.5, 0.5);
        assert_eq!(ev, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_single_constraint() {
        let mut plate = NlPlate::new();
        plate.add_constraint(NlPtConstraint::new([0.5, 0.5], [3.0, 4.0, 5.0]));
        plate.solve(32, 1e-4);
        // Evaluation at the constraint UV should be close to the target point.
        let ev = plate.evaluate(0.5, 0.5);
        assert!(
            (ev[0] - 3.0).abs() < 1.0,
            "ev[0] = {}",
            ev[0]
        );
    }

    #[test]
    fn test_is_solved_resets_on_add_constraint() {
        let mut plate = NlPlate::new();
        plate.add_constraint(NlPtConstraint::new([0.0, 0.0], [0.0, 0.0, 0.0]));
        plate.solve(8, 1e-6);
        assert!(plate.is_solved());
        plate.add_constraint(NlPtConstraint::new([1.0, 1.0], [1.0, 1.0, 0.0]));
        assert!(!plate.is_solved());
    }

    #[test]
    fn test_default_equals_new() {
        let a = NlPlate::new();
        let b = NlPlate::default();
        assert_eq!(a.done, b.done);
        assert_eq!(a.constraints.len(), b.constraints.len());
    }
}
