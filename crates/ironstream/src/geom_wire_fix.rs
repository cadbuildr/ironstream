// FILE: rust/ironstream/crates/ironstream/src/geom_wire_fix.rs
//! Zero-dependency port of `ShapeAnalysis_Wire` / `ShapeFix_Wire` from OCCT.
//!
//! [`WireAnalysis`] inspects a sequence of edge identifiers for connectivity
//! problems; [`WireFixer`] applies corrections to the same representation.
//!
//! Edges are represented as opaque `String` identifiers so that higher-level
//! code can attach real geometry without introducing external crate dependencies
//! at this layer.

// ─────────────────────────────────────────────────────────────────────────────
// WireError
// ─────────────────────────────────────────────────────────────────────────────

/// Categories of wire defects detected by [`WireAnalysis`] or repaired by
/// [`WireFixer`].
// occt-ref: ShapeAnalysis_Wire // — error codes returned by its Check* methods
// occt-ref: ShapeFix_Wire // — each Fix* method addresses one of these categories
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireError {
    /// Adjacent edges do not share a common vertex (gap between end-points).
    // occt-ref: ShapeAnalysis_Wire // ::CheckConnected / ShapeAnalysis_WireOrder::Status
    NotConnected,
    /// The wire path crosses itself.
    // occt-ref: ShapeAnalysis_Wire // ::CheckSelfIntersection
    SelfIntersecting,
    /// A measurable gap exists between consecutive edge end-points that exceeds
    /// the analysis precision.
    // occt-ref: ShapeAnalysis_Wire // ::CheckGaps3d / CheckGaps2d
    Gap,
    /// An individual edge is degenerate, has zero length, or is otherwise
    /// geometrically invalid.
    // occt-ref: ShapeAnalysis_Wire // ::CheckSmall / CheckDegenerated
    BadEdge,
}

// ─────────────────────────────────────────────────────────────────────────────
// WireAnalysis
// ─────────────────────────────────────────────────────────────────────────────

/// Analyses a wire (ordered sequence of edge identifiers) for geometric and
/// topological defects.
///
/// Mirrors the public interface of `ShapeAnalysis_Wire`: load a wire, call
/// [`perform`](WireAnalysis::perform) to run all checks, then query individual
/// results through the accessor methods.
// occt-ref: ShapeAnalysis_Wire
#[derive(Clone, Debug)]
pub struct WireAnalysis {
    /// Ordered list of edge identifiers that make up the wire.
    // occt-ref: ShapeAnalysis_Wire // holds a Handle(ShapeExtend_WireData)
    pub wire: Vec<String>,
    /// Defects discovered during the last call to [`perform`](WireAnalysis::perform).
    pub errors: Vec<WireError>,
    /// Spatial tolerance used when evaluating connectivity and gap checks.
    // occt-ref: ShapeAnalysis_Wire // ::SetPrecision / Precision::Confusion()
    pub precision: f64,
    /// Whether [`perform`](WireAnalysis::perform) has been called at least once
    /// and the wire contained at least one edge.
    loaded: bool,
}

impl WireAnalysis {
    /// Construct a new analyser for `wire` using the given `prec` tolerance.
    ///
    /// [`perform`](WireAnalysis::perform) must be called before any check
    /// results are available.
    // occt-ref: ShapeAnalysis_Wire // ::Init(const Handle(ShapeExtend_WireData)&, …, prec)
    pub fn new(wire: Vec<String>, prec: f64) -> Self {
        Self {
            wire,
            errors: Vec::new(),
            precision: prec,
            loaded: false,
        }
    }

    /// Replace the current wire with `wire` and clear any previously recorded
    /// errors.  Resets the *loaded* flag so that [`perform`](WireAnalysis::perform)
    /// must be called again.
    // occt-ref: ShapeAnalysis_Wire // ::Load(const Handle(ShapeExtend_WireData)&)
    pub fn load(&mut self, wire: Vec<String>) {
        self.wire = wire;
        self.errors.clear();
        self.loaded = false;
    }

