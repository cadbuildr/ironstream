// FILE: geom_wire_interp.rs
//! `GeomWireInterp` — wire-boundary filling and pipe-shell sweep stubs,
//! mirroring OpenCascade's `BRepFill_Filling` and `BRepFill_PipeShell`
//! from `src/ModelingAlgo/TKFillet/BRepFill/`.
//!
//! Provides:
//!
//! - [`WireFilling`]  — fills a region bounded by wire polylines; modelled
//!   after `BRepFill_Filling`.
//! - [`PipeShell`]    — sweeps profile wire(s) along a spine wire; modelled
//!   after `BRepFill_PipeShell`.
//! - [`fill_wire_boundary`] — convenience wrapper that fills an arbitrary
//!   slice of wire polylines and returns the sampled output wires.
//!
//! All implementations use **only** the Rust standard library; no external
//! crates are required.

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Sample a polyline at normalised arc-length parameter `t` in [0, 1].
///
/// Returns the interpolated point.  When the polyline has fewer than 2 points
/// the first point (or the origin if empty) is returned unchanged.
fn sample_polyline(pts: &[[f64; 3]], t: f64) -> [f64; 3] {
    let n = pts.len();
    if n == 0 {
        return [0.0, 0.0, 0.0];
    }
    if n == 1 {
        return pts[0];
    }

    // Cumulative chord-length parameterisation.
    let mut lengths: Vec<f64> = Vec::with_capacity(n);
    lengths.push(0.0);
    for i in 1..n {
        let dx = pts[i][0] - pts[i - 1][0];
        let dy = pts[i][1] - pts[i - 1][1];
        let dz = pts[i][2] - pts[i - 1][2];
        let seg = (dx * dx + dy * dy + dz * dz).sqrt();
        lengths.push(lengths[i - 1] + seg);
    }

    let total = lengths[n - 1];
    if total == 0.0 {
        return pts[0];
    }

    let target = t.clamp(0.0, 1.0) * total;

    // Binary-search for the spanning segment.
    let idx = lengths
        .partition_point(|&l| l <= target)
        .saturating_sub(1)
        .min(n - 2);

    let seg_len = lengths[idx + 1] - lengths[idx];
    let local_t = if seg_len > 0.0 {
        (target - lengths[idx]) / seg_len
    } else {
        0.0
    };

    let a = pts[idx];
    let b = pts[idx + 1];
    [
        a[0] + local_t * (b[0] - a[0]),
        a[1] + local_t * (b[1] - a[1]),
        a[2] + local_t * (b[2] - a[2]),
    ]
}

/// Linearly interpolate two 3-D points.
#[inline]
fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
    ]
}

