// FILE: brep_feat_tool.rs
//! `BRepFeat` tool builders — draft-prism and linear-form feature operations,
//! mirroring OpenCascade's `BRepFeat_MakeDPrism` and
//! `BRepFeat_MakeLinearForm` packages.
//!
//! # Overview
//!
//! * [`DPrism`] — draft-angle prismatic extrusion of a face on a base solid.
//!   Mirrors `BRepFeat_MakeDPrism`.
//!
//! * [`LinearForm`] — sweeps a profile along a linear direction to form a rib
//!   or slot on a base solid.  Mirrors `BRepFeat_MakeLinearForm`.
//!
//! * [`FeatureTool`] — lightweight accumulator that collects named feature
//!   descriptors and serialises them into a single shape tag.
//!
//! # Notes
//!
//! This is a pure-Rust zero-dependency stub.  No external crates are used.
//! Shape values are represented as `String` tags so that higher-level layers
//! can attach real topology without coupling this crate to a kernel.

// ─────────────────────────────────────────────────────────────────────────────
// DPrism  (mirrors BRepFeat_MakeDPrism)
// ─────────────────────────────────────────────────────────────────────────────

/// Draft-angle prismatic extrusion of a face on a base solid.
///
/// A *draft prism* extrudes a planar face in a given direction by `depth`
/// units while applying a taper angle.  This stub stores the inputs as
/// `String` shape tags and synthesises a deterministic result tag on
/// `build()`.
///
/// Mirrors `BRepFeat_MakeDPrism` from OCCT.
// occt: BRepFeat_MakeDPrism
#[derive(Debug, Clone)]
pub struct DPrism {
    /// Tag identifying the base solid.
    pub shape: String,
    /// Tag identifying the face to extrude.
    pub face: String,
    /// Extrusion direction vector `[dx, dy, dz]`.
    pub direction: [f64; 3],
    /// Extrusion depth (signed; negative reverses the direction).
    pub depth: f64,
    /// Cached result tag, populated by `build()`.
    result: Option<String>,
}

impl DPrism {
    /// Construct a new `DPrism` feature.
    ///
    /// * `shape`     — tag of the base solid.
    /// * `face`      — tag of the face to extrude.
    /// * `dir`       — extrusion direction `[dx, dy, dz]`.
    /// * `depth`     — extrusion depth.
    // occt: BRepFeat_MakeDPrism::BRepFeat_MakeDPrism
    pub fn new(shape: &str, face: &str, dir: [f64; 3], depth: f64) -> Self {
        Self {
            shape: shape.to_owned(),
            face: face.to_owned(),
            direction: dir,
            depth,
            result: None,
        }
    }

    /// Build the draft prism and return a result shape tag.
    ///
    /// The tag encodes all inputs so callers can detect no-op builds.
    /// Repeated calls return the same cached tag without recomputing.
    ///
    /// Mirrors `BRepFeat_MakeDPrism::Perform` + `Shape`.
    // occt: BRepFeat_MakeDPrism::Shape
    pub fn build(&mut self) -> String {
        if let Some(ref cached) = self.result {
            return cached.clone();
        }
        let tag = if self.depth.abs() < 1e-12 || self.direction_norm() < 1e-12 {
            // Degenerate: return the base shape unchanged.
            self.shape.clone()
        } else {
            format!(
                "DPrism(shape={},face={},dir=[{:.6},{:.6},{:.6}],depth={:.6})",
                self.shape,
                self.face,
                self.direction[0],
                self.direction[1],
                self.direction[2],
                self.depth,
            )
        };
        self.result = Some(tag.clone());
        tag
    }

    /// Return `true` when `build()` has been called and produced a non-empty,
    /// non-degenerate result.
    ///
    /// Mirrors `BRepFeat_MakeDPrism::IsDone`.
    // occt: BRepFeat_MakeDPrism::IsDone
    pub fn is_done(&mut self) -> bool {
        let result = self.build();
        !result.is_empty() && result != self.shape
    }

    // ── private helpers ───────────────────────────────────────────────────────

