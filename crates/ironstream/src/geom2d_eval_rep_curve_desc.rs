// FILE: geom2d_eval_rep_curve_desc.rs
// occt: Geom2dEval_RepCurveDesc
// occt: Geom2dEval_RepCurveDesc // ::Map1d
// occt: Geom2dEval_RepCurveDesc // ::Domain1d
// occt: Geom2dEval_RepCurveDesc // ::Base
// occt: Geom2dEval_RepCurveDesc // ::Full
// occt: Geom2dEval_RepCurveDesc // ::DerivBounded
// occt: Geom2dEval_RepCurveDesc // ::Mapped

const CONFUSION_THRESHOLD: f64 = 1e-7;

/// 1D affine parameter map: uRep = Scale * u + Offset.
#[derive(Clone, Copy, Debug)]
pub struct Map1d {
    pub scale: f64,
    pub offset: f64,
}

impl Map1d {
    pub fn new() -> Self {
        Map1d {
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn with_scale_offset(scale: f64, offset: f64) -> Self {
        Map1d { scale, offset }
    }

    pub fn is_identity(&self) -> bool {
        (self.scale - 1.0).abs() < CONFUSION_THRESHOLD
            && self.offset.abs() < CONFUSION_THRESHOLD
    }

    pub fn is_valid(&self) -> bool {
        self.scale.abs() > CONFUSION_THRESHOLD
    }

    pub fn map(&self, u: f64) -> f64 {
        self.scale * u + self.offset
    }
}

impl Default for Map1d {
    fn default() -> Self {
        Map1d::new()
    }
}

/// 1D parameter domain interval.
#[derive(Clone, Copy, Debug)]
pub struct Domain1d {
    pub first: f64,
    pub last: f64,
}

impl Domain1d {
    pub fn new() -> Self {
        Domain1d {
            first: 0.0,
            last: 1.0,
        }
    }

    pub fn with_bounds(first: f64, last: f64) -> Self {
        Domain1d { first, last }
    }

    pub fn contains(&self, u: f64) -> bool {
        u >= (self.first - CONFUSION_THRESHOLD) && u <= (self.last + CONFUSION_THRESHOLD)
    }
}

impl Default for Domain1d {
    fn default() -> Self {
        Domain1d::new()
    }
}

/// Descriptor kind for switch-based dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptorKind {
    /// Fully equivalent, no derivative limit, no domain, no map
    Full,
    /// Full domain + identity map, limited to MaxDerivOrder
    DerivBounded,
    /// Has MaxDerivOrder + optional Domain + ParamMap
    Mapped,
}

/// Abstract base descriptor for 2D curve evaluation representation.
pub struct RepCurveDescBase {
    pub kind: DescriptorKind,
}

impl RepCurveDescBase {
    pub fn new(kind: DescriptorKind) -> Self {
        RepCurveDescBase { kind }
    }

    pub fn get_kind(&self) -> DescriptorKind {
        self.kind
    }
}

/// Fully equivalent descriptor: no derivative limit, no domain, no map.
/// Fastest evaluation path - direct delegation to Representation.
pub struct RepCurveDescFull {
    base: RepCurveDescBase,
}

impl RepCurveDescFull {
    pub fn new() -> Self {
        RepCurveDescFull {
            base: RepCurveDescBase::new(DescriptorKind::Full),
        }
    }

    pub fn get_kind(&self) -> DescriptorKind {
        self.base.get_kind()
    }
}

impl Default for RepCurveDescFull {
    fn default() -> Self {
        RepCurveDescFull::new()
    }
}

/// Derivative-bounded descriptor: full domain, identity map, limited to MaxDerivOrder.
pub struct RepCurveDescDerivBounded {
    base: RepCurveDescBase,
    pub max_deriv_order: usize,
}

impl RepCurveDescDerivBounded {
    pub fn new() -> Self {
        RepCurveDescDerivBounded {
            base: RepCurveDescBase::new(DescriptorKind::DerivBounded),
            max_deriv_order: 3,
        }
    }

