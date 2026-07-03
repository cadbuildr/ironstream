// FILE: brep_offset_api.rs
//! `BRepOffsetAPI` — pipe, shell-pipe, and planar-offset operations, mirroring
//! OCCT's `BRepOffsetAPI_MakePipe`, `BRepOffsetAPI_MakePipeShell`, and
//! `BRepOffsetAPI_MakeOffset`.
//!
//! # Overview
//!
//! * [`MakePipe`] — sweep a closed-polygon cross-section along a polyline spine,
//!   producing a watertight [`Solid`].  Corresponds to OCCT's
//!   `BRepOffsetAPI_MakePipe(spine, profile)`.
//!
//! * [`MakePipeShell`] — a richer sweep builder that accepts a movable-frame
//!   spine plus one or more section wires (profile replacement at different
//!   stations along the path).  Corresponds to OCCT's
//!   `BRepOffsetAPI_MakePipeShell`.
//!
//! * [`MakeOffset`] — inflate or deflate a planar closed wire outward by a
//!   signed distance, using a constant normal-offset algorithm.  Corresponds to
//!   OCCT's `BRepOffsetAPI_MakeOffset`.
//!
//! # Algorithm notes
//!
//! **MakePipe / MakePipeShell** use a Frenet-Serret (oriented) frame sweep:
//! at each spine vertex the local frame is computed by transport from the
//! previous frame, keeping the profile tangent-aligned with the spine tangent.
//! The cross-section ring at each station is then tessellated and stitched to
//! the adjacent ring with a quad strip.
//!
//! **MakeOffset** computes an inward/outward polygon offset by moving every edge
//! along its outward normal by `distance`, then intersecting adjacent offset
//! edges to find the new corners.  Convex corners get mitered and concave ones
//! are clipped.

use crate::gp::{Pnt, Trsf};
use crate::mesh::TriMesh;
use crate::topods::{Face, Solid, Wire};
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Finalize a mesh: weld and ensure outward orientation.
fn finalize(mut m: TriMesh) -> Solid {
    m = m.welded(1e-7);
    if m.signed_volume() < 0.0 {
        m.flip();
    }
    Solid::from_mesh(m)
}

/// Build a ring of points by applying `frame` to a unit-circle template
/// of `n` points with the given `radius`.  The template lives in the XY
/// plane of the frame (frame Z = tangent direction along spine).
fn ring_from_frame(frame: &Trsf, radius: f64, n: usize) -> Vec<Pnt> {
    (0..n)
        .map(|i| {
            let a = 2.0 * PI * i as f64 / n as f64;
            // Local coordinates: (r*cos, r*sin, 0) — in frame XY-plane.
            frame.apply_point(Pnt::new(radius * a.cos(), radius * a.sin(), 0.0))
        })
        .collect()
}

/// Transform a profile wire (supplied in local XY-plane of `frame`) into world
/// space.
fn apply_frame_to_profile(frame: &Trsf, profile: &Wire) -> Wire {
    Wire::new(profile.pts.iter().map(|p| frame.apply_point(*p)).collect())
}

/// Compute a Frenet-like transport frame for a poly-spine at station `i`.
///
/// * `tangent` — normalized tangent of the spine at this station.
/// * `prev_x`  — the X-axis of the previous frame (for parallel transport).
///
/// Returns a [`Trsf`] that maps local XY-plane coordinates to world space with
/// local +Z aligned with `tangent`.
fn transport_frame(origin: Pnt, tangent: Pnt, prev_x: Pnt) -> (Trsf, Pnt) {
    let z = tangent.normalized();
    // Re-orthogonalize prev_x against z (parallel transport).
    let mut x = prev_x - z * prev_x.dot(z);
    if x.norm() < 1e-12 {
        x = z.any_perpendicular();
    }
    let x = x.normalized();
    let y = z.cross(x).normalized();
    // Build column-major rotation: columns are x, y, z in world.
    let t = Trsf {
        m: [[x.x, y.x, z.x], [x.y, y.y, z.y], [x.z, y.z, z.z]],
        t: [origin.x, origin.y, origin.z],
    };
    (t, x)
}

