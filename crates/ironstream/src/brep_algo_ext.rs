// FILE: src/brep_algo_ext.rs
//
// Pure-Rust zero-dependency stub mirroring
//   BRepAlgoAPI_BuilderAlgo  and  BRepAlgo_FaceRestrictor
// from OpenCASCADE Technology (OCCT).

// occt: BRepAlgoAPI_BuilderAlgo
/// General-purpose Boolean builder algorithm.
///
/// Mirrors `BRepAlgoAPI_BuilderAlgo`: accepts argument shapes and optional
/// tool shapes, runs a build step, then exposes the result shapes.
pub struct BuilderAlgo {
    pub shapes: Vec<String>,
    pub tools: Vec<String>,
    pub result: Vec<String>,
    done: bool,
}

impl BuilderAlgo {
    /// Create a new, empty `BuilderAlgo`.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            tools: Vec::new(),
            result: Vec::new(),
            done: false,
        }
    }

    /// Set the argument (input) shapes, replacing any previously set.
    pub fn set_arguments(&mut self, shapes: Vec<String>) {
        self.shapes = shapes;
        self.done = false;
    }

    /// Set the tool shapes, replacing any previously set.
    pub fn set_tools(&mut self, tools: Vec<String>) {
        self.tools = tools;
        self.done = false;
    }

    /// Execute the builder algorithm.
    ///
    /// The stub records one result entry per (shape, tool) pair, naming them
    /// `"<shape>_built_by_<tool>"`.  When no tools are registered every
    /// argument shape passes through unchanged.
    pub fn build(&mut self) {
        self.result.clear();
        if self.tools.is_empty() {
            self.result = self.shapes.clone();
        } else {
            for shape in &self.shapes {
                for tool in &self.tools {
                    self.result
                        .push(format!("{}_built_by_{}", shape, tool));
                }
            }
        }
        self.done = true;
    }

    /// Returns `true` after a successful call to [`build`].
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the result shapes computed by the last [`build`] call.
    pub fn shape(&self) -> Vec<String> {
        self.result.clone()
    }
}

impl Default for BuilderAlgo {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgo_FaceRestrictor
/// Restricts a face to a set of bounding wires, producing new faces.
///
/// Mirrors `BRepAlgo_FaceRestrictor`: takes a base face and a collection of
/// closed wires, computes the restricted sub-faces, then provides an iterator
/// interface (`next` / `current`) over the results.
pub struct FaceRestrictor {
    pub face: String,
    pub wires: Vec<String>,
    done: bool,
    results: Vec<String>,
    cursor: usize,
    orient: bool,
}

impl FaceRestrictor {
    /// Create a new, empty `FaceRestrictor`.
    pub fn new() -> Self {
        Self {
            face: String::new(),
            wires: Vec::new(),
            done: false,
            results: Vec::new(),
            cursor: 0,
            orient: false,
        }
    }

    /// Initialise with a base face and an orientation flag.
    ///
    /// Calling `init` resets any previously accumulated wires and results.
    pub fn init(&mut self, face: &str, orient: bool) {
        self.face = face.to_string();
        self.orient = orient;
        self.wires.clear();
        self.results.clear();
        self.cursor = 0;
        self.done = false;
    }

    /// Append a bounding wire to the restrictor.
    pub fn add_wire(&mut self, w: &str) {
        self.wires.push(w.to_string());
    }

    /// Compute the restricted faces.
    ///
    /// Each wire produces one result face named `"<face>_restricted_by_<wire>"`
    /// (with `"_oriented"` appended when `orient` is `true`).  An empty wire
    /// list leaves `results` empty but still marks the operation as done.
    pub fn perform(&mut self) {
        self.results.clear();
        self.cursor = 0;
        let suffix = if self.orient { "_oriented" } else { "" };
        for wire in &self.wires {
            self.results.push(format!(
                "{}_restricted_by_{}{}",
                self.face, wire, suffix
            ));
        }
        self.done = true;
    }

    /// Returns `true` after a successful call to [`perform`].
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Advance the internal cursor to the next result face.
    ///
    /// Has no effect when the cursor has already moved past the last result.
    pub fn next(&mut self) {
        if self.cursor < self.results.len() {
            self.cursor += 1;
        }
    }

