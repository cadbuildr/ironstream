// FILE: tcol_geom.rs
//! `TColGeom` — collection types for 3D geometry objects, mirroring
//! OpenCascade's `TColGeom` package (TKG3d).
//!
//! These collections map closely to `NCollection_Array1` and
//! `NCollection_Sequence` specialised for `Geom_Curve` and `Geom_Surface`.
//! Since pure Rust does not use trait objects in this port, curve and surface
//! descriptors are represented as `String` type-name tags.
//!
//! Array indexing follows the OCCT convention: the lower bound is supplied at
//! construction time (need not be 1). Sequence indexing is 0-based on the
//! underlying `Vec` but the public API exposes OCCT-style methods (append,
//! prepend, first, last, get by 0-based `usize`).

// ─── TColGeom_Array1OfCurve ──────────────────────────────────────────────────

/// A 1-D fixed-size array of curve descriptors with an arbitrary lower index.
///
/// Mirrors `TColGeom_Array1OfCurve` from the OpenCascade `TColGeom` package.
/// Each slot holds an `Option<String>` curve-type name; unset slots are `None`.
/// Indices run from `lower` to `upper` inclusive. Bounds violations panic,
/// mirroring `Standard_OutOfRange`.
// occt: TColGeom_Array1OfCurve
#[derive(Clone, Debug)]
pub struct ColGeom1dCurve {
    data: Vec<Option<String>>,
    lower: i32,
    upper: i32,
}

impl ColGeom1dCurve {
    /// Construct a new array with indices `lower..=upper`, all slots `None`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "ColGeom1dCurve: upper ({upper}) < lower ({lower})"
        );
        let size = (upper - lower + 1) as usize;
        Self {
            data: vec![None; size],
            lower,
            upper,
        }
    }

    /// Set the slot at index `idx` to `name`.
    ///
    /// # Panics
    /// Panics when `idx` is outside `[lower, upper]`.
    pub fn set(&mut self, idx: i32, name: String) {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize] = Some(name);
    }

    /// Return the curve name at index `idx`, or `None` if unset.
    ///
    /// # Panics
    /// Panics when `idx` is outside `[lower, upper]`.
    pub fn get(&self, idx: i32) -> Option<&str> {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize].as_deref()
    }

    /// Lower bound of the index range.
    #[inline]
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Upper bound of the index range.
    #[inline]
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Number of slots in the array (`upper - lower + 1`).
    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// `true` when the array has zero slots (not constructable in practice).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    fn check_bounds(&self, idx: i32) {
        assert!(
            idx >= self.lower && idx <= self.upper,
            "ColGeom1dCurve: index {idx} out of range [{}, {}]",
            self.lower,
            self.upper
        );
    }
}

// ─── TColGeom_Array1OfSurface ────────────────────────────────────────────────

/// A 1-D fixed-size array of surface descriptors with an arbitrary lower index.
///
/// Mirrors `TColGeom_Array1OfSurface`. API is identical to
/// [`ColGeom1dCurve`] but typed for surfaces.
// occt: TColGeom_Array1OfSurface
#[derive(Clone, Debug)]
pub struct ColGeom1dSurface {
    data: Vec<Option<String>>,
    lower: i32,
    upper: i32,
}

impl ColGeom1dSurface {
    /// Construct a new array with indices `lower..=upper`, all slots `None`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "ColGeom1dSurface: upper ({upper}) < lower ({lower})"
        );
        let size = (upper - lower + 1) as usize;
        Self {
            data: vec![None; size],
            lower,
            upper,
        }
    }

    /// Set the slot at index `idx` to `name`.
    ///
    /// # Panics
    /// Panics when `idx` is outside `[lower, upper]`.
    pub fn set(&mut self, idx: i32, name: String) {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize] = Some(name);
    }

    /// Return the surface name at index `idx`, or `None` if unset.
    ///
    /// # Panics
    /// Panics when `idx` is outside `[lower, upper]`.
    pub fn get(&self, idx: i32) -> Option<&str> {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize].as_deref()
    }

    /// Lower bound of the index range.
    #[inline]
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Upper bound of the index range.
    #[inline]
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Number of slots in the array (`upper - lower + 1`).
    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// `true` when the array has zero slots (not constructable in practice).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    fn check_bounds(&self, idx: i32) {
        assert!(
            idx >= self.lower && idx <= self.upper,
            "ColGeom1dSurface: index {idx} out of range [{}, {}]",
            self.lower,
            self.upper
        );
    }
}

// ─── TColGeom_SequenceOfCurve ────────────────────────────────────────────────

/// A dynamic ordered sequence of curve-type names.
///
/// Mirrors `TColGeom_SequenceOfCurve`, itself a specialisation of
/// `NCollection_Sequence<Handle(Geom_Curve)>`. The collection grows or shrinks
/// freely. Access by 0-based `usize` index via [`get`](ColGeomSeqCurve::get).
// occt: TColGeom_SequenceOfCurve
#[derive(Clone, Debug, Default)]
pub struct ColGeomSeqCurve {
    data: Vec<String>,
}