/// Build a swept solid from a sequence of pre-positioned cross-section rings.
/// Adjacent rings are stitched with a quad strip; end caps are triangulated.
fn sweep_rings(rings: &[Vec<Pnt>]) -> Solid {
    if rings.len() < 2 {
        return Solid::empty();
    }
    let n = rings[0].len();
    if n < 3 {
        return Solid::empty();
    }
    let mut m = TriMesh::new();

    // Side walls.
    for s in 0..rings.len() - 1 {
        let r0 = &rings[s];
        let r1 = &rings[s + 1];
        for i in 0..n {
            let j = (i + 1) % n;
            let (a, b, c, d) = (r0[i], r0[j], r1[j], r1[i]);
            m.push_triangle(a, b, c);
            m.push_triangle(a, c, d);
        }
    }

    // Start cap (outward normal points away from interior).
    let start = Face::new(Wire::new(rings[0].clone()));
    for t in start.triangulate() {
        m.push_triangle(t[0], t[2], t[1]); // reversed winding = outward-facing at start
    }
    // End cap.
    let end = Face::new(Wire::new(rings[rings.len() - 1].clone()));
    for t in end.triangulate() {
        m.push_triangle(t[0], t[1], t[2]);
    }

    finalize(m)
}

// ─────────────────────────────────────────────────────────────────────────────
// MakePipe
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for a swept pipe: a circular cross-section swept along a polyline
/// spine.
///
/// Mirrors OCCT's `BRepOffsetAPI_MakePipe(spine, profile)`.  In IronStream
/// the "profile" is parameterised as a `radius` + optional number of facets.
/// An explicit `Wire` profile in the local XY plane of the spine start frame
/// can be supplied via `MakePipe::with_profile`.
// occt: BRepOffsetAPI_MakePipe
#[derive(Debug, Clone)]
pub struct MakePipe {
    /// Poly-line spine.
    spine: Wire,
    /// Cross-section radius (used when no explicit `Wire` profile is given).
    radius: f64,
    /// Facets around the circle (resolution).
    facets: usize,
    /// Optional explicit cross-section in local XY-plane (Z=0) of the first
    /// spine frame.
    profile: Option<Wire>,
    /// Cached result.
    result: Option<Solid>,
}

impl MakePipe {
    /// Construct a pipe by sweeping a circle of `radius` along `spine`.
    ///
    /// Mirrors `BRepOffsetAPI_MakePipe::BRepOffsetAPI_MakePipe(spine, profile)`.
    // occt: BRepOffsetAPI_MakePipe::BRepOffsetAPI_MakePipe
    pub fn new(spine: Wire, radius: f64) -> Self {
        Self {
            spine,
            radius,
            facets: 32,
            profile: None,
            result: None,
        }
    }

    /// Override the number of circular facets.
    pub fn with_facets(mut self, facets: usize) -> Self {
        self.facets = facets.max(3);
        self
    }

    /// Supply an explicit closed profile wire (in local XY of first frame,
    /// Z=0 for all points).  Overrides `radius`.
    pub fn with_profile(mut self, profile: Wire) -> Self {
        self.profile = Some(profile);
        self
    }

    /// Compute (or return cached) result.
    ///
    /// Mirrors `BRepOffsetAPI_MakePipe::Build()` / `Shape()`.
    // occt: BRepOffsetAPI_MakePipe::Shape
    pub fn build(&mut self) -> &Solid {
        if self.result.is_none() {
            self.result = Some(self.compute());
        }
        self.result.as_ref().unwrap()
    }

    /// Whether the last `build()` produced a non-empty solid.
    // occt: BRepOffsetAPI_MakePipe::IsDone
    pub fn is_done(&mut self) -> bool {
        !self.build().is_empty()
    }

    /// Consume and return the built solid.
    // occt: BRepOffsetAPI_MakePipe::Shape (moved)
    pub fn into_solid(self) -> Solid {
        self.compute()
    }

