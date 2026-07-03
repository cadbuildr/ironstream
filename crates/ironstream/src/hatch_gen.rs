// FILE: hatch_gen.rs
//! `HatchGen` — hatching domain types, mirroring OpenCascade's `HatchGen` package.
//!
//! # Scope
//!
//! This module provides the pure data types used by the hatching algorithm:
//!
//! * [`HatchGen_IntersectionType`] — classifies how a hatch line crosses a
//!   domain boundary (touching vs. crossing, interior vs. exterior).
//! * [`HatchGen_IntersectionPoint`] — one intersection between a hatch line and
//!   a 2D geometry element, carrying the line parameter and intersection type.
//! * [`HatchGen_PointOnHatching`] — an intersection point viewed from the
//!   hatching line side, keeping track of which geometry element index was hit.
//! * [`HatchGen_PointOnElement`] — an intersection point viewed from the
//!   element side, keeping track of which hatch line index produced the hit.
//! * [`HatchGen_Domain`] — a connected interval on a hatch line between two
//!   boundary intersections (or an open/semi-open segment).
//!
//! # Notes
//!
//! All coordinates are 1-D line parameters (`f64`).  The "hatching" package in
//! OCCT operates in 2D parameter space, but the domain types themselves only
//! carry scalar parameter values; the 2D geometry lives in `HatchGen_Hatcher`.

/// Classifies the topological nature of a hatch/element intersection.
///
/// In OCCT's `HatchGen_IntersectionType` enumeration these four variants
/// correspond to whether the hatch line merely touches the boundary (TANGENT)
/// or actually crosses it (CROSSING), and whether the intersection falls inside
/// (`IN`) or outside (`OUT`) the hatched region.
// occt: HatchGen_IntersectionType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HatchGen_IntersectionType {
    /// The hatch line crosses from outside to inside the hatched domain
    /// at this point.
    IN,
    /// The hatch line crosses from inside to outside the hatched domain
    /// at this point.
    OUT,
    /// The hatch line is tangent to the boundary from inside (touches and
    /// stays inside).
    TOUCH_IN,
    /// The hatch line is tangent to the boundary from outside (touches and
    /// stays outside).
    TOUCH_OUT,
    /// The hatch line is collinear / coincident with the boundary element
    /// over a finite interval; this point marks the start or end of that
    /// coincidence.
    IDENTICAL,
}

// ─────────────────────────────────────────────────────────────────────────────
// HatchGen_IntersectionPoint
// ─────────────────────────────────────────────────────────────────────────────

/// One intersection between a hatch line and a 2-D boundary element.
///
/// Mirrors `HatchGen_IntersectionPoint` from OCCT's `HatchGen` package.
// occt: HatchGen_IntersectionPoint
#[derive(Clone, Debug)]
pub struct HatchGen_IntersectionPoint {
    /// Parameter value on the *hatch line* at the intersection.
    index: i32,
    /// Parameter on the hatch line (distance along the line from its origin).
    parameter: f64,
    /// How the line crosses the boundary at this point.
    intersection_type: HatchGen_IntersectionType,
    /// Indicates that this intersection was computed as a segment start rather
    /// than a discrete point.  Corresponds to OCCT's `SegmentBeginning` flag.
    is_segment_beginning: bool,
    /// Indicates that this intersection was computed as a segment end.
    /// Corresponds to OCCT's `SegmentEnd` flag.
    is_segment_end: bool,
}

impl HatchGen_IntersectionPoint {
    /// Create a new intersection point.
    ///
    /// `index`   — 1-based index of the intersecting element.
    /// `param`   — parameter on the hatch line at the intersection.
    /// `itype`   — topological classification.
    pub fn new(index: i32, param: f64, itype: HatchGen_IntersectionType) -> Self {
        Self {
            index,
            parameter: param,
            intersection_type: itype,
            is_segment_beginning: false,
            is_segment_end: false,
        }
    }

    /// Return the element index of this intersection.
    ///
    /// Mirrors `HatchGen_IntersectionPoint::Index()`.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Return the hatch-line parameter at this intersection.
    ///
    /// Mirrors `HatchGen_IntersectionPoint::Parameter()`.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Return the intersection type.
    ///
    /// Mirrors `HatchGen_IntersectionPoint::IntersectionType()`.
    pub fn intersection_type(&self) -> HatchGen_IntersectionType {
        self.intersection_type
    }

