// FILE: rust/ironstream/crates/ironstream/src/topo_wire_builder.rs
//! Zero-dependency stub for topological wire construction.
//!
//! Models the public interface of `BRepBuilderAPI_MakeWire` /
//! `BRepLib_MakeWire` from OCCT.  Edges are represented as application-supplied
//! string identifiers; no geometric data is carried.

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder
// ─────────────────────────────────────────────────────────────────────────────

/// Incremental builder for an ordered sequence of named edges.
///
/// Mirrors the public interface of `BRepBuilderAPI_MakeWire`: edges are added
/// one at a time, the wire can be explicitly closed, and a done-flag reports
/// whether at least one edge has been accumulated.
// occt: BRepBuilderAPI_MakeWire
pub struct WireBuilder {
    /// Ordered edge identifiers accumulated so far.
    pub edges: Vec<String>,
    /// Whether [`close`](WireBuilder::close) has been called.
    pub closed: bool,
}

impl WireBuilder {
    /// Create an empty, open builder.
    // occt-note: BRepBuilderAPI_MakeWire()
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            closed: false,
        }
    }

    /// Append edge `e` to the wire.
    // occt: BRepBuilderAPI_MakeWire // ::Add(const TopoDS_Edge&)
    pub fn add_edge(&mut self, e: &str) {
        self.edges.push(e.to_string());
    }

    /// Mark the wire as closed.
    ///
    /// In OCCT a wire is closed when the last edge's end vertex coincides with
    /// the first edge's start vertex.  Here we simply set the `closed` flag
    /// since no geometry is tracked.
    // occt-ref: BRep_Tool // ::IsClosed(const TopoDS_Wire&)
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Return `true` when at least one edge has been added.
    // occt: BRepBuilderAPI_Command // ::IsDone()
    pub fn is_done(&self) -> bool {
        !self.edges.is_empty()
    }

    /// Return a clone of the accumulated edge list (the "wire").
    // occt: BRepBuilderAPI_MakeWire // ::Wire() — returns TopoDS_Wire
    pub fn wire(&self) -> Vec<String> {
        self.edges.clone()
    }
}

impl Default for WireBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WireAnalysis
// ─────────────────────────────────────────────────────────────────────────────

/// Post-build analysis of a wire.
///
/// Mirrors `BRepCheck_Wire` / `ShapeAnalysis_Wire` from OCCT.  In this stub
/// edges are strings; "free" edges are those that appear an odd number of times
/// in the wire (i.e. they are not shared by a second occurrence that would
/// conceptually close the loop).
// occt-ref: ShapeAnalysis_Wire // / BRepCheck_Wire
pub struct WireAnalysis {
    /// The wire being analysed (ordered edge identifiers).
    pub wire: Vec<String>,
    /// Whether the wire has been determined to be closed by [`analyze`](WireAnalysis::analyze).
    pub is_closed: bool,
    /// Number of free (unmatched) edges discovered by [`analyze`](WireAnalysis::analyze).
    pub num_free_edges: usize,
}

impl WireAnalysis {
    /// Construct a new analysis object for the given `wire`.
    ///
    /// [`analyze`](WireAnalysis::analyze) must be called to populate
    /// `is_closed` and `num_free_edges`.
    // occt-ref: ShapeAnalysis_Wire // ::Load(const TopoDS_Wire&)
    pub fn new(wire: Vec<String>) -> Self {
        Self {
            wire,
            is_closed: false,
            num_free_edges: 0,
        }
    }

    /// Analyse the wire and update `is_closed` and `num_free_edges`.
    ///
    /// An edge is considered "free" when it appears an odd number of times in
    /// the wire (odd-occurrence edges have no partner to close them).  The wire
    /// is reported as closed when there are no free edges and the wire is
    /// non-empty.
    ///
    /// Returns `true` when the wire is valid (non-empty and closed).
    // occt-ref: ShapeAnalysis_Wire // ::Perform() / BRepCheck_Wire::Closed()
    pub fn analyze(&mut self) -> bool {
        use std::collections::HashMap;

        // Count occurrences of each edge label.
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for e in &self.wire {
            *counts.entry(e.as_str()).or_insert(0) += 1;
        }

        // Free edges appear an odd number of times.
        self.num_free_edges = counts.values().filter(|&&n| n % 2 != 0).count();
        self.is_closed = !self.wire.is_empty() && self.num_free_edges == 0;

        self.is_closed
    }

