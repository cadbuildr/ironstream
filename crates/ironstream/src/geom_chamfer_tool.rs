// FILE: geom_chamfer_tool.rs

// occt: BRepFilletAPI_MakeChamfer

/// Parameters describing a single chamfer operation on an edge.
///
/// Mirrors the two-distance (asymmetric) and single-distance (symmetric) modes
/// supported by BRepFilletAPI_MakeChamfer in OCCT.
#[derive(Clone, Debug, PartialEq)]
pub struct ChamferParams {
    pub d1: f64,
    pub d2: f64,
    pub symmetric: bool,
}

impl ChamferParams {
    /// Create a symmetric chamfer where both distances are equal.
    // occt: BRepFilletAPI_MakeChamfer::Add(dist, edge)
    pub fn symmetric(d: f64) -> Self {
        Self {
            d1: d,
            d2: d,
            symmetric: true,
        }
    }

    /// Create an asymmetric chamfer with two distinct distances.
    // occt: BRepFilletAPI_MakeChamfer::Add(d1, d2, edge, face)
    pub fn asymmetric(d1: f64, d2: f64) -> Self {
        Self {
            d1,
            d2,
            symmetric: false,
        }
    }
}

/// High-level chamfer builder for BRep solids.
///
/// Collects edge identifiers with their chamfer distances, then produces a
/// result shape string via `build()`.  The internal `chamfers` vec stores
/// tuples of `(edge_id, d1, d2)`.
// occt: BRepFilletAPI_MakeChamfer
pub struct ChamferTool {
    pub shape: String,
    pub chamfers: Vec<(String, f64, f64)>,
    done: bool,
    faults: usize,
}

impl ChamferTool {
    /// Initialise a new builder for the given input shape.
    // occt: BRepFilletAPI_MakeChamfer::BRepFilletAPI_MakeChamfer(shape)
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            chamfers: Vec::new(),
            done: false,
            faults: 0,
        }
    }

    /// Register an asymmetric chamfer on `edge` with distances `d1` and `d2`.
    // occt: BRepFilletAPI_MakeChamfer::Add(d1, d2, edge, face)
    pub fn add_edge(&mut self, edge: &str, d1: f64, d2: f64) {
        self.chamfers.push((edge.to_owned(), d1, d2));
    }

    /// Register a symmetric chamfer on `edge` (both distances equal `d`).
    // occt: BRepFilletAPI_MakeChamfer::Add(dist, edge)
    pub fn add_symmetric(&mut self, edge: &str, d: f64) {
        self.chamfers.push((edge.to_owned(), d, d));
    }

    /// Execute the chamfer computation and return the resulting shape string.
    ///
    /// In the stub implementation the result is a deterministic string that
    /// encodes the input shape and all registered chamfers.  A real
    /// implementation would call the underlying OCCT algorithm here.
    // occt: BRepFilletAPI_MakeChamfer::Build / Shape
    pub fn build(&mut self) -> String {
        if self.chamfers.is_empty() {
            self.done = false;
            self.faults = 1;
            return format!("chamfer_failed({})", self.shape);
        }

        self.faults = 0;
        self.done = true;

        let entries: Vec<String> = self
            .chamfers
            .iter()
            .map(|(edge, d1, d2)| {
                if (d1 - d2).abs() < f64::EPSILON {
                    format!("{}:{}", edge, d1)
                } else {
                    format!("{}:{}/{}", edge, d1, d2)
                }
            })
            .collect();

        format!("chamfer({}, [{}])", self.shape, entries.join(", "))
    }

    /// Returns `true` after a successful call to `build()`.
    // occt: BRepFilletAPI_MakeChamfer::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of contours for which the computation failed.
    // occt: BRepFilletAPI_MakeChamfer::NbFaultyContours
    pub fn nb_faults(&self) -> usize {
        self.faults
    }
}

/// Convenience function: apply a single symmetric chamfer of distance `d`
/// to `edge` on `shape` and return the result shape string.
// occt: BRepFilletAPI_MakeChamfer one-shot helper pattern
pub fn chamfer_edge(shape: &str, edge: &str, d: f64) -> String {
    let mut tool = ChamferTool::new(shape);
    tool.add_symmetric(edge, d);
    tool.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chamfer_tool_new_not_done() {
        let t = ChamferTool::new("box");
        assert!(!t.is_done());
        assert_eq!(t.nb_faults(), 0);
        assert!(t.chamfers.is_empty());
    }

    #[test]
    fn add_edge_populates_chamfers() {
        let mut t = ChamferTool::new("box");
        t.add_edge("e1", 1.0, 2.0);
        assert_eq!(t.chamfers.len(), 1);
        assert_eq!(t.chamfers[0], ("e1".to_owned(), 1.0, 2.0));
    }

    #[test]
    fn add_symmetric_sets_equal_distances() {
        let mut t = ChamferTool::new("box");
        t.add_symmetric("e2", 3.0);
        assert_eq!(t.chamfers[0], ("e2".to_owned(), 3.0, 3.0));
    }

    #[test]
    fn build_empty_not_done_with_fault() {
        let mut t = ChamferTool::new("box");
        let result = t.build();
        assert!(!t.is_done());
        assert_eq!(t.nb_faults(), 1);
        assert!(result.contains("chamfer_failed"));
    }

    #[test]
    fn build_symmetric_is_done() {
        let mut t = ChamferTool::new("box");
        t.add_symmetric("e1", 1.5);
        let result = t.build();
        assert!(t.is_done());
        assert_eq!(t.nb_faults(), 0);
        assert!(result.contains("box"));
        assert!(result.contains("e1"));
        assert!(result.contains("1.5"));
    }

    #[test]
    fn build_asymmetric_encodes_both_distances() {
        let mut t = ChamferTool::new("solid");
        t.add_edge("e3", 1.0, 2.5);
        let result = t.build();
        assert!(result.contains("e3"));
        assert!(result.contains("1"));
        assert!(result.contains("2.5"));
    }

    #[test]
    fn build_multiple_edges() {
        let mut t = ChamferTool::new("part");
        t.add_symmetric("e1", 0.5);
        t.add_edge("e2", 1.0, 2.0);
        let result = t.build();
        assert!(t.is_done());
        assert!(result.contains("e1"));
        assert!(result.contains("e2"));
    }

    #[test]
    fn chamfer_edge_convenience() {
        let result = chamfer_edge("box", "top_edge", 0.75);
        assert!(result.contains("box"));
        assert!(result.contains("top_edge"));
        assert!(result.contains("0.75"));
    }

    #[test]
    fn chamfer_params_symmetric() {
        let p = ChamferParams::symmetric(2.0);
        assert!(p.symmetric);
        assert_eq!(p.d1, 2.0);
        assert_eq!(p.d2, 2.0);
    }

    #[test]
    fn chamfer_params_asymmetric() {
        let p = ChamferParams::asymmetric(1.0, 3.0);
        assert!(!p.symmetric);
        assert_eq!(p.d1, 1.0);
        assert_eq!(p.d2, 3.0);
    }
}
