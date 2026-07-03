// FILE: top_ope_b_rep_ds_edge_vertex_interference.rs
// occt: TopOpeBRepDS_EdgeVertexInterference

use crate::top_ope_b_rep_ds_shape_shape_interference::ShapeShapeInterference;
use crate::top_ope_b_rep_ds_transition::Transition;
use crate::top_ope_b_rep_ds_kind::Kind;
use crate::top_ope_b_rep_ds_config::Config;

/// Edge-vertex interference (extends ShapeShapeInterference with a parameter)
#[derive(Debug, Clone)]
pub struct EdgeVertexInterference {
    /// Base shape-shape interference
    base: ShapeShapeInterference,
    /// Parameter on the edge
    param: f64,
}

impl EdgeVertexInterference {
    /// Create an interference of VERTEX on a crossed EDGE
    pub fn new(
        transition: Transition,
        support_type: Kind,
        support: i32,
        geometry: i32,
        g_is_bound: bool,
        config: Config,
        param: f64,
    ) -> Self {
        let base = ShapeShapeInterference::new(
            transition,
            support_type,
            support,
            Kind::Vertex,
            geometry,
            g_is_bound,
            config,
        );
        EdgeVertexInterference { base, param }
    }

    /// Create an interference of VERTEX on crossed EDGE (simpler form)
    pub fn edge_vertex(
        transition: Transition,
        edge_support: i32,
        vertex_geometry: i32,
        g_is_bound: bool,
        config: Config,
        param: f64,
    ) -> Self {
        let base = ShapeShapeInterference::new(
            transition,
            Kind::Edge,
            edge_support,
            Kind::Vertex,
            vertex_geometry,
            g_is_bound,
            config,
        );
        EdgeVertexInterference { base, param }
    }

    /// Get the parameter
    pub fn parameter(&self) -> f64 {
        self.param
    }

    /// Set the parameter
    pub fn set_parameter(&mut self, p: f64) {
        self.param = p;
    }

    /// Get base interference
    pub fn base(&self) -> &ShapeShapeInterference {
        &self.base
    }

    /// Get mutable base interference
    pub fn base_mut(&mut self) -> &mut ShapeShapeInterference {
        &mut self.base
    }

    // Delegate key methods
    pub fn support_type(&self) -> Kind {
        self.base.support_type()
    }

    pub fn support(&self) -> i32 {
        self.base.support()
    }

    pub fn geometry(&self) -> i32 {
        self.base.geometry()
    }

    pub fn config(&self) -> Config {
        self.base.config()
    }

    pub fn g_bound(&self) -> bool {
        self.base.g_bound()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_vertex_interference_new() {
        let t = Transition::new();
        let evi = EdgeVertexInterference::new(
            t,
            Kind::Edge,
            5,
            10,
            true,
            Config::SameOriented,
            2.5,
        );
        assert_eq!(evi.support_type(), Kind::Edge);
        assert_eq!(evi.support(), 5);
        assert_eq!(evi.geometry(), 10);
        assert!(evi.g_bound());
        assert_eq!(evi.config(), Config::SameOriented);
        assert_eq!(evi.parameter(), 2.5);
    }

    #[test]
    fn test_edge_vertex_interference_edge_vertex() {
        let t = Transition::new();
        let evi = EdgeVertexInterference::edge_vertex(t, 3, 7, false, Config::DiffOriented, 1.5);
        assert_eq!(evi.support_type(), Kind::Edge);
        assert_eq!(evi.support(), 3);
        assert_eq!(evi.geometry(), 7);
        assert!(!evi.g_bound());
        assert_eq!(evi.parameter(), 1.5);
    }

    #[test]
    fn test_edge_vertex_interference_parameter() {
        let t = Transition::new();
        let mut evi = EdgeVertexInterference::new(t, Kind::Edge, 1, 2, true, Config::UnshGeometry, 0.5);
        assert_eq!(evi.parameter(), 0.5);
        evi.set_parameter(3.7);
        assert_eq!(evi.parameter(), 3.7);
    }
}
