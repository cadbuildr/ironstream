// FILE: geom_hatch.rs
//! `GeomHatch` — hatching domain and hatcher types, mirroring OpenCascade's
//! `HatchGen_Domain` and `Hatch_Hatcher` packages.
//!
//! # Scope
//!
//! This module provides:
//!
//! * [`HatchDomain`] — a parameter interval `[u_start, u_end]` on a hatch line,
//!   together with an optional edge index.  Mirrors `HatchGen_Domain`.
//! * [`Hatcher`] — a collection of trimmed hatch lines and the logic to add,
//!   trim, and query them.  Mirrors `Hatch_Hatcher`.
//! * [`hatch_pattern`] — a convenience function that generates a regular grid of
//!   hatch line segments clipped to an axis-aligned bounding box.
//!
//! # Notes
//!
//! All types are zero-dependency pure-Rust data structures; only `std` is used.
//! Coordinates are 2-D `[f64; 2]` pairs throughout.

// ─────────────────────────────────────────────────────────────────────────────
// HatchDomain
// ─────────────────────────────────────────────────────────────────────────────

/// A parameter interval on a single hatch line, bounded by `u_start` and
/// `u_end`, optionally associated with a boundary edge index.
///
/// The interval is always normalised so that `u_start <= u_end`.
///
// occt-ref: HatchGen_Domain
#[derive(Clone, Debug, PartialEq)]
pub struct HatchDomain {
    /// Lower parameter bound of the domain on the hatch line.
    pub u_start: f64,
    /// Upper parameter bound of the domain on the hatch line.
    pub u_end: f64,
    /// 1-based index of the boundary edge that produced this domain, or `-1`
    /// when the domain was created without an explicit edge reference.
    pub edge_index: i32,
}

impl HatchDomain {
    /// Create a new bounded domain spanning `[us, ue]`.
    ///
    /// If `us > ue` the values are silently swapped so the invariant
    /// `u_start <= u_end` always holds.  `edge_index` is set to `-1`.
    ///
    /// Mirrors `HatchGen_Domain(First, Last)`.
    pub fn new(us: f64, ue: f64) -> Self {
        let (u_start, u_end) = if us <= ue { (us, ue) } else { (ue, us) };
        Self {
            u_start,
            u_end,
            edge_index: -1,
        }
    }

    /// Return the length of the domain (`u_end - u_start`).
    ///
    /// Always non-negative because of the normalisation invariant.
    ///
    /// Mirrors the arithmetic on `HatchGen_Domain` endpoint parameters.
    pub fn length(&self) -> f64 {
        self.u_end - self.u_start
    }

