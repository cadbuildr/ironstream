// FILE: rust/ironstream/crates/ironstream/src/geom_fillet_tool.rs

/// High-level fillet tool operating on string-keyed BRep shapes and edges.
///
/// Models the BRepFilletAPI_MakeFillet workflow: construct with a shape, register
/// edges with radii, call build, then query results.
// occt: BRepFilletAPI_MakeFillet
pub struct FilletTool {
    /// The shape to apply fillets to, identified by a string key.
    pub shape: String,
    /// Pairs of (edge id, radius) registered for filleting.
    pub edges: Vec<(String, f64)>,
    /// Whether the last build call succeeded.
    done: bool,
    /// Number of edges that could not be filleted.
    faults: usize,
}

impl FilletTool {
    /// Creates a new FilletTool for the named shape with no edges registered.
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_string(),
            edges: Vec::new(),
            done: false,
            faults: 0,
        }
    }

    /// Registers an edge for filleting with the given radius.
    ///
    /// Edges with a negative radius are silently skipped to mirror the
    /// BRepFilletAPI_MakeFillet stub contract (negative radii are invalid).
    pub fn add_edge(&mut self, edge: &str, radius: f64) {
        if radius >= 0.0 {
            self.edges.push((edge.to_string(), radius));
        }
    }

    /// Runs the fillet computation and returns the resulting shape key.
    ///
    /// In this pure-Rust stub the result key is derived from the source shape
    /// and the number of registered edges. Build succeeds when at least one
    /// edge is registered; otherwise it is marked not done.
    pub fn build(&mut self) -> String {
        if self.edges.is_empty() {
            self.done = false;
            self.faults = 0;
            return String::new();
        }
        self.done = true;
        self.faults = 0;
        format!("{}_filleted_{}", self.shape, self.edges.len())
    }

    /// Returns whether the last [`build`](FilletTool::build) call succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of edges that could not be filleted.
    pub fn nb_faults(&self) -> usize {
        self.faults
    }
}

impl Default for FilletTool {
    fn default() -> Self {
        Self::new("")
    }
}

/// High-level 2D fillet tool operating on string-keyed planar faces and edges.
///
/// Models the BRepFilletAPI_MakeFillet2d workflow: construct with a face,
/// register edges with radii, call build to get filleted edge descriptors.
// occt: BRepFilletAPI_MakeFillet2d
pub struct Fillet2d {
    /// The planar face to apply fillets to, identified by a string key.
    pub face: String,
    /// Pairs of (edge id, radius) registered for filleting.
    pub edges: Vec<(String, f64)>,
    /// Whether the last build call succeeded.
    done: bool,
}

impl Fillet2d {
    /// Creates a new Fillet2d for the named face with no edges registered.
    pub fn new(face: &str) -> Self {
        Self {
            face: face.to_string(),
            edges: Vec::new(),
            done: false,
        }
    }

    /// Registers an edge on the face for 2D filleting with the given radius.
    ///
    /// Edges with a negative radius are silently skipped.
    pub fn add_edge(&mut self, e: &str, radius: f64) {
        if radius >= 0.0 {
            self.edges.push((e.to_string(), radius));
        }
    }

    /// Runs the 2D fillet computation and returns descriptors for each
    /// resulting fillet arc.
    ///
    /// Each descriptor string encodes the source edge and radius in the form
    /// `"fillet2d:<face>/<edge>@<radius>"`, matching the ChFi2d stub approach
    /// used elsewhere in this crate. Returns an empty vec when no edges are
    /// registered, marking the build as not done.
    pub fn build(&mut self) -> Vec<String> {
        if self.edges.is_empty() {
            self.done = false;
            return Vec::new();
        }
        self.done = true;
        self.edges
            .iter()
            .map(|(e, r)| format!("fillet2d:{}/{}@{}", self.face, e, r))
            .collect()
    }