impl ColGeomSeqCurve {
    /// Empty sequence.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `name` to the end of the sequence.
    #[inline]
    pub fn append(&mut self, name: String) {
        self.data.push(name);
    }

    /// Prepend `name` at the front of the sequence.
    #[inline]
    pub fn prepend(&mut self, name: String) {
        self.data.insert(0, name);
    }

    /// Return the first element, or `None` when the sequence is empty.
    #[inline]
    pub fn first(&self) -> Option<&str> {
        self.data.first().map(String::as_str)
    }

    /// Return the last element, or `None` when the sequence is empty.
    #[inline]
    pub fn last(&self) -> Option<&str> {
        self.data.last().map(String::as_str)
    }

    /// Return element at 0-based index `idx`, or `None` when out of range.
    #[inline]
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.data.get(idx).map(String::as_str)
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// `true` when the sequence contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Remove all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Remove the element at 0-based index `idx`.
    ///
    /// # Panics
    /// Panics when `idx >= length()`.
    pub fn remove(&mut self, idx: usize) {
        assert!(
            idx < self.data.len(),
            "ColGeomSeqCurve::remove: index {idx} out of range [0, {})",
            self.data.len()
        );
        self.data.remove(idx);
    }
}

// ─── TColGeom_SequenceOfSurface ──────────────────────────────────────────────

/// A dynamic ordered sequence of surface-type names.
///
/// Mirrors `TColGeom_SequenceOfSurface`. API is identical to
/// [`ColGeomSeqCurve`] but typed for surfaces.
// occt: TColGeom_SequenceOfSurface
#[derive(Clone, Debug, Default)]
pub struct ColGeomSeqSurface {
    data: Vec<String>,
}

impl ColGeomSeqSurface {
    /// Empty sequence.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `name` to the end of the sequence.
    #[inline]
    pub fn append(&mut self, name: String) {
        self.data.push(name);
    }

    /// Prepend `name` at the front of the sequence.
    #[inline]
    pub fn prepend(&mut self, name: String) {
        self.data.insert(0, name);
    }

    /// Return the first element, or `None` when the sequence is empty.
    #[inline]
    pub fn first(&self) -> Option<&str> {
        self.data.first().map(String::as_str)
    }

    /// Return the last element, or `None` when the sequence is empty.
    #[inline]
    pub fn last(&self) -> Option<&str> {
        self.data.last().map(String::as_str)
    }

    /// Return element at 0-based index `idx`, or `None` when out of range.
    #[inline]
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.data.get(idx).map(String::as_str)
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// `true` when the sequence contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Remove all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Remove the element at 0-based index `idx`.
    ///
    /// # Panics
    /// Panics when `idx >= length()`.
    pub fn remove(&mut self, idx: usize) {
        assert!(
            idx < self.data.len(),
            "ColGeomSeqSurface::remove: index {idx} out of range [0, {})",
            self.data.len()
        );
        self.data.remove(idx);
    }
}

// ─── TColGeom_HSequenceOfBoundedCurve ───────────────────────────────────────

/// A dynamic sequence of bounded-curve-type names.
///
/// Mirrors `TColGeom_HSequenceOfBoundedCurve` (a handle-based sequence of
/// `Handle(Geom_BoundedCurve)`). Provides the basic sequence operations:
/// append, length, indexed get, and clear.
// occt: TColGeom_HSequenceOfBoundedCurve
#[derive(Clone, Debug, Default)]
pub struct ColGeomHSeqBezierCurve {
    data: Vec<String>,
}

impl ColGeomHSeqBezierCurve {
    /// Empty sequence.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `name` to the end of the sequence.
    #[inline]
    pub fn append(&mut self, name: String) {
        self.data.push(name);
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Return element at 0-based index `idx`, or `None` when out of range.
    #[inline]
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.data.get(idx).map(String::as_str)
    }

    /// Remove all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// `true` when the sequence contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// ─── Internal unit tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ColGeom1dCurve ───────────────────────────────────────────────────────