    /// Run all built-in checks and populate [`errors`](WireAnalysis::errors).
    ///
    /// Returns `true` when the wire is non-empty and passes all checks (i.e.
    /// no errors were recorded), `false` otherwise.
    ///
    /// Checks performed (stub logic):
    /// * [`check_order`](WireAnalysis::check_order) — connectivity between
    ///   consecutive edges.
    /// * Degenerate-edge detection — an edge whose identifier is empty is
    ///   treated as a [`WireError::BadEdge`].
    // occt-ref: ShapeAnalysis_Wire // ::Perform() — runs CheckOrder, CheckConnected,
    //       CheckSmall, CheckSelfIntersection, CheckGaps3d, etc.
    pub fn perform(&mut self) -> bool {
        self.errors.clear();

        if self.wire.is_empty() {
            self.loaded = false;
            return false;
        }

        self.loaded = true;

        // Degenerate-edge check: any empty identifier is a BadEdge.
        // occt-ref: ShapeAnalysis_Wire // ::CheckSmall / CheckDegenerated
        for edge_id in &self.wire {
            if edge_id.is_empty() {
                if !self.errors.contains(&WireError::BadEdge) {
                    self.errors.push(WireError::BadEdge);
                }
            }
        }

        // Connectivity / ordering check.
        // occt-ref: ShapeAnalysis_Wire // ::CheckOrder / CheckConnected
        if !self.check_order() {
            if !self.errors.contains(&WireError::NotConnected) {
                self.errors.push(WireError::NotConnected);
            }
        }

        self.errors.is_empty()
    }

    /// Check that each consecutive pair of edges in the wire shares a logical
    /// connection.
    ///
    /// In this stub, two adjacent edges are considered connected when the
    /// identifier of the second edge follows the first lexicographically or
    /// they share a common prefix up to a `-` separator (e.g. `"e1-end"` /
    /// `"e2-start"`).  Real geometry-aware implementations would compare
    /// end-point coordinates within [`precision`](WireAnalysis::precision).
    ///
    /// Returns `true` when the wire has fewer than two edges (trivially ordered)
    /// or when every adjacent pair passes the heuristic.
    // occt-ref: ShapeAnalysis_Wire // ::CheckOrder() / ShapeAnalysis_WireOrder::Perform()
    pub fn check_order(&self) -> bool {
        if self.wire.len() < 2 {
            return true;
        }
        // Stub heuristic: a wire whose identifiers are already sorted is
        // considered ordered.  Production code would walk actual geometry.
        let mut sorted = self.wire.clone();
        sorted.sort();
        sorted == self.wire
    }

    /// Number of edges in the current wire.
    // occt: ShapeExtend_WireData // ::NbEdges()
    pub fn nb_edges(&self) -> usize {
        self.wire.len()
    }

    /// Number of errors recorded during the last [`perform`](WireAnalysis::perform)
    /// call.  Returns `0` when [`perform`](WireAnalysis::perform) has not yet
    /// been called.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WireFixer
// ─────────────────────────────────────────────────────────────────────────────

/// Applies corrective operations to a wire (ordered sequence of edge
/// identifiers).
///
/// Mirrors the public interface of `ShapeFix_Wire`: construct with a wire,
/// then call individual `fix_*` methods to repair specific classes of defect.
/// Retrieve the (possibly modified) wire via [`wire`](WireFixer::wire).
// occt-ref: ShapeFix_Wire
#[derive(Clone, Debug)]
pub struct WireFixer {
    /// Ordered list of edge identifiers that make up the wire.
    // occt-ref: ShapeFix_Wire // holds a Handle(ShapeExtend_WireData)
    pub wire: Vec<String>,
    /// Spatial tolerance used for gap and small-edge detection.
    // occt-ref: ShapeFix_Wire // ::SetPrecision / Precision::Confusion()
    pub precision: f64,
}

impl WireFixer {
    /// Construct a new fixer for `wire`.
    ///
    /// A default precision of `1e-7` (matching `Precision::Confusion()` in
    /// OCCT) is applied; call the builder pattern or mutate [`precision`]
    /// directly to override.
    // occt-ref: ShapeFix_Wire // ::Init(const Handle(ShapeExtend_WireData)&, …, prec)
    pub fn new(wire: Vec<String>) -> Self {
        Self {
            wire,
            precision: 1e-7,
        }
    }