    pub fn with_max_deriv_order(max_deriv_order: usize) -> Self {
        RepCurveDescDerivBounded {
            base: RepCurveDescBase::new(DescriptorKind::DerivBounded),
            max_deriv_order,
        }
    }

    pub fn get_kind(&self) -> DescriptorKind {
        self.base.get_kind()
    }
}

impl Default for RepCurveDescDerivBounded {
    fn default() -> Self {
        RepCurveDescDerivBounded::new()
    }
}

/// Mapped descriptor for 2D curve evaluation representation.
/// Adds optional bounded domain and affine parameter map.
pub struct RepCurveDescMapped {
    base: RepCurveDescBase,
    pub max_deriv_order: usize,
    pub domain: Option<Domain1d>,
    pub param_map: Map1d,
}

impl RepCurveDescMapped {
    pub fn new() -> Self {
        RepCurveDescMapped {
            base: RepCurveDescBase::new(DescriptorKind::Mapped),
            max_deriv_order: 3,
            domain: None,
            param_map: Map1d::new(),
        }
    }

    pub fn with_domain(mut self, domain: Domain1d) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn with_param_map(mut self, param_map: Map1d) -> Self {
        self.param_map = param_map;
        self
    }

    pub fn with_max_deriv_order(mut self, max_deriv_order: usize) -> Self {
        self.max_deriv_order = max_deriv_order;
        self
    }

    pub fn get_kind(&self) -> DescriptorKind {
        self.base.get_kind()
    }
}

impl Default for RepCurveDescMapped {
    fn default() -> Self {
        RepCurveDescMapped::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_map1d_identity() {
        let map = Map1d::new();
        assert!(map.is_identity());
        assert!(map.is_valid());
        assert!((map.map(5.0) - 5.0).abs() < PRECISION);
    }

    #[test]
    fn test_map1d_scale() {
        let map = Map1d::with_scale_offset(2.0, 0.0);
        assert!(!map.is_identity());
        assert!(map.is_valid());
        assert!((map.map(3.0) - 6.0).abs() < PRECISION);
    }

    #[test]
    fn test_map1d_scale_offset() {
        let map = Map1d::with_scale_offset(2.0, 5.0);
        assert!((map.map(3.0) - 11.0).abs() < PRECISION);
    }

    #[test]
    fn test_map1d_invalid() {
        let map = Map1d::with_scale_offset(0.0, 1.0);
        assert!(!map.is_valid());
    }

    #[test]
    fn test_domain1d_default() {
        let domain = Domain1d::new();
        assert!((domain.first - 0.0).abs() < PRECISION);
        assert!((domain.last - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_domain1d_contains() {
        let domain = Domain1d::with_bounds(0.0, 1.0);
        assert!(domain.contains(0.0));
        assert!(domain.contains(0.5));
        assert!(domain.contains(1.0));
        assert!(!domain.contains(-0.1));
        assert!(!domain.contains(1.1));
    }

    #[test]
    fn test_descriptor_kind_full() {
        let desc = RepCurveDescFull::new();
        assert_eq!(desc.get_kind(), DescriptorKind::Full);
    }

    #[test]
    fn test_descriptor_kind_deriv_bounded() {
        let desc = RepCurveDescDerivBounded::with_max_deriv_order(2);
        assert_eq!(desc.get_kind(), DescriptorKind::DerivBounded);
        assert_eq!(desc.max_deriv_order, 2);
    }

    #[test]
    fn test_descriptor_kind_mapped() {
        let mut desc = RepCurveDescMapped::new();
        desc = desc.with_max_deriv_order(4);
        desc = desc.with_param_map(Map1d::with_scale_offset(2.0, 1.0));
        assert_eq!(desc.get_kind(), DescriptorKind::Mapped);
        assert_eq!(desc.max_deriv_order, 4);
        assert!((desc.param_map.scale - 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_descriptor_mapped_with_domain() {
        let domain = Domain1d::with_bounds(1.0, 5.0);
        let desc = RepCurveDescMapped::new().with_domain(domain);
        assert!(desc.domain.is_some());
        let d = desc.domain.unwrap();
        assert!((d.first - 1.0).abs() < PRECISION);
        assert!((d.last - 5.0).abs() < PRECISION);
    }
}
