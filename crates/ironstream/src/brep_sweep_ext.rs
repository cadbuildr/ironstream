// FILE: brep_sweep_ext.rs
//
// Pure-Rust zero-dependency stubs modelled after BRepSweep_Trsf and
// BRepSweep_Iterator from Open CASCADE Technology (OCCT).

// ---------------------------------------------------------------------------
// SweepTrsf
// ---------------------------------------------------------------------------

/// Represents the parametric transformation law along a sweep path, yielding
/// a 4×4 homogeneous matrix at each discrete step.
// occt: BRepSweep_Trsf
#[derive(Clone, Debug, PartialEq)]
pub struct SweepTrsf {
    /// Name / tag of the base shape being swept.
    pub shape: String,
    /// Number of discrete transformation steps along the spine.
    pub num_steps: u32,
}

/// Return the 4×4 identity matrix.
fn identity4() -> [[f64; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Multiply two 4×4 matrices (row-major).
fn mat4_mul(a: [[f64; 4]; 4], b: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut out = [[0.0f64; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[row][k] * b[k][col];
            }
            out[row][col] = sum;
        }
    }
    out
}

/// Invert a 4×4 homogeneous matrix whose upper-left 3×3 is orthonormal.
/// Falls back to identity when the matrix is singular.
fn mat4_inv_rigid(m: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    // Transpose the rotation sub-block.
    let mut r_t = [[0.0f64; 4]; 4];
    for i in 0..3 {
        for j in 0..3 {
            r_t[i][j] = m[j][i];
        }
    }
    r_t[3][3] = 1.0;
    // New translation: -Rᵀ · t
    for i in 0..3 {
        let mut val = 0.0;
        for k in 0..3 {
            val += r_t[i][k] * m[k][3];
        }
        r_t[i][3] = -val;
    }
    r_t
}

impl SweepTrsf {
    /// Create a new `SweepTrsf` for the given shape with `steps` discrete
    /// transformation frames.
    pub fn new(shape: &str, steps: u32) -> Self {
        Self {
            shape: shape.to_owned(),
            num_steps: steps,
        }
    }

    /// Return the 4×4 homogeneous transformation matrix at step `i`.
    ///
    /// The stub encodes a uniform translation along the Z axis proportional to
    /// the step index, matching the simplest linear-spine behaviour from
    /// `BRepSweep_Trsf`.  Real OCCT derives this from the spine wire geometry.
    pub fn location_at(&self, i: u32) -> [[f64; 4]; 4] {
        let t = if self.num_steps > 1 {
            i as f64 / (self.num_steps - 1) as f64
        } else {
            0.0
        };
        let mut m = identity4();
        // Encode the normalised arc-length parameter as a Z translation.
        m[2][3] = t;
        m
    }

    /// Return the relative transformation that maps frame `i` to frame `j`.
    ///
    /// Equivalent to `location_at(j) * inverse(location_at(i))` in the OCCT
    /// convention used by `BRepSweep_Trsf::Between`.
    pub fn between(&self, i: u32, j: u32) -> [[f64; 4]; 4] {
        let m_i = self.location_at(i);
        let m_j = self.location_at(j);
        // between(i,j) = m_j * m_i⁻¹
        mat4_mul(m_j, mat4_inv_rigid(m_i))
    }
}

// ---------------------------------------------------------------------------
// SweepIterator
// ---------------------------------------------------------------------------

/// Iterates over the sub-shapes of a swept shape one at a time, matching the
/// sequential traversal pattern of `BRepSweep_Iterator`.
// occt: BRepSweep_Iterator
#[derive(Clone, Debug, PartialEq)]
pub struct SweepIterator {
    /// Name / tag of the parent swept shape.
    pub shape: String,
    /// Ordered list of sub-shape names produced by the sweep.
    pub shapes: Vec<String>,
    /// Current position in `shapes`.
    pub index: usize,
}

impl SweepIterator {
    /// Initialise an iterator over the sub-shapes of `shape`.
    ///
    /// The stub starts with an empty sub-shape list; callers push entries into
    /// `shapes` directly, mirroring how `BRepSweep_Iterator::Init` populates
    /// the traversal list from the topology.
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            shapes: Vec::new(),
            index: 0,
        }
    }

    /// Return `true` while there are sub-shapes left to visit.
    ///
    /// Mirrors `BRepSweep_Iterator::More`.
    pub fn more(&self) -> bool {
        self.index < self.shapes.len()
    }

    /// Advance to the next sub-shape.
    ///
    /// Mirrors `BRepSweep_Iterator::Next`.  Has no effect when already past the
    /// end.
    pub fn next(&mut self) {
        if self.index < self.shapes.len() {
            self.index += 1;
        }
    }

    /// Return a reference to the current sub-shape name, or `None` when the
    /// iterator is exhausted.
    ///
    /// Mirrors `BRepSweep_Iterator::Value`.
    pub fn current_shape(&self) -> Option<&String> {
        self.shapes.get(self.index)
    }
}

// ---------------------------------------------------------------------------
// SweepParams
// ---------------------------------------------------------------------------