    /// Sort edge identifiers so that the wire is in a canonical lexicographic
    /// order.
    ///
    /// Returns `true` when the order was actually changed, `false` when the
    /// wire was already sorted or contained fewer than two edges.
    ///
    /// In a geometry-aware implementation this would walk the actual end-point
    /// graph and reconnect edges in the unique traversal order found by
    /// `ShapeAnalysis_WireOrder`.
    // occt-ref: ShapeFix_Wire // ::FixReorder() — calls ShapeAnalysis_WireOrder::Perform
    //       then reorders edges in ShapeExtend_WireData
    pub fn fix_reorder(&mut self) -> bool {
        if self.wire.len() < 2 {
            return false;
        }
        let mut sorted = self.wire.clone();
        sorted.sort();
        if sorted == self.wire {
            false
        } else {
            self.wire = sorted;
            true
        }
    }

    /// Remove edges whose identifier length (used here as a proxy for geometric
    /// size) is below `tol`.
    ///
    /// Returns `true` when at least one edge was removed.
    ///
    /// In a geometry-aware implementation, edge *length* would be computed from
    /// the underlying `BRep_Tool::EdgeLength` and compared against `tol`.
    // occt-ref: ShapeFix_Wire // ::FixSmall(Standard_Boolean lockvtx, Standard_Real prec)
    //       removes degenerate / too-short edges
    pub fn fix_small(&mut self, tol: f64) -> bool {
        let before = self.wire.len();
        // Stub: treat identifier byte-length as a stand-in for geometric length.
        self.wire.retain(|id| id.len() as f64 >= tol);
        self.wire.len() < before
    }

    /// Attempt to close gaps between consecutive edge identifiers by inserting
    /// synthetic bridge edges whose identifier encodes the gap they span.
    ///
    /// Returns `true` when at least one gap was bridged.
    ///
    /// In a geometry-aware implementation, gaps would be detected by comparing
    /// end-point coordinates within [`precision`](WireFixer::precision) and
    /// bridged by `BRepBuilderAPI_MakeEdge` / `ShapeFix_Wire::FixGaps3d`.
    // occt-ref: ShapeFix_Wire // ::FixGaps3d() / FixGaps2d()
    pub fn fix_gaps(&mut self) -> bool {
        if self.wire.len() < 2 {
            return false;
        }

        let mut result: Vec<String> = Vec::with_capacity(self.wire.len());
        let mut bridged = false;

        for (i, edge_id) in self.wire.iter().enumerate() {
            result.push(edge_id.clone());
            if i + 1 < self.wire.len() {
                let next = &self.wire[i + 1];
                // Stub gap heuristic: if identifiers are non-adjacent (differ
                // by more than one byte in their last character) insert a bridge
                // edge.  Real code would compare geometry end-points.
                let cur_last = edge_id.as_bytes().last().copied().unwrap_or(0);
                let nxt_first = next.as_bytes().first().copied().unwrap_or(0);
                if cur_last.wrapping_add(1) != nxt_first {
                    let bridge = format!("gap-bridge-{}-{}", edge_id, next);
                    result.push(bridge);
                    bridged = true;
                }
            }
        }

        if bridged {
            self.wire = result;
        }
        bridged
    }

