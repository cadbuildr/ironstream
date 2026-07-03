// FILE: top_ope_b_rep_ds_face_edge_interference.rs
// occt: TopOpeBRepDS_FaceEdgeInterference

use crate::top_ope_b_rep_ds_shape_shape_interference::ShapeShapeInterference;
use crate::top_ope_b_rep_ds_transition::Transition;
use crate::top_ope_b_rep_ds_kind::Kind;
use crate::top_ope_b_rep_ds_config::Config;

/// Face-edge interference (extends ShapeShapeInterference)
#[derive(Debug, Clone)]
pub struct FaceEdgeInterference {
    /// Base shape-shape interference
    base: ShapeShapeInterference,
}

impl FaceEdgeInterference {
    /// Create a face-edge interference
    pub fn new(
        transition: Transition,
        support_type: Kind,
        support: i32,
        geometry: i32,
        config: Config,
    ) -> Self {
        let base = ShapeShapeInterference::new(
            transition,
            support_type,
            support,
            Kind::Edge,
            geometry,
            false,
            config,
        );
        FaceEdgeInterference { base }
    }

    /// Get base interference
    pub fn base(&self) -> &ShapeShapeInterference {
        &self.base
    }

    /// Get mutable base interference
    pub fn base_mut(&mut self) -> &mut ShapeShapeInterference {
        &mut self.base
    }

    pub fn support_type(&self) -> Kind {
        self.base.support_type()
    }

    pub fn support(&self) -> i32 {
        self.base.support()
    }

    pub fn geometry_type(&self) -> Kind {
        self.base.geometry_type()
    }

    pub fn geometry(&self) -> i32 {
        self.base.geometry()
    }

    pub fn config(&self) -> Config {
        self.base.config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_edge_interference_new() {
        let t = Transition::new();
        let fei = FaceEdgeInterference::new(
            t,
            Kind::Face,
            5,
            10,
            Config::DiffOriented,
        );
        assert_eq!(fei.support_type(), Kind::Face);
        assert_eq!(fei.support(), 5);
        assert_eq!(fei.geometry_type(), Kind::Edge);
        assert_eq!(fei.geometry(), 10);
        assert_eq!(fei.config(), Config::DiffOriented);
    }
}
