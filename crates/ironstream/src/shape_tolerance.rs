// FILE: rust/ironstream/crates/ironstream/src/shape_tolerance.rs
//! `ShapeTolerance` — tolerance analysis and management for topological shapes,
//! mirroring OpenCascade's `ShapeAnalysis_ShapeTolerance` and
//! `ShapeFix_ShapeTolerance` packages.
//!
//! # Scope
//!
//! * [`ToleranceType`] — kind of topological entity a tolerance record belongs to.
//! * [`ToleranceEntry`] — a single tolerance record with label, type, and value.
//! * [`ShapeToleranceAnalysis`] — collects tolerance entries and provides
//!   statistics: min, max, mean, and per-type queries.
//! * [`update_tolerances`] — scales all tolerance values in a collection by a
//!   factor (mirrors `ShapeFix_ShapeTolerance::SetTolerance`).

// ---------------------------------------------------------------------------
// ToleranceType
// ---------------------------------------------------------------------------

/// Identifies the kind of topological entity that owns a tolerance value.
///
/// // occt: ShapeAnalysis_ShapeTolerance // type — enum Vertex, Edge, Face, Overall
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToleranceType {
    /// A point-like topological vertex.
    // occt-ref: TopAbs_VERTEX
    Vertex,
    /// A curve-like topological edge.
    // occt-ref: TopAbs_EDGE
    Edge,
    /// A surface-like topological face.
    // occt-ref: TopAbs_FACE
    Face,
    /// An aggregate tolerance covering the whole shape.
    // occt: ShapeAnalysis_ShapeTolerance // — global/overall tolerance
    Overall,
}

// ---------------------------------------------------------------------------
// ToleranceEntry
// ---------------------------------------------------------------------------

/// A single tolerance record: a human-readable label, the entity type, and
/// the tolerance value (in the same length units as the model).
///
/// // occt-note: tolerance record — fields: label String, tolerance_type ToleranceType, value f64
#[derive(Clone, Debug)]
pub struct ToleranceEntry {
    label: String,
    tolerance_type: ToleranceType,
    value: f64,
}

impl ToleranceEntry {
    /// Create a new tolerance entry.
    ///
    /// * `label` — human-readable name (e.g. `"Edge_0"`, `"Vertex_3"`).
    /// * `tolerance_type` — which kind of topological entity this belongs to.
    /// * `value` — tolerance magnitude (must be finite; negative values are
    ///   accepted as-is so the caller can represent signed deviations).
    pub fn new(label: &str, tolerance_type: ToleranceType, value: f64) -> Self {
        Self {
            label: label.to_owned(),
            tolerance_type,
            value,
        }
    }

    /// The human-readable label for this entry.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The kind of topological entity this tolerance belongs to.
    pub fn tolerance_type(&self) -> ToleranceType {
        self.tolerance_type
    }

    /// The tolerance value.
    pub fn value(&self) -> f64 {
        self.value
    }
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis
// ---------------------------------------------------------------------------

/// Collects [`ToleranceEntry`] records for a shape and provides aggregate
/// statistics over them.
///
/// // occt: ShapeAnalysis_ShapeTolerance
#[derive(Clone, Debug, Default)]
pub struct ShapeToleranceAnalysis {
    /// All tolerance entries added to this analysis.
    // occt-note: internal NCollection_Sequence<entry> — stored in insertion order
    pub entries: Vec<ToleranceEntry>,
}

impl ShapeToleranceAnalysis {
    /// Create an empty analysis (no entries).
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Append a tolerance entry.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::AddTolerance
    pub fn add(&mut self, e: ToleranceEntry) {
        self.entries.push(e);
    }

    /// The minimum tolerance value across all entries.
    ///
    /// Returns `f64::INFINITY` when there are no entries.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::Tolerance (mode = 1 → min)
    pub fn min_tolerance(&self) -> f64 {
        self.entries
            .iter()
            .map(|e| e.value)
            .fold(f64::INFINITY, f64::min)
    }

