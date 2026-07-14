// FILE: crates/ironstream/src/topo_modify.rs
//! Zero-dependency stub for topological shape modification and transformation.
//!
//! Models the public interfaces of `BRepTools_Modifier` and
//! `BRepTools_TrsfModification` from OCCT.  Shapes are represented as opaque
//! string tokens (the same convention used throughout the IronStream stubs).
//! Transformation matrices are stored as row-major 4×4 `f64` arrays so that
//! affine operations (translation, uniform scale, arbitrary rotation) can all
//! be expressed uniformly.  No external crates are used — only `std`.

// ─────────────────────────────────────────────────────────────────────────────
// ShapeModification
// ─────────────────────────────────────────────────────────────────────────────

/// A 4×4 row-major homogeneous transformation matrix applied to a shape.
///
/// The matrix transforms column vectors on the right:
///
/// ```text
///   [x']   [m00 m01 m02 m03] [x]
///   [y'] = [m10 m11 m12 m13] [y]
///   [z']   [m20 m21 m22 m23] [z]
///   [1 ]   [m30 m31 m32 m33] [1]
/// ```
///
/// For all well-formed affine transformations the bottom row is `[0, 0, 0, 1]`.
// occt: BRepTools_TrsfModification
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeModification {
    /// Row-major 4×4 homogeneous matrix.  `matrix[row][col]`.
    pub matrix: [[f64; 4]; 4],
}

