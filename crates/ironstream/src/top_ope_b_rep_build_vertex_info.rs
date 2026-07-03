// FILE: top_ope_b_rep_build_vertex_info.rs
// occt: TopOpeBRepBuild_VertexInfo

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildVertexInfo {
    vertex_id: i32,
    parameter: f64,
    tolerance: f64,
}

impl TopOpeBRepBuildVertexInfo {
    pub fn new(vertex_id: i32) -> Self {
        TopOpeBRepBuildVertexInfo {
            vertex_id,
            parameter: 0.0,
            tolerance: 0.0,
        }
    }

    pub fn with_param(vertex_id: i32, parameter: f64, tolerance: f64) -> Self {
        TopOpeBRepBuildVertexInfo {
            vertex_id,
            parameter,
            tolerance,
        }
    }

    pub fn vertex_id(&self) -> i32 {
        self.vertex_id
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn set_parameter(&mut self, param: f64) {
        self.parameter = param;
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }
}

impl Default for TopOpeBRepBuildVertexInfo {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_info_new() {
        let vi = TopOpeBRepBuildVertexInfo::new(5);
        assert_eq!(vi.vertex_id(), 5);
        assert_eq!(vi.parameter(), 0.0);
    }

    #[test]
    fn test_vertex_info_with_param() {
        let vi = TopOpeBRepBuildVertexInfo::with_param(3, 0.5, 0.01);
        assert_eq!(vi.vertex_id(), 3);
        assert_eq!(vi.parameter(), 0.5);
        assert_eq!(vi.tolerance(), 0.01);
    }

    #[test]
    fn test_vertex_info_set_parameter() {
        let mut vi = TopOpeBRepBuildVertexInfo::new(1);
        vi.set_parameter(0.75);
        assert_eq!(vi.parameter(), 0.75);
    }

    #[test]
    fn test_vertex_info_set_tolerance() {
        let mut vi = TopOpeBRepBuildVertexInfo::new(1);
        vi.set_tolerance(0.05);
        assert_eq!(vi.tolerance(), 0.05);
    }
}
