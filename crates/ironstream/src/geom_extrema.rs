// FILE: rust/ironstream/crates/ironstream/src/geom_extrema.rs
//! `GeomAPI_ExtremaCurveCurve` / `GeomAPI_ExtremaCurveSurface` — extrema
//! (minimum-distance) algorithms between geometric entities, mirroring
//! OpenCascade's `GeomAPI` package
//! (`src/ModelingAlgorithms/TKGeomBase/GeomAPI`).
//!
//! These are **stub** implementations: the perform methods return synthetic
//! results (distance = 1.0) so the public API surface is available for
//! compilation and testing without requiring a full numerical solver.
//!
//! Covered types:
//! - [`GeomExtremaResult`]             — a matched point-pair with distance
//! - [`GeomApiExtremaCurveCurve`]      — `GeomAPI_ExtremaCurveCurve`
//! - [`GeomApiExtremaCurveSurface`]    — `GeomAPI_ExtremaCurveSurface`

// ─────────────────────────────────────────────────────────────────────────────
// GeomExtremaResult
// ─────────────────────────────────────────────────────────────────────────────

/// A single extrema result: a pair of closest (or farthest) points, one on
/// each geometric entity, together with the distance between them and the
/// corresponding parameter values.
// occt: extrema result point pair — returned by GeomAPI_ExtremaCurveCurve and
//       GeomAPI_ExtremaCurveSurface
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomExtremaResult {
    /// Point on the first entity (curve or curve).
    pt1: [f64; 3],
    /// Point on the second entity (curve or surface).
    pt2: [f64; 3],
    /// Distance between `pt1` and `pt2`.
    distance: f64,
    /// Parameter value on the first entity at `pt1`.
    param1: f64,
    /// Parameter value on the second entity at `pt2`.
    param2: f64,
}

impl GeomExtremaResult {
    /// Construct a new extrema result from its components.
    ///
    /// - `pt1`      — 3D point on the first entity
    /// - `pt2`      — 3D point on the second entity
    /// - `distance` — distance between the two points
    /// - `param1`   — parameter on the first entity
    /// - `param2`   — parameter on the second entity
    // occt: GeomAPI_ExtremaCurveCurve::NearestPoints / ::Points
    pub fn new(
        pt1: [f64; 3],
        pt2: [f64; 3],
        distance: f64,
        param1: f64,
        param2: f64,
    ) -> Self {
        Self { pt1, pt2, distance, param1, param2 }
    }

    /// The 3D point on the first entity.
    // occt: GeomAPI_ExtremaCurveCurve::NearestPoints(P1, P2) — P1
    pub fn pt1(&self) -> [f64; 3] {
        self.pt1
    }

    /// The 3D point on the second entity.
    // occt: GeomAPI_ExtremaCurveCurve::NearestPoints(P1, P2) — P2
    pub fn pt2(&self) -> [f64; 3] {
        self.pt2
    }