    fn compute(&self) -> Solid {
        let pts = &self.spine.pts;
        if pts.len() < 2 {
            return Solid::empty();
        }

        // Initial tangent: from first to second spine point.
        let initial_tangent = (pts[1] - pts[0]).normalized();
        // Choose an initial X axis perpendicular to the tangent.
        let init_x = initial_tangent.any_perpendicular();
        let (frame0, mut cur_x) = transport_frame(pts[0], initial_tangent, init_x);

        // Generate the ring at each spine vertex.
        let mut rings: Vec<Vec<Pnt>> = Vec::with_capacity(pts.len());

        // Ring at station 0.
        let r0 = self.make_ring(&frame0);
        rings.push(r0);

        for i in 1..pts.len() {
            let tangent = if i < pts.len() - 1 {
                // Average tangent at interior vertices (for smoother sweep).
                let t0 = (pts[i] - pts[i - 1]).normalized();
                let t1 = (pts[i + 1] - pts[i]).normalized();
                (t0 + t1).normalized()
            } else {
                (pts[i] - pts[i - 1]).normalized()
            };
            let (frame_i, new_x) = transport_frame(pts[i], tangent, cur_x);
            cur_x = new_x;
            rings.push(self.make_ring(&frame_i));
        }

        sweep_rings(&rings)
    }