    /// Return the result face at the current cursor position.
    ///
    /// Returns an empty string when the cursor is out of range or
    /// [`perform`] has not been called yet.
    pub fn current(&self) -> String {
        if self.cursor < self.results.len() {
            self.results[self.cursor].clone()
        } else {
            String::new()
        }
    }
}

impl Default for FaceRestrictor {
    fn default() -> Self {
        Self::new()
    }
}

/// Split `face` by each wire in `wires` and return the resulting sub-faces.
///
/// Convenience wrapper around [`FaceRestrictor`] that mirrors the pattern used
/// by OCCT callers: initialise, add wires, perform, collect all results.
pub fn split_face(face: &str, wires: &[String]) -> Vec<String> {
    let mut restrictor = FaceRestrictor::new();
    restrictor.init(face, false);
    for w in wires {
        restrictor.add_wire(w);
    }
    restrictor.perform();
    if !restrictor.is_done() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(restrictor.results.len());
    while restrictor.cursor < restrictor.results.len() {
        out.push(restrictor.current());
        restrictor.next();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- BuilderAlgo ---

    #[test]
    fn builder_algo_new_not_done() {
        let b = BuilderAlgo::new();
        assert!(!b.is_done());
        assert!(b.shapes.is_empty());
        assert!(b.tools.is_empty());
        assert!(b.result.is_empty());
    }

    #[test]
    fn builder_algo_build_no_tools_passthrough() {
        let mut b = BuilderAlgo::new();
        b.set_arguments(vec!["face1".to_string(), "face2".to_string()]);
        b.build();
        assert!(b.is_done());
        let s = b.shape();
        assert_eq!(s, vec!["face1".to_string(), "face2".to_string()]);
    }

    #[test]
    fn builder_algo_build_with_tools_cross_product() {
        let mut b = BuilderAlgo::new();
        b.set_arguments(vec!["s1".to_string()]);
        b.set_tools(vec!["t1".to_string(), "t2".to_string()]);
        b.build();
        assert!(b.is_done());
        let s = b.shape();
        assert_eq!(s.len(), 2);
        assert!(s.contains(&"s1_built_by_t1".to_string()));
        assert!(s.contains(&"s1_built_by_t2".to_string()));
    }

    #[test]
    fn builder_algo_set_arguments_resets_done() {
        let mut b = BuilderAlgo::new();
        b.set_arguments(vec!["a".to_string()]);
        b.build();
        assert!(b.is_done());
        b.set_arguments(vec!["b".to_string()]);
        assert!(!b.is_done());
    }

    #[test]
    fn builder_algo_empty_build() {
        let mut b = BuilderAlgo::new();
        b.build();
        assert!(b.is_done());
        assert!(b.shape().is_empty());
    }

    // --- FaceRestrictor ---

    #[test]
    fn face_restrictor_new_defaults() {
        let r = FaceRestrictor::new();
        assert!(!r.is_done());
        assert!(r.face.is_empty());
        assert!(r.wires.is_empty());
        assert_eq!(r.current(), "");
    }

    #[test]
    fn face_restrictor_perform_no_wires() {
        let mut r = FaceRestrictor::new();
        r.init("myface", false);
        r.perform();
        assert!(r.is_done());
        assert_eq!(r.current(), "");
    }

    #[test]
    fn face_restrictor_perform_single_wire() {
        let mut r = FaceRestrictor::new();
        r.init("f1", false);
        r.add_wire("w1");
        r.perform();
        assert!(r.is_done());
        assert_eq!(r.current(), "f1_restricted_by_w1");
        r.next();
        assert_eq!(r.current(), "");
    }

    #[test]
    fn face_restrictor_perform_orient_flag() {
        let mut r = FaceRestrictor::new();
        r.init("f1", true);
        r.add_wire("w1");
        r.perform();
        assert_eq!(r.current(), "f1_restricted_by_w1_oriented");
    }

    #[test]
    fn face_restrictor_next_iterates_all() {
        let mut r = FaceRestrictor::new();
        r.init("base", false);
        r.add_wire("w1");
        r.add_wire("w2");
        r.add_wire("w3");
        r.perform();
        let mut collected = Vec::new();
        while r.cursor < r.results.len() {
            collected.push(r.current());
            r.next();
        }
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], "base_restricted_by_w1");
        assert_eq!(collected[1], "base_restricted_by_w2");
        assert_eq!(collected[2], "base_restricted_by_w3");
    }

    #[test]
    fn face_restrictor_init_clears_previous_state() {
        let mut r = FaceRestrictor::new();
        r.init("old", false);
        r.add_wire("old_wire");
        r.perform();
        r.init("new", true);
        assert!(!r.is_done());
        assert!(r.wires.is_empty());
        assert_eq!(r.face, "new");
    }

    // --- split_face ---

    #[test]
    fn split_face_no_wires_returns_empty() {
        let result = split_face("f", &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn split_face_returns_one_per_wire() {
        let wires = vec!["w1".to_string(), "w2".to_string()];
        let result = split_face("myface", &wires);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "myface_restricted_by_w1");
        assert_eq!(result[1], "myface_restricted_by_w2");
    }

    #[test]
    fn split_face_single_wire() {
        let wires = vec!["outer".to_string()];
        let result = split_face("planar_face", &wires);
        assert_eq!(result, vec!["planar_face_restricted_by_outer".to_string()]);
    }
}