    /// The distance between `pt1` and `pt2`.
    // occt: GeomAPI_ExtremaCurveCurve::LowerDistance
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// The `(param1, param2)` parameter pair for this extremum.
    // occt: GeomAPI_ExtremaCurveCurve::Parameters(Index, U1, U2)
    pub fn parameters(&self) -> (f64, f64) {
        (self.param1, self.param2)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiExtremaCurveCurve
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `GeomAPI_ExtremaCurveCurve` — computes all extrema (minimum or
/// maximum distance points) between two 3D curves.
///
/// The `perform` method is a stub that injects `nb` synthetic results, each
/// with `distance = 1.0`, so that callers can exercise the full API without a
/// real numerical solver.
// occt: GeomAPI_ExtremaCurveCurve stub
#[derive(Clone, Debug)]
pub struct GeomApiExtremaCurveCurve {
    /// All extrema results found (or injected by the stub).
    results: Vec<GeomExtremaResult>,
    /// Whether the algorithm has been performed successfully.
    is_done: bool,
}

impl GeomApiExtremaCurveCurve {
    /// Construct a new, un-performed extrema computer.
    // occt: GeomAPI_ExtremaCurveCurve::GeomAPI_ExtremaCurveCurve()
    pub fn new() -> Self {
        Self { results: Vec::new(), is_done: false }
    }

    /// Stub `Perform`: injects `nb` synthetic extrema results, each with
    /// `distance = 1.0` and zero-origin points/parameters.
    ///
    /// In a full implementation this would accept the two curves and solve the
    /// minimax system numerically (OCCT delegates to `Extrema_ExtCC`).
    // occt: GeomAPI_ExtremaCurveCurve::Perform (stub — no real solver)
    pub fn perform(&mut self, nb: usize) {
        self.results.clear();
        for i in 0..nb {
            let f = i as f64;
            self.results.push(GeomExtremaResult::new(
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                1.0,
                f,
                f,
            ));
        }
        self.is_done = true;
    }

    /// `IsDone()` — true if `perform` has been called successfully.
    // occt: GeomAPI_ExtremaCurveCurve::IsDone
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `NbExtrema()` — total number of extrema found.
    // occt: GeomAPI_ExtremaCurveCurve::NbExtrema
    pub fn nb_extrema(&self) -> usize {
        self.results.len()
    }

    /// `Points(Index)` / `Distance(Index)` accessor — returns the i-th result
    /// (0-based index).  Returns `None` if out of range.
    // occt: GeomAPI_ExtremaCurveCurve::Points / ::Distance (1-based in OCCT)
    pub fn result(&self, i: usize) -> &GeomExtremaResult {
        &self.results[i]
    }

    /// Returns the indices `(i1, i2)` (0-based) of the two results that share
    /// the minimum distance.  In the stub all results have the same distance,
    /// so `(0, 0)` is returned when at least one result exists, otherwise
    /// `(0, 0)` with `is_done == false`.
    ///
    /// In a real implementation the pair with the globally smallest distance
    /// would be identified.
    // occt: GeomAPI_ExtremaCurveCurve::LowerDistanceParameters (index form)
    pub fn lower_distance_indices(&self) -> (usize, usize) {
        if self.results.is_empty() {
            return (0, 0);
        }
        let mut best_idx = 0;
        let mut best_dist = self.results[0].distance;
        for (i, r) in self.results.iter().enumerate() {
            if r.distance < best_dist {
                best_dist = r.distance;
                best_idx = i;
            }
        }
        (best_idx, best_idx)
    }

    /// `LowerDistance()` — the minimum distance among all extrema.
    // occt: GeomAPI_ExtremaCurveCurve::LowerDistance
    pub fn lower_distance(&self) -> f64 {
        self.results
            .iter()
            .map(|r| r.distance)
            .fold(f64::MAX, f64::min)
    }
}

impl Default for GeomApiExtremaCurveCurve {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiExtremaCurveSurface
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `GeomAPI_ExtremaCurveSurface` — computes all extrema (minimum
/// distance points) between a 3D curve and a surface.
///
/// The `perform` method is a stub that sets `is_done = true` and stores a
/// single synthetic result with `distance = 1.0`.
// occt: GeomAPI_ExtremaCurveSurface stub
#[derive(Clone, Debug)]
pub struct GeomApiExtremaCurveSurface {
    /// The single nearest-result (or stub result).
    result: GeomExtremaResult,
    /// Whether the algorithm has been performed successfully.
    is_done: bool,
}

impl GeomApiExtremaCurveSurface {
    /// Construct a new, un-performed extrema computer.
    // occt: GeomAPI_ExtremaCurveSurface::GeomAPI_ExtremaCurveSurface()
    pub fn new() -> Self {
        Self {
            result: GeomExtremaResult::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0, 0.0, 0.0),
            is_done: false,
        }
    }

    /// Stub `Perform`: sets `is_done = true` and stores a synthetic result
    /// with `distance = 1.0`.
    ///
    /// In a full implementation this would accept the curve and surface and
    /// solve the minimax system numerically (OCCT delegates to
    /// `Extrema_ExtCS`).
    // occt: GeomAPI_ExtremaCurveSurface::Perform (stub — no real solver)
    pub fn perform(&mut self) {
        self.result = GeomExtremaResult::new(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            1.0,
            0.0,
            0.0,
        );
        self.is_done = true;
    }

    /// `IsDone()` — true if `perform` has been called successfully.
    // occt: GeomAPI_ExtremaCurveSurface::IsDone
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `NbExtrema()` — always 1 after a successful `perform` (stub).
    // occt: GeomAPI_ExtremaCurveSurface::NbExtrema
    pub fn nb_extrema(&self) -> usize {
        if self.is_done { 1 } else { 0 }
    }

    /// `PointOnCurve()` — the point on the curve at the nearest extremum.
    // occt: GeomAPI_ExtremaCurveSurface::NearestPoints — curve side
    pub fn point_on_curve(&self) -> [f64; 3] {
        self.result.pt1()
    }

    /// `PointOnSurface()` — the point on the surface at the nearest extremum.
    // occt: GeomAPI_ExtremaCurveSurface::NearestPoints — surface side
    pub fn point_on_surface(&self) -> [f64; 3] {
        self.result.pt2()
    }

    /// `LowerDistance()` — the minimum distance found.
    // occt: GeomAPI_ExtremaCurveSurface::LowerDistance
    pub fn lower_distance(&self) -> f64 {
        self.result.distance()
    }

    /// The full result record (point pair, distance, parameters).
    // occt: GeomAPI_ExtremaCurveSurface combined accessor
    pub fn result(&self) -> &GeomExtremaResult {
        &self.result
    }
}

impl Default for GeomApiExtremaCurveSurface {
    fn default() -> Self {
        Self::new()
    }
}
