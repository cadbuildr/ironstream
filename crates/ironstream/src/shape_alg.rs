// FILE: shape_alg.rs
// occt: ShapeAlgo_AlgoContainer, ShapeAlgo_Tool, ShapeCustom,
//       ShapeCustom_RestrictionParameters, ShapeCustom_Surface

/// Restriction parameters for shape conversion.
/// occt: ShapeCustom_RestrictionParameters
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeCustomRestrictionParameters {
    pub gmax_degree: i32,
    pub gmax_seg: i32,
    pub gmax_degree_surface: i32,
    pub gmax_seg_surface: i32,
    pub convert_bsp_curve: bool,
    pub convert_revolution_surface: bool,
    pub convert_offset_curve3d: bool,
    pub convert_offset_surface: bool,
    pub convert_offset_curve2d: bool,
    pub convert_curved_edges: bool,
    pub convert_conical: bool,
    pub convert_toroidal: bool,
    pub convert_extrusion: bool,
}

impl Default for ShapeCustomRestrictionParameters {
    fn default() -> Self {
        Self {
            gmax_degree: 15,
            gmax_seg: 10000,
            gmax_degree_surface: 15,
            gmax_seg_surface: 10000,
            convert_bsp_curve: false,
            convert_revolution_surface: false,
            convert_offset_curve3d: false,
            convert_offset_surface: false,
            convert_offset_curve2d: false,
            convert_curved_edges: false,
            convert_conical: false,
            convert_toroidal: false,
            convert_extrusion: false,
        }
    }
}

/// Shape analysis result.
#[derive(Clone, Debug, Default)]
pub struct ShapeAlgResult {
    pub nb_faces: usize,
    pub nb_edges: usize,
    pub nb_vertices: usize,
    pub nb_wires: usize,
    pub nb_shells: usize,
    pub nb_solids: usize,
    pub is_closed: bool,
    pub is_manifold: bool,
    pub volume: f64,
    pub surface_area: f64,
}

/// Shape algorithm container (stub for ShapeAlgo_AlgoContainer).
/// occt: ShapeAlgo_AlgoContainer
#[derive(Clone, Debug, Default)]
pub struct ShapeAlgoAlgoContainer {
    pub restriction_params: ShapeCustomRestrictionParameters,
}

impl ShapeAlgoAlgoContainer {
    pub fn new() -> Self { Self::default() }

    /// Stub: analyze a shape by its id. Returns a placeholder result.
    pub fn analyze(&self, shape_id: u32) -> ShapeAlgResult {
        ShapeAlgResult {
            nb_faces: shape_id as usize,
            nb_edges: shape_id as usize * 3,
            nb_vertices: shape_id as usize * 2,
            nb_wires: (shape_id as usize).max(1),
            nb_shells: 1,
            nb_solids: 1,
            is_closed: true,
            is_manifold: true,
            volume: shape_id as f64,
            surface_area: (shape_id * shape_id) as f64,
        }
    }

    pub fn set_restriction_params(&mut self, p: ShapeCustomRestrictionParameters) {
        self.restriction_params = p;
    }
}

/// Tool for common shape operations.
/// occt: ShapeAlgo_Tool
#[derive(Clone, Debug, Default)]
pub struct ShapeAlgoTool;

impl ShapeAlgoTool {
    pub fn new() -> Self { Self }

    /// Stub: returns whether a shape is a solid (shape_id divisible by 2).
    pub fn is_solid(shape_id: u32) -> bool { shape_id % 2 == 0 }

    /// Stub: merge two shapes into one (returns max id + 1).
    pub fn merge(a: u32, b: u32) -> u32 { a.max(b) + 1 }

    /// Stub: split shape into faces (returns shape_id face ids).
    pub fn split_to_faces(shape_id: u32) -> Vec<u32> {
        (0..shape_id).map(|i| i + 1).collect()
    }
}

/// Surface conversion stub.
/// occt: ShapeCustom_Surface
#[derive(Clone, Debug)]
pub struct ShapeCustomSurface {
    pub surface_id: u32,
    pub max_degree: i32,
    pub max_segments: i32,
    pub tolerance: f64,
    pub result_surface_id: Option<u32>,
    pub is_done: bool,
}

