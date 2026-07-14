// FILE: rust/ironstream/crates/ironstream/src/geom_thickness.rs
// occt-ref: BRepOffsetAPI_MakeThickSolid // — shell-with-thickness stubs.
//
// Zero-dependency Rust stubs that mirror the public API shape of the
// OpenCascade `BRepOffsetAPI_MakeThickSolid` family.  No geometry is
// computed; the structs exist so that downstream code can be written and
// compiled against a stable interface before a real implementation is
// provided.

// ─────────────────────────────────────────────────────────────────────────────
// ThickSolid
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for a thick solid obtained by removing selected faces from a shape
/// and offsetting the remaining shell by a signed distance.
///
/// Models `BRepOffsetAPI_MakeThickSolid(shape, facesToRemove, offset, tol)`.
// occt-ref: BRepOffsetAPI_MakeThickSolid
#[derive(Clone, Debug)]
pub struct ThickSolid {
    /// The input solid shape label.
    pub shape: String,
    /// Labels of the faces to be removed before offsetting.
    pub faces_to_remove: Vec<String>,
    /// Signed offset distance (positive = outward, negative = inward).
    pub offset: f64,
    /// Linear tolerance used during offset intersection computations.
    pub tolerance: f64,
}

impl ThickSolid {
    /// Creates a new `ThickSolid` builder for the given shape and offset
    /// distance.  The default tolerance is `1e-3`.
    ///
    /// # Arguments
    ///
    /// * `shape`  — label of the input solid shape.
    /// * `offset` — signed offset distance.
    pub fn new(shape: &str, offset: f64) -> Self {
        Self {
            shape: shape.to_string(),
            faces_to_remove: Vec::new(),
            offset,
            tolerance: 1e-3,
        }
    }

    /// Registers a face to be removed from the solid before the offset shell
    /// is constructed.
    ///
    /// # Arguments
    ///
    /// * `face` — label of the face to remove.
    pub fn add_face_to_remove(&mut self, face: &str) {
        self.faces_to_remove.push(face.to_string());
    }

    /// Returns a new builder with the linear tolerance set to `tol`.
    ///
    /// Consumes `self` so that it can be used in a builder chain.
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Executes the thick-solid algorithm and returns a label for the
    /// resulting shape.
    ///
    /// In this stub, the result encodes the key parameters in a
    /// deterministic, human-readable string.
    pub fn build(&self) -> String {
        format!(
            "ThickSolid(shape={}, offset={}, tol={}, removed=[{}])",
            self.shape,
            self.offset,
            self.tolerance,
            self.faces_to_remove.join(", ")
        )
    }

    /// Returns `true` when the build has a valid result.
    ///
    /// In the stub this is `true` whenever the shape label is non-empty and
    /// the offset is finite and non-zero.
    pub fn is_done(&self) -> bool {
        !self.shape.is_empty() && self.offset != 0.0 && self.offset.is_finite()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ShellThickness
// ─────────────────────────────────────────────────────────────────────────────

/// Lightweight wrapper that applies a uniform thickness to an open shell,
/// producing a closed solid by offsetting the shell by `thickness`.
///
/// Corresponds to the simpler single-shell variant of the OCCT thick-solid
/// workflow where no individual face removal is required.
// occt-ref: BRepOffsetAPI_MakeThickSolid // (shell variant)
#[derive(Clone, Debug)]
pub struct ShellThickness {
    /// Label of the input open shell.
    pub shell: String,
    /// Wall thickness (must be positive).
    pub thickness: f64,
}

impl ShellThickness {
    /// Creates a new `ShellThickness` for the given shell label and wall
    /// thickness.
    ///
    /// # Arguments
    ///
    /// * `shell`     — label of the input open shell.
    /// * `thickness` — wall thickness (positive value).
    pub fn new(shell: &str, thickness: f64) -> Self {
        Self {
            shell: shell.to_string(),
            thickness,
        }
    }

    /// Executes the shell-thickening algorithm and returns a label for the
    /// resulting solid.
    ///
    /// In this stub the result encodes the key parameters in a
    /// deterministic, human-readable string.
    pub fn build(&self) -> String {
        format!(
            "ShellThickness(shell={}, thickness={})",
            self.shell, self.thickness
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free function
// ─────────────────────────────────────────────────────────────────────────────

/// Convenience function that constructs a thick solid from a shape, a slice
/// of face labels to remove, and an offset distance.
///
/// Equivalent to creating a [`ThickSolid`], registering each face, and
/// calling [`ThickSolid::build`].
///
/// # Arguments
///
/// * `shape`  — label of the input solid shape.
/// * `faces`  — labels of the faces to remove before offsetting.
/// * `offset` — signed offset distance.
///
/// # Returns
///
/// A label string for the resulting thick solid.
pub fn make_thick_solid(shape: &str, faces: &[&str], offset: f64) -> String {
    let mut builder = ThickSolid::new(shape, offset);
    for face in faces {
        builder.add_face_to_remove(face);
    }
    builder.build()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thick_solid_new_defaults() {
        let ts = ThickSolid::new("box_1", 2.5);
        assert_eq!(ts.shape, "box_1");
        assert_eq!(ts.offset, 2.5);
        assert_eq!(ts.tolerance, 1e-3);
        assert!(ts.faces_to_remove.is_empty());
    }

    #[test]
    fn thick_solid_add_face_to_remove() {
        let mut ts = ThickSolid::new("box_1", 2.5);
        ts.add_face_to_remove("face_top");
        ts.add_face_to_remove("face_front");
        assert_eq!(ts.faces_to_remove.len(), 2);
        assert_eq!(ts.faces_to_remove[0], "face_top");
        assert_eq!(ts.faces_to_remove[1], "face_front");
    }

    #[test]
    fn thick_solid_with_tolerance() {
        let ts = ThickSolid::new("box_1", 1.0).with_tolerance(1e-6);
        assert_eq!(ts.tolerance, 1e-6);
    }

    #[test]
    fn thick_solid_build_contains_key_info() {
        let mut ts = ThickSolid::new("shape_A", 3.0);
        ts.add_face_to_remove("f1");
        let result = ts.build();
        assert!(result.contains("shape_A"));
        assert!(result.contains('3'));
        assert!(result.contains("f1"));
    }

    #[test]
    fn thick_solid_is_done_valid() {
        let ts = ThickSolid::new("s", 1.0);
        assert!(ts.is_done());
    }

    #[test]
    fn thick_solid_is_done_zero_offset() {
        let ts = ThickSolid::new("s", 0.0);
        assert!(!ts.is_done());
    }

    #[test]
    fn thick_solid_is_done_empty_shape() {
        let ts = ThickSolid::new("", 1.0);
        assert!(!ts.is_done());
    }

    #[test]
    fn shell_thickness_new() {
        let st = ShellThickness::new("shell_1", 0.5);
        assert_eq!(st.shell, "shell_1");
        assert_eq!(st.thickness, 0.5);
    }

    #[test]
    fn shell_thickness_build() {
        let st = ShellThickness::new("shell_1", 0.5);
        let result = st.build();
        assert!(result.contains("shell_1"));
        assert!(result.contains("0.5"));
    }

    #[test]
    fn make_thick_solid_free_fn() {
        let result = make_thick_solid("body", &["top", "bottom"], -1.5);
        assert!(result.contains("body"));
        assert!(result.contains("top"));
        assert!(result.contains("bottom"));
        assert!(result.contains("-1.5"));
    }

    #[test]
    fn make_thick_solid_empty_faces() {
        let result = make_thick_solid("body", &[], 2.0);
        assert!(result.contains("body"));
        assert!(result.contains("2"));
    }
}