    /// The maximum tolerance value across all entries.
    ///
    /// Returns `f64::NEG_INFINITY` when there are no entries.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::Tolerance (mode = 2 → max)
    pub fn max_tolerance(&self) -> f64 {
        self.entries
            .iter()
            .map(|e| e.value)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// The arithmetic mean of all tolerance values.
    ///
    /// Returns `0.0` when there are no entries.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::Tolerance (mode = 0 → average)
    pub fn mean_tolerance(&self) -> f64 {
        let n = self.entries.len();
        if n == 0 {
            return 0.0;
        }
        let sum: f64 = self.entries.iter().map(|e| e.value).sum();
        sum / n as f64
    }

    /// Return the tolerance values for all entries whose type matches `t`.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::Tolerance filtered by shape type
    pub fn tolerance_for_type(&self, t: ToleranceType) -> Vec<f64> {
        self.entries
            .iter()
            .filter(|e| e.tolerance_type == t)
            .map(|e| e.value)
            .collect()
    }

    /// Number of entries in the analysis.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::NbTolerance
    pub fn nb_entries(&self) -> usize {
        self.entries.len()
    }

    /// Return a reference to the entry at index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= self.nb_entries()`.
    ///
    /// // occt: ShapeAnalysis_ShapeTolerance // ::Tolerance(i)
    pub fn entry(&self, i: usize) -> &ToleranceEntry {
        &self.entries[i]
    }
}

// ---------------------------------------------------------------------------
// update_tolerances
// ---------------------------------------------------------------------------

/// Scale every tolerance value in `entries` by `scale_factor`.
///
/// This mirrors `ShapeFix_ShapeTolerance::SetTolerance` which adjusts all
/// stored tolerances by a uniform factor (e.g. unit conversion from mm to
/// inches: `scale_factor = 1.0 / 25.4`).
///
/// // occt: ShapeFix_ShapeTolerance // ::SetTolerance
pub fn update_tolerances(entries: &mut Vec<ToleranceEntry>, scale_factor: f64) {
    for e in entries.iter_mut() {
        e.value *= scale_factor;
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(label: &str, t: ToleranceType, v: f64) -> ToleranceEntry {
        ToleranceEntry::new(label, t, v)
    }

    #[test]
    fn test_entry_accessors() {
        let e = make_entry("Vertex_0", ToleranceType::Vertex, 1e-6);
        assert_eq!(e.label(), "Vertex_0");
        assert_eq!(e.tolerance_type(), ToleranceType::Vertex);
        assert!((e.value() - 1e-6).abs() < 1e-18);
    }

    #[test]
    fn test_empty_analysis() {
        let a = ShapeToleranceAnalysis::new();
        assert_eq!(a.nb_entries(), 0);
        assert_eq!(a.min_tolerance(), f64::INFINITY);
        assert_eq!(a.max_tolerance(), f64::NEG_INFINITY);
        assert_eq!(a.mean_tolerance(), 0.0);
        assert!(a.tolerance_for_type(ToleranceType::Vertex).is_empty());
    }

    #[test]
    fn test_add_and_nb_entries() {
        let mut a = ShapeToleranceAnalysis::new();
        a.add(make_entry("e0", ToleranceType::Edge, 0.01));
        a.add(make_entry("v0", ToleranceType::Vertex, 0.001));
        assert_eq!(a.nb_entries(), 2);
    }

    #[test]
    fn test_min_max_mean() {
        let mut a = ShapeToleranceAnalysis::new();
        a.add(make_entry("e0", ToleranceType::Edge, 0.1));
        a.add(make_entry("e1", ToleranceType::Edge, 0.3));
        a.add(make_entry("f0", ToleranceType::Face, 0.2));
        assert!((a.min_tolerance() - 0.1).abs() < 1e-15);
        assert!((a.max_tolerance() - 0.3).abs() < 1e-15);
        assert!((a.mean_tolerance() - 0.2).abs() < 1e-15);
    }

    #[test]
    fn test_tolerance_for_type() {
        let mut a = ShapeToleranceAnalysis::new();
        a.add(make_entry("v0", ToleranceType::Vertex, 0.005));
        a.add(make_entry("e0", ToleranceType::Edge, 0.01));
        a.add(make_entry("v1", ToleranceType::Vertex, 0.003));
        let vs = a.tolerance_for_type(ToleranceType::Vertex);
        assert_eq!(vs.len(), 2);
        assert!((vs[0] - 0.005).abs() < 1e-15);
        assert!((vs[1] - 0.003).abs() < 1e-15);
        let es = a.tolerance_for_type(ToleranceType::Edge);
        assert_eq!(es.len(), 1);
        let fs = a.tolerance_for_type(ToleranceType::Face);
        assert!(fs.is_empty());
    }

    #[test]
    fn test_entry_by_index() {
        let mut a = ShapeToleranceAnalysis::new();
        a.add(make_entry("overall", ToleranceType::Overall, 1e-3));
        a.add(make_entry("face0", ToleranceType::Face, 2e-3));
        assert_eq!(a.entry(0).label(), "overall");
        assert_eq!(a.entry(1).label(), "face0");
        assert_eq!(a.entry(1).tolerance_type(), ToleranceType::Face);
    }

    #[test]
    fn test_update_tolerances_scale() {
        let mut entries = vec![
            make_entry("e0", ToleranceType::Edge, 1.0),
            make_entry("e1", ToleranceType::Edge, 2.0),
            make_entry("v0", ToleranceType::Vertex, 0.5),
        ];
        update_tolerances(&mut entries, 2.0);
        assert!((entries[0].value() - 2.0).abs() < 1e-15);
        assert!((entries[1].value() - 4.0).abs() < 1e-15);
        assert!((entries[2].value() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_update_tolerances_fractional_scale() {
        let mut entries = vec![make_entry("v0", ToleranceType::Vertex, 25.4)];
        // Convert mm → inches
        update_tolerances(&mut entries, 1.0 / 25.4);
        assert!((entries[0].value() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_update_tolerances_zero_scale() {
        let mut entries = vec![make_entry("f0", ToleranceType::Face, 99.0)];
        update_tolerances(&mut entries, 0.0);
        assert_eq!(entries[0].value(), 0.0);
    }

    #[test]
    fn test_single_entry_min_max_mean_equal() {
        let mut a = ShapeToleranceAnalysis::new();
        a.add(make_entry("only", ToleranceType::Overall, 0.07));
        assert!((a.min_tolerance() - 0.07).abs() < 1e-15);
        assert!((a.max_tolerance() - 0.07).abs() < 1e-15);
        assert!((a.mean_tolerance() - 0.07).abs() < 1e-15);
    }
}
