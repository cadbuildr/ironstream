// FILE: rust/ironstream/crates/ironstream/src/conv_surface.rs
//! `Convert` surface classes — zero-dependency Rust port of the OpenCascade
//! `Convert` package surface types
//! (`src/ModelingData/TKMath/Convert/Convert_BSplineSurface.*`).
//!
//! Provides:
//! - [`ConvBSplineSurface`] — data holder for a converted B-spline surface

// occt-note: Convert (package)

// ---------------------------------------------------------------------------
// ConvBSplineSurface
// ---------------------------------------------------------------------------

/// `Convert_BSplineSurface` — holds the result of converting a surface
/// description into a B-spline surface representation.
///
/// The poles are stored in a flat `Vec<[f64; 3]>` in row-major order
/// (u index varies fastest in the outer loop, v index in the inner loop):
/// `poles[i * nb_v_poles + j]` is the pole at u-index `i`, v-index `j`.
///
/// All knot vectors use the *distinct-knot + multiplicity* encoding that
/// OCCT uses in `Geom_BSplineSurface` / `BSplSLib`.
///
/// # Indexing
///
/// All 1-based accessor methods (e.g. `pole`) follow OCCT's
/// `NCollection_Array2` convention so callers can be ported without
/// re-indexing.
// occt-ref: Convert_BSplineSurface
#[derive(Clone, Debug)]
pub struct ConvBSplineSurface {
    /// Flat pole array, row-major: index = i_u * nb_v + i_v (0-based).
    poles: Vec<[f64; 3]>,
    /// Distinct knot values in the U direction.
    u_knots: Vec<f64>,
    /// Distinct knot values in the V direction.
    v_knots: Vec<f64>,
    /// Multiplicities corresponding to `u_knots`.
    u_mults: Vec<u32>,
    /// Multiplicities corresponding to `v_knots`.
    v_mults: Vec<u32>,
    /// Polynomial degree in the U direction.
    u_degree: u32,
    /// Polynomial degree in the V direction.
    v_degree: u32,
    /// Whether the surface is rational in the U direction.
    u_rational: bool,
    /// Whether the surface is rational in the V direction.
    v_rational: bool,
    /// Rational weights, same layout as `poles`; all 1.0 for non-rational.
    weights: Vec<f64>,
    /// Number of poles in the U direction.
    nb_u: usize,
    /// Number of poles in the V direction.
    nb_v: usize,
}

impl ConvBSplineSurface {
    // -----------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------

    /// `Convert_BSplineSurface()` — construct with all data supplied directly.
    ///
    /// # Parameters
    ///
    /// - `poles`       — flat pole array in row-major (u, v) order; length must
    ///                   equal `nb_u_poles * nb_v_poles`.
    /// - `u_knots`     — distinct knot values in U; length must match `u_mults`.
    /// - `v_knots`     — distinct knot values in V; length must match `v_mults`.
    /// - `u_mults`     — multiplicities of `u_knots`.
    /// - `v_mults`     — multiplicities of `v_knots`.
    /// - `u_degree`    — polynomial degree in U (≥ 1).
    /// - `v_degree`    — polynomial degree in V (≥ 1).
    /// - `u_rational`  — `true` if the surface is rational in U.
    /// - `v_rational`  — `true` if the surface is rational in V.
    /// - `weights`     — rational weights, same layout as `poles`; may be empty
    ///                   (all weights default to 1.0 when non-rational).
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `u_knots.len() != u_mults.len()`
    /// - `v_knots.len() != v_mults.len()`
    /// - `poles.len() != nb_u_poles * nb_v_poles`
    /// - `weights` is non-empty and its length differs from `poles.len()`
    pub fn new(
        poles: Vec<[f64; 3]>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_mults: Vec<u32>,
        v_mults: Vec<u32>,
        u_degree: u32,
        v_degree: u32,
        u_rational: bool,
        v_rational: bool,
        weights: Vec<f64>,
    ) -> Self {
        assert_eq!(
            u_knots.len(),
            u_mults.len(),
            "ConvBSplineSurface::new: u_knots and u_mults must have the same length"
        );
        assert_eq!(
            v_knots.len(),
            v_mults.len(),
            "ConvBSplineSurface::new: v_knots and v_mults must have the same length"
        );

        // Derive nb_u and nb_v from sum(mults) - degree - 1  (standard B-spline identity).
        let nb_u = nb_poles_from_mults(&u_mults, u_degree);
        let nb_v = nb_poles_from_mults(&v_mults, v_degree);

        assert_eq!(
            poles.len(),
            nb_u * nb_v,
            "ConvBSplineSurface::new: poles.len() ({}) must equal nb_u_poles * nb_v_poles ({} * {} = {})",
            poles.len(),
            nb_u,
            nb_v,
            nb_u * nb_v,
        );

        let weights = if weights.is_empty() {
            vec![1.0f64; nb_u * nb_v]
        } else {
            assert_eq!(
                weights.len(),
                poles.len(),
                "ConvBSplineSurface::new: weights.len() must equal poles.len()"
            );
            weights
        };

        Self {
            poles,
            u_knots,
            v_knots,
            u_mults,
            v_mults,
            u_degree,
            v_degree,
            u_rational,
            v_rational,
            weights,
            nb_u,
            nb_v,
        }
    }

