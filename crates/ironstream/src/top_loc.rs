// FILE: top_loc.rs
//! `TopLoc` — local coordinate frames and location composition, mirroring
//! OpenCascade's `TopLoc` package from `TKBRep`.
//!
//! The two central classes are:
//!
//! * [`Datum3D`] — an elementary, immutable 3-D rigid motion (a `Trsf` that
//!   is restricted to rotations and translations, i.e. no scale).  Mirrors
//!   `TopLoc_Datum3D`.
//!
//! * [`Location`] — a compound location built from a list of
//!   `(Datum3D, power)` pairs.  Applying the list in order gives the net
//!   transform.  Mirrors `TopLoc_Location`.
//!
//! ## Design
//!
//! In OCCT a `TopLoc_Location` is stored as a singly-linked "spine" of
//! `(datum, power)` records, making sharing cheap.  Here we use a `Vec`
//! instead, which is simpler and equally correct for our use-case.
//!
//! ## Reference
//! * OCCT sources: `src/TopLoc/TopLoc_Datum3D.*` and `TopLoc_Location.*`

use crate::gp::{Ax1, Pnt};

// We deliberately do NOT import gp_trsf::Trsf because the gp module already
// defines a lighter Trsf (used throughout this codebase). We depend on gp::Trsf.
use crate::gp::Trsf;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers (3×3 matrix arithmetic on gp::Trsf's `m` field)
// ─────────────────────────────────────────────────────────────────────────────

#[inline]
fn mul_mat3(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut r = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    r
}

#[inline]
fn mul_mat3_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

#[inline]
fn mat3_det(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn mat3_inv(m: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let det = mat3_det(m);
    if det.abs() < 1.0e-15 {
        return None;
    }
    let inv = 1.0 / det;
    Some([
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv,
        ],
    ])
}

/// Invert a `Trsf` (linear + translation).  Returns `None` if singular.
fn trsf_inv(t: &Trsf) -> Option<Trsf> {
    let inv_m = mat3_inv(&t.m)?;
    let neg_t = [-t.t[0], -t.t[1], -t.t[2]];
    let inv_t = mul_mat3_vec(&inv_m, neg_t);
    Some(Trsf { m: inv_m, t: inv_t })
}

/// Compose `outer ∘ inner`: apply `inner` first, then `outer`.
fn trsf_compose(outer: &Trsf, inner: &Trsf) -> Trsf {
    let m = mul_mat3(&outer.m, &inner.m);
    let rt = mul_mat3_vec(&outer.m, inner.t);
    Trsf {
        m,
        t: [rt[0] + outer.t[0], rt[1] + outer.t[1], rt[2] + outer.t[2]],
    }
}

/// Raise `t` to integer power `n`.  `n = 0` → identity.
fn trsf_power(t: &Trsf, n: i32) -> Trsf {
    if n == 0 {
        return Trsf::identity();
    }
    if n == 1 {
        return *t;
    }
    if n == -1 {
        return trsf_inv(t).unwrap_or_else(Trsf::identity);
    }
    let base = if n < 0 {
        trsf_inv(t).unwrap_or_else(Trsf::identity)
    } else {
        *t
    };
    let mut npower = n.unsigned_abs();
    let mut cur = base;
    let mut result = Trsf::identity();
    while npower > 0 {
        if npower & 1 == 1 {
            result = trsf_compose(&result, &cur);
        }
        npower >>= 1;
        if npower > 0 {
            cur = trsf_compose(&cur, &cur);
        }
    }
    result
}

// ─────────────────────────────────────────────────────────────────────────────
// Datum3D
// ─────────────────────────────────────────────────────────────────────────────

/// An elementary, immutable rigid-body transformation (rotation + translation,
/// no scale, no mirror).
///
/// In OCCT a `TopLoc_Datum3D` is a reference-counted, immutable object.
/// Here we make it `Clone + Copy` because `Trsf` is already small (9 f64s for
/// the matrix plus 3 for the translation = 12 × 8 = 96 bytes).
///
/// Internally the datum stores a `gp::Trsf`; its determinant should always be
/// `+1` (orthogonal, orientation-preserving).
// occt: TopLoc_Datum3D
#[derive(Clone, Copy, Debug)]
pub struct Datum3D {
    trsf: Trsf,
}

impl Datum3D {
    /// Identity datum (no motion).
    pub fn identity() -> Self {
        Self { trsf: Trsf::identity() }
    }

    /// Construct from an arbitrary `Trsf`.  The caller is responsible for
    /// supplying a rigid-body (orthogonal, det = +1) transform; no normalisation
    /// is performed.
    pub fn from_trsf(trsf: Trsf) -> Self {
        Self { trsf }
    }

    /// Pure translation by vector `v`.
    pub fn translation(v: Pnt) -> Self {
        let mut t = Trsf::identity();
        t.t = [v.x, v.y, v.z];
        Self { trsf: t }
    }

    /// Pure rotation of `angle` radians about `axis` (Rodrigues' formula).
    pub fn rotation(axis: Ax1, angle: f64) -> Self {
        Self { trsf: Trsf::rotation(axis, angle) }
    }

    /// The underlying `Trsf`.
    pub fn trsf(&self) -> &Trsf {
        &self.trsf
    }

    /// Apply this datum to a point.
    pub fn apply(&self, p: Pnt) -> Pnt {
        self.trsf.apply_point(p)
    }

    /// Inverse datum.  Panics only if `trsf` is somehow singular (should not
    /// happen for a rigid-body transform).
    pub fn inverted(&self) -> Self {
        let inv = trsf_inv(&self.trsf).expect("Datum3D::inverted: singular transform");
        Self { trsf: inv }
    }

    /// `IsIdentity` — true when the transform is (approximately) the identity.
    pub fn is_identity(&self) -> bool {
        let t = &self.trsf;
        let m = &t.m;
        const EPS: f64 = 1.0e-12;
        (m[0][0] - 1.0).abs() < EPS
            && m[0][1].abs() < EPS
            && m[0][2].abs() < EPS
            && m[1][0].abs() < EPS
            && (m[1][1] - 1.0).abs() < EPS
            && m[1][2].abs() < EPS
            && m[2][0].abs() < EPS
            && m[2][1].abs() < EPS
            && (m[2][2] - 1.0).abs() < EPS
            && t.t[0].abs() < EPS
            && t.t[1].abs() < EPS
            && t.t[2].abs() < EPS
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Location
// ─────────────────────────────────────────────────────────────────────────────

/// A single element of the location spine: a datum and an integer power.
///
/// The net transform for this element is `datum.trsf^power`.
// occt: TopLoc_SListOfItemLocation (each node)
#[derive(Clone, Debug)]
pub struct LocationItem {
    /// The elementary rigid motion.
    pub datum: Datum3D,
    /// Integer power applied to the datum (usually ±1).
    pub power: i32,
}

/// A compound location: a list of `(Datum3D, power)` items applied left to
/// right (i.e. item 0 is the outermost transformation).
///
/// An empty list represents the identity location.  This mirrors
/// `TopLoc_Location`'s `IsIdentity` / `FirstDatum` / `NextLocation` interface
/// but in a flat `Vec`-based representation.
// occt: TopLoc_Location
#[derive(Clone, Debug)]
pub struct Location {
    items: Vec<LocationItem>,
}

impl Default for Location {
    fn default() -> Self {
        Self::identity()
    }
}

impl Location {
    /// Identity location (no displacement).
    pub fn identity() -> Self {
        Self { items: Vec::new() }
    }

    /// Build a location from a single datum (power = 1).
    pub fn from_datum(datum: Datum3D) -> Self {
        if datum.is_identity() {
            return Self::identity();
        }
        Self {
            items: vec![LocationItem { datum, power: 1 }],
        }
    }

    /// Build from a `Trsf` directly (convenience).
    pub fn from_trsf(trsf: Trsf) -> Self {
        Self::from_datum(Datum3D::from_trsf(trsf))
    }

    /// Pure translation location.
    pub fn translation(v: Pnt) -> Self {
        Self::from_datum(Datum3D::translation(v))
    }

    /// Pure rotation location.
    pub fn rotation(axis: Ax1, angle: f64) -> Self {
        Self::from_datum(Datum3D::rotation(axis, angle))
    }

    /// `IsIdentity` — true when there are no items (or all are identity datums).
    pub fn is_identity(&self) -> bool {
        self.items.is_empty()
    }

    /// Number of elementary datums in the spine.
    pub fn depth(&self) -> usize {
        self.items.len()
    }

    /// The composed `Trsf` representing this location.
    ///
    /// Items are applied left-to-right: `items[0]` is outermost so the
    /// effective transform is `T0^p0 ∘ T1^p1 ∘ … ∘ Tn^pn`.
    pub fn trsf(&self) -> Trsf {
        let mut result = Trsf::identity();
        for item in &self.items {
            let t = trsf_power(&item.datum.trsf, item.power);
            result = trsf_compose(&result, &t);
        }
        result
    }

    /// Apply this location to a point.
    pub fn apply(&self, p: Pnt) -> Pnt {
        self.trsf().apply_point(p)
    }

    /// Inverted location (reverses the entire chain).
    ///
    /// The inverse of `T0^p0 ∘ T1^p1 ∘ …` is `… ∘ T1^{-p1} ∘ T0^{-p0}`.
    pub fn inverted(&self) -> Self {
        let mut items: Vec<LocationItem> = self
            .items
            .iter()
            .rev()
            .map(|item| LocationItem {
                datum: item.datum,
                power: -item.power,
            })
            .collect();
        // Drop any that reduce to identity
        items.retain(|item| !item.datum.is_identity() || item.power != 0);
        Self { items }
    }

    /// Compose `self ∘ other` (apply `other` after `self`).
    ///
    /// OCCT: `TopLoc_Location::Multiplied`.
    pub fn multiplied(&self, other: &Location) -> Location {
        let mut items = self.items.clone();
        items.extend_from_slice(&other.items);
        Location { items }
    }

    /// In-place composition: `self = self ∘ other`.
    pub fn multiply(&mut self, other: &Location) {
        self.items.extend_from_slice(&other.items);
    }

    /// `Powered(n)` — raise the whole location to integer power `n`.
    ///
    /// For `n = 0` returns identity; for `n < 0` inverts first.
    pub fn powered(&self, n: i32) -> Location {
        if n == 0 {
            return Location::identity();
        }
        if n == 1 {
            return self.clone();
        }
        if n == -1 {
            return self.inverted();
        }
        // Use repeated composition via squaring on the Trsf level, then wrap.
        let base_trsf = self.trsf();
        let result_trsf = trsf_power(&base_trsf, n);
        Location::from_trsf(result_trsf)
    }

    /// `FirstDatum` — the first (outermost) datum, or `None` if identity.
    pub fn first_datum(&self) -> Option<&Datum3D> {
        self.items.first().map(|item| &item.datum)
    }

    /// `FirstPower` — the power of the first datum, or 0 if identity.
    pub fn first_power(&self) -> i32 {
        self.items.first().map(|item| item.power).unwrap_or(0)
    }

    /// `NextLocation` — the location with the first item removed.
    pub fn next_location(&self) -> Location {
        if self.items.is_empty() {
            return Location::identity();
        }
        Location {
            items: self.items[1..].to_vec(),
        }
    }

    /// Iterator over the spine items.
    pub fn items(&self) -> impl Iterator<Item = &LocationItem> {
        self.items.iter()
    }

    /// `IsEqual` — two locations are equal when their composed transforms are
    /// approximately equal (coefficient-wise).
    pub fn is_equal(&self, other: &Location, tol: f64) -> bool {
        let a = self.trsf();
        let b = other.trsf();
        for i in 0..3 {
            for j in 0..3 {
                if (a.m[i][j] - b.m[i][j]).abs() > tol {
                    return false;
                }
            }
        }
        (a.t[0] - b.t[0]).abs() <= tol
            && (a.t[1] - b.t[1]).abs() <= tol
            && (a.t[2] - b.t[2]).abs() <= tol
    }

    /// Hash of the location — for use in `IndexedMap` / shape maps.
    /// Uses the 12 floating-point coefficients, quantised to a tolerance.
    pub fn hash_code(&self) -> u64 {
        let t = self.trsf();
        // Quantise each coefficient to ~1e-7 and combine with FNV-1a.
        const QUANT: f64 = 1.0e7;
        let mut h: u64 = 14695981039346656037;
        let bytes: [f64; 12] = [
            t.m[0][0], t.m[0][1], t.m[0][2],
            t.m[1][0], t.m[1][1], t.m[1][2],
            t.m[2][0], t.m[2][1], t.m[2][2],
            t.t[0], t.t[1], t.t[2],
        ];
        for &v in &bytes {
            let qi = (v * QUANT).round() as i64;
            let b = qi.to_le_bytes();
            for byte in b {
                h ^= byte as u64;
                h = h.wrapping_mul(1099511628211);
            }
        }
        h
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LocationBuilder
// ─────────────────────────────────────────────────────────────────────────────

/// Convenience builder for assembling a `Location` from multiple datums.
///
/// ```
/// use ironstream::top_loc::{LocationBuilder, Datum3D};
/// use ironstream::gp::Pnt;
///
/// let loc = LocationBuilder::new()
///     .push(Datum3D::translation(Pnt::new(1.0, 2.0, 3.0)), 1)
///     .build();
/// ```
// occt: no direct equivalent — utility helper
pub struct LocationBuilder {
    items: Vec<LocationItem>,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Append a datum with the given power.
    pub fn push(mut self, datum: Datum3D, power: i32) -> Self {
        if !datum.is_identity() && power != 0 {
            self.items.push(LocationItem { datum, power });
        }
        self
    }

    /// Consume the builder and produce a `Location`.
    pub fn build(self) -> Location {
        Location { items: self.items }
    }
}

impl Default for LocationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    const EPS: f64 = 1.0e-9;

    // ── Datum3D ──────────────────────────────────────────────────────────────

    #[test]
    fn datum_identity_is_identity() {
        let d = Datum3D::identity();
        assert!(d.is_identity());
    }

    #[test]
    fn datum_translation_apply() {
        let d = Datum3D::translation(Pnt::new(3.0, 0.0, 0.0));
        let p = d.apply(Pnt::new(1.0, 2.0, 3.0));
        assert!((p.x - 4.0).abs() < EPS);
        assert!((p.y - 2.0).abs() < EPS);
        assert!((p.z - 3.0).abs() < EPS);
    }

    #[test]
    fn datum_rotation_90_about_z() {
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let d = Datum3D::rotation(axis, FRAC_PI_2);
        let p = d.apply(Pnt::new(1.0, 0.0, 0.0));
        assert!((p.x - 0.0).abs() < EPS);
        assert!((p.y - 1.0).abs() < EPS);
        assert!((p.z - 0.0).abs() < EPS);
    }

    #[test]
    fn datum_inverted_cancels() {
        let d = Datum3D::translation(Pnt::new(5.0, -3.0, 7.0));
        let inv = d.inverted();
        let p = Pnt::new(1.0, 2.0, 3.0);
        let q = inv.apply(d.apply(p));
        assert!((q.x - p.x).abs() < EPS);
        assert!((q.y - p.y).abs() < EPS);
        assert!((q.z - p.z).abs() < EPS);
    }

    #[test]
    fn datum_rotation_inverted_cancels() {
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
        let d = Datum3D::rotation(axis, PI / 3.0);
        let inv = d.inverted();
        let p = Pnt::new(2.0, 3.0, 4.0);
        let q = inv.apply(d.apply(p));
        assert!((q.x - p.x).abs() < EPS);
        assert!((q.y - p.y).abs() < EPS);
        assert!((q.z - p.z).abs() < EPS);
    }

    #[test]
    fn datum_non_identity_is_not_identity() {
        let d = Datum3D::translation(Pnt::new(0.0, 0.0, 1.0));
        assert!(!d.is_identity());
    }

    // ── Location ─────────────────────────────────────────────────────────────

    #[test]
    fn location_identity_is_identity() {
        let loc = Location::identity();
        assert!(loc.is_identity());
        let p = Pnt::new(1.0, 2.0, 3.0);
        let q = loc.apply(p);
        assert!((q.x - p.x).abs() < EPS);
        assert!((q.y - p.y).abs() < EPS);
        assert!((q.z - p.z).abs() < EPS);
    }

    #[test]
    fn location_translation_apply() {
        let loc = Location::translation(Pnt::new(10.0, 0.0, 0.0));
        let p = loc.apply(Pnt::new(1.0, 2.0, 3.0));
        assert!((p.x - 11.0).abs() < EPS);
        assert!((p.y - 2.0).abs() < EPS);
        assert!((p.z - 3.0).abs() < EPS);
    }

    #[test]
    fn location_compose_two_translations() {
        let a = Location::translation(Pnt::new(1.0, 0.0, 0.0));
        let b = Location::translation(Pnt::new(0.0, 2.0, 0.0));
        let ab = a.multiplied(&b);
        let p = ab.apply(Pnt::origin());
        assert!((p.x - 1.0).abs() < EPS);
        assert!((p.y - 2.0).abs() < EPS);
        assert!((p.z - 0.0).abs() < EPS);
    }

    #[test]
    fn location_inverted_cancels() {
        let loc = Location::translation(Pnt::new(3.0, -1.0, 2.0));
        let inv = loc.inverted();
        let p = Pnt::new(5.0, 5.0, 5.0);
        let q = inv.apply(loc.apply(p));
        assert!((q.x - p.x).abs() < EPS);
        assert!((q.y - p.y).abs() < EPS);
        assert!((q.z - p.z).abs() < EPS);
    }

    #[test]
    fn location_rotation_compose_translation() {
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let rot = Location::rotation(axis, FRAC_PI_2);
        let trans = Location::translation(Pnt::new(0.0, 0.0, 5.0));
        let combined = rot.multiplied(&trans);
        // Rotate (1,0,0) → (0,1,0), then translate z+5 → (0,1,5)
        let p = combined.apply(Pnt::new(1.0, 0.0, 0.0));
        assert!((p.x - 0.0).abs() < EPS);
        assert!((p.y - 1.0).abs() < EPS);
        assert!((p.z - 5.0).abs() < EPS);
    }

    #[test]
    fn location_powered_zero_is_identity() {
        let loc = Location::translation(Pnt::new(1.0, 2.0, 3.0));
        let loc0 = loc.powered(0);
        assert!(loc0.is_identity());
    }

    #[test]
    fn location_powered_negative_one_is_inverse() {
        let loc = Location::translation(Pnt::new(4.0, 5.0, 6.0));
        let inv = loc.powered(-1);
        let p = Pnt::new(1.0, 1.0, 1.0);
        let q = inv.apply(loc.apply(p));
        assert!((q.x - p.x).abs() < EPS);
        assert!((q.y - p.y).abs() < EPS);
        assert!((q.z - p.z).abs() < EPS);
    }

    #[test]
    fn location_powered_two_doubles_translation() {
        let loc = Location::translation(Pnt::new(3.0, 0.0, 0.0));
        let loc2 = loc.powered(2);
        let p = loc2.apply(Pnt::origin());
        assert!((p.x - 6.0).abs() < EPS);
    }

    #[test]
    fn location_depth_counts_items() {
        let a = Location::translation(Pnt::new(1.0, 0.0, 0.0));
        let b = Location::translation(Pnt::new(0.0, 1.0, 0.0));
        let ab = a.multiplied(&b);
        assert_eq!(ab.depth(), 2);
    }

    #[test]
    fn location_next_location() {
        let a = Location::translation(Pnt::new(1.0, 0.0, 0.0));
        let b = Location::translation(Pnt::new(0.0, 1.0, 0.0));
        let ab = a.multiplied(&b);
        let next = ab.next_location();
        assert_eq!(next.depth(), 1);
        // next contains only b's datum
        let p = next.apply(Pnt::origin());
        assert!((p.x - 0.0).abs() < EPS);
        assert!((p.y - 1.0).abs() < EPS);
    }

    #[test]
    fn location_is_equal_self() {
        let loc = Location::translation(Pnt::new(1.0, 2.0, 3.0));
        assert!(loc.is_equal(&loc, 1.0e-9));
    }

    #[test]
    fn location_builder_empty_is_identity() {
        let loc = LocationBuilder::new().build();
        assert!(loc.is_identity());
    }

    #[test]
    fn location_builder_push_datum() {
        let d = Datum3D::translation(Pnt::new(7.0, 0.0, 0.0));
        let loc = LocationBuilder::new().push(d, 1).build();
        let p = loc.apply(Pnt::origin());
        assert!((p.x - 7.0).abs() < EPS);
    }

    #[test]
    fn location_hash_same_for_equal_transforms() {
        let a = Location::translation(Pnt::new(1.0, 2.0, 3.0));
        let b = Location::translation(Pnt::new(1.0, 2.0, 3.0));
        assert_eq!(a.hash_code(), b.hash_code());
    }

    #[test]
    fn trsf_power_n2_is_double_rotation() {
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let t = Trsf::rotation(axis, FRAC_PI_2);
        let t2 = trsf_power(&t, 2);
        // 2 × 90° = 180°: (1,0,0) → (-1,0,0)
        let p = t2.apply_point(Pnt::new(1.0, 0.0, 0.0));
        assert!((p.x - (-1.0)).abs() < EPS);
        assert!((p.y - 0.0).abs() < EPS);
    }
}