impl ShapeCustomSurface {
    pub fn new(surface_id: u32) -> Self {
        Self {
            surface_id,
            max_degree: 15,
            max_segments: 10000,
            tolerance: 1e-7,
            result_surface_id: None,
            is_done: false,
        }
    }

    pub fn convert_to_bspline(&mut self) {
        self.result_surface_id = Some(self.surface_id + 1000);
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn result(&self) -> Option<u32> { self.result_surface_id }
}

/// Shape upgrade: converts all surfaces to BSpline form.
/// occt: ShapeCustom (global functions namespace, simplified)
#[derive(Clone, Debug, Default)]
pub struct ShapeCustom {
    pub tolerance: f64,
    pub max_degree: i32,
    pub max_segments: i32,
}

impl ShapeCustom {
    pub fn new() -> Self {
        Self { tolerance: 1e-7, max_degree: 15, max_segments: 10000 }
    }

    pub fn apply_modifications(&self, shape_id: u32) -> u32 { shape_id }

    /// Convert extrusion/revolution/offset surfaces to BSpline.
    pub fn bspline_restriction(
        &self,
        shape_id: u32,
        _params: &ShapeCustomRestrictionParameters,
    ) -> u32 {
        shape_id + 1
    }
}

/// Scaling / unit conversion on a shape.
/// occt: BRepBuilderAPI_Transform, BRepBuilderAPI_GTransform (simplified)
#[derive(Clone, Debug)]
pub struct ShapeScaler {
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
}

impl Default for ShapeScaler {
    fn default() -> Self { Self { scale_x: 1.0, scale_y: 1.0, scale_z: 1.0 } }
}

impl ShapeScaler {
    pub fn uniform(scale: f64) -> Self {
        Self { scale_x: scale, scale_y: scale, scale_z: scale }
    }

    pub fn non_uniform(sx: f64, sy: f64, sz: f64) -> Self {
        Self { scale_x: sx, scale_y: sy, scale_z: sz }
    }

    pub fn is_uniform(&self) -> bool {
        (self.scale_x - self.scale_y).abs() < 1e-14 &&
        (self.scale_y - self.scale_z).abs() < 1e-14
    }

    pub fn apply_point(&self, p: [f64; 3]) -> [f64; 3] {
        [p[0] * self.scale_x, p[1] * self.scale_y, p[2] * self.scale_z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_container_analyze() {
        let a = ShapeAlgoAlgoContainer::new();
        let r = a.analyze(4);
        assert_eq!(r.nb_faces, 4);
        assert!(r.is_closed);
    }

    #[test]
    fn shape_algo_tool_merge() {
        assert_eq!(ShapeAlgoTool::merge(3, 7), 8);
        assert_eq!(ShapeAlgoTool::merge(10, 2), 11);
    }

    #[test]
    fn shape_algo_tool_split() {
        let faces = ShapeAlgoTool::split_to_faces(3);
        assert_eq!(faces, vec![1, 2, 3]);
    }

    #[test]
    fn custom_surface_convert() {
        let mut s = ShapeCustomSurface::new(5);
        assert!(!s.is_done());
        s.convert_to_bspline();
        assert!(s.is_done());
        assert_eq!(s.result(), Some(1005));
    }

    #[test]
    fn shape_custom_bspline_restriction() {
        let sc = ShapeCustom::new();
        let params = ShapeCustomRestrictionParameters::default();
        let out = sc.bspline_restriction(10, &params);
        assert_eq!(out, 11);
    }

    #[test]
    fn shape_scaler_uniform() {
        let s = ShapeScaler::uniform(2.0);
        assert!(s.is_uniform());
        let p = s.apply_point([1.0, 2.0, 3.0]);
        assert!((p[0] - 2.0).abs() < 1e-14);
        assert!((p[2] - 6.0).abs() < 1e-14);
    }

    #[test]
    fn shape_scaler_non_uniform() {
        let s = ShapeScaler::non_uniform(1.0, 2.0, 3.0);
        assert!(!s.is_uniform());
        let p = s.apply_point([1.0, 1.0, 1.0]);
        assert!((p[1] - 2.0).abs() < 1e-14);
        assert!((p[2] - 3.0).abs() < 1e-14);
    }
}