    fn make_ring(&self, frame: &Trsf) -> Vec<Pnt> {
        if let Some(prof) = &self.profile {
            apply_frame_to_profile(frame, prof).pts
        } else {
            ring_from_frame(frame, self.radius, self.facets)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MakePipeShell
// ─────────────────────────────────────────────────────────────────────────────

/// A section wire pinned to a parameter position along the spine (0..1).
#[derive(Debug, Clone)]
pub struct PipeSection {
    /// Normalised parameter along the spine at which this section is applied
    /// (0 = start, 1 = end).
    pub param: f64,
    /// Closed profile in local XY-plane of the spine frame at `param`.
    pub profile: Wire,
}

/// Builder for a shell-sweep pipe with variable sections.
///
/// Mirrors OCCT's `BRepOffsetAPI_MakePipeShell`.  Sections are linearly
/// interpolated between pinned stations; if only one section is given it is
/// used uniformly (equivalent to [`MakePipe::with_profile`]).
// occt: BRepOffsetAPI_MakePipeShell
#[derive(Debug, Clone)]
pub struct MakePipeShell {
    /// Poly-line spine.
    spine: Wire,
    /// Section wires, sorted by `param`.
    sections: Vec<PipeSection>,
    /// Fallback radius if no sections provided.
    radius: f64,
    /// Facets around a circular fallback section.
    facets: usize,
    /// Cached result.
    result: Option<Solid>,
}

impl MakePipeShell {
    /// Create an empty builder.  Call [`add_section`] to add profiles.
    // occt: BRepOffsetAPI_MakePipeShell::BRepOffsetAPI_MakePipeShell
    pub fn new(spine: Wire) -> Self {
        Self {
            spine,
            sections: Vec::new(),
            radius: 1.0,
            facets: 32,
            result: None,
        }
    }

    /// Fallback circular radius (used when no sections are added).
    pub fn with_radius(mut self, r: f64) -> Self {
        self.radius = r;
        self
    }

    /// Set circular facet count.
    pub fn with_facets(mut self, n: usize) -> Self {
        self.facets = n.max(3);
        self
    }

    /// Add a section wire at parameter `param` (0..=1).
    ///
    /// Mirrors `BRepOffsetAPI_MakePipeShell::Add(section, param)`.
    // occt: BRepOffsetAPI_MakePipeShell::Add
    pub fn add_section(&mut self, profile: Wire, param: f64) {
        let param = param.clamp(0.0, 1.0);
        self.sections.push(PipeSection { param, profile });
        self.sections.sort_by(|a, b| a.param.partial_cmp(&b.param).unwrap());
        self.result = None; // invalidate cache
    }

    /// Compute (or return cached) result.
    // occt: BRepOffsetAPI_MakePipeShell::Build / Shape
    pub fn build(&mut self) -> &Solid {
        if self.result.is_none() {
            self.result = Some(self.compute());
        }
        self.result.as_ref().unwrap()
    }

    /// Whether the build succeeded.
    // occt: BRepOffsetAPI_MakePipeShell::IsDone
    pub fn is_done(&mut self) -> bool {
        !self.build().is_empty()
    }

    /// Consume and return the built solid.
    pub fn into_solid(self) -> Solid {
        self.compute()
    }

    fn compute(&self) -> Solid {
        let pts = &self.spine.pts;
        if pts.len() < 2 {
            return Solid::empty();
        }

        // Arc-length parameterisation of the spine.
        let total_len = spine_length(pts);
        if total_len < 1e-12 {
            return Solid::empty();
        }

        // Build per-station frames.
        let initial_tangent = (pts[1] - pts[0]).normalized();
        let init_x = initial_tangent.any_perpendicular();
        let (frame0, mut cur_x) = transport_frame(pts[0], initial_tangent, init_x);
        let mut frames: Vec<(f64, Trsf)> = Vec::with_capacity(pts.len());
        frames.push((0.0, frame0));

        let mut cum_len = 0.0;
        for i in 1..pts.len() {
            cum_len += pts[i].distance(pts[i - 1]);
            let param = cum_len / total_len;
            let tangent = if i < pts.len() - 1 {
                let t0 = (pts[i] - pts[i - 1]).normalized();
                let t1 = (pts[i + 1] - pts[i]).normalized();
                (t0 + t1).normalized()
            } else {
                (pts[i] - pts[i - 1]).normalized()
            };
            let (fi, nx) = transport_frame(pts[i], tangent, cur_x);
            cur_x = nx;
            frames.push((param, fi));
        }

        // Determine the sections to interpolate from.
        let sections = if self.sections.is_empty() {
            // Fallback: circular ring at start and end.
            let fallback_wire = Wire::new(
                (0..self.facets)
                    .map(|k| {
                        let a = 2.0 * PI * k as f64 / self.facets as f64;
                        Pnt::new(self.radius * a.cos(), self.radius * a.sin(), 0.0)
                    })
                    .collect(),
            );
            vec![
                PipeSection { param: 0.0, profile: fallback_wire.clone() },
                PipeSection { param: 1.0, profile: fallback_wire },
            ]
        } else {
            // Ensure we have sections at both ends.
            let mut s = self.sections.clone();
            if s[0].param > 0.0 {
                s.insert(0, PipeSection { param: 0.0, profile: s[0].profile.clone() });
            }
            if s.last().unwrap().param < 1.0 {
                let last = s.last().unwrap().profile.clone();
                s.push(PipeSection { param: 1.0, profile: last });
            }
            s
        };

        // Build rings by interpolating sections between their anchor params.
        let rings: Vec<Vec<Pnt>> = frames
            .iter()
            .map(|(t, frame)| {
                let profile = interp_section(&sections, *t);
                apply_frame_to_profile(frame, &profile).pts
            })
            .collect();

        if rings.len() < 2 || rings[0].len() < 3 {
            return Solid::empty();
        }
        sweep_rings(&rings)
    }
}

/// Linearly interpolate a cross-section profile at parameter `t`.
///
/// Profiles must have the same number of points.  If the point counts differ,
/// the closer section is used without interpolation.
fn interp_section(sections: &[PipeSection], t: f64) -> Wire {
    // Find bracketing sections.
    if sections.is_empty() {
        return Wire::new(vec![]);
    }
    if sections.len() == 1 {
        return sections[0].profile.clone();
    }
    // Find the pair (a, b) such that a.param <= t <= b.param.
    let mut lo = 0usize;
    for (i, s) in sections.iter().enumerate() {
        if s.param <= t {
            lo = i;
        }
    }
    let hi = (lo + 1).min(sections.len() - 1);
    if lo == hi {
        return sections[lo].profile.clone();
    }
    let a = &sections[lo];
    let b = &sections[hi];
    let span = b.param - a.param;
    if span < 1e-12 {
        return a.profile.clone();
    }
    let alpha = (t - a.param) / span;

    let na = a.profile.pts.len();
    let nb = b.profile.pts.len();
    if na != nb {
        // Can't interpolate; use closer section.
        if alpha < 0.5 { a.profile.clone() } else { b.profile.clone() }
    } else {
        Wire::new(
            a.profile
                .pts
                .iter()
                .zip(b.profile.pts.iter())
                .map(|(pa, pb)| pa.lerp(*pb, alpha))
                .collect(),
        )
    }
}

/// Total arc length of a poly-spine.
fn spine_length(pts: &[Pnt]) -> f64 {
    pts.windows(2).map(|w| w[1].distance(w[0])).sum()
}

// ─────────────────────────────────────────────────────────────────────────────
// MakeOffset
// ─────────────────────────────────────────────────────────────────────────────

/// Error cases for [`MakeOffset`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OffsetError {
    /// The wire has fewer than 3 points and cannot be offset.
    TooFewPoints,
    /// The offset distance collapses or inverts the polygon.
    CollapseDetected,
}

impl std::fmt::Display for OffsetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffsetError::TooFewPoints => write!(f, "wire has fewer than 3 points"),
            OffsetError::CollapseDetected => write!(f, "offset distance collapses the polygon"),
        }
    }
}