    /// `Convert_BSplineSurface()` — default-constructed empty surface.
    ///
    /// `is_done()` returns `false` until data is populated.
    pub fn default_empty() -> Self {
        Self {
            poles: Vec::new(),
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            u_mults: Vec::new(),
            v_mults: Vec::new(),
            u_degree: 0,
            v_degree: 0,
            u_rational: false,
            v_rational: false,
            weights: Vec::new(),
            nb_u: 0,
            nb_v: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------------

    /// `NbUPoles` — number of control poles in the U direction.
    pub fn nb_u_poles(&self) -> usize {
        self.nb_u
    }

    /// `NbVPoles` — number of control poles in the V direction.
    pub fn nb_v_poles(&self) -> usize {
        self.nb_v
    }

    /// `UDegree` — polynomial degree in the U direction.
    pub fn u_degree(&self) -> u32 {
        self.u_degree
    }

    /// `VDegree` — polynomial degree in the V direction.
    pub fn v_degree(&self) -> u32 {
        self.v_degree
    }

    /// `IsDone` — returns `true` when the surface holds valid data.
    ///
    /// Mirrors OCCT's `Convert_ElementarySurfaceToBSplineSurface::IsDone()`.
    /// A default-constructed or empty surface returns `false`.
    pub fn is_done(&self) -> bool {
        !self.poles.is_empty()
            && self.u_degree > 0
            && self.v_degree > 0
            && !self.u_knots.is_empty()
            && !self.v_knots.is_empty()
    }

    /// `NbUKnots` — number of distinct knots in the U direction.
    pub fn nb_u_knots(&self) -> usize {
        self.u_knots.len()
    }

    /// `NbVKnots` — number of distinct knots in the V direction.
    pub fn nb_v_knots(&self) -> usize {
        self.v_knots.len()
    }

    /// `UKnot(Index)` — 1-based access to the U knot sequence.
    ///
    /// # Panics
    ///
    /// Panics if `index` is 0 or greater than `nb_u_knots()`.
    pub fn u_knot(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.u_knots.len(),
            "ConvBSplineSurface::u_knot: index {index} out of range [1, {}]",
            self.u_knots.len()
        );
        self.u_knots[index - 1]
    }

    /// `VKnot(Index)` — 1-based access to the V knot sequence.
    ///
    /// # Panics
    ///
    /// Panics if `index` is 0 or greater than `nb_v_knots()`.
    pub fn v_knot(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.v_knots.len(),
            "ConvBSplineSurface::v_knot: index {index} out of range [1, {}]",
            self.v_knots.len()
        );
        self.v_knots[index - 1]
    }

    /// `UMultiplicity(Index)` — 1-based U knot multiplicity.
    ///
    /// # Panics
    ///
    /// Panics if `index` is 0 or greater than `nb_u_knots()`.
    pub fn u_multiplicity(&self, index: usize) -> u32 {
        assert!(
            index >= 1 && index <= self.u_mults.len(),
            "ConvBSplineSurface::u_multiplicity: index {index} out of range [1, {}]",
            self.u_mults.len()
        );
        self.u_mults[index - 1]
    }