    /// `true` when this point marks the beginning of a coincident segment.
    ///
    /// Mirrors `HatchGen_IntersectionPoint::SegmentBeginning()`.
    pub fn is_segment_beginning(&self) -> bool {
        self.is_segment_beginning
    }

    /// `true` when this point marks the end of a coincident segment.
    ///
    /// Mirrors `HatchGen_IntersectionPoint::SegmentEnd()`.
    pub fn is_segment_end(&self) -> bool {
        self.is_segment_end
    }

    /// Set the segment-beginning flag.
    pub fn set_segment_beginning(&mut self, val: bool) {
        self.is_segment_beginning = val;
    }

    /// Set the segment-end flag.
    pub fn set_segment_end(&mut self, val: bool) {
        self.is_segment_end = val;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HatchGen_PointOnHatching
// ─────────────────────────────────────────────────────────────────────────────

/// An intersection point as seen from the hatch line: records the parameter on
/// the hatch line and which geometry element (1-based index) produced the hit.
///
/// Mirrors `HatchGen_PointOnHatching` from OCCT's `HatchGen` package.
// occt: HatchGen_PointOnHatching
#[derive(Clone, Debug)]
pub struct HatchGen_PointOnHatching {
    /// Intersection data common to both hatch and element sides.
    intersection: HatchGen_IntersectionPoint,
    /// 1-based index of the boundary element that was intersected.
    element_index: i32,
}

impl HatchGen_PointOnHatching {
    /// Create a new `HatchGen_PointOnHatching`.
    ///
    /// `hatch_param`   — parameter on the hatch line.
    /// `itype`         — intersection type.
    /// `element_index` — 1-based index of the intersected element.
    pub fn new(
        hatch_param: f64,
        itype: HatchGen_IntersectionType,
        element_index: i32,
    ) -> Self {
        Self {
            intersection: HatchGen_IntersectionPoint::new(element_index, hatch_param, itype),
            element_index,
        }
    }

    /// Parameter on the hatch line.
    ///
    /// Mirrors `HatchGen_PointOnHatching::Parameter()`.
    pub fn parameter(&self) -> f64 {
        self.intersection.parameter()
    }

    /// Intersection type.
    ///
    /// Mirrors `HatchGen_PointOnHatching::IntersectionType()`.
    pub fn intersection_type(&self) -> HatchGen_IntersectionType {
        self.intersection.intersection_type()
    }

    /// 1-based index of the intersected element.
    ///
    /// Mirrors `HatchGen_PointOnHatching::Index()`.
    pub fn element_index(&self) -> i32 {
        self.element_index
    }

    /// Access the underlying [`HatchGen_IntersectionPoint`].
    pub fn intersection_point(&self) -> &HatchGen_IntersectionPoint {
        &self.intersection
    }

    /// Mutable access to the underlying [`HatchGen_IntersectionPoint`].
    pub fn intersection_point_mut(&mut self) -> &mut HatchGen_IntersectionPoint {
        &mut self.intersection
    }

    /// Compare two `HatchGen_PointOnHatching` values by their line parameter.
    ///
    /// Returns `std::cmp::Ordering::Less` if `self` is closer to the hatch
    /// line origin than `other`.  Matches the sort order OCCT uses when
    /// building domain lists.
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.parameter()
            .partial_cmp(&other.parameter())
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HatchGen_PointOnElement
// ─────────────────────────────────────────────────────────────────────────────

/// An intersection point as seen from the boundary element: records the
/// parameter on the element and which hatch line (1-based index) produced the
/// hit.
///
/// Mirrors `HatchGen_PointOnElement` from OCCT's `HatchGen` package.
// occt: HatchGen_PointOnElement
#[derive(Clone, Debug)]
pub struct HatchGen_PointOnElement {
    /// Intersection data: parameter stored here is the parameter on the
    /// *element* (not the hatch line).
    intersection: HatchGen_IntersectionPoint,
    /// 1-based index of the hatch line that produced this intersection.
    hatch_index: i32,
}

impl HatchGen_PointOnElement {
    /// Create a new `HatchGen_PointOnElement`.
    ///
    /// `element_param` — parameter on the boundary element at the hit.
    /// `itype`         — intersection type.
    /// `hatch_index`   — 1-based index of the hatch line.
    pub fn new(
        element_param: f64,
        itype: HatchGen_IntersectionType,
        hatch_index: i32,
    ) -> Self {
        Self {
            intersection: HatchGen_IntersectionPoint::new(hatch_index, element_param, itype),
            hatch_index,
        }
    }

    /// Parameter on the boundary element.
    ///
    /// Mirrors `HatchGen_PointOnElement::Parameter()`.
    pub fn parameter(&self) -> f64 {
        self.intersection.parameter()
    }

    /// Intersection type.
    ///
    /// Mirrors `HatchGen_PointOnElement::IntersectionType()`.
    pub fn intersection_type(&self) -> HatchGen_IntersectionType {
        self.intersection.intersection_type()
    }

    /// 1-based index of the hatch line.
    ///
    /// Mirrors `HatchGen_PointOnElement::Index()`.
    pub fn hatch_index(&self) -> i32 {
        self.hatch_index
    }

    /// Access the underlying [`HatchGen_IntersectionPoint`].
    pub fn intersection_point(&self) -> &HatchGen_IntersectionPoint {
        &self.intersection
    }

    /// Mutable access to the underlying [`HatchGen_IntersectionPoint`].
    pub fn intersection_point_mut(&mut self) -> &mut HatchGen_IntersectionPoint {
        &mut self.intersection
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HatchGen_Domain
// ─────────────────────────────────────────────────────────────────────────────

/// A connected interval (domain) on a hatch line inside a hatched region.
///
/// A domain is bounded by up to two intersection points — a `FirstPoint` and
/// a `LastPoint`.  Either or both ends may be "open" (i.e. the hatch line
/// extends to ±∞ in that direction inside the region).
///
/// Mirrors `HatchGen_Domain` from OCCT's `HatchGen` package.
// occt: HatchGen_Domain
#[derive(Clone, Debug)]
pub struct HatchGen_Domain {
    /// The intersection point at the beginning (lower parameter) of the domain,
    /// or `None` if the domain is open at the start.
    first_point: Option<HatchGen_PointOnHatching>,
    /// The intersection point at the end (higher parameter) of the domain,
    /// or `None` if the domain is open at the end.
    last_point: Option<HatchGen_PointOnHatching>,
    /// Precomputed flag: `true` when both endpoints are set and the domain has
    /// a finite length.
    has_first_point: bool,
    has_last_point: bool,
}

impl HatchGen_Domain {
    /// Create an empty (invalid) domain.
    ///
    /// Both `first_point` and `last_point` are unset.  This mirrors the
    /// default constructor of `HatchGen_Domain`.
    pub fn new() -> Self {
        Self {
            first_point: None,
            last_point: None,
            has_first_point: false,
            has_last_point: false,
        }
    }

    /// Create a bounded domain from two existing hatch-line intersection
    /// points.
    ///
    /// `first` must have a parameter ≤ `last`.  If they are provided in the
    /// wrong order the values are silently swapped to maintain the invariant.
    ///
    /// Mirrors `HatchGen_Domain(First, Last)`.
    pub fn from_points(
        first: HatchGen_PointOnHatching,
        last: HatchGen_PointOnHatching,
    ) -> Self {
        let (f, l) = if first.parameter() <= last.parameter() {
            (first, last)
        } else {
            (last, first)
        };
        Self {
            first_point: Some(f),
            last_point: Some(l),
            has_first_point: true,
            has_last_point: true,
        }
    }

    /// Create an open-ended domain that starts at `first` and has no upper
    /// bound (open at the right / last end).
    ///
    /// Mirrors `HatchGen_Domain(First)` in OCCT.
    pub fn from_first_point(first: HatchGen_PointOnHatching) -> Self {
        Self {
            first_point: Some(first),
            last_point: None,
            has_first_point: true,
            has_last_point: false,
        }
    }

    // ── Setters ──────────────────────────────────────────────────────────────

    /// Set or replace the first (lower) boundary point.
    ///
    /// Mirrors `HatchGen_Domain::SetFirstPoint(P)`.
    pub fn set_first_point(&mut self, p: HatchGen_PointOnHatching) {
        self.first_point = Some(p);
        self.has_first_point = true;
    }

    /// Clear the first boundary point, making the domain open at the start.
    ///
    /// Mirrors `HatchGen_Domain::SetFirstPoint()` (no-arg overload in OCCT).
    pub fn clear_first_point(&mut self) {
        self.first_point = None;
        self.has_first_point = false;
    }

    /// Set or replace the last (upper) boundary point.
    ///
    /// Mirrors `HatchGen_Domain::SetLastPoint(P)`.
    pub fn set_last_point(&mut self, p: HatchGen_PointOnHatching) {
        self.last_point = Some(p);
        self.has_last_point = true;
    }

    /// Clear the last boundary point, making the domain open at the end.
    ///
    /// Mirrors `HatchGen_Domain::SetLastPoint()` (no-arg overload in OCCT).
    pub fn clear_last_point(&mut self) {
        self.last_point = None;
        self.has_last_point = false;
    }

    // ── Queries ──────────────────────────────────────────────────────────────

    /// `true` when a lower boundary point has been set.
    ///
    /// Mirrors `HatchGen_Domain::HasFirstPoint()`.
    pub fn has_first_point(&self) -> bool {
        self.has_first_point
    }

    /// `true` when an upper boundary point has been set.
    ///
    /// Mirrors `HatchGen_Domain::HasLastPoint()`.
    pub fn has_last_point(&self) -> bool {
        self.has_last_point
    }

    /// Return the lower boundary point.
    ///
    /// Returns `None` when the domain is open at the start.
    ///
    /// Mirrors `HatchGen_Domain::FirstPoint()`.
    pub fn first_point(&self) -> Option<&HatchGen_PointOnHatching> {
        self.first_point.as_ref()
    }

    /// Return the upper boundary point.
    ///
    /// Returns `None` when the domain is open at the end.
    ///
    /// Mirrors `HatchGen_Domain::LastPoint()`.
    pub fn last_point(&self) -> Option<&HatchGen_PointOnHatching> {
        self.last_point.as_ref()
    }

    /// Compute the length of the domain (difference of the two parameters).
    ///
    /// Returns `None` if either endpoint is unset (open domain).
    /// Returns `Some(0.0)` when both endpoints coincide.
    pub fn length(&self) -> Option<f64> {
        match (&self.first_point, &self.last_point) {
            (Some(f), Some(l)) => Some((l.parameter() - f.parameter()).abs()),
            _ => None,
        }
    }

    /// `true` when the domain is fully bounded (both endpoints set) and has a
    /// strictly positive length (up to `tol`).
    ///
    /// `tol` is a linear tolerance; typical OCCT values are `1e-10` to `1e-7`.
    pub fn is_valid(&self, tol: f64) -> bool {
        self.length().map_or(false, |l| l >= tol)
    }

    /// `true` when the domain is a closed bounded interval (both endpoints
    /// set), regardless of length.
    pub fn is_closed(&self) -> bool {
        self.has_first_point && self.has_last_point
    }

    /// `true` when neither endpoint is set (the whole hatch line is the
    /// domain — unusual but valid in degenerate cases).
    pub fn is_infinite(&self) -> bool {
        !self.has_first_point && !self.has_last_point
    }

    /// Check whether a hatch-line parameter `t` falls strictly inside this
    /// domain (exclusive of endpoints).
    ///
    /// Open ends are treated as ±∞ for the purpose of this test.
    pub fn contains(&self, t: f64, tol: f64) -> bool {
        let above_first = match &self.first_point {
            Some(fp) => t > fp.parameter() + tol,
            None => true,
        };
        let below_last = match &self.last_point {
            Some(lp) => t < lp.parameter() - tol,
            None => true,
        };
        above_first && below_last
    }

    /// Check whether a hatch-line parameter `t` falls inside or on this domain
    /// (inclusive of endpoints within `tol`).
    pub fn contains_inclusive(&self, t: f64, tol: f64) -> bool {
        let above_first = match &self.first_point {
            Some(fp) => t >= fp.parameter() - tol,
            None => true,
        };
        let below_last = match &self.last_point {
            Some(lp) => t <= lp.parameter() + tol,
            None => true,
        };
        above_first && below_last
    }

    /// Compute the midpoint of the domain.
    ///
    /// Returns `None` if either endpoint is unset.
    pub fn midpoint(&self) -> Option<f64> {
        match (&self.first_point, &self.last_point) {
            (Some(f), Some(l)) => Some(0.5 * (f.parameter() + l.parameter())),
            _ => None,
        }
    }
}

impl Default for HatchGen_Domain {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility: sort a slice of PointOnHatching by parameter
// ─────────────────────────────────────────────────────────────────────────────

/// Sort a collection of [`HatchGen_PointOnHatching`] values by their line
/// parameter in ascending order.
///
/// This mirrors the ordering that OCCT's `HatchGen_Hatcher` imposes when
/// building domain lists.
pub fn sort_points_on_hatching(points: &mut Vec<HatchGen_PointOnHatching>) {
    points.sort_by(|a, b| a.compare(b));
}

/// Build a list of [`HatchGen_Domain`] values from an already-sorted sequence
/// of IN/OUT intersection points.
///
/// The function pairs consecutive `IN` and `OUT` points to form closed domains.
/// Points with `TOUCH_IN` / `TOUCH_OUT` / `IDENTICAL` types are skipped for
/// domain construction purposes (they do not open or close a hatched interval).
///
/// This mirrors the high-level logic of `HatchGen_Hatcher::TrimDomains()`.
pub fn build_domains(
    points: &[HatchGen_PointOnHatching],
) -> Vec<HatchGen_Domain> {
    let mut domains = Vec::new();
    let mut pending_in: Option<HatchGen_PointOnHatching> = None;

    for pt in points {
        match pt.intersection_type() {
            HatchGen_IntersectionType::IN => {
                // Opening a new domain.  If there is already an open domain
                // (i.e. two consecutive IN points), the first one is discarded.
                pending_in = Some(pt.clone());
            }
            HatchGen_IntersectionType::OUT => {
                if let Some(in_pt) = pending_in.take() {
                    domains.push(HatchGen_Domain::from_points(in_pt, pt.clone()));
                }
                // If there is no pending IN point (OUT without IN), discard.
            }
            // Tangent and identical points do not open/close intervals.
            _ => {}
        }
    }

    // An unclosed domain (IN with no matching OUT) becomes a half-open domain.
    if let Some(in_pt) = pending_in {
        domains.push(HatchGen_Domain::from_first_point(in_pt));
    }

    domains
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
    // HatchGen_IntersectionPoint: field accessors round-trip.
    #[test]
    fn intersection_point_accessors() {
        let p = HatchGen_IntersectionPoint::new(3, 2.5, HatchGen_IntersectionType::IN);
        assert_eq!(p.index(), 3);
        near(p.parameter(), 2.5, "parameter");
        assert_eq!(p.intersection_type(), HatchGen_IntersectionType::IN);
        assert!(!p.is_segment_beginning());
        assert!(!p.is_segment_end());
    }

    // ── Test 2 ──────────────────────────────────────────────────────────────
    // HatchGen_IntersectionPoint: segment flags can be set.
    #[test]
    fn intersection_point_segment_flags() {
        let mut p = HatchGen_IntersectionPoint::new(1, 0.0, HatchGen_IntersectionType::IDENTICAL);
        p.set_segment_beginning(true);
        p.set_segment_end(false);
        assert!(p.is_segment_beginning());
        assert!(!p.is_segment_end());
        p.set_segment_end(true);
        assert!(p.is_segment_end());
    }

    // ── Test 3 ──────────────────────────────────────────────────────────────
    // HatchGen_PointOnHatching: accessors.
    #[test]
    fn point_on_hatching_accessors() {
        let ph = HatchGen_PointOnHatching::new(3.14, HatchGen_IntersectionType::OUT, 7);
        near(ph.parameter(), 3.14, "parameter");
        assert_eq!(ph.intersection_type(), HatchGen_IntersectionType::OUT);
        assert_eq!(ph.element_index(), 7);
    }

    // ── Test 4 ──────────────────────────────────────────────────────────────
    // HatchGen_PointOnHatching: compare orders by parameter.
    #[test]
    fn point_on_hatching_compare_ordering() {
        let a = HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 1);
        let b = HatchGen_PointOnHatching::new(2.0, HatchGen_IntersectionType::OUT, 1);
        assert_eq!(a.compare(&b), std::cmp::Ordering::Less);
        assert_eq!(b.compare(&a), std::cmp::Ordering::Greater);
        let c = HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 2);
        assert_eq!(a.compare(&c), std::cmp::Ordering::Equal);
    }

    // ── Test 5 ──────────────────────────────────────────────────────────────
    // HatchGen_PointOnElement: accessors.
    #[test]
    fn point_on_element_accessors() {
        let pe = HatchGen_PointOnElement::new(-0.5, HatchGen_IntersectionType::TOUCH_IN, 2);
        near(pe.parameter(), -0.5, "element param");
        assert_eq!(pe.intersection_type(), HatchGen_IntersectionType::TOUCH_IN);
        assert_eq!(pe.hatch_index(), 2);
    }

    // ── Test 6 ──────────────────────────────────────────────────────────────
    // HatchGen_Domain: bounded domain length and midpoint.
    #[test]
    fn domain_length_and_midpoint() {
        let p1 = HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 1);
        let p2 = HatchGen_PointOnHatching::new(5.0, HatchGen_IntersectionType::OUT, 2);
        let d = HatchGen_Domain::from_points(p1, p2);
        assert!(d.has_first_point());
        assert!(d.has_last_point());
        near(d.length().unwrap(), 4.0, "length");
        near(d.midpoint().unwrap(), 3.0, "midpoint");
    }

    // ── Test 7 ──────────────────────────────────────────────────────────────
    // HatchGen_Domain: contains and contains_inclusive.
    #[test]
    fn domain_contains() {
        let p1 = HatchGen_PointOnHatching::new(2.0, HatchGen_IntersectionType::IN, 1);
        let p2 = HatchGen_PointOnHatching::new(8.0, HatchGen_IntersectionType::OUT, 2);
        let d = HatchGen_Domain::from_points(p1, p2);

        // Strictly inside.
        assert!(d.contains(5.0, 1e-9), "5.0 inside [2,8]");
        // At boundaries — exclusive test should return false.
        assert!(!d.contains(2.0, 1e-9), "2.0 not strictly inside");
        assert!(!d.contains(8.0, 1e-9), "8.0 not strictly inside");
        // Inclusive test.
        assert!(d.contains_inclusive(2.0, 1e-9), "2.0 on boundary inclusive");
        assert!(d.contains_inclusive(8.0, 1e-9), "8.0 on boundary inclusive");
        // Outside.
        assert!(!d.contains(1.0, 1e-9), "1.0 outside");
        assert!(!d.contains_inclusive(9.0, 1e-9), "9.0 outside inclusive");
    }

    // ── Test 8 ──────────────────────────────────────────────────────────────
    // HatchGen_Domain: open (half-infinite) domain.
    #[test]
    fn domain_half_open() {
        let p = HatchGen_PointOnHatching::new(3.0, HatchGen_IntersectionType::IN, 1);
        let d = HatchGen_Domain::from_first_point(p);
        assert!(d.has_first_point());
        assert!(!d.has_last_point());
        assert!(d.length().is_none(), "open domain has no finite length");
        assert!(d.midpoint().is_none());
        // Contains: should be true for any value above 3.0.
        assert!(d.contains(1000.0, 1e-9), "open end: 1000 inside");
        assert!(!d.contains(2.0, 1e-9), "2.0 below open start");
    }

    // ── Test 9 ──────────────────────────────────────────────────────────────
    // build_domains: simple IN/OUT pair produces one domain.
    #[test]
    fn build_domains_single_pair() {
        let points = vec![
            HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 1),
            HatchGen_PointOnHatching::new(4.0, HatchGen_IntersectionType::OUT, 2),
        ];
        let domains = build_domains(&points);
        assert_eq!(domains.len(), 1);
        near(domains[0].first_point().unwrap().parameter(), 1.0, "first");
        near(domains[0].last_point().unwrap().parameter(), 4.0, "last");
    }

    // ── Test 10 ──────────────────────────────────────────────────────────────
    // sort_points_on_hatching and build_domains: two crossing pairs.
    #[test]
    fn sort_and_build_two_domains() {
        let mut points = vec![
            HatchGen_PointOnHatching::new(7.0, HatchGen_IntersectionType::OUT, 4),
            HatchGen_PointOnHatching::new(2.0, HatchGen_IntersectionType::IN, 1),
            HatchGen_PointOnHatching::new(5.0, HatchGen_IntersectionType::IN, 3),
            HatchGen_PointOnHatching::new(9.0, HatchGen_IntersectionType::OUT, 2),
        ];
        sort_points_on_hatching(&mut points);
        // After sort: 2 IN, 5 IN, 7 OUT, 9 OUT.
        let domains = build_domains(&points);
        // 2..7 (first IN closes at first OUT), 5..9.
        // Because the second IN overwrites the pending IN from parameter 2,
        // build_domains discards the first IN when a second IN arrives.
        // Domain 1: [5, 7], Domain 2: no pending after 9.
        // Actually with our logic: first IN sets pending=2; second IN replaces
        // pending=5; OUT at 7 closes pending 5 → domain [5,7]; OUT at 9 has
        // no pending → discarded.
        assert_eq!(domains.len(), 1, "one closed domain from second IN/OUT pair");
        near(domains[0].first_point().unwrap().parameter(), 5.0, "start");
        near(domains[0].last_point().unwrap().parameter(), 7.0, "end");
    }
}