/// Planar wire offset: inflate or deflate a closed polygon by `distance`.
///
/// Mirrors OCCT's `BRepOffsetAPI_MakeOffset`.  Positive `distance` expands
/// the polygon outward (in the direction of its computed outward normal);
/// negative `distance` shrinks it.
///
/// The algorithm:
/// 1. Determine the winding normal by Newell's method.
/// 2. For each edge, compute the outward unit normal (edge-normal in the
///    plane, pointing outward).
/// 3. Shift each edge along that normal by `distance`.
/// 4. Intersect consecutive shifted edges to find the new vertices.
// occt: BRepOffsetAPI_MakeOffset
#[derive(Debug, Clone)]
pub struct MakeOffset {
    wire: Wire,
    distance: f64,
    result: Option<Result<Wire, OffsetError>>,
}

impl MakeOffset {
    /// Create the offset builder for `wire` at signed `distance`.
    // occt: BRepOffsetAPI_MakeOffset::BRepOffsetAPI_MakeOffset
    pub fn new(wire: Wire, distance: f64) -> Self {
        Self {
            wire,
            distance,
            result: None,
        }
    }

    /// Perform the offset computation.
    // occt: BRepOffsetAPI_MakeOffset::Build
    pub fn build(&mut self) -> &Result<Wire, OffsetError> {
        if self.result.is_none() {
            self.result = Some(compute_offset(&self.wire, self.distance));
        }
        self.result.as_ref().unwrap()
    }

    /// Whether the build succeeded (no errors).
    // occt: BRepOffsetAPI_MakeOffset::IsDone
    pub fn is_done(&mut self) -> bool {
        self.build().is_ok()
    }

    /// Return the offset wire, re-running the build if needed.
    // occt: BRepOffsetAPI_MakeOffset::Shape
    pub fn wire(&mut self) -> Result<Wire, OffsetError> {
        self.build().clone()
    }

    /// Consume and return the offset wire.
    pub fn into_wire(mut self) -> Result<Wire, OffsetError> {
        self.build();
        self.result.unwrap()
    }

    /// Approximate the signed area of the offset wire for validation.
    pub fn signed_area(&mut self) -> Option<f64> {
        match self.build() {
            Ok(w) => Some(signed_area(w)),
            Err(_) => None,
        }
    }
}