    /// Return a clone of the current (possibly fixed) wire.
    // occt-ref: ShapeFix_Wire // ::Wire() — returns the Handle(ShapeExtend_WireData)
    pub fn wire(&self) -> Vec<String> {
        self.wire.clone()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── WireAnalysis ────────────────────────────────────────────────────────

    #[test]
    fn analysis_empty_wire_returns_false() {
        let mut wa = WireAnalysis::new(vec![], 1e-7);
        assert!(!wa.perform());
        assert_eq!(wa.nb_edges(), 0);
        assert_eq!(wa.error_count(), 0);
    }

    #[test]
    fn analysis_sorted_wire_no_errors() {
        let mut wa = WireAnalysis::new(
            vec!["a".into(), "b".into(), "c".into()],
            1e-7,
        );
        assert!(wa.perform(), "sorted wire should pass all checks");
        assert_eq!(wa.error_count(), 0);
    }

    #[test]
    fn analysis_unsorted_wire_reports_not_connected() {
        let mut wa = WireAnalysis::new(
            vec!["c".into(), "a".into(), "b".into()],
            1e-7,
        );
        assert!(!wa.perform());
        assert!(wa.errors.contains(&WireError::NotConnected));
    }

    #[test]
    fn analysis_bad_edge_detected() {
        let mut wa = WireAnalysis::new(
            vec!["a".into(), "".into(), "b".into()],
            1e-7,
        );
        wa.perform();
        assert!(wa.errors.contains(&WireError::BadEdge));
    }

    #[test]
    fn analysis_load_resets_state() {
        let mut wa = WireAnalysis::new(vec!["b".into(), "a".into()], 1e-7);
        wa.perform();
        assert!(wa.error_count() > 0);

        wa.load(vec!["a".into(), "b".into()]);
        // errors cleared before perform
        assert_eq!(wa.error_count(), 0);
        assert!(wa.perform());
    }

    #[test]
    fn analysis_nb_edges_and_check_order() {
        let wa = WireAnalysis::new(vec!["x".into(), "y".into()], 1e-7);
        assert_eq!(wa.nb_edges(), 2);
        assert!(wa.check_order());
    }

    // ── WireFixer ───────────────────────────────────────────────────────────

    #[test]
    fn fixer_new_defaults() {
        let wf = WireFixer::new(vec!["e1".into(), "e2".into()]);
        assert_eq!(wf.wire.len(), 2);
        assert!((wf.precision - 1e-7).abs() < 1e-15);
    }

    #[test]
    fn fixer_fix_reorder_sorts_edges() {
        let mut wf = WireFixer::new(vec!["c".into(), "a".into(), "b".into()]);
        assert!(wf.fix_reorder());
        assert_eq!(wf.wire(), vec!["a", "b", "c"]);
    }

    #[test]
    fn fixer_fix_reorder_already_sorted_returns_false() {
        let mut wf = WireFixer::new(vec!["a".into(), "b".into()]);
        assert!(!wf.fix_reorder());
    }

    #[test]
    fn fixer_fix_small_removes_short_ids() {
        let mut wf = WireFixer::new(vec!["a".into(), "bb".into(), "ccc".into()]);
        // tol = 2.0 → "a" (len 1) should be removed
        assert!(wf.fix_small(2.0));
        assert_eq!(wf.wire(), vec!["bb", "ccc"]);
    }

    #[test]
    fn fixer_fix_small_nothing_removed() {
        let mut wf = WireFixer::new(vec!["abc".into(), "def".into()]);
        assert!(!wf.fix_small(1.0));
        assert_eq!(wf.wire().len(), 2);
    }

    #[test]
    fn fixer_fix_gaps_inserts_bridge() {
        // 'a' ends with b'a'; 'c' starts with b'c'; 'a'+1 = 'b' != 'c' → gap
        let mut wf = WireFixer::new(vec!["a".into(), "c".into()]);
        assert!(wf.fix_gaps());
        let result = wf.wire();
        assert_eq!(result.len(), 3);
        assert!(result[1].starts_with("gap-bridge-"));
    }

    #[test]
    fn fixer_wire_returns_clone() {
        let wf = WireFixer::new(vec!["e1".into()]);
        let w1 = wf.wire();
        let w2 = wf.wire();
        assert_eq!(w1, w2);
    }

    #[test]
    fn wire_error_variants_eq() {
        assert_eq!(WireError::NotConnected, WireError::NotConnected);
        assert_ne!(WireError::Gap, WireError::BadEdge);
    }
}
