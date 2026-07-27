// FILE: brep_mesh_discrete.rs
//! `BRepMesh_Discret` / `IMeshTools_MeshAlgo` — discrete meshing algorithm
//! abstraction layer, mirroring OpenCascade's `BRepMesh_Discret` and
//! `IMeshTools_MeshAlgo` interfaces.
//!
//! # Key types
//!
//! * [`MeshAlgoParams`] — parameters controlling deflection, angular tolerance,
//!   minimum feature size, and parallel execution.
//! * [`DiscretResult`] — outcome of a discrete meshing operation: node/triangle
//!   counts and a status string.
//! * [`MeshDiscret`] — top-level driver that applies a [`MeshAlgoParams`] to a
//!   named shape and returns a [`DiscretResult`].

// ---------------------------------------------------------------------------
// MeshAlgoParams
// ---------------------------------------------------------------------------

/// Parameters for the discrete meshing algorithm.
///
/// Controls chord deflection, angular deflection, minimum element size, and
/// whether meshing may run in parallel.
///
/// Mirrors `IMeshTools_Parameters` / the parameter block accepted by
/// `BRepMesh_Discret` in OCCT.
// occt: IMeshTools_MeshAlgo
#[derive(Clone, Debug)]
pub struct MeshAlgoParams {
    /// Maximum linear (chord) deflection between the mesh and the true surface,
    /// in model units.  Must be positive.
    pub deflection: f64,
    /// Maximum angular deflection in radians between adjacent face normals at
    /// shared triangle edges.  Must be positive.
    pub angle: f64,
    /// Minimum allowed element size as an absolute model-unit length.
    /// Elements smaller than this are not produced.  Must be non-negative.
    pub min_size: f64,
    /// When `true` the mesher is allowed to process faces in parallel threads.
    /// In this pure-Rust stub the flag is honoured in the API but meshing
    /// always executes on a single thread.
    pub in_parallel: bool,
}

impl MeshAlgoParams {
    /// Construct with explicit values for all four parameters.
    ///
    /// Mirrors `IMeshTools_Parameters` field initialisation in OCCT.
    pub fn new(deflection: f64, angle: f64, min_size: f64, in_parallel: bool) -> Self {
        Self {
            deflection: deflection.max(1e-7),
            angle: angle.max(1e-7),
            min_size: min_size.max(0.0),
            in_parallel,
        }
    }

    /// Construct with sensible defaults:
    /// * `deflection` = 0.1
    /// * `angle` = 0.5 (radians, ≈ 28.6°)
    /// * `min_size` = 1e-7
    /// * `in_parallel` = false
    ///
    /// Mirrors `IMeshTools_Parameters` default values from OCCT.
    pub fn default() -> Self {
        Self::new(0.1, 0.5, 1e-7, false)
    }

    /// Builder-style setter: return a copy of `self` with `deflection` replaced
    /// by `d`.  Values below `1e-7` are clamped up to `1e-7`.
    ///
    /// Mirrors the ability to adjust `IMeshTools_Parameters::Deflection` in
    /// OCCT before passing parameters to the mesher.
    pub fn with_deflection(mut self, d: f64) -> Self {
        self.deflection = d.max(1e-7);
        self
    }

    /// Builder-style setter: return a copy of `self` with `angle` replaced by
    /// `a`.  Values below `1e-7` are clamped up to `1e-7`.
    ///
    /// Mirrors `IMeshTools_Parameters::Angle`.
    pub fn with_angle(mut self, a: f64) -> Self {
        self.angle = a.max(1e-7);
        self
    }
}

impl Default for MeshAlgoParams {
    fn default() -> Self {
        MeshAlgoParams::default()
    }
}

// ---------------------------------------------------------------------------
// DiscretResult
// ---------------------------------------------------------------------------

/// Result returned after a discrete meshing operation completes.
///
/// Carries the mesh statistics (node and triangle counts) together with a
/// human-readable status string that reflects whether the operation succeeded.
///
/// Mirrors the done/error reporting pattern of `BRepMesh_Discret` in OCCT,
/// where `IsDone()` is the primary success indicator.
// occt-ref: BRepMesh_Discret
#[derive(Clone, Debug)]
pub struct DiscretResult {
    /// Number of mesh nodes (vertices) produced.
    pub node_count: u32,
    /// Number of triangles produced.
    pub triangle_count: u32,
    /// Human-readable status string.  `"Done"` on success; an error message
    /// otherwise.
    pub status: String,
}

