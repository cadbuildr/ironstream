// FILE: rust/ironstream/crates/ironstream/src/geom_curve_on_surface.rs

//! `Geom_CurveOnSurface` — a 2-D parametric curve lifted to 3-D through a
//! surface map, zero-dependency Rust port.
//!
//! In OCCT a *curve on surface* (also called a *P-curve* or *pcurve*) stores a
//! 2-D B-spline in the (u, v) parameter space of a carrier surface and computes
//! the corresponding 3-D point on demand.  This module provides a lightweight
//! stub that:
//!
//! - represents the 2-D spine as two parallel `Vec<f64>` of pole coordinates
//!   (`u_poles`, `v_poles`) with an associated knot vector and multiplicities,
//! - evaluates (u, v) from the parameter `t` via piecewise-linear interpolation,
//! - lifts (u, v) to 3-D using a caller-supplied surface normal.

// occt-note: Geom_CurveOnSurface — a 2D curve lifted to 3D via a surface map

/// Classification of a curve-on-surface role.
// occt-note: curve-on-surface type — enum Pcurve2d, Restriction, Boundary
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveOnSurfaceType {
    /// A general parametric (p-curve) stored in the surface parameter space.
    // occt-ref: Pcurve2d
    Pcurve2d,
    /// A restriction curve that bounds a region on the surface.
    // occt-ref: Restriction
    Restriction,
    /// A boundary curve that forms part of the surface's natural boundary.
    // occt-ref: Boundary
    Boundary,
}

/// A 2-D B-spline curve in (u, v) parameter space, together with enough
/// surface metadata to lift every evaluated point to 3-D.
///
/// The poles are stored in two parallel vectors — `u_poles` and `v_poles` —
/// matching OCCT's separation of the two parameter components.  The knot vector
/// and multiplicities follow the usual B-spline convention: `knots` holds the
/// **distinct** breakpoints and `mults` their repetition counts.
// occt-note: Geom_CurveOnSurface — a 2D curve lifted to 3D via a surface map
#[derive(Clone, Debug)]
pub struct CurveOnSurface {
    /// U-coordinates of the 2-D control polygon in surface parameter space.
    // occt-ref: u_poles
    pub u_poles: Vec<f64>,
    /// V-coordinates of the 2-D control polygon in surface parameter space.
    // occt-ref: v_poles
    pub v_poles: Vec<f64>,
    /// Distinct knot values (strictly increasing).
    // occt-ref: knots
    pub knots: Vec<f64>,
    /// Multiplicity of each distinct knot.
    // occt-ref: mults
    pub mults: Vec<u32>,
    /// Polynomial degree of the B-spline basis.
    // occt-ref: degree
    pub degree: u32,
    /// Whether the curve is periodic.
    // occt-ref: is_periodic
    pub is_periodic: bool,
    /// Tag identifying the kind of carrier surface (e.g. `"Plane"`, `"Cylinder"`).
    // occt-ref: surface_type
    pub surface_type: String,
    /// Role of this curve on the surface.
    // occt-ref: curve_type
    pub curve_type: CurveOnSurfaceType,
}

impl CurveOnSurface {
    /// Construct a non-periodic `Pcurve2d` curve on a surface.
    ///
    /// # Arguments
    /// * `u_poles`      – U-components of the 2-D control polygon.
    /// * `v_poles`      – V-components; must have the same length as `u_poles`.
    /// * `knots`        – Distinct breakpoints of the B-spline knot vector.
    /// * `mults`        – Multiplicity of each breakpoint.
    /// * `degree`       – Polynomial degree.
    /// * `surface_type` – Human-readable surface tag (e.g. `"Plane"`).
    // occt-note: Geom_CurveOnSurface constructor
    pub fn new(
        u_poles: Vec<f64>,
        v_poles: Vec<f64>,
        knots: Vec<f64>,
        mults: Vec<u32>,
        degree: u32,
        surface_type: &str,
    ) -> Self {
        CurveOnSurface {
            u_poles,
            v_poles,
            knots,
            mults,
            degree,
            is_periodic: false,
            surface_type: surface_type.to_owned(),
            curve_type: CurveOnSurfaceType::Pcurve2d,
        }
    }

