// FILE: top_ope_b_rep_ds_shape_shape_interference.rs
// occt: TopOpeBRepDS_ShapeShapeInterference

use crate::top_ope_b_rep_ds_interference::Interference;
use crate::top_ope_b_rep_ds_transition::Transition;
use crate::top_ope_b_rep_ds_kind::Kind;
use crate::top_ope_b_rep_ds_config::Config;

/// Shape-shape interference (extends Interference)
#[derive(Debug, Clone)]
pub struct ShapeShapeInterference {
    /// Base interference
    interference: Interference,
    /// Whether geometry is a bound of the support
    g_bound: bool,
    /// Configuration
    config: Config,
}

impl ShapeShapeInterference {
    /// Create a new ShapeShapeInterference
    pub fn new(
        transition: Transition,
        support_type: Kind,
        support: i32,
        geometry_type: Kind,
        geometry: i32,
        g_bound: bool,
        config: Config,
    ) -> Self {
        ShapeShapeInterference {
            interference: Interference::with_data(
                transition,
                support_type,
                support,
                geometry_type,
                geometry,
            ),
            g_bound,
            config,
        }
    }

    /// Get base interference (read-only)
    pub fn interference(&self) -> &Interference {
        &self.interference
    }

    /// Get mutable interference
    pub fn interference_mut(&mut self) -> &mut Interference {
        &mut self.interference
    }

    /// Get config
    pub fn config(&self) -> Config {
        self.config
    }

    /// Get g_bound flag
    pub fn g_bound(&self) -> bool {
        self.g_bound
    }

    /// Set g_bound flag
    pub fn set_g_bound(&mut self, b: bool) {
        self.g_bound = b;
    }

    // Delegate methods to Interference

    /// Get transition
    pub fn transition(&self) -> &Transition {
        self.interference.transition()
    }

    /// Get mutable transition
    pub fn transition_mut(&mut self) -> &mut Transition {
        self.interference.transition_mut()
    }

    /// Get support type
    pub fn support_type(&self) -> Kind {
        self.interference.support_type()
    }

    /// Get support index
    pub fn support(&self) -> i32 {
        self.interference.support()
    }

    /// Get geometry type
    pub fn geometry_type(&self) -> Kind {
        self.interference.geometry_type()
    }

    /// Get geometry index
    pub fn geometry(&self) -> i32 {
        self.interference.geometry()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_shape_interference_new() {
        let t = Transition::new();
        let ssi = ShapeShapeInterference::new(
            t,
            Kind::Face,
            5,
            Kind::Vertex,
            10,
            true,
            Config::SameOriented,
        );
        assert_eq!(ssi.support_type(), Kind::Face);
        assert_eq!(ssi.support(), 5);
        assert_eq!(ssi.geometry_type(), Kind::Vertex);
        assert_eq!(ssi.geometry(), 10);
        assert!(ssi.g_bound());
        assert_eq!(ssi.config(), Config::SameOriented);
    }

    #[test]
    fn test_shape_shape_interference_g_bound() {
        let t = Transition::new();
        let mut ssi = ShapeShapeInterference::new(
            t,
            Kind::Edge,
            1,
            Kind::Vertex,
            2,
            true,
            Config::DiffOriented,
        );
        assert!(ssi.g_bound());
        ssi.set_g_bound(false);
        assert!(!ssi.g_bound());
    }

    #[test]
    fn test_shape_shape_interference_delegates() {
        let t = Transition::new();
        let ssi = ShapeShapeInterference::new(
            t,
            Kind::Shell,
            20,
            Kind::Edge,
            30,
            false,
            Config::UnshGeometry,
        );
        assert_eq!(ssi.support_type(), Kind::Shell);
        assert_eq!(ssi.support(), 20);
        assert_eq!(ssi.geometry_type(), Kind::Edge);
        assert_eq!(ssi.geometry(), 30);
    }
}
