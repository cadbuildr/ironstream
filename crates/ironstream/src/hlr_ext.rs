// FILE: hlr_ext.rs
// occt: HLRBRep_Algo, HLRBRep_PolyAlgo, HLRBRep_HLRToShape, HLRAlgo_Projector

use std::f64::consts::PI;

/// Projection type for hidden-line removal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionType {
    Orthographic,
    Perspective,
}

/// A projector definition (camera transform) for HLR.
#[derive(Clone, Debug)]
pub struct HlrProjector {
    pub view_dir: [f64; 3],    // direction from eye to scene (normalized)
    pub up_dir: [f64; 3],
    pub focal: f64,            // for perspective (0 = orthographic)
    pub proj_type: ProjectionType,
}

impl HlrProjector {
    pub fn orthographic(view_dir: [f64; 3], up: [f64; 3]) -> Self {
        Self { view_dir: normalize(view_dir), up_dir: normalize(up), focal: 0.0, proj_type: ProjectionType::Orthographic }
    }

    pub fn perspective(view_dir: [f64; 3], up: [f64; 3], focal: f64) -> Self {
        Self { view_dir: normalize(view_dir), up_dir: normalize(up), focal, proj_type: ProjectionType::Perspective }
    }

    /// Project a 3D point to 2D screen coordinates.
    pub fn project(&self, p: [f64; 3]) -> [f64; 2] {
        let right = normalize(cross(self.view_dir, self.up_dir));
        let up = normalize(cross(right, self.view_dir));
        let x = dot(p, right);
        let y = dot(p, up);
        match self.proj_type {
            ProjectionType::Orthographic => [x, y],
            ProjectionType::Perspective => {
                let depth = dot(p, self.view_dir).max(0.001);
                [x * self.focal / depth, y * self.focal / depth]
            }
        }
    }

    pub fn standard_front() -> Self { Self::orthographic([0.0, -1.0, 0.0], [0.0, 0.0, 1.0]) }
    pub fn standard_top() -> Self { Self::orthographic([0.0, 0.0, -1.0], [0.0, 1.0, 0.0]) }
    pub fn standard_right() -> Self { Self::orthographic([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]) }
    pub fn isometric() -> Self {
        Self::orthographic(
            normalize([1.0, 1.0, -1.0]),
            normalize([-1.0, -1.0, -2.0]),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeVisibility {
    Visible,
    Hidden,
    Unknown,
}

/// A 2D edge result after HLR processing.
#[derive(Clone, Debug)]
pub struct HlrEdge {
    pub id: usize,
    pub start2d: [f64; 2],
    pub end2d: [f64; 2],
    pub visibility: EdgeVisibility,
    pub is_outline: bool,
}

impl HlrEdge {
    pub fn new(id: usize, start: [f64; 2], end: [f64; 2], vis: EdgeVisibility) -> Self {
        Self { id, start2d: start, end2d: end, visibility: vis, is_outline: false }
    }

    pub fn length_2d(&self) -> f64 {
        let dx=self.end2d[0]-self.start2d[0]; let dy=self.end2d[1]-self.start2d[1];
        (dx*dx+dy*dy).sqrt()
    }
}

/// Simplified HLR computation result.
#[derive(Clone, Debug, Default)]
pub struct HlrResult {
    pub edges: Vec<HlrEdge>,
    pub nb_visible: usize,
    pub nb_hidden: usize,
}

impl HlrResult {
    pub fn visible_edges(&self) -> Vec<&HlrEdge> {
        self.edges.iter().filter(|e| e.visibility == EdgeVisibility::Visible).collect()
    }

    pub fn hidden_edges(&self) -> Vec<&HlrEdge> {
        self.edges.iter().filter(|e| e.visibility == EdgeVisibility::Hidden).collect()
    }

    pub fn outline_edges(&self) -> Vec<&HlrEdge> {
        self.edges.iter().filter(|e| e.is_outline).collect()
    }
}

/// Approximate HLR: project edges and naively classify by z-depth.
pub struct HlrPolyAlgo;

impl HlrPolyAlgo {
    /// Project and classify a set of (start3d, end3d, depth) edges.
    pub fn compute(
        proj: &HlrProjector,
        edges: &[([f64; 3], [f64; 3], f64)],  // (start, end, depth_hint: 0=visible, 1=hidden)
    ) -> HlrResult {
        let mut result = HlrResult::default();
        for (i, &(s, e, depth)) in edges.iter().enumerate() {
            let s2 = proj.project(s); let e2 = proj.project(e);
            let vis = if depth < 0.5 { EdgeVisibility::Visible } else { EdgeVisibility::Hidden };
            if vis == EdgeVisibility::Visible { result.nb_visible += 1; }
            else { result.nb_hidden += 1; }
            result.edges.push(HlrEdge::new(i, s2, e2, vis));
        }
        result
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn normalize(v: [f64; 3]) -> [f64; 3] { let m=mag(v); if m>1e-14{[v[0]/m,v[1]/m,v[2]/m]}else{[0.0,0.0,1.0]} }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ortho_projection() {
        let p = HlrProjector::standard_front();
        let pt = p.project([5.0, 0.0, 3.0]);
        assert!((pt[0] - 5.0).abs() < 1e-10);  // x maps to x (right = [1,0,0])
        assert!((pt[1] - 3.0).abs() < 1e-10);  // z maps to y (up = [0,0,1])
    }

    #[test]
    fn hlr_poly_algo() {
        let proj = HlrProjector::standard_front();
        let edges = vec![
            ([0.0,0.0,0.0], [1.0,0.0,0.0], 0.0),  // visible
            ([0.0,0.0,0.0], [0.0,0.0,1.0], 1.0),  // hidden
        ];
        let r = HlrPolyAlgo::compute(&proj, &edges);
        assert_eq!(r.nb_visible, 1);
        assert_eq!(r.nb_hidden, 1);
        assert_eq!(r.visible_edges().len(), 1);
    }

    #[test]
    fn hlr_edge_length() {
        let e = HlrEdge::new(0, [0.0, 0.0], [3.0, 4.0], EdgeVisibility::Visible);
        assert!((e.length_2d() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn projector_standard_views() {
        let _front = HlrProjector::standard_front();
        let _top = HlrProjector::standard_top();
        let _right = HlrProjector::standard_right();
        let _iso = HlrProjector::isometric();
        // Just verify they all construct without panic
    }
}