    /// Returns whether the last [`build`](Fillet2d::build) call succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for Fillet2d {
    fn default() -> Self {
        Self::new("")
    }
}

/// Convenience function: apply a single fillet of the given radius to one edge
/// of a shape and return the resulting shape key.
///
/// Equivalent to constructing a [`FilletTool`], registering one edge, and
/// calling [`FilletTool::build`].
pub fn fillet_edge(shape: &str, edge: &str, radius: f64) -> String {
    let mut tool = FilletTool::new(shape);
    tool.add_edge(edge, radius);
    tool.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    // FilletTool tests

    #[test]
    fn fillet_tool_new_is_not_done() {
        let t = FilletTool::new("box1");
        assert!(!t.is_done());
        assert_eq!(t.nb_faults(), 0);
        assert!(t.edges.is_empty());
    }

    #[test]
    fn fillet_tool_add_and_build_succeeds() {
        let mut t = FilletTool::new("box1");
        t.add_edge("e1", 0.5);
        t.add_edge("e2", 1.0);
        let result = t.build();
        assert!(t.is_done());
        assert_eq!(t.nb_faults(), 0);
        assert_eq!(result, "box1_filleted_2");
    }

    #[test]
    fn fillet_tool_build_empty_not_done() {
        let mut t = FilletTool::new("box1");
        let result = t.build();
        assert!(!t.is_done());
        assert!(result.is_empty());
    }

    #[test]
    fn fillet_tool_negative_radius_rejected() {
        let mut t = FilletTool::new("box1");
        t.add_edge("e1", -1.0);
        assert!(t.edges.is_empty());
    }

    #[test]
    fn fillet_tool_zero_radius_accepted() {
        let mut t = FilletTool::new("box1");
        t.add_edge("e1", 0.0);
        assert_eq!(t.edges.len(), 1);
    }

    #[test]
    fn fillet_tool_shape_preserved() {
        let t = FilletTool::new("my_shape");
        assert_eq!(t.shape, "my_shape");
    }

    // Fillet2d tests

    #[test]
    fn fillet2d_new_is_not_done() {
        let f = Fillet2d::new("face1");
        assert!(!f.is_done());
        assert!(f.edges.is_empty());
    }

    #[test]
    fn fillet2d_add_and_build_returns_descriptors() {
        let mut f = Fillet2d::new("face1");
        f.add_edge("e1", 2.0);
        f.add_edge("e2", 3.5);
        let arcs = f.build();
        assert!(f.is_done());
        assert_eq!(arcs.len(), 2);
        assert_eq!(arcs[0], "fillet2d:face1/e1@2");
        assert_eq!(arcs[1], "fillet2d:face1/e2@3.5");
    }

    #[test]
    fn fillet2d_build_empty_not_done() {
        let mut f = Fillet2d::new("face1");
        let arcs = f.build();
        assert!(!f.is_done());
        assert!(arcs.is_empty());
    }

    #[test]
    fn fillet2d_negative_radius_rejected() {
        let mut f = Fillet2d::new("face1");
        f.add_edge("e1", -0.5);
        assert!(f.edges.is_empty());
    }

    #[test]
    fn fillet2d_face_preserved() {
        let f = Fillet2d::new("my_face");
        assert_eq!(f.face, "my_face");
    }

    // fillet_edge convenience function tests

    #[test]
    fn fillet_edge_returns_nonempty_string() {
        let result = fillet_edge("solid1", "top_edge", 1.0);
        assert!(!result.is_empty());
        assert_eq!(result, "solid1_filleted_1");
    }

    #[test]
    fn fillet_edge_zero_radius_succeeds() {
        let result = fillet_edge("s", "e", 0.0);
        assert_eq!(result, "s_filleted_1");
    }

    #[test]
    fn fillet_edge_negative_radius_returns_empty() {
        let result = fillet_edge("s", "e", -1.0);
        assert!(result.is_empty());
    }
}