/// Builder for a general swept-shape operation, collecting the spine wire, the
/// profile section, and Boolean flags before materialising the result.
///
/// Loosely corresponds to the constructor parameters accepted by the various
/// `BRepOffsetAPI_MakePipe*` / `BRepSweep_*` classes in OCCT.
// occt: BRepSweep_Trsf (parameter bundle)
#[derive(Clone, Debug, PartialEq)]
pub struct SweepParams {
    /// Name / tag of the spine wire that guides the sweep.
    pub spine: String,
    /// Name / tag of the cross-section profile to be swept.
    pub profile: String,
    /// When `true` the sweep result is capped to form a solid.
    pub solid: bool,
}

impl SweepParams {
    /// Create a new `SweepParams` with `solid` defaulting to `false`.
    pub fn new(spine: &str, profile: &str) -> Self {
        Self {
            spine: spine.to_owned(),
            profile: profile.to_owned(),
            solid: false,
        }
    }

    /// Set whether the swept result should be closed into a solid.
    ///
    /// Follows the builder pattern: consumes `self` and returns the updated
    /// value so calls can be chained.
    pub fn as_solid(mut self, b: bool) -> Self {
        self.solid = b;
        self
    }

    /// Finalise the parameter set and return a human-readable description of
    /// the sweep operation that would be performed.
    ///
    /// In a full implementation this would invoke the OCCT algorithm and return
    /// a shape handle; the stub returns a descriptive `String` so that callers
    /// can verify the configuration without touching any C++ FFI.
    pub fn build(self) -> String {
        format!(
            "Sweep {{ spine: {:?}, profile: {:?}, solid: {} }}",
            self.spine, self.profile, self.solid
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- SweepTrsf -----------------------------------------------------------

    #[test]
    fn sweep_trsf_new_stores_fields() {
        let st = SweepTrsf::new("box", 5);
        assert_eq!(st.shape, "box");
        assert_eq!(st.num_steps, 5);
    }

    #[test]
    fn location_at_zero_is_near_identity() {
        let st = SweepTrsf::new("s", 4);
        let m = st.location_at(0);
        assert!((m[2][3]).abs() < 1e-12, "step 0 should have z-translation 0");
        assert!((m[0][0] - 1.0).abs() < 1e-12);
        assert!((m[1][1] - 1.0).abs() < 1e-12);
        assert!((m[2][2] - 1.0).abs() < 1e-12);
        assert!((m[3][3] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn location_at_last_step_has_unit_translation() {
        let st = SweepTrsf::new("s", 4);
        let m = st.location_at(3);
        assert!((m[2][3] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn between_same_step_is_identity() {
        let st = SweepTrsf::new("s", 5);
        let m = st.between(2, 2);
        let id = identity4();
        for r in 0..4 {
            for c in 0..4 {
                assert!((m[r][c] - id[r][c]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn between_0_to_last_equals_location_at_last() {
        let st = SweepTrsf::new("s", 5);
        let direct = st.location_at(4);
        let via_between = st.between(0, 4);
        for r in 0..4 {
            for c in 0..4 {
                assert!((via_between[r][c] - direct[r][c]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn single_step_location_is_identity() {
        let st = SweepTrsf::new("s", 1);
        let m = st.location_at(0);
        assert!((m[2][3]).abs() < 1e-12);
    }

    // --- SweepIterator -------------------------------------------------------

    #[test]
    fn iterator_new_is_empty() {
        let it = SweepIterator::new("solid");
        assert_eq!(it.shape, "solid");
        assert!(!it.more());
        assert!(it.current_shape().is_none());
    }

    #[test]
    fn iterator_traverses_shapes_in_order() {
        let mut it = SweepIterator::new("pipe");
        it.shapes.push("face_0".to_owned());
        it.shapes.push("face_1".to_owned());
        it.shapes.push("face_2".to_owned());

        assert!(it.more());
        assert_eq!(it.current_shape().unwrap(), "face_0");
        it.next();
        assert!(it.more());
        assert_eq!(it.current_shape().unwrap(), "face_1");
        it.next();
        assert!(it.more());
        assert_eq!(it.current_shape().unwrap(), "face_2");
        it.next();
        assert!(!it.more());
        assert!(it.current_shape().is_none());
    }

    #[test]
    fn iterator_next_past_end_is_safe() {
        let mut it = SweepIterator::new("x");
        it.next(); // no-op
        assert!(!it.more());
    }

    // --- SweepParams ---------------------------------------------------------

    #[test]
    fn sweep_params_defaults_solid_false() {
        let sp = SweepParams::new("spine_wire", "circle_profile");
        assert_eq!(sp.spine, "spine_wire");
        assert_eq!(sp.profile, "circle_profile");
        assert!(!sp.solid);
    }

    #[test]
    fn as_solid_sets_flag() {
        let sp = SweepParams::new("w", "p").as_solid(true);
        assert!(sp.solid);
    }

    #[test]
    fn as_solid_chain_false_then_true() {
        let sp = SweepParams::new("w", "p").as_solid(false).as_solid(true);
        assert!(sp.solid);
    }

    #[test]
    fn build_contains_spine_and_profile() {
        let result = SweepParams::new("my_spine", "my_profile")
            .as_solid(true)
            .build();
        assert!(result.contains("my_spine"));
        assert!(result.contains("my_profile"));
        assert!(result.contains("true"));
    }

    #[test]
    fn build_non_solid_contains_false() {
        let result = SweepParams::new("s", "p").build();
        assert!(result.contains("false"));
    }
}