    fn direction_norm(&self) -> f64 {
        let [dx, dy, dz] = self.direction;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LinearForm  (mirrors BRepFeat_MakeLinearForm)
// ─────────────────────────────────────────────────────────────────────────────

/// Linear-form (rib / slot) feature builder.
///
/// Sweeps a profile along a direction vector to form a rib or slot on a base
/// solid.  This stub stores the inputs as `String` shape tags and synthesises
/// a deterministic result tag on `build()`.
///
/// Mirrors `BRepFeat_MakeLinearForm` from OCCT.
// occt: BRepFeat_MakeLinearForm
#[derive(Debug, Clone)]
pub struct LinearForm {
    /// Tag identifying the base solid.
    pub shape: String,
    /// Tag identifying the sweep profile (wire or face).
    pub profile: String,
    /// Sweep direction vector `[dx, dy, dz]`.
    pub direction: [f64; 3],
    /// Cached result tag, populated by `build()`.
    result: Option<String>,
}

impl LinearForm {
    /// Construct a new `LinearForm` feature.
    ///
    /// * `shape`   — tag of the base solid.
    /// * `profile` — tag of the wire / face profile to sweep.
    /// * `dir`     — sweep direction `[dx, dy, dz]`.
    // occt: BRepFeat_MakeLinearForm::BRepFeat_MakeLinearForm
    pub fn new(shape: &str, profile: &str, dir: [f64; 3]) -> Self {
        Self {
            shape: shape.to_owned(),
            profile: profile.to_owned(),
            direction: dir,
            result: None,
        }
    }

    /// Build the linear form and return a result shape tag.
    ///
    /// Repeated calls return the same cached tag without recomputing.
    ///
    /// Mirrors `BRepFeat_MakeLinearForm::Perform` + `Shape`.
    // occt: BRepFeat_MakeLinearForm::Shape
    pub fn build(&mut self) -> String {
        if let Some(ref cached) = self.result {
            return cached.clone();
        }
        let tag = if self.direction_norm() < 1e-12 {
            // Degenerate zero-length direction: return base unchanged.
            self.shape.clone()
        } else {
            format!(
                "LinearForm(shape={},profile={},dir=[{:.6},{:.6},{:.6}])",
                self.shape,
                self.profile,
                self.direction[0],
                self.direction[1],
                self.direction[2],
            )
        };
        self.result = Some(tag.clone());
        tag
    }

    /// Return `true` when `build()` has been called and produced a non-empty,
    /// non-degenerate result.
    ///
    /// Mirrors `BRepFeat_MakeLinearForm::IsDone`.
    // occt: BRepFeat_MakeLinearForm::IsDone
    pub fn is_done(&mut self) -> bool {
        let result = self.build();
        !result.is_empty() && result != self.shape
    }

    // ── private helpers ───────────────────────────────────────────────────────

    fn direction_norm(&self) -> f64 {
        let [dx, dy, dz] = self.direction;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FeatureTool
// ─────────────────────────────────────────────────────────────────────────────

/// Lightweight accumulator for named feature descriptors.
///
/// `FeatureTool` collects an ordered list of feature tags (produced by
/// [`DPrism::build`] or [`LinearForm::build`]) and serialises them together
/// with the base shape into a single compound shape tag on `build()`.
///
/// It does not perform boolean operations itself; that responsibility belongs
/// to the individual feature builders.  This type is useful when an ordered
/// history of features must be recorded and replayed.
// occt: BRepFeat_Builder (partial analogue for feature accumulation)
#[derive(Debug, Clone)]
pub struct FeatureTool {
    /// Tag identifying the base solid that features are applied to.
    pub base_shape: String,
    /// Ordered list of feature tags collected via `add_feature`.
    pub features: Vec<String>,
}

impl FeatureTool {
    /// Create a new `FeatureTool` anchored to `base`.
    ///
    /// * `base` — tag of the base solid.
    pub fn new(base: &str) -> Self {
        Self {
            base_shape: base.to_owned(),
            features: Vec::new(),
        }
    }

    /// Append a feature tag to the accumulator.
    ///
    /// * `f` — tag string returned by a feature builder's `build()` method.
    pub fn add_feature(&mut self, f: &str) {
        self.features.push(f.to_owned());
    }

    /// Build a compound shape tag from the base and all accumulated features.
    ///
    /// Features appear in insertion order.  If no features have been added the
    /// tag is identical to `base_shape`.
    pub fn build(&self) -> String {
        if self.features.is_empty() {
            return self.base_shape.clone();
        }
        format!(
            "FeatureTool(base={},features=[{}])",
            self.base_shape,
            self.features.join(";"),
        )
    }

    /// Return the number of features currently accumulated.
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── DPrism ────────────────────────────────────────────────────────────────

    #[test]
    fn dprism_new_stores_fields() {
        let dp = DPrism::new("solid1", "face1", [0.0, 0.0, 1.0], 5.0);
        assert_eq!(dp.shape, "solid1");
        assert_eq!(dp.face, "face1");
        assert_eq!(dp.direction, [0.0, 0.0, 1.0]);
        assert!((dp.depth - 5.0).abs() < 1e-12);
    }

    #[test]
    fn dprism_build_returns_tag() {
        let mut dp = DPrism::new("S", "F", [0.0, 0.0, 1.0], 3.0);
        let tag = dp.build();
        assert!(tag.contains("DPrism"));
        assert!(tag.contains("S"));
        assert!(tag.contains("F"));
    }

    #[test]
    fn dprism_build_is_idempotent() {
        let mut dp = DPrism::new("S", "F", [1.0, 0.0, 0.0], 2.0);
        let first = dp.build();
        let second = dp.build();
        assert_eq!(first, second);
    }

    #[test]
    fn dprism_is_done_true_for_valid_inputs() {
        let mut dp = DPrism::new("solid", "face", [0.0, 1.0, 0.0], 4.0);
        assert!(dp.is_done());
    }

    #[test]
    fn dprism_is_done_false_for_zero_depth() {
        let mut dp = DPrism::new("solid", "face", [0.0, 1.0, 0.0], 0.0);
        assert!(!dp.is_done());
    }

    #[test]
    fn dprism_is_done_false_for_zero_direction() {
        let mut dp = DPrism::new("solid", "face", [0.0, 0.0, 0.0], 5.0);
        assert!(!dp.is_done());
    }

    #[test]
    fn dprism_zero_depth_returns_base_shape() {
        let mut dp = DPrism::new("base_solid", "face", [0.0, 0.0, 1.0], 0.0);
        assert_eq!(dp.build(), "base_solid");
    }

    // ── LinearForm ────────────────────────────────────────────────────────────

    #[test]
    fn linearform_new_stores_fields() {
        let lf = LinearForm::new("solid2", "wire1", [1.0, 0.0, 0.0]);
        assert_eq!(lf.shape, "solid2");
        assert_eq!(lf.profile, "wire1");
        assert_eq!(lf.direction, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn linearform_build_returns_tag() {
        let mut lf = LinearForm::new("S", "P", [0.0, 1.0, 0.0]);
        let tag = lf.build();
        assert!(tag.contains("LinearForm"));
        assert!(tag.contains('S'));
        assert!(tag.contains('P'));
    }

    #[test]
    fn linearform_build_is_idempotent() {
        let mut lf = LinearForm::new("S", "P", [0.0, 0.0, 1.0]);
        let first = lf.build();
        let second = lf.build();
        assert_eq!(first, second);
    }

    #[test]
    fn linearform_is_done_true_for_valid_inputs() {
        let mut lf = LinearForm::new("solid", "profile", [1.0, 0.0, 0.0]);
        assert!(lf.is_done());
    }

    #[test]
    fn linearform_is_done_false_for_zero_direction() {
        let mut lf = LinearForm::new("solid", "profile", [0.0, 0.0, 0.0]);
        assert!(!lf.is_done());
    }

    #[test]
    fn linearform_zero_direction_returns_base_shape() {
        let mut lf = LinearForm::new("base_solid", "wire", [0.0, 0.0, 0.0]);
        assert_eq!(lf.build(), "base_solid");
    }

    // ── FeatureTool ───────────────────────────────────────────────────────────

    #[test]
    fn feature_tool_new_stores_base() {
        let ft = FeatureTool::new("base");
        assert_eq!(ft.base_shape, "base");
        assert_eq!(ft.feature_count(), 0);
    }

    #[test]
    fn feature_tool_add_feature_increments_count() {
        let mut ft = FeatureTool::new("base");
        ft.add_feature("feat1");
        ft.add_feature("feat2");
        assert_eq!(ft.feature_count(), 2);
    }

    #[test]
    fn feature_tool_build_no_features_returns_base() {
        let ft = FeatureTool::new("my_solid");
        assert_eq!(ft.build(), "my_solid");
    }

    #[test]
    fn feature_tool_build_includes_all_features() {
        let mut ft = FeatureTool::new("base");
        ft.add_feature("f1");
        ft.add_feature("f2");
        let result = ft.build();
        assert!(result.contains("base"));
        assert!(result.contains("f1"));
        assert!(result.contains("f2"));
    }

    #[test]
    fn feature_tool_build_is_deterministic() {
        let mut ft = FeatureTool::new("base");
        ft.add_feature("x");
        assert_eq!(ft.build(), ft.build());
    }

    #[test]
    fn feature_tool_round_trip_with_dprism_and_linearform() {
        let mut dp = DPrism::new("solid", "top_face", [0.0, 0.0, 1.0], 10.0);
        let mut lf = LinearForm::new("solid", "rib_wire", [1.0, 0.0, 0.0]);

        let dp_tag = dp.build();
        let lf_tag = lf.build();

        let mut ft = FeatureTool::new("solid");
        ft.add_feature(&dp_tag);
        ft.add_feature(&lf_tag);

        assert_eq!(ft.feature_count(), 2);
        let compound = ft.build();
        assert!(compound.contains("DPrism"));
        assert!(compound.contains("LinearForm"));
    }
}