    #[test]
    fn array1_curve_new_size() {
        let arr = ColGeom1dCurve::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.size(), 5);
        assert!(!arr.is_empty());
    }

    #[test]
    fn array1_curve_set_and_get() {
        let mut arr = ColGeom1dCurve::new(1, 3);
        arr.set(1, "Line".to_string());
        arr.set(2, "Circle".to_string());
        arr.set(3, "BSplineCurve".to_string());
        assert_eq!(arr.get(1), Some("Line"));
        assert_eq!(arr.get(2), Some("Circle"));
        assert_eq!(arr.get(3), Some("BSplineCurve"));
    }

    #[test]
    fn array1_curve_unset_slot_is_none() {
        let arr = ColGeom1dCurve::new(1, 4);
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.get(4), None);
    }

    #[test]
    fn array1_curve_nonunit_lower_bound() {
        let mut arr = ColGeom1dCurve::new(3, 6);
        assert_eq!(arr.lower(), 3);
        assert_eq!(arr.upper(), 6);
        assert_eq!(arr.size(), 4);
        arr.set(4, "Ellipse".to_string());
        assert_eq!(arr.get(3), None);
        assert_eq!(arr.get(4), Some("Ellipse"));
    }

    #[test]
    #[should_panic]
    fn array1_curve_set_out_of_bounds_panics() {
        let mut arr = ColGeom1dCurve::new(1, 3);
        arr.set(4, "Line".to_string());
    }

    #[test]
    #[should_panic]
    fn array1_curve_get_out_of_bounds_panics() {
        let arr = ColGeom1dCurve::new(1, 3);
        let _ = arr.get(0);
    }

    // ── ColGeom1dSurface ─────────────────────────────────────────────────────

    #[test]
    fn array1_surface_set_and_get() {
        let mut arr = ColGeom1dSurface::new(1, 4);
        arr.set(1, "Plane".to_string());
        arr.set(3, "CylindricalSurface".to_string());
        assert_eq!(arr.get(1), Some("Plane"));
        assert_eq!(arr.get(2), None);
        assert_eq!(arr.get(3), Some("CylindricalSurface"));
        assert_eq!(arr.size(), 4);
    }

    #[test]
    fn array1_surface_lower_upper() {
        let arr = ColGeom1dSurface::new(0, 2);
        assert_eq!(arr.lower(), 0);
        assert_eq!(arr.upper(), 2);
        assert_eq!(arr.size(), 3);
    }

    // ── ColGeomSeqCurve ──────────────────────────────────────────────────────

    #[test]
    fn seq_curve_append_and_length() {
        let mut seq = ColGeomSeqCurve::new();
        assert!(seq.is_empty());
        seq.append("Line".to_string());
        seq.append("Circle".to_string());
        seq.append("BezierCurve".to_string());
        assert_eq!(seq.length(), 3);
        assert!(!seq.is_empty());
    }

    #[test]
    fn seq_curve_prepend() {
        let mut seq = ColGeomSeqCurve::new();
        seq.append("B".to_string());
        seq.prepend("A".to_string());
        assert_eq!(seq.first(), Some("A"));
        assert_eq!(seq.last(), Some("B"));
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn seq_curve_get_by_index() {
        let mut seq = ColGeomSeqCurve::new();
        seq.append("X".to_string());
        seq.append("Y".to_string());
        seq.append("Z".to_string());
        assert_eq!(seq.get(0), Some("X"));
        assert_eq!(seq.get(1), Some("Y"));
        assert_eq!(seq.get(2), Some("Z"));
        assert_eq!(seq.get(3), None);
    }

    #[test]
    fn seq_curve_remove_middle() {
        let mut seq = ColGeomSeqCurve::new();
        seq.append("A".to_string());
        seq.append("B".to_string());
        seq.append("C".to_string());
        seq.remove(1);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.get(0), Some("A"));
        assert_eq!(seq.get(1), Some("C"));
    }

    #[test]
    fn seq_curve_clear() {
        let mut seq = ColGeomSeqCurve::new();
        seq.append("Line".to_string());
        seq.append("Arc".to_string());
        seq.clear();
        assert!(seq.is_empty());
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn seq_curve_first_last_empty() {
        let seq = ColGeomSeqCurve::new();
        assert_eq!(seq.first(), None);
        assert_eq!(seq.last(), None);
    }

    // ── ColGeomSeqSurface ────────────────────────────────────────────────────

    #[test]
    fn seq_surface_append_prepend_remove() {
        let mut seq = ColGeomSeqSurface::new();
        seq.append("Plane".to_string());
        seq.append("SphericalSurface".to_string());
        seq.prepend("ToroidalSurface".to_string());
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.first(), Some("ToroidalSurface"));
        assert_eq!(seq.last(), Some("SphericalSurface"));
        seq.remove(0);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.first(), Some("Plane"));
    }

    // ── ColGeomHSeqBezierCurve ───────────────────────────────────────────────

    #[test]
    fn hseq_bezier_append_and_get() {
        let mut seq = ColGeomHSeqBezierCurve::new();
        assert!(seq.is_empty());
        seq.append("BezierCurve".to_string());
        seq.append("BSplineCurve".to_string());
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.get(0), Some("BezierCurve"));
        assert_eq!(seq.get(1), Some("BSplineCurve"));
        assert_eq!(seq.get(2), None);
    }

    #[test]
    fn hseq_bezier_clear() {
        let mut seq = ColGeomHSeqBezierCurve::new();
        seq.append("BezierCurve".to_string());
        seq.clear();
        assert!(seq.is_empty());
        assert_eq!(seq.length(), 0);
    }
}
