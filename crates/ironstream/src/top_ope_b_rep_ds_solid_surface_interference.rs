// FILE: top_ope_b_rep_ds_solid_surface_interference.rs
// occt: TopOpeBRepDS_SolidSurfaceInterference

use crate::top_ope_b_rep_ds_shape_shape_interference::ShapeShapeInterference;
use crate::top_ope_b_rep_ds_transition::Transition;
use crate::top_ope_b_rep_ds_kind::Kind;
use crate::top_ope_b_rep_ds_config::Config;

/// Solid-surface interference (extends ShapeShapeInterference)
#[derive(Debug, Clone)]
pub struct SolidSurfaceInterference {
    /// Base shape-shape interference
    base: ShapeShapeInterference,
}

impl SolidSurfaceInterference {
    /// Create a solid-surface interference
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
            Kind::Surface,
            geometry,
            false,
            config,
        );
        SolidSurfaceInterference { base }
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
    fn test_solid_surface_interference_new() {
        let t = Transition::new();
        let ssi = SolidSurfaceInterference::new(
            t,
            Kind::Solid,
            5,
            10,
            Config::SameOriented,
        );
        assert_eq!(ssi.support_type(), Kind::Solid);
        assert_eq!(ssi.support(), 5);
        assert_eq!(ssi.geometry_type(), Kind::Surface);
        assert_eq!(ssi.geometry(), 10);
        assert_eq!(ssi.config(), Config::SameOriented);
    }
}
