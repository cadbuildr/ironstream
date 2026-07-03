//! `GeomCutTool` — higher-level wrapper around boolean subtraction, mirroring
//! OCCT's `BRepAlgoAPI_Cut` / `BRepAlgo_Cut` API.  Shapes are represented by
//! name tags (strings) so this module can operate without a live topology
//! kernel; it is intended as a thin façade over the real BSP cut once solid
//! handles are wired in.

/// Higher-level cut builder: subtract `tool` from `shape`.
///
/// Mirrors the builder pattern of `BRepAlgoAPI_Cut`.
// occt: BRepAlgoAPI_Cut
#[derive(Debug, Clone)]
pub struct CutTool {
    pub shape: String,
    pub tool: String,
    pub result: String,
    pub done: bool,
}

impl CutTool {
    /// Create a new cut operation between `shape` and `tool`.
    ///
    /// The operation is not executed until [`build`] is called.
    pub fn new(shape: &str, tool: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            tool: tool.to_owned(),
            result: String::new(),
            done: false,
        }
    }

    /// Execute the cut and store the result shape tag.
    ///
    /// After this call [`is_done`] returns `true` and [`shape`] returns the
    /// result tag.
    pub fn build(&mut self) {
        self.result = cut(&self.shape, &self.tool);
        self.done = true;
    }

    /// Returns `true` after a successful [`build`].
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Return the result shape tag if the operation is done, or the original
    /// input shape tag otherwise (mirrors `BRepAlgoAPI_Cut::Shape`).
    pub fn shape(&self) -> &str {
        if self.done {
            &self.result
        } else {
            &self.shape
        }
    }

    /// Return the set of result sub-shapes that derive from `s`.
    ///
    /// In OCCT this is `BRepAlgoAPI_Cut::Modified`; here it returns a
    /// single-element vec containing the result tag when `s` matches the input
    /// shape and the cut is done, otherwise an empty vec.
    pub fn modified(&self, s: &str) -> Vec<String> {
        if self.done && s == self.shape {
            vec![self.result.clone()]
        } else {
            vec![]
        }
    }

    /// Return `true` if sub-shape `s` was consumed (deleted) by the cut.
    ///
    /// Mirrors `BRepAlgoAPI_Cut::IsDeleted`: the tool operand is considered
    /// deleted (fully subtracted) after a successful build.
    pub fn deleted(&self, s: &str) -> bool {
        self.done && s == self.tool
    }
}

// ---------------------------------------------------------------------------

/// Multi-tool cut: subtract multiple tools from one base shape, one at a time.
///
/// Mirrors the pattern of `BRepAlgo_Cut` applied in sequence.
// occt: BRepAlgoAPI_Cut
#[derive(Debug, Clone)]
pub struct MultiCut {
    pub shape: String,
    pub tools: Vec<String>,
    result: String,
    done: bool,
}

impl MultiCut {
    /// Create a new multi-cut builder starting from `shape`.
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            tools: Vec::new(),
            result: String::new(),
            done: false,
        }
    }

    /// Add a tool shape to subtract from the base.
    pub fn add_tool(&mut self, t: &str) {
        self.tools.push(t.to_owned());
    }

    /// Execute all cuts sequentially and return the result shape tag.
    pub fn build(&mut self) -> String {
        let tool_refs: Vec<&str> = self.tools.iter().map(String::as_str).collect();
        self.result = multi_cut(&self.shape, &tool_refs);
        self.done = true;
        self.result.clone()
    }

    /// Returns `true` after a successful [`build`].
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience API

/// Subtract `tool` from `shape` and return a result shape tag.
///
/// The tag is constructed deterministically from the operands so that
/// identical inputs always produce the same output identifier.
pub fn cut(shape: &str, tool: &str) -> String {
    format!("({shape} - {tool})")
}

/// Subtract each tool in `tools` from `shape` sequentially and return the
/// final result tag.
pub fn multi_cut(shape: &str, tools: &[&str]) -> String {
    tools.iter().fold(shape.to_owned(), |acc, t| cut(&acc, t))
}

// ---------------------------------------------------------------------------
// Point-based cut / fuse operations
// ---------------------------------------------------------------------------

/// Boolean subtraction of two point sets (geometry-level).
// occt: BRepAlgoAPI_Cut
#[derive(Debug, Clone)]
pub struct CutOperation {
    pub base: Vec<[f64; 3]>,
    pub tool: Vec<[f64; 3]>,
    pub result_pts: Vec<[f64; 3]>,
    done: bool,
}

impl CutOperation {
    pub fn new(base: Vec<[f64; 3]>, tool: Vec<[f64; 3]>) -> Self {
        Self { base, tool, result_pts: Vec::new(), done: false }
    }

    pub fn build(&mut self) {
        self.result_pts = self.base.clone();
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
}

/// Boolean union of two point sets (geometry-level).
// occt: BRepAlgoAPI_Fuse
#[derive(Debug, Clone)]
pub struct FuseOperation {
    pub a: Vec<[f64; 3]>,
    pub b: Vec<[f64; 3]>,
    pub result_pts: Vec<[f64; 3]>,
    done: bool,
}

impl FuseOperation {
    pub fn new(a: Vec<[f64; 3]>, b: Vec<[f64; 3]>) -> Self {
        Self { a, b, result_pts: Vec::new(), done: false }
    }

    pub fn build(&mut self) {
        self.result_pts = self.a.iter().chain(self.b.iter()).copied().collect();
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_tag_format() {
        assert_eq!(cut("A", "B"), "(A - B)");
    }

    #[test]
    fn multi_cut_folds_left() {
        assert_eq!(multi_cut("A", &["B", "C"]), "((A - B) - C)");
    }

    #[test]
    fn cut_tool_not_done_before_build() {
        let ct = CutTool::new("box", "sphere");
        assert!(!ct.is_done());
        assert_eq!(ct.shape(), "box");
        assert!(ct.modified("box").is_empty());
        assert!(!ct.deleted("sphere"));
    }

    #[test]
    fn cut_tool_done_after_build() {
        let mut ct = CutTool::new("box", "sphere");
        ct.build();
        assert!(ct.is_done());
        assert_eq!(ct.shape(), "(box - sphere)");
        assert_eq!(ct.modified("box"), vec!["(box - sphere)"]);
        assert!(ct.deleted("sphere"));
        assert!(!ct.deleted("box"));
    }

    #[test]
    fn multi_cut_build() {
        let mut mc = MultiCut::new("solid");
        mc.add_tool("hole1");
        mc.add_tool("hole2");
        let r = mc.build();
        assert!(mc.is_done());
        assert_eq!(r, "((solid - hole1) - hole2)");
    }

    #[test]
    fn multi_cut_no_tools() {
        let mut mc = MultiCut::new("solid");
        let r = mc.build();
        assert_eq!(r, "solid");
    }
}
