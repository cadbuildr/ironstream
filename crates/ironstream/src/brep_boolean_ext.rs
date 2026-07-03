// FILE: src/brep_boolean_ext.rs

// occt: BRepAlgoAPI_Splitter
pub struct Splitter {
    pub tools: Vec<String>,
    pub shapes: Vec<String>,
}

impl Splitter {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            shapes: Vec::new(),
        }
    }

    pub fn add_tool(&mut self, name: &str) {
        self.tools.push(name.to_string());
    }

    pub fn add_shape(&mut self, name: &str) {
        self.shapes.push(name.to_string());
    }

    /// Produces one result entry per (shape, tool) pair, modelling the split
    /// geometry that BRepAlgoAPI_Splitter would return.
    pub fn build(&self) -> Vec<String> {
        let mut result = Vec::new();
        for shape in &self.shapes {
            for tool in &self.tools {
                result.push(format!("{}_split_by_{}", shape, tool));
            }
        }
        result
    }
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgoAPI_Section
pub struct Section {
    pub shape1: String,
    pub shape2: String,
}

impl Section {
    pub fn new(s1: &str, s2: &str) -> Self {
        Self {
            shape1: s1.to_string(),
            shape2: s2.to_string(),
        }
    }

    /// Returns the intersection curve/wire names that BRepAlgoAPI_Section
    /// would produce between the two shapes.
    pub fn build(&self) -> Vec<String> {
        vec![format!("{}_section_{}", self.shape1, self.shape2)]
    }
}

// Mirrors the four Boolean operations supported by BRepAlgoAPI_BooleanOperation.
pub enum BooleanOp {
    Fuse,
    Common,
    Cut,
    Section,
}

pub struct BooleanResult {
    pub op: BooleanOp,
    pub shapes: Vec<String>,
}