    /// Return `true` when parameter `u` falls strictly inside the domain
    /// (exclusive of the endpoints).
    ///
    /// Mirrors the containment check implicit in `HatchGen_Domain`.
    pub fn contains(&self, u: f64) -> bool {
        u > self.u_start && u < self.u_end
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal: HatchElement
// ─────────────────────────────────────────────────────────────────────────────

/// An internal record representing one hatch line together with its trimmed
/// parameter domains.
///
/// This mirrors the `Hatch_Line` record inside `Hatch_Hatcher`.
#[derive(Clone, Debug)]
struct HatchElement {
    origin: [f64; 2],
    direction: [f64; 2],
    /// Accumulated trimmed domains on this line.
    line_domains: Vec<HatchDomain>,
}

impl HatchElement {
    fn new(origin: [f64; 2], direction: [f64; 2]) -> Self {
        Self {
            origin,
            direction,
            line_domains: Vec::new(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hatcher
// ─────────────────────────────────────────────────────────────────────────────

/// Creates and manages a hatch pattern by accumulating directed hatch lines
/// and trimming them with bounded parameter-space domains.
///
/// Typical usage:
/// 1. Construct with [`Hatcher::new`].
/// 2. Add lines with [`add_element`](Self::add_element).
/// 3. For each line, call [`trim`](Self::trim) to record which parameter
///    interval lies inside the hatched region.
/// 4. Iterate over [`domain`](Self::domain) / [`nb_domains`](Self::nb_domains)
///    to read back the trimmed pattern.
///
// occt: Hatch_Hatcher
#[derive(Clone, Debug)]
pub struct Hatcher {
    /// Flat list of all trimmed domains across all hatch lines.
    pub domains: Vec<HatchDomain>,
    /// Positional tolerance used when accepting trim intervals.
    pub tolerance: f64,
    /// Internal per-line records, kept for geometry lookups.
    elements: Vec<HatchElement>,
    /// Flag set to `true` once at least one `trim` call has been made.
    done: bool,
}

impl Hatcher {
    /// Create an empty hatcher with the given positional `tol`erance.
    ///
    /// The tolerance is used to discard zero-length (degenerate) domains when
    /// [`trim`](Self::trim) is called.
    ///
    /// Mirrors `Hatch_Hatcher(Tolerance)`.
    pub fn new(tol: f64) -> Self {
        Self {
            domains: Vec::new(),
            tolerance: tol,
            elements: Vec::new(),
            done: false,
        }
    }

    /// Append a new hatch line defined by a 2-D `origin` point and a
    /// `dir`ection vector.
    ///
    /// The direction need not be unit-length; callers are responsible for
    /// normalization if required.
    ///
    /// Mirrors `Hatch_Hatcher::AddLine(Origin, Direction)`.
    pub fn add_element(&mut self, origin: [f64; 2], dir: [f64; 2]) {
        self.elements.push(HatchElement::new(origin, dir));
    }

    /// Add a bounded domain `[start, end]` to the most-recently added hatch
    /// line and append it to the flat [`domains`](Self::domains) list.
    ///
    /// The domain is only accepted when its length exceeds
    /// [`tolerance`](Self::tolerance); degenerate intervals are silently
    /// discarded.
    ///
    /// If no line has been added yet, this call is a no-op.
    ///
    /// Mirrors `Hatch_Hatcher::Trim(First, Last)`.
    pub fn trim(&mut self, start: f64, end: f64) {
        let (lo, hi) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        if hi - lo <= self.tolerance {
            return;
        }
        let domain = HatchDomain::new(lo, hi);
        if let Some(elem) = self.elements.last_mut() {
            elem.line_domains.push(domain.clone());
            self.domains.push(domain);
            self.done = true;
        }
    }

    /// Return the total number of trimmed domains across all hatch lines.
    ///
    /// Mirrors `Hatch_Hatcher::NbDomains()`.
    pub fn nb_domains(&self) -> usize {
        self.domains.len()
    }

    /// Borrow the domain at 0-based index `i`, or `None` when out of range.
    ///
    /// Mirrors `Hatch_Hatcher::Domain(i)` (OCCT uses 1-based indexing; here
    /// 0-based indexing is used for idiomatic Rust).
    pub fn domain(&self, i: usize) -> Option<&HatchDomain> {
        self.domains.get(i)
    }

    /// Return `true` when at least one successful [`trim`](Self::trim) call
    /// has been made (i.e. the hatcher has produced at least one domain).
    ///
    /// Mirrors `Hatch_Hatcher::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// hatch_pattern
// ─────────────────────────────────────────────────────────────────────────────

/// Generate a regular hatch pattern as a list of line segments clipped to an
/// axis-aligned bounding box.
///
/// # Parameters
///
/// * `origin`  — a point that one hatch line passes through, given as `[x, y]`.
/// * `dir`     — direction vector of every hatch line (need not be unit-length).
/// * `spacing` — perpendicular distance between adjacent hatch lines.
/// * `angle`   — rotation angle **in radians** applied to `dir` before use.
///               Pass `0.0` to use `dir` unchanged.
/// * `bounds`  — axis-aligned bounding box as `(min_corner, max_corner)` where
///               each corner is `[x, y]`.
///
/// # Returns
///
/// A `Vec` of line segments; each segment is represented as `[[x0,y0],[x1,y1]]`.
/// Segments whose intersection with the bounding box is degenerate (length ≈ 0)
/// are omitted.
///
/// # Notes
///
/// The algorithm:
/// 1. Rotates `dir` by `angle`.
/// 2. Computes a perpendicular step vector of magnitude `spacing`.
/// 3. Projects all four corners of the bounding box onto the perpendicular axis
///    to determine the range of offsets that produce lines intersecting the box.
/// 4. For each offset in that range, parametrically clips the infinite hatch
///    line against the four sides of the bounding box using a Liang–Barsky-style
///    slab intersection.
pub fn hatch_pattern(
    origin: [f64; 2],
    dir: [f64; 2],
    spacing: f64,
    angle: f64,
    bounds: ([f64; 2], [f64; 2]),
) -> Vec<[[f64; 2]; 2]> {
    // ── 1. Rotate direction by `angle` ──────────────────────────────────────
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let dx = dir[0] * cos_a - dir[1] * sin_a;
    let dy = dir[0] * sin_a + dir[1] * cos_a;

    // Normalise direction.
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-14 {
        return Vec::new();
    }
    let ndx = dx / len;
    let ndy = dy / len;

    // Perpendicular unit vector (rotated 90° counter-clockwise).
    let px = -ndy;
    let py = ndx;

    // ── 2. Determine offset range from bounding-box corners ─────────────────
    let (lo, hi) = bounds;
    let corners = [
        [lo[0], lo[1]],
        [hi[0], lo[1]],
        [hi[0], hi[1]],
        [lo[0], hi[1]],
    ];

    // Project each corner onto the perpendicular axis relative to `origin`.
    let mut proj_min = f64::INFINITY;
    let mut proj_max = f64::NEG_INFINITY;
    for c in &corners {
        let rel = [c[0] - origin[0], c[1] - origin[1]];
        let proj = rel[0] * px + rel[1] * py;
        if proj < proj_min {
            proj_min = proj;
        }
        if proj > proj_max {
            proj_max = proj;
        }
    }

    if spacing <= 0.0 || proj_min >= proj_max {
        return Vec::new();
    }

    // First offset: smallest multiple of `spacing` that is >= proj_min.
    let first_k = (proj_min / spacing).ceil() as i64;
    let last_k = (proj_max / spacing).floor() as i64;

    let mut segments: Vec<[[f64; 2]; 2]> = Vec::new();

    // ── 3. Clip each hatch line against the bounding box ────────────────────
    for k in first_k..=last_k {
        let offset = k as f64 * spacing;

        // Point on this hatch line: origin + offset * perp.
        let lx = origin[0] + offset * px;
        let ly = origin[1] + offset * py;

        // Parametric clip: find t range where the ray
        //   P(t) = [lx + t*ndx, ly + t*ndy]  lies inside the box.
        //
        // Each axis contributes two slabs.
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;

        // X slab: lo[0] <= lx + t*ndx <= hi[0]
        if ndx.abs() > 1e-14 {
            let t1 = (lo[0] - lx) / ndx;
            let t2 = (hi[0] - lx) / ndx;
            let (ta, tb) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
            if ta > t_min {
                t_min = ta;
            }
            if tb < t_max {
                t_max = tb;
            }
        } else if lx < lo[0] || lx > hi[0] {
            // Line is parallel to Y axis and outside X bounds — skip.
            continue;
        }

        // Y slab: lo[1] <= ly + t*ndy <= hi[1]
        if ndy.abs() > 1e-14 {
            let t1 = (lo[1] - ly) / ndy;
            let t2 = (hi[1] - ly) / ndy;
            let (ta, tb) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
            if ta > t_min {
                t_min = ta;
            }
            if tb < t_max {
                t_max = tb;
            }
        } else if ly < lo[1] || ly > hi[1] {
            // Line is parallel to X axis and outside Y bounds — skip.
            continue;
        }

        if t_max - t_min < 1e-14 {
            continue; // Degenerate intersection.
        }

        let p0 = [lx + t_min * ndx, ly + t_min * ndy];
        let p1 = [lx + t_max * ndx, ly + t_max * ndy];
        segments.push([p0, p1]);
    }

    segments
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    fn near(a: f64, b: f64, label: &str) {
        assert!(
            (a - b).abs() <= TOL,
            "{label}: expected {a} ≈ {b}, diff = {}",
            (a - b).abs()
        );
    }

    // ── Test 1 ──────────────────────────────────────────────────────────────
    // HatchDomain: construction, length, contains.
    #[test]
    fn hatch_domain_basic() {
        let d = HatchDomain::new(1.0, 5.0);
        assert_eq!(d.u_start, 1.0);
        assert_eq!(d.u_end, 5.0);
        assert_eq!(d.edge_index, -1);
        near(d.length(), 4.0, "length");
        assert!(d.contains(3.0), "3.0 strictly inside [1,5]");
        assert!(!d.contains(1.0), "1.0 not strictly inside (endpoint)");
        assert!(!d.contains(5.0), "5.0 not strictly inside (endpoint)");
        assert!(!d.contains(0.0), "0.0 outside");
        assert!(!d.contains(6.0), "6.0 outside");
    }

    // ── Test 2 ──────────────────────────────────────────────────────────────
    // HatchDomain: inverted construction normalises order.
    #[test]
    fn hatch_domain_normalise() {
        let d = HatchDomain::new(9.0, 3.0);
        assert!(d.u_start <= d.u_end, "normalisation: u_start <= u_end");
        near(d.u_start, 3.0, "u_start after normalise");
        near(d.u_end, 9.0, "u_end after normalise");
        near(d.length(), 6.0, "length after normalise");
    }

    // ── Test 3 ──────────────────────────────────────────────────────────────
    // Hatcher: basic add/trim/query round-trip.
    #[test]
    fn hatcher_add_trim_query() {
        let mut h = Hatcher::new(1e-7);
        assert!(!h.is_done(), "not done before any trim");
        h.add_element([0.0, 0.0], [1.0, 0.0]);
        h.trim(1.0, 4.0);
        assert!(h.is_done(), "done after trim");
        assert_eq!(h.nb_domains(), 1);
        let d = h.domain(0).expect("domain 0 exists");
        near(d.u_start, 1.0, "u_start");
        near(d.u_end, 4.0, "u_end");
    }

    // ── Test 4 ──────────────────────────────────────────────────────────────
    // Hatcher: degenerate trim is discarded.
    #[test]
    fn hatcher_degenerate_trim_discarded() {
        let mut h = Hatcher::new(1e-7);
        h.add_element([0.0, 0.0], [1.0, 0.0]);
        h.trim(2.0, 2.0); // zero-length
        h.trim(2.0, 2.0 + 1e-9); // smaller than tolerance
        assert_eq!(h.nb_domains(), 0, "degenerate domains not added");
        assert!(!h.is_done(), "not done after only degenerate trims");
    }

    // ── Test 5 ──────────────────────────────────────────────────────────────
    // Hatcher: trim with reversed start/end normalises correctly.
    #[test]
    fn hatcher_trim_reversed() {
        let mut h = Hatcher::new(1e-7);
        h.add_element([0.0, 0.0], [1.0, 0.0]);
        h.trim(8.0, 2.0); // reversed
        assert_eq!(h.nb_domains(), 1);
        let d = h.domain(0).unwrap();
        near(d.u_start, 2.0, "u_start after reversed trim");
        near(d.u_end, 8.0, "u_end after reversed trim");
    }

    // ── Test 6 ──────────────────────────────────────────────────────────────
    // Hatcher: multiple lines and domains.
    #[test]
    fn hatcher_multiple_lines() {
        let mut h = Hatcher::new(1e-10);
        h.add_element([0.0, 0.0], [1.0, 0.0]);
        h.trim(0.0, 3.0);
        h.add_element([0.0, 1.0], [1.0, 0.0]);
        h.trim(1.0, 5.0);
        assert_eq!(h.nb_domains(), 2);
        near(h.domain(0).unwrap().length(), 3.0, "line0 domain length");
        near(h.domain(1).unwrap().length(), 4.0, "line1 domain length");
        assert!(h.domain(2).is_none(), "out-of-range returns None");
    }

    // ── Test 7 ──────────────────────────────────────────────────────────────
    // Hatcher: trim with no prior add_element is a no-op (no panic).
    #[test]
    fn hatcher_trim_without_element() {
        let mut h = Hatcher::new(1e-7);
        h.trim(0.0, 10.0); // no elements added yet
        assert_eq!(h.nb_domains(), 0);
    }

    // ── Test 8 ──────────────────────────────────────────────────────────────
    // hatch_pattern: horizontal lines through a unit square.
    #[test]
    fn hatch_pattern_horizontal() {
        // Direction along X, no rotation, spacing 0.5, unit square [0,1]x[0,1].
        let segs = hatch_pattern(
            [0.0, 0.0],
            [1.0, 0.0],
            0.5,
            0.0,
            ([0.0, 0.0], [1.0, 1.0]),
        );
        // Expected lines at y = 0.0, 0.5, 1.0 → 3 segments.
        assert_eq!(segs.len(), 3, "3 horizontal lines in [0,1]x[0,1] at spacing 0.5");
        for seg in &segs {
            // Each segment should span the full width [0, 1] in X.
            near(seg[0][0], 0.0, "seg x0");
            near(seg[1][0], 1.0, "seg x1");
        }
    }

    // ── Test 9 ──────────────────────────────────────────────────────────────
    // hatch_pattern: vertical lines through a unit square.
    #[test]
    fn hatch_pattern_vertical() {
        // Direction along Y, no rotation, spacing 0.25, unit square.
        let segs = hatch_pattern(
            [0.0, 0.0],
            [0.0, 1.0],
            0.25,
            0.0,
            ([0.0, 0.0], [1.0, 1.0]),
        );
        // Expected lines at x = 0.0, 0.25, 0.50, 0.75, 1.0 → 5 segments.
        assert_eq!(segs.len(), 5, "5 vertical lines at spacing 0.25");
    }

    // ── Test 10 ──────────────────────────────────────────────────────────────
    // hatch_pattern: zero-length direction returns empty vec (no panic).
    #[test]
    fn hatch_pattern_zero_dir() {
        let segs = hatch_pattern(
            [0.0, 0.0],
            [0.0, 0.0],
            1.0,
            0.0,
            ([0.0, 0.0], [10.0, 10.0]),
        );
        assert!(segs.is_empty(), "zero direction yields no segments");
    }

    // ── Test 11 ──────────────────────────────────────────────────────────────
    // hatch_pattern: non-zero angle rotates direction correctly.
    #[test]
    fn hatch_pattern_45_degrees() {
        use std::f64::consts::FRAC_PI_4;
        // Direction [1,0] rotated 45° → diagonal lines.
        // Bounding box [0,0]..[4,4], spacing 1.
        let segs = hatch_pattern(
            [0.0, 0.0],
            [1.0, 0.0],
            1.0,
            FRAC_PI_4,
            ([0.0, 0.0], [4.0, 4.0]),
        );
        // There should be several diagonal segments clipped to the box.
        assert!(!segs.is_empty(), "45-degree hatch has segments");
        // Every segment endpoint must lie on or inside the box.
        for seg in &segs {
            for pt in seg {
                assert!(pt[0] >= -1e-9 && pt[0] <= 4.0 + 1e-9, "x in bounds");
                assert!(pt[1] >= -1e-9 && pt[1] <= 4.0 + 1e-9, "y in bounds");
            }
        }
    }
}
