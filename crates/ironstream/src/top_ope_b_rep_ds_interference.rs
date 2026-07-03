// FILE: top_ope_b_rep_ds_interference.rs
// occt: TopOpeBRepDS_Interference

use crate::top_ope_b_rep_ds_kind::Kind;
use crate::top_ope_b_rep_ds_transition::Transition;

/// An interference description of attachment of new geometry on existing geometry
#[derive(Debug, Clone)]
pub struct Interference {
    /// Transition describing the attachment
    transition: Transition,

    /// Type of the object supporting the interference
    support_type: Kind,

    /// Index in DS of the supporting object
    support: i32,

    /// Type of the geometry of the interference
    geometry_type: Kind,

    /// Index in DS of the geometry
    geometry: i32,
}

impl Interference {
    /// Create a new default Interference
    pub fn new() -> Self {
        Interference {
            transition: Transition::new(),
            support_type: Kind::Unknown,
            support: 0,
            geometry_type: Kind::Unknown,
            geometry: 0,
        }
    }

    /// Create an Interference with all components
    pub fn with_data(
        transition: Transition,
        support_type: Kind,
        support: i32,
        geometry_type: Kind,
        geometry: i32,
    ) -> Self {
        Interference {
            transition,
            support_type,
            support,
            geometry_type,
            geometry,
        }
    }

    /// Copy constructor from another Interference
    pub fn from_interference(other: &Interference) -> Self {
        Interference {
            transition: other.transition.clone(),
            support_type: other.support_type,
            support: other.support,
            geometry_type: other.geometry_type,
            geometry: other.geometry,
        }
    }

    /// Get transition
    pub fn transition(&self) -> &Transition {
        &self.transition
    }

    /// Get mutable transition
    pub fn transition_mut(&mut self) -> &mut Transition {
        &mut self.transition
    }

    /// Set transition
    pub fn set_transition(&mut self, t: Transition) {
        self.transition = t;
    }

    /// Get GKGSKS: GeometryType, Geometry, SupportType, Support
    pub fn gkgsks(&self) -> (Kind, i32, Kind, i32) {
        (self.geometry_type, self.geometry, self.support_type, self.support)
    }

    /// Get support type
    pub fn support_type(&self) -> Kind {
        self.support_type
    }

    /// Get support index
    pub fn support(&self) -> i32 {
        self.support
    }

    /// Get geometry type
    pub fn geometry_type(&self) -> Kind {
        self.geometry_type
    }

    /// Get geometry index
    pub fn geometry(&self) -> i32 {
        self.geometry
    }

    /// Set geometry index
    pub fn set_geometry(&mut self, g: i32) {
        self.geometry = g;
    }

    /// Set support type
    pub fn set_support_type(&mut self, st: Kind) {
        self.support_type = st;
    }

    /// Set support index
    pub fn set_support(&mut self, s: i32) {
        self.support = s;
    }

    /// Set geometry type
    pub fn set_geometry_type(&mut self, gt: Kind) {
        self.geometry_type = gt;
    }

    /// Check if two interferences have the same support
    pub fn has_same_support(&self, other: &Interference) -> bool {
        self.support_type == other.support_type && self.support == other.support
    }

    /// Check if two interferences have the same geometry
    pub fn has_same_geometry(&self, other: &Interference) -> bool {
        self.geometry_type == other.geometry_type && self.geometry == other.geometry
    }
}

impl Default for Interference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_new() {
        let i = Interference::new();
        assert_eq!(i.support_type(), Kind::Unknown);
        assert_eq!(i.support(), 0);
        assert_eq!(i.geometry_type(), Kind::Unknown);
        assert_eq!(i.geometry(), 0);
    }

    #[test]
    fn test_interference_with_data() {
        let t = Transition::new();
        let i = Interference::with_data(t, Kind::Face, 5, Kind::Edge, 10);
        assert_eq!(i.support_type(), Kind::Face);
        assert_eq!(i.support(), 5);
        assert_eq!(i.geometry_type(), Kind::Edge);
        assert_eq!(i.geometry(), 10);
    }

    #[test]
    fn test_interference_gkgsks() {
        let t = Transition::new();
        let i = Interference::with_data(t, Kind::Face, 5, Kind::Curve, 10);
        let (gk, g, sk, s) = i.gkgsks();
        assert_eq!(gk, Kind::Curve);
        assert_eq!(g, 10);
        assert_eq!(sk, Kind::Face);
        assert_eq!(s, 5);
    }

    #[test]
    fn test_interference_copy() {
        let t = Transition::new();
        let i1 = Interference::with_data(t, Kind::Face, 7, Kind::Point, 12);
        let i2 = Interference::from_interference(&i1);
        assert_eq!(i2.support_type(), Kind::Face);
        assert_eq!(i2.support(), 7);
        assert_eq!(i2.geometry_type(), Kind::Point);
        assert_eq!(i2.geometry(), 12);
    }

    #[test]
    fn test_interference_transition_mut() {
        let mut i = Interference::new();
        {
            let t = i.transition_mut();
            t.set_index(42);
        }
        assert_eq!(i.transition().index(), 42);
    }

    #[test]
    fn test_interference_setters() {
        let mut i = Interference::new();
        i.set_support_type(Kind::Edge);
        i.set_support(3);
        i.set_geometry_type(Kind::Surface);
        i.set_geometry(9);
        assert_eq!(i.support_type(), Kind::Edge);
        assert_eq!(i.support(), 3);
        assert_eq!(i.geometry_type(), Kind::Surface);
        assert_eq!(i.geometry(), 9);
    }

    #[test]
    fn test_interference_same_support() {
        let i1 = Interference::with_data(Transition::new(), Kind::Face, 5, Kind::Edge, 10);
        let i2 = Interference::with_data(Transition::new(), Kind::Face, 5, Kind::Curve, 20);
        assert!(i1.has_same_support(&i2));

        let i3 = Interference::with_data(Transition::new(), Kind::Face, 6, Kind::Edge, 10);
        assert!(!i1.has_same_support(&i3));
    }

    #[test]
    fn test_interference_same_geometry() {
        let i1 = Interference::with_data(Transition::new(), Kind::Face, 5, Kind::Edge, 10);
        let i2 = Interference::with_data(Transition::new(), Kind::Shell, 7, Kind::Edge, 10);
        assert!(i1.has_same_geometry(&i2));

        let i3 = Interference::with_data(Transition::new(), Kind::Face, 5, Kind::Edge, 11);
        assert!(!i1.has_same_geometry(&i3));
    }

    #[test]
    fn test_interference_default() {
        let i = Interference::default();
        assert_eq!(i.support_type(), Kind::Unknown);
    }
}