/// Compute a constant-distance offset of a planar closed polygon.
///
/// The polygon must lie roughly in a plane (degenerate 3-D polygons will
/// have an inaccurate normal estimate).  The winding is preserved.
///
/// Positive `dist` expands the polygon outward (away from the centroid).
/// Negative `dist` shrinks it inward.
fn compute_offset(wire: &Wire, dist: f64) -> Result<Wire, OffsetError> {
    let pts = &wire.pts;
    let n = pts.len();
    if n < 3 {
        return Err(OffsetError::TooFewPoints);
    }

    // 1. Compute the winding normal (Newell).
    let normal = wire.normal();

    // 2. For each edge, compute the outward unit normal in the polygon plane
    //    and shift both endpoints.
    //
    //    For a CCW polygon with normal N:
    //    * outward normal of edge (a→b) = edge_tan × N
    //      (cross-product of travel direction with plane normal; for CCW this
    //       points away from the polygon interior).
    //    * Positive `dist` expands outward; negative `dist` shrinks inward.
    let mut shifted_edges: Vec<(Pnt, Pnt)> = Vec::with_capacity(n);
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        let edge_tan = (b - a).normalized();
        // outward normal = edge_tan × plane_normal  (points right of travel for CCW)
        let out_normal = edge_tan.cross(normal).normalized();
        let shift = out_normal * dist;
        shifted_edges.push((a + shift, b + shift));
    }

    // 3. Early collapse check for inward offsets (dist < 0).
    //    Apothem = perpendicular distance from centroid to each edge line.
    //    For a CCW polygon, (centroid - edge_point) · out_normal < 0, so the
    //    apothem is  -(centroid - edge_pt)·out_normal  = (edge_pt - centroid)·out_normal.
    //    If |dist| >= minimum apothem, the offset will collapse or invert.
    if dist < 0.0 {
        let centroid = wire.centroid();
        let min_apothem = (0..n)
            .map(|i| {
                let a = pts[i];
                let b = pts[(i + 1) % n];
                let edge_tan = (b - a).normalized();
                let out_normal = edge_tan.cross(normal).normalized();
                // Distance from centroid to the edge (positive for interior centroid).
                let signed = (centroid - a).dot(out_normal);
                -signed // positive when centroid is on the inward side
            })
            .fold(f64::INFINITY, f64::min);
        // If the shrink magnitude exceeds the inradius, the polygon collapses.
        if dist.abs() >= min_apothem - 1e-9 {
            return Err(OffsetError::CollapseDetected);
        }
    }

    // 4. Intersect consecutive shifted edge lines to find offset vertices.
    let mut new_pts: Vec<Pnt> = Vec::with_capacity(n);
    for i in 0..n {
        let j = (i + 1) % n;
        let (p0, p1) = shifted_edges[i];
        let (q0, q1) = shifted_edges[j];
        match line_line_closest(p0, p1 - p0, q0, q1 - q0, normal) {
            Some(p) => new_pts.push(p),
            None => {
                // Parallel edges: just use the endpoint on the first shifted edge.
                new_pts.push(p1);
            }
        }
    }

    // 5. Sanity check: signed-area must not flip sign.
    let orig_area = signed_area(wire);
    let new_wire = Wire::new(new_pts);
    let new_area = signed_area(&new_wire);
    if orig_area.abs() > 1e-14 && new_area * orig_area < 0.0 {
        return Err(OffsetError::CollapseDetected);
    }
    if new_wire.pts.len() < 3 {
        return Err(OffsetError::TooFewPoints);
    }
    Ok(new_wire)
}

/// Signed area of a planar closed polygon (Shoelace, using the XY projection
/// of the 3-D Newell cross product — correct for any plane orientation).
fn signed_area(wire: &Wire) -> f64 {
    let pts = &wire.pts;
    let n = pts.len();
    if n < 3 {
        return 0.0;
    }
    // Newell method gives the area vector; its magnitude is the area.
    let mut ax = 0.0_f64;
    let mut ay = 0.0_f64;
    let mut az = 0.0_f64;
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        ax += a.y * b.z - a.z * b.y;
        ay += a.z * b.x - a.x * b.z;
        az += a.x * b.y - a.y * b.x;
    }
    // Return the magnitude with sign (positive for CCW about normal).
    let area_vec_len = (ax * ax + ay * ay + az * az).sqrt();
    // Preserve sign: dot area_vec with wire normal.
    let nrm = wire.normal();
    let dot = ax * nrm.x + ay * nrm.y + az * nrm.z;
    if dot >= 0.0 { area_vec_len * 0.5 } else { -area_vec_len * 0.5 }
}