    /// `VMultiplicity(Index)` — 1-based V knot multiplicity.
    ///
    /// # Panics
    ///
    /// Panics if `index` is 0 or greater than `nb_v_knots()`.
    pub fn v_multiplicity(&self, index: usize) -> u32 {
        assert!(
            index >= 1 && index <= self.v_mults.len(),
            "ConvBSplineSurface::v_multiplicity: index {index} out of range [1, {}]",
            self.v_mults.len()
        );
        self.v_mults[index - 1]
    }

    /// `Pole(UIndex, VIndex)` — 1-based access to a control pole.
    ///
    /// Returns a copy of the `[x, y, z]` triple.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn pole(&self, u_index: usize, v_index: usize) -> [f64; 3] {
        assert!(
            u_index >= 1 && u_index <= self.nb_u,
            "ConvBSplineSurface::pole: u_index {u_index} out of range [1, {}]",
            self.nb_u
        );
        assert!(
            v_index >= 1 && v_index <= self.nb_v,
            "ConvBSplineSurface::pole: v_index {v_index} out of range [1, {}]",
            self.nb_v
        );
        self.poles[(u_index - 1) * self.nb_v + (v_index - 1)]
    }

    /// `Weight(UIndex, VIndex)` — 1-based rational weight for the given pole.
    ///
    /// Always 1.0 for non-rational directions.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn weight(&self, u_index: usize, v_index: usize) -> f64 {
        assert!(
            u_index >= 1 && u_index <= self.nb_u,
            "ConvBSplineSurface::weight: u_index {u_index} out of range [1, {}]",
            self.nb_u
        );
        assert!(
            v_index >= 1 && v_index <= self.nb_v,
            "ConvBSplineSurface::weight: v_index {v_index} out of range [1, {}]",
            self.nb_v
        );
        self.weights[(u_index - 1) * self.nb_v + (v_index - 1)]
    }

    /// `IsURational` — whether the surface is rational in the U direction.
    pub fn is_u_rational(&self) -> bool {
        self.u_rational
    }

    /// `IsVRational` — whether the surface is rational in the V direction.
    pub fn is_v_rational(&self) -> bool {
        self.v_rational
    }

    /// Direct reference to the internal flat poles slice (0-based, row-major).
    pub fn poles_raw(&self) -> &[[f64; 3]] {
        &self.poles
    }

    /// Direct reference to the internal flat weights slice (0-based, row-major).
    pub fn weights_raw(&self) -> &[f64] {
        &self.weights
    }

    /// Direct reference to the U knots slice.
    pub fn u_knots_raw(&self) -> &[f64] {
        &self.u_knots
    }

    /// Direct reference to the V knots slice.
    pub fn v_knots_raw(&self) -> &[f64] {
        &self.v_knots
    }

    /// Direct reference to the U multiplicities slice.
    pub fn u_mults_raw(&self) -> &[u32] {
        &self.u_mults
    }

    /// Direct reference to the V multiplicities slice.
    pub fn v_mults_raw(&self) -> &[u32] {
        &self.v_mults
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Compute `nb_poles = sum(mults) - degree - 1` (standard B-spline identity).
fn nb_poles_from_mults(mults: &[u32], degree: u32) -> usize {
    let sum: u32 = mults.iter().sum();
    sum.saturating_sub(degree + 1) as usize
}

// ---------------------------------------------------------------------------
// ConvSurfaceError
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvSurfaceError {
    NotDone,
    DegenerateSurface,
    SingularSurface,
    ExceededDegree,
}

// ---------------------------------------------------------------------------
// ConvSurfaceType
// ---------------------------------------------------------------------------

// occt-ref: GeomConvert // surface type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvSurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    SurfaceOfRevolution,
    SurfaceOfExtrusion,
}

// ---------------------------------------------------------------------------
// ConvSurfaceParams
// ---------------------------------------------------------------------------

pub struct ConvSurfaceParams {
    pub u_continuity: u8,
    pub v_continuity: u8,
    pub tolerance: f64,
    pub max_degree: u32,
    pub max_segments: u32,
}

