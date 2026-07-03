// FILE: geom_eval_rep_surface_desc.rs
// occt: GeomEval_RepSurfaceDesc

//! Surface evaluation representation descriptors with 2D parameter mapping,
//! optional UV swap, and domain constraints.

/// 2D diagonal affine parameter map with optional UV swap.
/// Without swap: uRep = ScaleU*u + OffsetU, vRep = ScaleV*v + OffsetV.
/// With swap:    uRep = ScaleU*v + OffsetU, vRep = ScaleV*u + OffsetV.
#[derive(Clone, Copy, Debug)]
pub struct Map2d {
    pub scale_u: f64,
    pub offset_u: f64,
    pub scale_v: f64,
    pub offset_v: f64,
    pub swap_uv: bool,
}

impl Map2d {
    pub fn new(scale_u: f64, offset_u: f64, scale_v: f64, offset_v: f64, swap_uv: bool) -> Self {
        Map2d {
            scale_u,
            offset_u,
            scale_v,
            offset_v,
            swap_uv,
        }
    }

    pub fn identity() -> Self {
        Map2d {
            scale_u: 1.0,
            offset_u: 0.0,
            scale_v: 1.0,
            offset_v: 0.0,
            swap_uv: false,
        }
    }

    pub fn is_identity(&self) -> bool {
        (self.scale_u - 1.0).abs() < 1e-12
            && (self.scale_v - 1.0).abs() < 1e-12
            && self.offset_u.abs() < 1e-12
            && self.offset_v.abs() < 1e-12
            && !self.swap_uv
    }

    pub fn is_valid(&self) -> bool {
        self.scale_u.abs() > 1e-12 && self.scale_v.abs() > 1e-12
    }

    pub fn map(&self, u: f64, v: f64) -> (f64, f64) {
        if self.swap_uv {
            (
                self.scale_u * v + self.offset_u,
                self.scale_v * u + self.offset_v,
            )
        } else {
            (
                self.scale_u * u + self.offset_u,
                self.scale_v * v + self.offset_v,
            )
        }
    }
}

/// 2D parameter domain rectangle
#[derive(Clone, Copy, Debug)]
pub struct Domain2d {
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
}

impl Domain2d {
    pub fn new(u_first: f64, u_last: f64, v_first: f64, v_last: f64) -> Self {
        Domain2d {
            u_first,
            u_last,
            v_first,
            v_last,
        }
    }

    pub fn contains(&self, u: f64, v: f64) -> bool {
        (u >= self.u_first - 1e-12 && u <= self.u_last + 1e-12)
            && (v >= self.v_first - 1e-12 && v <= self.v_last + 1e-12)
    }
}

/// Abstract base descriptor for surface evaluation representation.
/// Kind tag enables switch-based dispatch without RTTI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptorKind {
    /// Fully equivalent, no derivative limit, no domain, no map
    Full,
    /// Full domain + identity map, limited to MaxDerivOrder
    DerivBounded,
    /// Has MaxDerivOrder + optional Domain + ParamMap
    Mapped,
}

/// Surface descriptor holding representation and kind.
#[derive(Clone, Debug)]
pub struct Descriptor {
    pub kind: DescriptorKind,
    pub max_deriv_order: usize,
    pub domain: Option<Domain2d>,
    pub param_map: Option<Map2d>,
}

impl Descriptor {
    /// Create a fully equivalent descriptor (no limits).
    pub fn full() -> Self {
        Descriptor {
            kind: DescriptorKind::Full,
            max_deriv_order: usize::MAX,
            domain: None,
            param_map: None,
        }
    }

    /// Create a derivative-bounded descriptor (full domain, identity map).
    pub fn deriv_bounded(max_deriv_order: usize) -> Self {
        Descriptor {
            kind: DescriptorKind::DerivBounded,
            max_deriv_order,
            domain: None,
            param_map: None,
        }
    }

    /// Create a mapped descriptor with optional domain and parameter map.
    pub fn mapped(
        max_deriv_order: usize,
        domain: Option<Domain2d>,
        param_map: Option<Map2d>,
    ) -> Self {
        Descriptor {
            kind: DescriptorKind::Mapped,
            max_deriv_order,
            domain,
            param_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map2d_identity() {
        let map = Map2d::identity();
        assert!(map.is_identity());
        assert_eq!(map.map(3.0, 5.0), (3.0, 5.0));
    }

    #[test]
    fn test_map2d_scale() {
        let map = Map2d::new(2.0, 0.0, 3.0, 0.0, false);
        assert_eq!(map.map(2.0, 4.0), (4.0, 12.0));
    }

    #[test]
    fn test_map2d_offset() {
        let map = Map2d::new(1.0, 5.0, 1.0, 10.0, false);
        assert_eq!(map.map(1.0, 2.0), (6.0, 12.0));
    }

    #[test]
    fn test_map2d_swap() {
        let map = Map2d::new(1.0, 0.0, 1.0, 0.0, true);
        assert_eq!(map.map(2.0, 3.0), (3.0, 2.0));
    }

    #[test]
    fn test_map2d_swap_with_scale() {
        let map = Map2d::new(2.0, 0.0, 3.0, 0.0, true);
        let (u_rep, v_rep) = map.map(1.0, 2.0);
        assert_eq!(u_rep, 4.0);  // 2.0 * 2.0
        assert_eq!(v_rep, 3.0);  // 3.0 * 1.0
    }

    #[test]
    fn test_domain2d_contains() {
        let domain = Domain2d::new(0.0, 10.0, 0.0, 5.0);
        assert!(domain.contains(5.0, 2.5));
        assert!(!domain.contains(15.0, 2.5));
        assert!(!domain.contains(5.0, 7.0));
    }

    #[test]
    fn test_descriptor_full() {
        let desc = Descriptor::full();
        assert_eq!(desc.kind, DescriptorKind::Full);
        assert_eq!(desc.max_deriv_order, usize::MAX);
    }

    #[test]
    fn test_descriptor_deriv_bounded() {
        let desc = Descriptor::deriv_bounded(2);
        assert_eq!(desc.kind, DescriptorKind::DerivBounded);
        assert_eq!(desc.max_deriv_order, 2);
    }

    #[test]
    fn test_descriptor_mapped() {
        let domain = Domain2d::new(0.0, 1.0, 0.0, 1.0);
        let map = Map2d::new(2.0, 0.0, 2.0, 0.0, false);
        let desc = Descriptor::mapped(3, Some(domain), Some(map));
        assert_eq!(desc.kind, DescriptorKind::Mapped);
        assert!(desc.domain.is_some());
        assert!(desc.param_map.is_some());
    }
}