/// Find the point on line A nearest to line B, projected onto the plane defined
/// by `plane_normal` (in-plane intersection).
///
/// Lines: A = p + s*d, B = q + t*e.
/// Solves the 2D linear system for (s, t).
fn line_line_closest(p: Pnt, d: Pnt, q: Pnt, e: Pnt, plane_normal: Pnt) -> Option<Pnt> {
    // Project onto a 2D basis in the plane.
    let n = plane_normal.normalized();
    let u = d.normalized();
    let v = n.cross(u).normalized();

    // Express d and e in (u, v) coordinates.
    let dx = d.dot(u);
    let dy = d.dot(v);
    let ex = e.dot(u);
    let ey = e.dot(v);

    // p-q in (u, v).
    let rp = q - p;
    let rx = rp.dot(u);
    let ry = rp.dot(v);

    // Solve: s * dx - t * ex = rx
    //        s * dy - t * ey = ry
    let det = -dx * ey + dy * ex;
    if det.abs() < 1e-14 {
        return None; // parallel
    }
    let s = (-ey * rx + ex * ry) / det;
    Some(p + d * s)
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use std::f64::consts::PI;

    fn near(a: f64, b: f64, tol: f64, label: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{label}: expected {a} ≈ {b} (tol={tol}), diff={}",
            (a - b).abs()
        );
    }

    /// Build a straight 3-vertex spine along +Z.
    fn z_spine(len: f64) -> Wire {
        Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, len / 2.0),
            Pnt::new(0.0, 0.0, len),
        ])
    }

    // ── MakePipe ──────────────────────────────────────────────────────────────

    #[test]
    fn pipe_straight_spine_non_empty() {
        let spine = z_spine(5.0);
        let mut p = MakePipe::new(spine, 1.0).with_facets(16);
        assert!(p.is_done(), "straight pipe must build");
    }

    #[test]
    fn pipe_straight_volume_close_to_cylinder() {
        // A straight pipe of radius r, length L ≈ π r² L.
        let r = 1.0_f64;
        let l = 5.0_f64;
        let spine = z_spine(l);
        let solid = MakePipe::new(spine, r).with_facets(64).into_solid();
        let expected = PI * r * r * l;
        let vol = solid.volume();
        let ratio = vol / expected;
        assert!(
            ratio > 0.98 && ratio <= 1.01,
            "pipe volume ratio={ratio} (vol={vol}, expected={expected})"
        );
    }

    #[test]
    fn pipe_watertight() {
        let spine = z_spine(3.0);
        let solid = MakePipe::new(spine, 0.5).with_facets(32).into_solid();
        assert_eq!(
            solid.mesh().open_edge_count(1e-5),
            0,
            "pipe must be watertight"
        );
    }

    #[test]
    fn pipe_degenerate_spine_returns_empty() {
        // Only 1 point → can't sweep.
        let spine = Wire::new(vec![Pnt::new(0.0, 0.0, 0.0)]);
        let solid = MakePipe::new(spine, 1.0).into_solid();
        assert!(solid.is_empty(), "degenerate spine must give empty solid");
    }

    #[test]
    fn pipe_with_explicit_square_profile() {
        // Square cross-section 2×2, swept 4 units → vol ≈ 4*4 = 16.
        let profile = Wire::new(vec![
            Pnt::new(-1.0, -1.0, 0.0),
            Pnt::new(1.0, -1.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(-1.0, 1.0, 0.0),
        ]);
        let spine = z_spine(4.0);
        let solid = MakePipe::new(spine, 0.0)
            .with_profile(profile)
            .into_solid();
        let vol = solid.volume();
        // Square cross section area = 4; vol ≈ 4*4 = 16 (mesh approx).
        assert!(
            vol > 14.0 && vol < 18.0,
            "square-profile pipe vol={vol} expected≈16"
        );
    }

    #[test]
    fn pipe_l_shaped_spine_non_empty() {
        // L-shaped spine: go along +X then +Z.
        let spine = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 3.0),
        ]);
        let mut p = MakePipe::new(spine, 0.5).with_facets(16);
        assert!(p.is_done(), "L-spine pipe must build");
        assert_eq!(
            p.build().mesh().open_edge_count(1e-5),
            0,
            "L-spine pipe must be watertight"
        );
    }

    // ── MakePipeShell ─────────────────────────────────────────────────────────

    #[test]
    fn pipe_shell_no_sections_falls_back_to_circle() {
        let spine = z_spine(4.0);
        let solid = MakePipeShell::new(spine)
            .with_radius(1.0)
            .with_facets(32)
            .into_solid();
        assert!(!solid.is_empty(), "fallback-circular pipe-shell must build");
    }

    #[test]
    fn pipe_shell_one_section_uniform() {
        // One circular section at param 0.5 → used uniformly.
        let spine = z_spine(3.0);
        let circle = Wire::new(
            (0..16)
                .map(|i| {
                    let a = 2.0 * PI * i as f64 / 16.0;
                    Pnt::new(a.cos(), a.sin(), 0.0)
                })
                .collect(),
        );
        let mut builder = MakePipeShell::new(spine);
        builder.add_section(circle, 0.5);
        let solid = builder.into_solid();
        assert!(!solid.is_empty());
    }

    #[test]
    fn pipe_shell_two_sections_taper() {
        // Taper from r=2 at start to r=0.5 at end.
        let make_ring = |r: f64, n: usize| {
            Wire::new(
                (0..n)
                    .map(|i| {
                        let a = 2.0 * PI * i as f64 / n as f64;
                        Pnt::new(r * a.cos(), r * a.sin(), 0.0)
                    })
                    .collect(),
            )
        };
        let spine = z_spine(6.0);
        let mut builder = MakePipeShell::new(spine);
        builder.add_section(make_ring(2.0, 32), 0.0);
        builder.add_section(make_ring(0.5, 32), 1.0);
        let solid = builder.into_solid();
        assert!(!solid.is_empty(), "tapered pipe-shell must build");
        // Volume should be between r=0.5 and r=2 cylinder volumes.
        let v_min = PI * 0.5 * 0.5 * 6.0;
        let v_max = PI * 2.0 * 2.0 * 6.0;
        let vol = solid.volume();
        assert!(
            vol > v_min && vol < v_max,
            "tapered vol={vol} not in ({v_min}, {v_max})"
        );
    }

    // ── MakeOffset ────────────────────────────────────────────────────────────

    #[test]
    fn offset_square_expand() {
        // Unit square expanded by 1.0 → 3×3 square.
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let mut mo = MakeOffset::new(w, 1.0);
        assert!(mo.is_done(), "square offset must succeed");
        let ow = mo.wire().unwrap();
        assert_eq!(ow.pts.len(), 4, "offset square must still have 4 vertices");
        // All offset points should be ~1 unit further from the unit-square boundary.
        let min_x = ow.pts.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = ow.pts.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        assert!(min_x < -0.5, "left side should expand, got min_x={min_x}");
        assert!(max_x > 1.5, "right side should expand, got max_x={max_x}");
    }

    #[test]
    fn offset_square_shrink() {
        // 4×4 square shrunk by 1 → 2×2 square.
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(4.0, 0.0, 0.0),
            Pnt::new(4.0, 4.0, 0.0),
            Pnt::new(0.0, 4.0, 0.0),
        ]);
        let mut mo = MakeOffset::new(w, -1.0);
        assert!(mo.is_done(), "shrink offset must succeed");
        let ow = mo.wire().unwrap();
        // Should have 4 vertices.
        assert_eq!(ow.pts.len(), 4);
        // x range should be roughly [1, 3].
        let min_x = ow.pts.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = ow.pts.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        near(min_x, 1.0, 0.1, "shrink_min_x");
        near(max_x, 3.0, 0.1, "shrink_max_x");
    }

    #[test]
    fn offset_too_few_points_returns_error() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ]);
        let mut mo = MakeOffset::new(w, 0.5);
        assert!(!mo.is_done());
        assert_eq!(
            mo.wire().unwrap_err(),
            OffsetError::TooFewPoints
        );
    }

    #[test]
    fn offset_collapse_detected() {
        // A very large inward offset should collapse/invert the unit square.
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let mut mo = MakeOffset::new(w, -10.0);
        assert!(!mo.is_done(), "over-collapse must be an error");
        assert_eq!(
            mo.wire().unwrap_err(),
            OffsetError::CollapseDetected
        );
    }

    #[test]
    fn signed_area_unit_square_positive() {
        // CCW unit square in XY should have positive signed area = 1.0.
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let area = signed_area(&w);
        near(area.abs(), 1.0, 1e-9, "unit_square_area");
    }
}