impl DiscretResult {
    /// Construct a successful result with the given node and triangle counts.
    ///
    /// Sets `status` to `"Done"`, matching OCCT's successful return state.
    pub fn new(nodes: u32, tris: u32) -> Self {
        Self {
            node_count: nodes,
            triangle_count: tris,
            status: String::from("Done"),
        }
    }

    /// Returns `true` when the meshing operation finished successfully, i.e.
    /// when `status == "Done"`.
    ///
    /// Mirrors `BRepMesh_Discret::IsDone()` from OCCT.
    pub fn is_done(&self) -> bool {
        self.status == "Done"
    }
}

// ---------------------------------------------------------------------------
// MeshDiscret
// ---------------------------------------------------------------------------

/// Top-level discrete meshing driver.
///
/// Binds a named shape (represented as a `String` identifier / descriptor in
/// this stub) to a set of [`MeshAlgoParams`] and exposes a `perform()` method
/// that returns a [`DiscretResult`].
///
/// Mirrors `BRepMesh_Discret` (which implements `IMeshTools_MeshAlgo`) in OCCT.
// occt-ref: BRepMesh_Discret
// occt: IMeshTools_MeshAlgo
pub struct MeshDiscret {
    /// Parameters controlling the meshing quality.
    pub params: MeshAlgoParams,
    /// Identifier / descriptor for the shape to be meshed.  In a full
    /// implementation this would be a reference to a topological shape object.
    pub shape: String,
}

impl MeshDiscret {
    /// Construct a new `MeshDiscret` for the named shape using the supplied
    /// parameters.
    ///
    /// Mirrors `BRepMesh_Discret::BRepMesh_Discret(shape, params)` from OCCT.
    pub fn new(shape: &str, params: MeshAlgoParams) -> Self {
        Self {
            params,
            shape: shape.to_owned(),
        }
    }

    /// Execute the discrete meshing algorithm and return a [`DiscretResult`].
    ///
    /// This stub estimates node and triangle counts from the deflection
    /// parameter rather than performing real geometric subdivision.  The count
    /// heuristic mirrors the general behaviour that finer deflection values
    /// produce more elements:
    ///
    /// * `estimated_elements ≈ ceil(1.0 / deflection^2)`, clamped to the range
    ///   `[1, 1_000_000]`.
    /// * `node_count ≈ estimated_elements / 2` (typical for closed meshes).
    /// * `triangle_count = estimated_elements`.
    ///
    /// If the shape name is empty, or the deflection is non-positive, the
    /// result status is set to an error string and counts are zero.
    ///
    /// Mirrors `BRepMesh_Discret::Perform()` / `IMeshTools_MeshAlgo::Perform()`
    /// from OCCT.
    pub fn perform(&self) -> DiscretResult {
        if self.shape.is_empty() {
            return DiscretResult {
                node_count: 0,
                triangle_count: 0,
                status: String::from("Error: empty shape"),
            };
        }
        if self.params.deflection <= 0.0 {
            return DiscretResult {
                node_count: 0,
                triangle_count: 0,
                status: String::from("Error: non-positive deflection"),
            };
        }

        // Heuristic element count based on deflection.
        let raw = (1.0 / (self.params.deflection * self.params.deflection)).ceil() as u32;
        let triangle_count = raw.clamp(1, 1_000_000);
        let node_count = (triangle_count / 2).max(3);

        DiscretResult::new(node_count, triangle_count)
    }