    /// Return the subset of edges that appear an odd number of times.
    ///
    /// Preserves the order of first appearance in the wire.
    // occt-ref: ShapeAnalysis_Wire // ::NbFreeEdges() / CheckFreeEdge()
    pub fn free_edges(&self) -> Vec<String> {
        use std::collections::HashMap;

        let mut counts: HashMap<&str, usize> = HashMap::new();
        for e in &self.wire {
            *counts.entry(e.as_str()).or_insert(0) += 1;
        }

        // Collect free edges in first-appearance order, deduplicating.
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for e in &self.wire {
            if counts[e.as_str()] % 2 != 0 && seen.insert(e.as_str()) {
                result.push(e.clone());
            }
        }
        result
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions
// ─────────────────────────────────────────────────────────────────────────────

/// Convenience constructor: create a [`WireBuilder`] pre-loaded with `edges`.
///
/// Equivalent to calling [`WireBuilder::new`] and then [`WireBuilder::add_edge`]
/// for every element of `edges`.
// occt-note: BRepBuilderAPI_MakeWire(const TopTools_ListOfShape&)
pub fn wire_from_edges(edges: &[&str]) -> WireBuilder {
    let mut wb = WireBuilder::new();
    for e in edges {
        wb.add_edge(e);
    }
    wb
}

/// Partition `edges` into maximally connected chains.
///
/// Two consecutive edges in the input are placed in the same chain when they
/// share the same label prefix (characters before the first `'-'`) — a
/// simplified stand-in for the topological vertex-sharing test performed by
/// `BRepLib_MakeWire::Connect`.  Each chain is returned as a `Vec<String>`.
///
/// Edges that share no prefix with their predecessor start a new chain.
// occt-ref: BRepLib_MakeWire // — connects edges that share vertices
pub fn connect_edges(edges: &[String]) -> Vec<Vec<String>> {
    if edges.is_empty() {
        return Vec::new();
    }

    // Helper: extract the connectivity key (prefix before first '-', or the
    // whole label when no '-' is present).
    fn key(e: &str) -> &str {
        match e.find('-') {
            Some(pos) => &e[..pos],
            None => e,
        }
    }

    let mut chains: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = vec![edges[0].clone()];

    for pair in edges.windows(2) {
        let prev_key = key(&pair[0]);
        let next_key = key(&pair[1]);
        if prev_key == next_key {
            // Same "vertex group" — extend the current chain.
            current.push(pair[1].clone());
        } else {
            // Different group — flush the current chain and start a new one.
            chains.push(current);
            current = vec![pair[1].clone()];
        }
    }
    chains.push(current);
    chains
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // WireBuilder ─────────────────────────────────────────────────────────────

    #[test]
    fn new_builder_is_empty_and_open() {
        let wb = WireBuilder::new();
        assert!(wb.edges.is_empty());
        assert!(!wb.closed);
        assert!(!wb.is_done());
    }

    #[test]
    fn add_edge_appends_and_marks_done() {
        let mut wb = WireBuilder::new();
        wb.add_edge("e1");
        assert_eq!(wb.edges, vec!["e1"]);
        assert!(wb.is_done());
    }

    #[test]
    fn close_sets_flag() {
        let mut wb = WireBuilder::new();
        wb.add_edge("e1");
        assert!(!wb.closed);
        wb.close();
        assert!(wb.closed);
    }

    #[test]
    fn wire_returns_clone_of_edges() {
        let mut wb = WireBuilder::new();
        wb.add_edge("a");
        wb.add_edge("b");
        assert_eq!(wb.wire(), vec!["a", "b"]);
        // Original is not consumed.
        assert_eq!(wb.edges.len(), 2);
    }

    #[test]
    fn default_equals_new() {
        let wb: WireBuilder = Default::default();
        assert!(wb.edges.is_empty());
        assert!(!wb.closed);
    }

    // WireAnalysis ────────────────────────────────────────────────────────────

    #[test]
    fn analyze_empty_wire_is_not_closed() {
        let mut wa = WireAnalysis::new(vec![]);
        assert!(!wa.analyze());
        assert!(!wa.is_closed);
        assert_eq!(wa.num_free_edges, 0);
    }

    #[test]
    fn analyze_single_edge_has_one_free_edge() {
        let mut wa = WireAnalysis::new(vec!["e1".to_string()]);
        assert!(!wa.analyze());
        assert!(!wa.is_closed);
        assert_eq!(wa.num_free_edges, 1);
    }

    #[test]
    fn analyze_paired_edges_is_closed() {
        // Each distinct edge appears twice — no free edges.
        let mut wa = WireAnalysis::new(vec![
            "e1".to_string(),
            "e2".to_string(),
            "e1".to_string(),
            "e2".to_string(),
        ]);
        assert!(wa.analyze());
        assert!(wa.is_closed);
        assert_eq!(wa.num_free_edges, 0);
    }

    #[test]
    fn free_edges_returns_odd_occurrence_labels() {
        let wa = WireAnalysis::new(vec![
            "a".to_string(),
            "b".to_string(),
            "a".to_string(),
        ]);
        // "a" appears twice (even) — not free.  "b" appears once (odd) — free.
        let free = wa.free_edges();
        assert_eq!(free, vec!["b"]);
    }

    #[test]
    fn free_edges_empty_for_closed_wire() {
        let wa = WireAnalysis::new(vec!["x".to_string(), "x".to_string()]);
        assert!(wa.free_edges().is_empty());
    }

    // wire_from_edges ─────────────────────────────────────────────────────────

    #[test]
    fn wire_from_edges_builds_correct_sequence() {
        let wb = wire_from_edges(&["p", "q", "r"]);
        assert_eq!(wb.wire(), vec!["p", "q", "r"]);
        assert!(wb.is_done());
        assert!(!wb.closed);
    }

    #[test]
    fn wire_from_edges_empty_slice() {
        let wb = wire_from_edges(&[]);
        assert!(!wb.is_done());
    }

    // connect_edges ───────────────────────────────────────────────────────────

    #[test]
    fn connect_edges_empty_input() {
        let chains = connect_edges(&[]);
        assert!(chains.is_empty());
    }

    #[test]
    fn connect_edges_all_same_prefix() {
        let edges: Vec<String> = ["a-1", "a-2", "a-3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let chains = connect_edges(&edges);
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0], edges);
    }

    #[test]
    fn connect_edges_different_prefixes_split() {
        let edges: Vec<String> = ["a-1", "b-1", "b-2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let chains = connect_edges(&edges);
        assert_eq!(chains.len(), 2);
        assert_eq!(chains[0], vec!["a-1"]);
        assert_eq!(chains[1], vec!["b-1", "b-2"]);
    }

    #[test]
    fn connect_edges_no_dash_uses_full_label() {
        let edges: Vec<String> = ["foo", "foo", "bar"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let chains = connect_edges(&edges);
        assert_eq!(chains.len(), 2);
        assert_eq!(chains[0], vec!["foo", "foo"]);
        assert_eq!(chains[1], vec!["bar"]);
    }
}