impl BooleanResult {
    pub fn new(op: BooleanOp) -> Self {
        Self {
            op,
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, s: &str) {
        self.shapes.push(s.to_string());
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
}

// occt: BRepAlgoAPI_Section result
pub struct BooleanSectionResult {
    nb_edges: usize,
    nb_vertices: usize,
    is_done: bool,
}

impl BooleanSectionResult {
    pub fn new() -> Self {
        Self {
            nb_edges: 0,
            nb_vertices: 0,
            is_done: false,
        }
    }

    pub fn build(&mut self, nb_edges: usize, nb_vertices: usize) {
        self.nb_edges = nb_edges;
        self.nb_vertices = nb_vertices;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    pub fn nb_vertices(&self) -> usize {
        self.nb_vertices
    }
}

impl Default for BooleanSectionResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgoAPI_Section — compute section between two shapes
pub struct BooleanSection {
    pub approximation: bool,
    pub result: BooleanSectionResult,
}

impl BooleanSection {
    pub fn new(approximation: bool) -> Self {
        Self {
            approximation,
            result: BooleanSectionResult::new(),
        }
    }

    pub fn build(&mut self, nb_edges: usize) {
        self.result.build(nb_edges, nb_edges + 1);
    }

    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    pub fn result(&self) -> &BooleanSectionResult {
        &self.result
    }
}

// occt: BRepAlgoAPI_Splitter result
pub struct BooleanSplitResult {
    nb_parts: usize,
    is_done: bool,
}

impl BooleanSplitResult {
    pub fn new() -> Self {
        Self {
            nb_parts: 0,
            is_done: false,
        }
    }

    pub fn build(&mut self, nb_parts: usize) {
        self.nb_parts = nb_parts;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_parts(&self) -> usize {
        self.nb_parts
    }
}

impl Default for BooleanSplitResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgoAPI_Splitter — splits shapes by tools
pub struct BooleanSplitter {
    pub nb_arguments: usize,
    pub nb_tools: usize,
    pub result: BooleanSplitResult,
}

impl BooleanSplitter {
    pub fn new(nb_arguments: usize, nb_tools: usize) -> Self {
        Self {
            nb_arguments,
            nb_tools,
            result: BooleanSplitResult::new(),
        }
    }

    pub fn build(&mut self) {
        let nb_parts = self.nb_arguments * self.nb_tools;
        self.result.build(nb_parts);
    }

    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    pub fn result(&self) -> &BooleanSplitResult {
        &self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Splitter ---

    #[test]
    fn splitter_new_is_empty() {
        let sp = Splitter::new();
        assert!(sp.tools.is_empty());
        assert!(sp.shapes.is_empty());
    }

    #[test]
    fn splitter_build_empty_returns_empty() {
        let sp = Splitter::new();
        assert!(sp.build().is_empty());
    }

    #[test]
    fn splitter_build_produces_cross_product() {
        let mut sp = Splitter::new();
        sp.add_shape("box");
        sp.add_shape("sphere");
        sp.add_tool("plane");
        let result = sp.build();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"box_split_by_plane".to_string()));
        assert!(result.contains(&"sphere_split_by_plane".to_string()));
    }

    #[test]
    fn splitter_multiple_tools_multiple_shapes() {
        let mut sp = Splitter::new();
        sp.add_shape("s1");
        sp.add_tool("t1");
        sp.add_tool("t2");
        let result = sp.build();
        assert_eq!(result.len(), 2);
    }

    // --- Section ---

    #[test]
    fn section_stores_shapes() {
        let sec = Section::new("shapeA", "shapeB");
        assert_eq!(sec.shape1, "shapeA");
        assert_eq!(sec.shape2, "shapeB");
    }

    #[test]
    fn section_build_returns_one_entry() {
        let sec = Section::new("boxA", "boxB");
        let result = sec.build();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "boxA_section_boxB");
    }

    // --- BooleanResult ---

    #[test]
    fn boolean_result_new_is_empty() {
        let br = BooleanResult::new(BooleanOp::Fuse);
        assert!(br.is_empty());
    }

    #[test]
    fn boolean_result_add_not_empty() {
        let mut br = BooleanResult::new(BooleanOp::Common);
        br.add("part1");
        assert!(!br.is_empty());
        assert_eq!(br.shapes.len(), 1);
    }

    #[test]
    fn boolean_result_multiple_adds() {
        let mut br = BooleanResult::new(BooleanOp::Cut);
        br.add("a");
        br.add("b");
        br.add("c");
        assert_eq!(br.shapes.len(), 3);
    }

    // --- Legacy BooleanSectionResult ---

    #[test]
    fn section_result_new_not_done() {
        let r = BooleanSectionResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_edges(), 0);
        assert_eq!(r.nb_vertices(), 0);
    }

    #[test]
    fn section_result_build_sets_fields() {
        let mut r = BooleanSectionResult::new();
        r.build(5, 6);
        assert!(r.is_done());
        assert_eq!(r.nb_edges(), 5);
        assert_eq!(r.nb_vertices(), 6);
    }

    #[test]
    fn boolean_section_new_not_done() {
        let s = BooleanSection::new(true);
        assert!(s.approximation);
        assert!(!s.is_done());
    }

    #[test]
    fn boolean_section_build_sets_done() {
        let mut s = BooleanSection::new(false);
        s.build(3);
        assert!(s.is_done());
        assert_eq!(s.result().nb_edges(), 3);
        assert_eq!(s.result().nb_vertices(), 4);
    }

    #[test]
    fn boolean_section_result_vertices_is_edges_plus_one() {
        let mut s = BooleanSection::new(true);
        s.build(10);
        assert_eq!(s.result().nb_vertices(), s.result().nb_edges() + 1);
    }

    #[test]
    fn split_result_new_not_done() {
        let r = BooleanSplitResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_parts(), 0);
    }

    #[test]
    fn split_result_build_sets_fields() {
        let mut r = BooleanSplitResult::new();
        r.build(4);
        assert!(r.is_done());
        assert_eq!(r.nb_parts(), 4);
    }

    #[test]
    fn boolean_splitter_new_not_done() {
        let sp = BooleanSplitter::new(2, 3);
        assert_eq!(sp.nb_arguments, 2);
        assert_eq!(sp.nb_tools, 3);
        assert!(!sp.is_done());
    }

    #[test]
    fn boolean_splitter_build_computes_parts() {
        let mut sp = BooleanSplitter::new(3, 4);
        sp.build();
        assert!(sp.is_done());
        assert_eq!(sp.result().nb_parts(), 12);
    }

    #[test]
    fn boolean_splitter_zero_args_zero_parts() {
        let mut sp = BooleanSplitter::new(0, 5);
        sp.build();
        assert!(sp.is_done());
        assert_eq!(sp.result().nb_parts(), 0);
    }
}