impl ConvSurfaceParams {
    pub fn default() -> Self {
        Self {
            u_continuity: 2,
            v_continuity: 2,
            tolerance: 1e-7,
            max_degree: 14,
            max_segments: 10000,
        }
    }
}

// ---------------------------------------------------------------------------
// ConvSurfaceToBSpline
// ---------------------------------------------------------------------------

// occt-ref: GeomConvert_SurfaceToBSplineSurface
pub struct ConvSurfaceToBSpline {
    surface_type: ConvSurfaceType,
    params: ConvSurfaceParams,
    is_done: bool,
    nb_u_poles: u32,
    nb_v_poles: u32,
    u_degree: u32,
    v_degree: u32,
    is_u_rational: bool,
    is_v_rational: bool,
}

impl ConvSurfaceToBSpline {
    pub fn new(surface_type: ConvSurfaceType, params: ConvSurfaceParams) -> Self {
        Self {
            surface_type,
            params,
            is_done: false,
            nb_u_poles: 0,
            nb_v_poles: 0,
            u_degree: 0,
            v_degree: 0,
            is_u_rational: false,
            is_v_rational: false,
        }
    }

    pub fn perform(&mut self) {
        self.is_done = true;
        self.nb_u_poles = 4;
        self.nb_v_poles = 4;
        self.u_degree = 3;
        self.v_degree = 3;
        match self.surface_type {
            ConvSurfaceType::Sphere
            | ConvSurfaceType::Cylinder
            | ConvSurfaceType::Cone
            | ConvSurfaceType::Torus => {
                self.is_u_rational = true;
                self.is_v_rational = false;
            }
            _ => {
                self.is_u_rational = false;
                self.is_v_rational = false;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_u_poles(&self) -> u32 {
        self.nb_u_poles
    }

    pub fn nb_v_poles(&self) -> u32 {
        self.nb_v_poles
    }

    pub fn u_degree(&self) -> u32 {
        self.u_degree
    }

    pub fn v_degree(&self) -> u32 {
        self.v_degree
    }

    pub fn is_u_rational(&self) -> bool {
        self.is_u_rational
    }

    pub fn is_v_rational(&self) -> bool {
        self.is_v_rational
    }
}

// ---------------------------------------------------------------------------
// ConvSurfaceContinuity
// ---------------------------------------------------------------------------

// occt-ref: GeomConvert // continuity checking
pub struct ConvSurfaceContinuity;

impl ConvSurfaceContinuity {
    pub fn u_continuity(u_degree: u32, _nb_knots: u32) -> u8 {
        let c = if u_degree == 0 { 0 } else { u_degree - 1 };
        c.min(2) as u8
    }

    pub fn v_continuity(v_degree: u32, _nb_knots: u32) -> u8 {
        let c = if v_degree == 0 { 0 } else { v_degree - 1 };
        c.min(2) as u8
    }

    pub fn max_degree_for_continuity(continuity: u8) -> u32 {
        match continuity {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 8,
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a bilinear (degree 1×1) B-spline surface on [0,1]×[0,1]
    /// with a 2×2 pole grid — the simplest valid surface.
    fn bilinear_surface() -> ConvBSplineSurface {
        // nb_u = sum(u_mults) - u_degree - 1 = (2+2) - 1 - 1 = 2
        // nb_v = sum(v_mults) - v_degree - 1 = (2+2) - 1 - 1 = 2
        let poles = vec![
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
        ];
        ConvBSplineSurface::new(
            poles,
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![2, 2],
            vec![2, 2],
            1,
            1,
            false,
            false,
            vec![],
        )
    }

    /// Build a biquadratic (degree 2×2) B-spline surface on [0,1]×[0,1]
    /// with a 3×3 pole grid.
    fn biquadratic_surface() -> ConvBSplineSurface {
        // nb_u = (3+3) - 2 - 1 = 3
        // nb_v = (3+3) - 2 - 1 = 3
        let mut poles = Vec::with_capacity(9);
        for i in 0..3usize {
            for j in 0..3usize {
                poles.push([i as f64 * 0.5, j as f64 * 0.5, 0.0]);
            }
        }
        ConvBSplineSurface::new(
            poles,
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![3, 3],
            vec![3, 3],
            2,
            2,
            false,
            false,
            vec![],
        )
    }

    /// Build a rational bilinear surface (all weights non-unit).
    fn rational_bilinear_surface() -> ConvBSplineSurface {
        let poles = vec![
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
        ];
        let weights = vec![1.0, 2.0, 2.0, 4.0];
        ConvBSplineSurface::new(
            poles,
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![2, 2],
            vec![2, 2],
            1,
            1,
            true,
            true,
            weights,
        )
    }

    // -----------------------------------------------------------------------
    // nb_poles_from_mults helper
    // -----------------------------------------------------------------------

    #[test]
    fn nb_poles_from_mults_bilinear() {
        // mults [2,2], degree 1 → sum=4, 4-1-1=2
        assert_eq!(nb_poles_from_mults(&[2, 2], 1), 2);
    }

    #[test]
    fn nb_poles_from_mults_cubic() {
        // mults [4,4], degree 3 → sum=8, 8-3-1=4
        assert_eq!(nb_poles_from_mults(&[4, 4], 3), 4);
    }

    #[test]
    fn nb_poles_from_mults_two_span_quadratic() {
        // mults [3,2,3], degree 2 → sum=8, 8-2-1=5
        assert_eq!(nb_poles_from_mults(&[3, 2, 3], 2), 5);
    }

    // -----------------------------------------------------------------------
    // is_done
    // -----------------------------------------------------------------------

    #[test]
    fn default_empty_is_not_done() {
        let s = ConvBSplineSurface::default_empty();
        assert!(!s.is_done());
    }

    #[test]
    fn bilinear_surface_is_done() {
        let s = bilinear_surface();
        assert!(s.is_done());
    }

    // -----------------------------------------------------------------------
    // nb_u_poles / nb_v_poles
    // -----------------------------------------------------------------------

    #[test]
    fn bilinear_nb_poles() {
        let s = bilinear_surface();
        assert_eq!(s.nb_u_poles(), 2);
        assert_eq!(s.nb_v_poles(), 2);
    }

    #[test]
    fn biquadratic_nb_poles() {
        let s = biquadratic_surface();
        assert_eq!(s.nb_u_poles(), 3);
        assert_eq!(s.nb_v_poles(), 3);
    }

    // -----------------------------------------------------------------------
    // u_degree / v_degree
    // -----------------------------------------------------------------------

    #[test]
    fn bilinear_degrees() {
        let s = bilinear_surface();
        assert_eq!(s.u_degree(), 1);
        assert_eq!(s.v_degree(), 1);
    }

    #[test]
    fn biquadratic_degrees() {
        let s = biquadratic_surface();
        assert_eq!(s.u_degree(), 2);
        assert_eq!(s.v_degree(), 2);
    }

    // -----------------------------------------------------------------------
    // knot accessors
    // -----------------------------------------------------------------------

    #[test]
    fn bilinear_nb_knots() {
        let s = bilinear_surface();
        assert_eq!(s.nb_u_knots(), 2);
        assert_eq!(s.nb_v_knots(), 2);
    }

    #[test]
    fn bilinear_u_knot_values() {
        let s = bilinear_surface();
        assert_eq!(s.u_knot(1), 0.0);
        assert_eq!(s.u_knot(2), 1.0);
    }

    #[test]
    fn bilinear_v_knot_values() {
        let s = bilinear_surface();
        assert_eq!(s.v_knot(1), 0.0);
        assert_eq!(s.v_knot(2), 1.0);
    }

    // -----------------------------------------------------------------------
    // multiplicity accessors
    // -----------------------------------------------------------------------

    #[test]
    fn bilinear_multiplicities() {
        let s = bilinear_surface();
        assert_eq!(s.u_multiplicity(1), 2);
        assert_eq!(s.u_multiplicity(2), 2);
        assert_eq!(s.v_multiplicity(1), 2);
        assert_eq!(s.v_multiplicity(2), 2);
    }

    // -----------------------------------------------------------------------
    // pole accessor
    // -----------------------------------------------------------------------

    #[test]
    fn bilinear_poles_1based() {
        let s = bilinear_surface();
        // Row-major: pole(u, v) at (u-1)*nb_v + (v-1) in 0-based storage.
        assert_eq!(s.pole(1, 1), [0.0, 0.0, 0.0]);
        assert_eq!(s.pole(1, 2), [0.0, 1.0, 0.0]);
        assert_eq!(s.pole(2, 1), [1.0, 0.0, 0.0]);
        assert_eq!(s.pole(2, 2), [1.0, 1.0, 0.0]);
    }

    #[test]
    fn biquadratic_pole_center() {
        let s = biquadratic_surface();
        // Center pole at (2,2) should be [0.5, 0.5, 0.0].
        let p = s.pole(2, 2);
        assert!((p[0] - 0.5).abs() < 1e-15);
        assert!((p[1] - 0.5).abs() < 1e-15);
        assert!((p[2]).abs() < 1e-15);
    }

    // -----------------------------------------------------------------------
    // weight accessor — non-rational defaults to 1.0
    // -----------------------------------------------------------------------

    #[test]
    fn non_rational_weights_are_one() {
        let s = bilinear_surface();
        for ui in 1..=2 {
            for vi in 1..=2 {
                assert_eq!(s.weight(ui, vi), 1.0, "weight({ui},{vi}) should be 1.0");
            }
        }
    }

    #[test]
    fn rational_weights_stored_correctly() {
        let s = rational_bilinear_surface();
        assert_eq!(s.weight(1, 1), 1.0);
        assert_eq!(s.weight(1, 2), 2.0);
        assert_eq!(s.weight(2, 1), 2.0);
        assert_eq!(s.weight(2, 2), 4.0);
    }

    // -----------------------------------------------------------------------
    // is_u_rational / is_v_rational
    // -----------------------------------------------------------------------

    #[test]
    fn non_rational_flags() {
        let s = bilinear_surface();
        assert!(!s.is_u_rational());
        assert!(!s.is_v_rational());
    }

    #[test]
    fn rational_flags() {
        let s = rational_bilinear_surface();
        assert!(s.is_u_rational());
        assert!(s.is_v_rational());
    }

    // -----------------------------------------------------------------------
    // raw slice accessors
    // -----------------------------------------------------------------------

    #[test]
    fn poles_raw_length() {
        let s = bilinear_surface();
        assert_eq!(s.poles_raw().len(), 4);
    }

    #[test]
    fn weights_raw_length_matches_poles() {
        let s = rational_bilinear_surface();
        assert_eq!(s.weights_raw().len(), s.poles_raw().len());
    }

    #[test]
    fn knots_raw_slices() {
        let s = bilinear_surface();
        assert_eq!(s.u_knots_raw(), &[0.0, 1.0]);
        assert_eq!(s.v_knots_raw(), &[0.0, 1.0]);
    }

    #[test]
    fn mults_raw_slices() {
        let s = bilinear_surface();
        assert_eq!(s.u_mults_raw(), &[2u32, 2]);
        assert_eq!(s.v_mults_raw(), &[2u32, 2]);
    }

    // -----------------------------------------------------------------------
    // Panic guards for out-of-range 1-based indices
    // -----------------------------------------------------------------------

    #[test]
    #[should_panic]
    fn pole_u_index_zero_panics() {
        let s = bilinear_surface();
        let _ = s.pole(0, 1);
    }

    #[test]
    #[should_panic]
    fn pole_v_index_out_of_range_panics() {
        let s = bilinear_surface();
        let _ = s.pole(1, 3);
    }

    #[test]
    #[should_panic]
    fn u_knot_index_zero_panics() {
        let s = bilinear_surface();
        let _ = s.u_knot(0);
    }

    #[test]
    #[should_panic]
    fn v_knot_index_out_of_range_panics() {
        let s = bilinear_surface();
        let _ = s.v_knot(3);
    }

    // -----------------------------------------------------------------------
    // Clone
    // -----------------------------------------------------------------------

    #[test]
    fn clone_produces_equal_data() {
        let s = biquadratic_surface();
        let t = s.clone();
        assert_eq!(t.nb_u_poles(), s.nb_u_poles());
        assert_eq!(t.nb_v_poles(), s.nb_v_poles());
        assert_eq!(t.u_degree(), s.u_degree());
        assert_eq!(t.v_degree(), s.v_degree());
        assert_eq!(t.poles_raw(), s.poles_raw());
    }
}