impl ShapeModification {
    /// Return the 4×4 identity transformation (no-op).
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::ShapeModification;
    /// let m = ShapeModification::identity();
    /// assert_eq!(m.apply_to_point([1.0, 2.0, 3.0]), [1.0, 2.0, 3.0]);
    /// ```
    // occt: BRepTools_TrsfModification // — identity trsf
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Build a pure translation by the vector `v = [dx, dy, dz]`.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::ShapeModification;
    /// let m = ShapeModification::from_translation([1.0, 2.0, 3.0]);
    /// assert_eq!(m.apply_to_point([0.0, 0.0, 0.0]), [1.0, 2.0, 3.0]);
    /// ```
    // occt: BRepTools_TrsfModification // — SetTranslation
    pub fn from_translation(v: [f64; 3]) -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, v[0]],
                [0.0, 1.0, 0.0, v[1]],
                [0.0, 0.0, 1.0, v[2]],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Build a uniform scale transformation about the origin by factor `s`.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::ShapeModification;
    /// let m = ShapeModification::from_scale(2.0);
    /// assert_eq!(m.apply_to_point([1.0, 1.0, 1.0]), [2.0, 2.0, 2.0]);
    /// ```
    // occt: BRepTools_TrsfModification // — SetScale
    pub fn from_scale(s: f64) -> Self {
        Self {
            matrix: [
                [s,   0.0, 0.0, 0.0],
                [0.0, s,   0.0, 0.0],
                [0.0, 0.0, s,   0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Apply this transformation to a 3-D point and return the result.
    ///
    /// The point is treated as a homogeneous column vector `[x, y, z, 1]ᵀ`.
    /// The returned value is the transformed `[x', y', z']` after the
    /// perspective divide (division by the `w` component).
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::ShapeModification;
    /// let m = ShapeModification::from_translation([5.0, 0.0, 0.0]);
    /// assert_eq!(m.apply_to_point([1.0, 2.0, 3.0]), [6.0, 2.0, 3.0]);
    /// ```
    // occt: BRepTools_TrsfModification // — transforms a gp_Pnt
    pub fn apply_to_point(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.matrix;
        let x = p[0];
        let y = p[1];
        let z = p[2];

        let rx = m[0][0] * x + m[0][1] * y + m[0][2] * z + m[0][3];
        let ry = m[1][0] * x + m[1][1] * y + m[1][2] * z + m[1][3];
        let rz = m[2][0] * x + m[2][1] * y + m[2][2] * z + m[2][3];
        let rw = m[3][0] * x + m[3][1] * y + m[3][2] * z + m[3][3];

        // Perspective divide — for standard affine transforms rw == 1.0.
        if rw == 0.0 || rw == 1.0 {
            [rx, ry, rz]
        } else {
            [rx / rw, ry / rw, rz / rw]
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TopoModifier
// ─────────────────────────────────────────────────────────────────────────────

/// Applies a [`ShapeModification`] to a topological shape token and produces
/// a new modified shape token.
///
/// The workflow mirrors `BRepTools_Modifier`:
///
/// 1. Create via [`TopoModifier::new`] with the input shape token.
/// 2. Optionally supply a modification via [`TopoModifier::with_modification`].
/// 3. Call [`TopoModifier::perform`] to produce the output shape token.
/// 4. Check [`TopoModifier::is_done`] to confirm success.
// occt-ref: BRepTools_Modifier
#[derive(Debug, Clone)]
pub struct TopoModifier {
    /// The input shape token (opaque string representation of a TopoDS_Shape).
    pub shape: String,
    /// The transformation to apply.
    pub modification: ShapeModification,
    /// Set to `true` after a successful call to [`perform`](Self::perform).
    done: bool,
    /// The output shape token produced by the last [`perform`](Self::perform).
    result: Option<String>,
}

impl TopoModifier {
    /// Create a new modifier for the given shape token.
    ///
    /// The modifier starts with an identity transformation and is not yet done.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::TopoModifier;
    /// let modifier = TopoModifier::new("solid:box");
    /// assert!(!modifier.is_done());
    /// ```
    // occt-ref: BRepTools_Modifier // ::BRepTools_Modifier(const TopoDS_Shape&)
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_string(),
            modification: ShapeModification::identity(),
            done: false,
            result: None,
        }
    }

    /// Replace the current transformation and return `self` for chaining.
    ///
    /// Calling this method resets the `done` flag so that [`perform`](Self::perform)
    /// must be called again to produce a fresh result.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::{ShapeModification, TopoModifier};
    /// let modifier = TopoModifier::new("solid:box")
    ///     .with_modification(ShapeModification::from_scale(2.0));
    /// assert!(!modifier.is_done());
    /// ```
    // occt-ref: BRepTools_Modifier // ::Init — sets a new BRepTools_Modification
    pub fn with_modification(mut self, m: ShapeModification) -> Self {
        self.modification = m;
        self.done = false;
        self.result = None;
        self
    }

    /// Execute the modification and return the resulting shape token.
    ///
    /// The method encodes the transformation matrix into the token so that the
    /// result is deterministic and can be round-tripped through other stubs.
    /// After a successful call [`is_done`](Self::is_done) returns `true`.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::{ShapeModification, TopoModifier};
    /// let result = TopoModifier::new("solid:box")
    ///     .with_modification(ShapeModification::from_scale(3.0))
    ///     .perform();
    /// assert!(result.starts_with("modified:"));
    /// ```
    // occt-ref: BRepTools_Modifier // ::Perform
    pub fn perform(&mut self) -> String {
        let token = format_modified_token(&self.shape, &self.modification);
        self.result = Some(token.clone());
        self.done = true;
        token
    }

    /// Returns `true` if [`perform`](Self::perform) has been called successfully.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_modify::TopoModifier;
    /// let mut modifier = TopoModifier::new("solid:sphere");
    /// assert!(!modifier.is_done());
    /// modifier.perform();
    /// assert!(modifier.is_done());
    /// ```
    // occt-ref: BRepTools_Modifier // ::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions
// ─────────────────────────────────────────────────────────────────────────────

/// Apply a [`ShapeModification`] to `shape` and return the new shape token.
///
/// This is a convenience wrapper around [`TopoModifier`] for one-shot use.
///
/// # Example
/// ```
/// # use ironstream::topo_modify::{ShapeModification, transform_shape};
/// let m = ShapeModification::from_translation([1.0, 0.0, 0.0]);
/// let out = transform_shape("solid:box", &m);
/// assert!(out.starts_with("modified:"));
/// ```
// occt-ref: BRepTools_Modifier // ::Perform (convenience form)
pub fn transform_shape(shape: &str, m: &ShapeModification) -> String {
    format_modified_token(shape, m)
}

/// Return a shallow copy of the given shape token.
///
/// In OCCT a copy duplicates the underlying `TopoDS_Shape` while sharing
/// underlying geometry.  Here we encode the copy relationship in the token so
/// that the origin is traceable.
///
/// # Example
/// ```
/// # use ironstream::topo_modify::copy_shape;
/// let original = "solid:box";
/// let copy = copy_shape(original);
/// assert_eq!(copy, "copy:solid:box");
/// ```
// occt-ref: BRepTools_Modifier // — shape copy via BRepTools_TrsfModification identity
pub fn copy_shape(shape: &str) -> String {
    format!("copy:{}", shape)
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Produce the canonical token string for a modified shape.
///
/// The matrix is serialised row by row so the token is self-documenting and
/// stable across platforms (no floating-point ambiguity beyond six decimal
/// places).
fn format_modified_token(shape: &str, m: &ShapeModification) -> String {
    // Encode only the non-trivial upper-left 3×4 sub-matrix; the bottom row
    // is always `[0, 0, 0, 1]` for affine transformations.
    let mat = &m.matrix;
    format!(
        "modified:{}|mat=[{:.6},{:.6},{:.6},{:.6};\
{:.6},{:.6},{:.6},{:.6};\
{:.6},{:.6},{:.6},{:.6}]",
        shape,
        mat[0][0], mat[0][1], mat[0][2], mat[0][3],
        mat[1][0], mat[1][1], mat[1][2], mat[1][3],
        mat[2][0], mat[2][1], mat[2][2], mat[2][3],
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ShapeModification tests

    #[test]
    fn identity_leaves_point_unchanged() {
        let m = ShapeModification::identity();
        assert_eq!(m.apply_to_point([3.0, -1.0, 7.5]), [3.0, -1.0, 7.5]);
    }

    #[test]
    fn translation_shifts_point() {
        let m = ShapeModification::from_translation([1.0, 2.0, 3.0]);
        let out = m.apply_to_point([0.0, 0.0, 0.0]);
        assert!((out[0] - 1.0).abs() < 1e-10);
        assert!((out[1] - 2.0).abs() < 1e-10);
        assert!((out[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn translation_accumulates_with_existing_coords() {
        let m = ShapeModification::from_translation([10.0, 0.0, -5.0]);
        let out = m.apply_to_point([1.0, 2.0, 3.0]);
        assert!((out[0] - 11.0).abs() < 1e-10);
        assert!((out[1] - 2.0).abs() < 1e-10);
        assert!((out[2] + 2.0).abs() < 1e-10);
    }

    #[test]
    fn scale_multiplies_all_coordinates() {
        let m = ShapeModification::from_scale(3.0);
        let out = m.apply_to_point([1.0, 2.0, 4.0]);
        assert!((out[0] - 3.0).abs() < 1e-10);
        assert!((out[1] - 6.0).abs() < 1e-10);
        assert!((out[2] - 12.0).abs() < 1e-10);
    }

    #[test]
    fn scale_zero_collapses_to_origin() {
        let m = ShapeModification::from_scale(0.0);
        assert_eq!(m.apply_to_point([99.0, -7.0, 3.14]), [0.0, 0.0, 0.0]);
    }

    // TopoModifier tests

    #[test]
    fn new_modifier_is_not_done() {
        let modifier = TopoModifier::new("solid:box");
        assert!(!modifier.is_done());
    }

    #[test]
    fn perform_sets_done_flag() {
        let mut modifier = TopoModifier::new("solid:sphere");
        modifier.perform();
        assert!(modifier.is_done());
    }

    #[test]
    fn perform_returns_modified_token() {
        let mut modifier = TopoModifier::new("solid:cone");
        let result = modifier.perform();
        assert!(result.starts_with("modified:solid:cone|"));
    }

    #[test]
    fn with_modification_resets_done() {
        let mut modifier = TopoModifier::new("solid:box");
        modifier.perform();
        assert!(modifier.is_done());
        let modifier = modifier.with_modification(ShapeModification::from_scale(2.0));
        assert!(!modifier.is_done());
    }

    #[test]
    fn chained_builder_pattern() {
        let result = TopoModifier::new("solid:box")
            .with_modification(ShapeModification::from_translation([1.0, 2.0, 3.0]))
            .perform();
        assert!(result.contains("modified:solid:box"));
    }

    // Free function tests

    #[test]
    fn transform_shape_produces_modified_token() {
        let m = ShapeModification::identity();
        let out = transform_shape("solid:box", &m);
        assert!(out.starts_with("modified:solid:box|"));
    }

    #[test]
    fn copy_shape_prepends_copy_prefix() {
        assert_eq!(copy_shape("solid:box"), "copy:solid:box");
        assert_eq!(copy_shape("edge:[v0,v1]"), "copy:edge:[v0,v1]");
    }

    #[test]
    fn transform_and_modifier_agree() {
        let m = ShapeModification::from_scale(2.0);
        let shape = "solid:cylinder";
        let via_fn = transform_shape(shape, &m);
        let via_modifier = TopoModifier::new(shape)
            .with_modification(m)
            .perform();
        assert_eq!(via_fn, via_modifier);
    }
}