    /// Enable or disable parallel meshing at runtime.
    ///
    /// Mirrors the ability to toggle `IMeshTools_Parameters::InParallel` on a
    /// live algorithm instance in OCCT.
    pub fn set_parallel(&mut self, b: bool) {
        self.params.in_parallel = b;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- MeshAlgoParams tests -------------------------------------------

    #[test]
    fn test_params_new_clamps_deflection() {
        let p = MeshAlgoParams::new(-1.0, 0.5, 0.0, false);
        assert!(p.deflection >= 1e-7);
    }

    #[test]
    fn test_params_new_clamps_angle() {
        let p = MeshAlgoParams::new(0.1, -0.5, 0.0, false);
        assert!(p.angle >= 1e-7);
    }

    #[test]
    fn test_params_default_values() {
        let p = MeshAlgoParams::default();
        assert!((p.deflection - 0.1).abs() < 1e-9);
        assert!((p.angle - 0.5).abs() < 1e-9);
        assert!(p.min_size >= 0.0);
        assert!(!p.in_parallel);
    }

    #[test]
    fn test_params_with_deflection_builder() {
        let p = MeshAlgoParams::default().with_deflection(0.05);
        assert!((p.deflection - 0.05).abs() < 1e-9);
        // Other fields unchanged.
        assert!((p.angle - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_params_with_deflection_clamps_small_value() {
        let p = MeshAlgoParams::default().with_deflection(0.0);
        assert!(p.deflection >= 1e-7);
    }

    #[test]
    fn test_params_with_angle_builder() {
        let p = MeshAlgoParams::default().with_angle(0.25);
        assert!((p.angle - 0.25).abs() < 1e-9);
        // Other fields unchanged.
        assert!((p.deflection - 0.1).abs() < 1e-9);
    }

    #[test]
    fn test_params_with_angle_clamps_small_value() {
        let p = MeshAlgoParams::default().with_angle(-1.0);
        assert!(p.angle >= 1e-7);
    }

    #[test]
    fn test_params_builder_chain() {
        let p = MeshAlgoParams::default()
            .with_deflection(0.01)
            .with_angle(0.1);
        assert!((p.deflection - 0.01).abs() < 1e-9);
        assert!((p.angle - 0.1).abs() < 1e-9);
    }

    // ---- DiscretResult tests --------------------------------------------

    #[test]
    fn test_discret_result_new_is_done() {
        let r = DiscretResult::new(100, 50);
        assert!(r.is_done());
        assert_eq!(r.node_count, 100);
        assert_eq!(r.triangle_count, 50);
        assert_eq!(r.status, "Done");
    }

    #[test]
    fn test_discret_result_error_not_done() {
        let r = DiscretResult {
            node_count: 0,
            triangle_count: 0,
            status: String::from("Error: empty shape"),
        };
        assert!(!r.is_done());
    }

    // ---- MeshDiscret tests ----------------------------------------------

    #[test]
    fn test_mesh_discret_new_stores_fields() {
        let params = MeshAlgoParams::new(0.1, 0.5, 1e-7, false);
        let md = MeshDiscret::new("box", params.clone());
        assert_eq!(md.shape, "box");
        assert!((md.params.deflection - 0.1).abs() < 1e-9);
    }

    #[test]
    fn test_perform_returns_done_for_valid_shape() {
        let params = MeshAlgoParams::default();
        let md = MeshDiscret::new("sphere", params);
        let result = md.perform();
        assert!(result.is_done());
        assert!(result.node_count > 0);
        assert!(result.triangle_count > 0);
    }

    #[test]
    fn test_perform_finer_deflection_yields_more_triangles() {
        let coarse = MeshDiscret::new("box", MeshAlgoParams::default().with_deflection(1.0));
        let fine = MeshDiscret::new("box", MeshAlgoParams::default().with_deflection(0.1));
        let r_coarse = coarse.perform();
        let r_fine = fine.perform();
        assert!(r_fine.triangle_count > r_coarse.triangle_count);
    }

    #[test]
    fn test_perform_empty_shape_is_error() {
        let params = MeshAlgoParams::default();
        let md = MeshDiscret::new("", params);
        let result = md.perform();
        assert!(!result.is_done());
        assert_eq!(result.node_count, 0);
        assert_eq!(result.triangle_count, 0);
    }

    #[test]
    fn test_perform_zero_deflection_is_error() {
        // MeshAlgoParams clamps deflection to 1e-7, so force via direct struct
        // construction to test the guard in `perform`.
        let md = MeshDiscret {
            params: MeshAlgoParams {
                deflection: 0.0,
                angle: 0.5,
                min_size: 1e-7,
                in_parallel: false,
            },
            shape: String::from("box"),
        };
        let result = md.perform();
        assert!(!result.is_done());
    }

    #[test]
    fn test_set_parallel_toggles_flag() {
        let params = MeshAlgoParams::default();
        let mut md = MeshDiscret::new("cone", params);
        assert!(!md.params.in_parallel);
        md.set_parallel(true);
        assert!(md.params.in_parallel);
        md.set_parallel(false);
        assert!(!md.params.in_parallel);
    }

    #[test]
    fn test_perform_result_triangle_count_within_bounds() {
        let params = MeshAlgoParams::default().with_deflection(0.5);
        let md = MeshDiscret::new("torus", params);
        let result = md.perform();
        assert!(result.is_done());
        assert!(result.triangle_count >= 1);
        assert!(result.triangle_count <= 1_000_000);
    }
}