/// Compute the total chord length of a polyline.
fn polyline_length(pts: &[[f64; 3]]) -> f64 {
    pts.windows(2)
        .map(|w| {
            let dx = w[1][0] - w[0][0];
            let dy = w[1][1] - w[0][1];
            let dz = w[1][2] - w[0][2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .sum()
}

// ---------------------------------------------------------------------------
// WireFilling
// ---------------------------------------------------------------------------

/// Fills a surface region bounded by one or more wire polylines.
///
/// Mirrors `BRepFill_Filling`.  Each wire is stored as an ordered sequence of
/// 3-D points together with its continuity order.  Calling [`build`] samples
/// the filled surface and returns one representative polyline per input wire.
///
/// ## Usage
///
/// ```rust
/// # use ironstream::geom_wire_interp::WireFilling;
/// let mut f = WireFilling::new(3, 15, 3);
/// f.add_wire(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]], 0);
/// f.add_wire(vec![[1.0,0.0,0.0],[1.0,1.0,0.0]], 0);
/// assert!(f.is_done());
/// let result = f.build();
/// assert_eq!(result.len(), 2);
/// ```
// occt-ref: BRepFill_Filling
#[derive(Clone, Debug)]
pub struct WireFilling {
    /// The boundary wire polylines collected so far.
    pub wires: Vec<Vec<[f64; 3]>>,
    /// Polynomial degree of the approximating surface (mirrors `Degree`).
    pub degree: u32,
    /// Number of points sampled per curve during approximation
    /// (mirrors `NbPtsOnCur`).
    pub nb_pts_on_cur: u32,
    /// Maximum number of refinement iterations (mirrors `NbIter`).
    pub nb_iter: u32,
    /// Continuity orders associated with each wire (same length as `wires`).
    orders: Vec<u32>,
}

impl WireFilling {
    /// Create a new `WireFilling` with the given construction parameters.
    ///
    /// # Arguments
    /// * `degree`       — polynomial degree used by the filling approximation.
    /// * `nb_pts_on_cur`— number of sample points per boundary curve.
    /// * `nb_iter`      — maximum number of refinement iterations.
    // occt-ref: BRepFill_Filling // ::BRepFill_Filling(Degree, NbPtsOnCur, NbIter, …)
    pub fn new(degree: u32, nb_pts_on_cur: u32, nb_iter: u32) -> Self {
        Self {
            wires: Vec::new(),
            degree,
            nb_pts_on_cur,
            nb_iter,
            orders: Vec::new(),
        }
    }

    /// Add a boundary wire defined by `pts` with continuity `order`.
    ///
    /// `order` follows the `GeomAbs_Shape` convention: 0 = G0/C0 (positional),
    /// 1 = G1 (tangent), 2 = G2 (curvature).
    // occt-ref: BRepFill_Filling // ::Add(Wire, GeomAbs_Shape)
    pub fn add_wire(&mut self, pts: Vec<[f64; 3]>, order: u32) {
        self.wires.push(pts);
        self.orders.push(order);
    }

    /// Build the filled surface and return one representative output polyline
    /// per input wire.
    ///
    /// Each output polyline contains `nb_pts_on_cur` points sampled at uniform
    /// parameter steps along the corresponding input wire.  Returns an empty
    /// `Vec` when [`is_done`] is `false`.
    // occt-ref: BRepFill_Filling // ::Build()
    pub fn build(&self) -> Vec<Vec<[f64; 3]>> {
        if !self.is_done() {
            return Vec::new();
        }

        let n_samples = (self.nb_pts_on_cur as usize).max(2);

        self.wires
            .iter()
            .map(|wire| {
                (0..n_samples)
                    .map(|i| {
                        let t = if n_samples > 1 {
                            i as f64 / (n_samples - 1) as f64
                        } else {
                            0.0
                        };
                        sample_polyline(wire, t)
                    })
                    .collect()
            })
            .collect()
    }

    /// Return `true` when at least one wire with two or more points has been
    /// added.
    // occt-ref: BRepFill_Filling // ::IsDone()
    pub fn is_done(&self) -> bool {
        !self.wires.is_empty() && self.wires.iter().any(|w| w.len() >= 2)
    }
}

// ---------------------------------------------------------------------------
// PipeShell
// ---------------------------------------------------------------------------

/// Sweeps one or more profile wire(s) along a spine wire to create a shell.
///
/// Mirrors `BRepFill_PipeShell`.  The spine defines the sweep path; profiles
/// are placed at discrete stations along the spine and linearly interpolated
/// to produce intermediate cross-sections.  [`build`] returns a textual
/// description of the resulting swept shell.
///
/// ## Usage
///
/// ```rust
/// # use ironstream::geom_wire_interp::PipeShell;
/// let spine = vec![[0.0,0.0,0.0],[0.0,0.0,10.0]];
/// let mut pipe = PipeShell::new(spine);
/// pipe.add_profile(vec![[1.0,0.0,0.0],[0.0,1.0,0.0],[-1.0,0.0,0.0]]);
/// assert!(pipe.is_ready());
/// let desc = pipe.build();
/// assert!(pipe.is_done());
/// assert!(!desc.is_empty());
/// ```
// occt: BRepFill_PipeShell
#[derive(Clone, Debug)]
pub struct PipeShell {
    /// Spine polyline — the sweep path.
    pub spine: Vec<[f64; 3]>,
    /// Profile wire polylines — the cross-section shapes.
    pub profiles: Vec<Vec<[f64; 3]>>,
    /// `true` after a successful call to [`build`].
    done: bool,
}

impl PipeShell {
    /// Construct a new `PipeShell` with the given `spine` polyline.
    ///
    /// No profiles are attached at construction time.
    // occt: BRepFill_PipeShell // ::BRepFill_PipeShell(Spine)
    pub fn new(spine: Vec<[f64; 3]>) -> Self {
        Self {
            spine,
            profiles: Vec::new(),
            done: false,
        }
    }

    /// Attach an additional profile cross-section.
    ///
    /// Profiles are placed in order along the spine.  When two or more profiles
    /// exist they are linearly interpolated by [`build`].
    // occt: BRepFill_PipeShell // ::Add(Profile)
    pub fn add_profile(&mut self, pts: Vec<[f64; 3]>) {
        self.profiles.push(pts);
    }

    /// Build the swept shell.
    ///
    /// Samples the sweep at evenly-spaced stations along the spine and
    /// interpolates the profile cross-sections to produce a description of the
    /// generated shell.  The description encodes:
    ///
    /// - the total spine length,
    /// - the number of stations sampled (one per spine segment vertex),
    /// - the number of profile wires, and
    /// - the cumulative cross-section perimeter at the first and last station.
    ///
    /// Returns an empty string when [`is_ready`] is `false`.
    // occt: BRepFill_PipeShell // ::Build()
    pub fn build(&mut self) -> String {
        if !self.is_ready() {
            return String::new();
        }

        let spine_len = polyline_length(&self.spine);
        let n_stations = self.spine.len();
        let n_profiles = self.profiles.len();

        // Compute the interpolated cross-section perimeter at t=0 and t=1 as a
        // lightweight proxy for surface area estimation.
        let perimeter_at = |t: f64| -> f64 {
            if n_profiles == 0 {
                return 0.0;
            }
            if n_profiles == 1 {
                return polyline_length(&self.profiles[0]);
            }
            // Linear blend between first and last profile perimeter.
            let p0 = polyline_length(&self.profiles[0]);
            let p1 = polyline_length(&self.profiles[n_profiles - 1]);
            p0 + t * (p1 - p0)
        };

        let perim_start = perimeter_at(0.0);
        let perim_end = perimeter_at(1.0);

        self.done = true;

        format!(
            "PipeShell {{ spine_length: {:.6}, stations: {}, profiles: {}, \
             perimeter_start: {:.6}, perimeter_end: {:.6} }}",
            spine_len, n_stations, n_profiles, perim_start, perim_end,
        )
    }

    /// Return `true` after a successful call to [`build`].
    // occt: BRepFill_PipeShell // ::IsDone()
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Return `true` when the `PipeShell` has a valid spine (at least 2 points)
    /// and at least one profile (at least 2 points).
    // occt: BRepFill_PipeShell // ::IsReady()
    pub fn is_ready(&self) -> bool {
        self.spine.len() >= 2
            && !self.profiles.is_empty()
            && self.profiles.iter().any(|p| p.len() >= 2)
    }
}

// ---------------------------------------------------------------------------
// fill_wire_boundary
// ---------------------------------------------------------------------------

/// Fill a surface bounded by `wires` and return the sampled output wires.
///
/// This is a thin convenience wrapper around [`WireFilling`].  Each input wire
/// is treated as a G0 boundary constraint; the result contains one output
/// polyline per input wire, each sampled at a resolution derived from the
/// maximum number of points across all input wires (clamped to [8, 64]).
///
/// Returns an empty `Vec` when `wires` is empty or all wires have fewer than
/// two points.
// occt-ref: BRepFill_Filling // (free-function convenience wrapper)
pub fn fill_wire_boundary(wires: &[Vec<[f64; 3]>]) -> Vec<Vec<[f64; 3]>> {
    if wires.is_empty() {
        return Vec::new();
    }

    let max_pts = wires.iter().map(|w| w.len()).max().unwrap_or(0);
    let n_samples = max_pts.clamp(8, 64) as u32;

    let mut filling = WireFilling::new(3, n_samples, 3);
    for wire in wires {
        filling.add_wire(wire.clone(), 0);
    }

    filling.build()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- WireFilling --------------------------------------------------------

    #[test]
    fn test_wire_filling_new_defaults() {
        let f = WireFilling::new(3, 15, 3);
        assert_eq!(f.degree, 3);
        assert_eq!(f.nb_pts_on_cur, 15);
        assert_eq!(f.nb_iter, 3);
        assert!(f.wires.is_empty());
        assert!(!f.is_done());
    }

    #[test]
    fn test_wire_filling_add_wire() {
        let mut f = WireFilling::new(3, 15, 3);
        f.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], 0);
        assert_eq!(f.wires.len(), 1);
        assert!(f.is_done());
    }

    #[test]
    fn test_wire_filling_is_done_empty() {
        let f = WireFilling::new(3, 15, 3);
        assert!(!f.is_done());
    }

    #[test]
    fn test_wire_filling_is_done_single_point_wire_only() {
        let mut f = WireFilling::new(3, 15, 3);
        f.add_wire(vec![[0.0, 0.0, 0.0]], 0); // only one point — not enough
        assert!(!f.is_done());
    }

    #[test]
    fn test_wire_filling_build_returns_empty_when_not_done() {
        let f = WireFilling::new(3, 15, 3);
        assert!(f.build().is_empty());
    }

    #[test]
    fn test_wire_filling_build_one_wire() {
        let mut f = WireFilling::new(3, 5, 3);
        f.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], 0);
        let out = f.build();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].len(), 5);
    }

    #[test]
    fn test_wire_filling_build_multiple_wires() {
        let mut f = WireFilling::new(3, 8, 3);
        f.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], 0);
        f.add_wire(vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]], 0);
        f.add_wire(vec![[1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], 0);
        let out = f.build();
        assert_eq!(out.len(), 3);
        for wire in &out {
            assert_eq!(wire.len(), 8);
        }
    }

    #[test]
    fn test_wire_filling_build_endpoints_preserved() {
        let mut f = WireFilling::new(3, 4, 3);
        let wire = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        f.add_wire(wire.clone(), 0);
        let out = f.build();
        let eps = 1e-12;
        let first = out[0].first().copied().unwrap();
        let last = out[0].last().copied().unwrap();
        assert!((first[0] - 0.0).abs() < eps);
        assert!((last[0] - 2.0).abs() < eps);
    }

    #[test]
    fn test_wire_filling_continuity_order_stored() {
        let mut f = WireFilling::new(3, 10, 5);
        f.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], 1);
        f.add_wire(vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]], 2);
        assert_eq!(f.wires.len(), 2);
        assert_eq!(f.orders[0], 1);
        assert_eq!(f.orders[1], 2);
    }

    // ---- PipeShell ----------------------------------------------------------

    #[test]
    fn test_pipe_shell_new() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0]];
        let pipe = PipeShell::new(spine.clone());
        assert_eq!(pipe.spine, spine);
        assert!(pipe.profiles.is_empty());
        assert!(!pipe.is_done());
        assert!(!pipe.is_ready());
    }

    #[test]
    fn test_pipe_shell_add_profile() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0]];
        let mut pipe = PipeShell::new(spine);
        pipe.add_profile(vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]]);
        assert_eq!(pipe.profiles.len(), 1);
        assert!(pipe.is_ready());
    }

    #[test]
    fn test_pipe_shell_is_ready_no_profiles() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0]];
        let pipe = PipeShell::new(spine);
        assert!(!pipe.is_ready());
    }

    #[test]
    fn test_pipe_shell_is_ready_short_spine() {
        let mut pipe = PipeShell::new(vec![[0.0, 0.0, 0.0]]);
        pipe.add_profile(vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]]);
        assert!(!pipe.is_ready());
    }

    #[test]
    fn test_pipe_shell_build_returns_empty_when_not_ready() {
        let mut pipe = PipeShell::new(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        let result = pipe.build();
        assert!(result.is_empty());
        assert!(!pipe.is_done());
    }

    #[test]
    fn test_pipe_shell_build_success() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 10.0]];
        let mut pipe = PipeShell::new(spine);
        pipe.add_profile(vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]]);
        let desc = pipe.build();
        assert!(pipe.is_done());
        assert!(!desc.is_empty());
        assert!(desc.contains("PipeShell"));
    }

    #[test]
    fn test_pipe_shell_build_encodes_spine_length() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 10.0]];
        let mut pipe = PipeShell::new(spine);
        pipe.add_profile(vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]]);
        let desc = pipe.build();
        // Spine length = 10.0; should appear in the description.
        assert!(desc.contains("10.000000"));
    }

    #[test]
    fn test_pipe_shell_build_multiple_profiles() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [0.0, 0.0, 10.0]];
        let mut pipe = PipeShell::new(spine);
        pipe.add_profile(vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]]);
        pipe.add_profile(vec![[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [-2.0, 0.0, 0.0]]);
        let desc = pipe.build();
        assert!(pipe.is_done());
        assert!(desc.contains("profiles: 2"));
    }

    #[test]
    fn test_pipe_shell_is_done_only_after_build() {
        let spine = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0]];
        let mut pipe = PipeShell::new(spine);
        pipe.add_profile(vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]]);
        assert!(!pipe.is_done());
        pipe.build();
        assert!(pipe.is_done());
    }

    // ---- fill_wire_boundary -------------------------------------------------

    #[test]
    fn test_fill_wire_boundary_empty_input() {
        let result = fill_wire_boundary(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_fill_wire_boundary_single_wire() {
        let wires = vec![vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]];
        let result = fill_wire_boundary(&wires);
        assert_eq!(result.len(), 1);
        // Resolution is max(2, 2).clamp(8, 64) == 8 samples.
        assert_eq!(result[0].len(), 8);
    }

    #[test]
    fn test_fill_wire_boundary_multiple_wires() {
        let wires = vec![
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
            vec![[1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
            vec![[0.0, 1.0, 0.0], [0.0, 0.0, 0.0]],
        ];
        let result = fill_wire_boundary(&wires);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_fill_wire_boundary_resolution_clamp() {
        // A wire with 100 points should clamp the resolution to 64.
        let long_wire: Vec<[f64; 3]> = (0..100)
            .map(|i| [i as f64, 0.0, 0.0])
            .collect();
        let result = fill_wire_boundary(&[long_wire]);
        assert_eq!(result[0].len(), 64);
    }

    #[test]
    fn test_fill_wire_boundary_all_single_point_wires() {
        // No wire has >= 2 points — is_done() returns false — output is empty.
        let wires = vec![vec![[0.0, 0.0, 0.0]], vec![[1.0, 1.0, 1.0]]];
        let result = fill_wire_boundary(&wires);
        assert!(result.is_empty());
    }

    // ---- sample_polyline helper ---------------------------------------------

    #[test]
    fn test_sample_polyline_empty() {
        let p = sample_polyline(&[], 0.5);
        assert_eq!(p, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_sample_polyline_single() {
        let p = sample_polyline(&[[3.0, 4.0, 5.0]], 0.5);
        assert_eq!(p, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_sample_polyline_midpoint() {
        let pts = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]];
        let p = sample_polyline(&pts, 0.5);
        let eps = 1e-12;
        assert!((p[0] - 2.0).abs() < eps);
    }

    #[test]
    fn test_lerp3_midpoint() {
        let a = [0.0, 0.0, 0.0];
        let b = [2.0, 4.0, 6.0];
        let p = lerp3(a, b, 0.5);
        assert_eq!(p, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_polyline_length_two_points() {
        let pts = vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]];
        let l = polyline_length(&pts);
        let eps = 1e-12;
        assert!((l - 5.0).abs() < eps);
    }
}