    /// Number of control poles.
    // occt-ref: NbPoles
    pub fn nb_poles(&self) -> usize {
        self.u_poles.len()
    }

    /// Return the U-component of the i-th pole (0-based index).
    // occt-note: Pole (U component)
    pub fn u_pole(&self, i: usize) -> f64 {
        self.u_poles[i]
    }

    /// Return the V-component of the i-th pole (0-based index).
    // occt-note: Pole (V component)
    pub fn v_pole(&self, i: usize) -> f64 {
        self.v_poles[i]
    }

    /// Polynomial degree of the B-spline basis.
    // occt-ref: Degree
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Whether the curve is periodic.
    // occt-ref: IsPeriodic
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// Role of this curve on the surface.
    // occt-note: CurveType (custom extension)
    pub fn curve_type(&self) -> CurveOnSurfaceType {
        self.curve_type
    }

    /// First valid parameter value (the minimum knot).
    // occt-ref: FirstParameter
    pub fn first_param(&self) -> f64 {
        self.knots.first().copied().unwrap_or(0.0)
    }

    /// Last valid parameter value (the maximum knot).
    // occt-ref: LastParameter
    pub fn last_param(&self) -> f64 {
        self.knots.last().copied().unwrap_or(0.0)
    }

    /// Evaluate the 2-D (u, v) position on the surface at parameter `t` using
    /// **piecewise-linear interpolation** between consecutive poles.
    ///
    /// This is a stub implementation: it builds the flat knot sequence from
    /// `knots` / `mults`, clamps `t` to `[first_param, last_param]`, locates
    /// the knot span, and linearly interpolates the two bracketing poles.
    ///
    /// A future revision may replace this with full de-Boor evaluation.
    // occt-note: Value / D0 — evaluate 2D point on surface parameter domain
    pub fn evaluate_uv(&self, t: f64) -> (f64, f64) {
        let n = self.u_poles.len();
        if n == 0 {
            return (0.0, 0.0);
        }
        if n == 1 {
            return (self.u_poles[0], self.v_poles[0]);
        }

        // Build the flat (expanded) knot sequence.
        let flat: Vec<f64> = self
            .knots
            .iter()
            .zip(self.mults.iter())
            .flat_map(|(&k, &m)| std::iter::repeat(k).take(m as usize))
            .collect();

        let t_min = flat.first().copied().unwrap_or(0.0);
        let t_max = flat.last().copied().unwrap_or(1.0);
        let t = t.clamp(t_min, t_max);

        // Map parameter t ∈ [t_min, t_max] into a pole-index fraction.
        let span = t_max - t_min;
        let frac = if span > 0.0 {
            (t - t_min) / span
        } else {
            0.0
        };

        let max_idx = (n - 1) as f64;
        let pos = frac * max_idx;
        let lo = pos.floor() as usize;
        let hi = (lo + 1).min(n - 1);
        let alpha = pos - lo as f64;

        let u = self.u_poles[lo] * (1.0 - alpha) + self.u_poles[hi] * alpha;
        let v = self.v_poles[lo] * (1.0 - alpha) + self.v_poles[hi] * alpha;
        (u, v)
    }

    /// Lift the 2-D parameter-space point at `t` to a 3-D point using the
    /// caller-supplied `surface_normal`.
    ///
    /// **Stub behaviour**: returns `[u, v, 0.0]` scaled/offset by the normal
    /// direction — specifically `[u * nx, v * ny, (u + v) * nz]` where
    /// `(nx, ny, nz) = surface_normal` — as a placeholder for the real surface
    /// evaluation that would require a concrete surface object.
    // occt-note: Value / D0 lifted to 3D — real impl calls surface->Value(u,v)
    pub fn evaluate_3d(&self, t: f64, surface_normal: [f64; 3]) -> [f64; 3] {
        let (u, v) = self.evaluate_uv(t);
        let [nx, ny, nz] = surface_normal;
        [u * nx, v * ny, (u + v) * nz]
    }
}
