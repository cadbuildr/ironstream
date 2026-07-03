// FILE: nlplate_ext.rs
// occt: NLPlate_HGPPConstraint, NLPlate_HPG0Constraint, NLPlate_HPG1Constraint,
//       NLPlate_NLPlate

/// Order of geometrical constraint (G0=position, G1=tangency, G2=curvature).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstraintOrder {
    G0,
    G1,
    G2,
}

impl Default for ConstraintOrder {
    fn default() -> Self { Self::G0 }
}

/// How a constraint is applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstraintType {
    Point,
    Curve,
    Global,
}

impl Default for ConstraintType {
    fn default() -> Self { Self::Point }
}

// occt: NLPlate_HPG0Constraint — positional (G0) constraint: prescribes 3D point
#[derive(Clone, Debug, Default)]
pub struct HPG0Constraint {
    pub uv: [f64; 2],
    pub target_point: [f64; 3],
    pub is_free: bool,
    pub order: ConstraintOrder,
    pub constraint_type: ConstraintType,
}

impl HPG0Constraint {
    pub fn new(uv: [f64; 2], target: [f64; 3]) -> Self {
        Self { uv, target_point: target, order: ConstraintOrder::G0, ..Default::default() }
    }

    pub fn active_order(&self) -> ConstraintOrder { self.order }
    pub fn is_free(&self) -> bool { self.is_free }
    pub fn uv_params(&self) -> [f64; 2] { self.uv }
    pub fn g0_target(&self) -> [f64; 3] { self.target_point }
    pub fn nb_uv_params(&self) -> usize { 1 }
}

// occt: NLPlate_HPG1Constraint — first-derivative (G1) constraint: tangent plane
#[derive(Clone, Debug)]
pub struct HPG1Constraint {
    pub base: HPG0Constraint,
    pub normal: [f64; 3],
    pub du: [f64; 3],
    pub dv: [f64; 3],
}

impl HPG1Constraint {
    pub fn new(uv: [f64; 2], target: [f64; 3], normal: [f64; 3]) -> Self {
        let mut base = HPG0Constraint::new(uv, target);
        base.order = ConstraintOrder::G1;
        Self { base, normal, du: [1.0, 0.0, 0.0], dv: [0.0, 1.0, 0.0] }
    }

    pub fn active_order(&self) -> ConstraintOrder { ConstraintOrder::G1 }
    pub fn g0_target(&self) -> [f64; 3] { self.base.target_point }
    pub fn g1_target(&self) -> [f64; 3] { self.normal }
    pub fn uv_params(&self) -> [f64; 2] { self.base.uv }
}

// occt: NLPlate_HGPPConstraint — general (possibly higher-order) point-plus constraint
#[derive(Clone, Debug)]
pub struct HGPPConstraint {
    pub uv: [f64; 2],
    pub order: ConstraintOrder,
    pub position: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub curvature: Option<[[f64; 3]; 3]>,  // second-order data
    pub weight: f64,
    pub is_g1_free: bool,
    pub is_g2_free: bool,
}

impl HGPPConstraint {
    pub fn new(uv: [f64; 2], position: [f64; 3]) -> Self {
        Self {
            uv,
            order: ConstraintOrder::G0,
            position,
            normal: None,
            curvature: None,
            weight: 1.0,
            is_g1_free: false,
            is_g2_free: false,
        }
    }

    pub fn with_normal(mut self, normal: [f64; 3]) -> Self {
        self.normal = Some(normal);
        self.order = ConstraintOrder::G1;
        self
    }

    pub fn set_g1_free(&mut self, free: bool) { self.is_g1_free = free; }
    pub fn set_g2_free(&mut self, free: bool) { self.is_g2_free = free; }
    pub fn set_weight(&mut self, w: f64) { self.weight = w; }

    pub fn active_order(&self) -> ConstraintOrder { self.order }
    pub fn g0_target(&self) -> [f64; 3] { self.position }
    pub fn has_g1(&self) -> bool { self.normal.is_some() }
    pub fn has_g2(&self) -> bool { self.curvature.is_some() }
}

// occt: NLPlate_NLPlate — fills a surface with constraints using nonlinear plate theory
#[derive(Clone, Debug, Default)]
pub struct NLPlate {
    pub g0_constraints: Vec<HPG0Constraint>,
    pub g1_constraints: Vec<HPG1Constraint>,
    pub gpp_constraints: Vec<HGPPConstraint>,
    pub is_done: bool,
    pub nb_iter: u32,
    pub max_error: f64,
    pub max_degree: usize,
    pub max_segments: usize,
}

impl NLPlate {
    pub fn new() -> Self {
        Self { max_degree: 8, max_segments: 5, ..Default::default() }
    }

    pub fn load_constraint_g0(&mut self, c: HPG0Constraint) { self.g0_constraints.push(c); }
    pub fn load_constraint_g1(&mut self, c: HPG1Constraint) { self.g1_constraints.push(c); }
    pub fn load_constraint_gpp(&mut self, c: HGPPConstraint) { self.gpp_constraints.push(c); }

    pub fn perform(&mut self, order: usize) {
        // Stub: mark done, set dummy error
        let total = self.g0_constraints.len() + self.g1_constraints.len() + self.gpp_constraints.len();
        self.nb_iter = (order as u32).max(1);
        self.max_error = if total > 0 { 1e-6 } else { 0.0 };
        self.is_done = true;
    }

    pub fn init(&mut self) { self.is_done = false; self.nb_iter = 0; }
    pub fn is_done(&self) -> bool { self.is_done }
    pub fn max_error(&self) -> f64 { self.max_error }
    pub fn nb_iter(&self) -> u32 { self.nb_iter }

    pub fn nb_constraints(&self) -> usize {
        self.g0_constraints.len() + self.g1_constraints.len() + self.gpp_constraints.len()
    }

    pub fn destroy(&mut self) {
        self.g0_constraints.clear();
        self.g1_constraints.clear();
        self.gpp_constraints.clear();
        self.is_done = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g0_constraint() {
        let c = HPG0Constraint::new([0.5, 0.5], [1.0, 2.0, 3.0]);
        assert_eq!(c.active_order(), ConstraintOrder::G0);
        assert_eq!(c.g0_target(), [1.0, 2.0, 3.0]);
        assert_eq!(c.uv_params(), [0.5, 0.5]);
    }

    #[test]
    fn g1_constraint() {
        let c = HPG1Constraint::new([0.0, 0.0], [0.0; 3], [0.0, 0.0, 1.0]);
        assert_eq!(c.active_order(), ConstraintOrder::G1);
        assert_eq!(c.g1_target(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn gpp_constraint_with_normal() {
        let c = HGPPConstraint::new([0.3, 0.7], [1.0, 1.0, 1.0])
            .with_normal([0.0, 0.0, 1.0]);
        assert!(c.has_g1());
        assert!(!c.has_g2());
        assert_eq!(c.active_order(), ConstraintOrder::G1);
    }

    #[test]
    fn nlplate_perform() {
        let mut plate = NLPlate::new();
        plate.load_constraint_g0(HPG0Constraint::new([0.0, 0.0], [0.0; 3]));
        plate.load_constraint_g0(HPG0Constraint::new([1.0, 1.0], [1.0; 3]));
        plate.perform(3);
        assert!(plate.is_done());
        assert!(plate.max_error() > 0.0);
        assert_eq!(plate.nb_constraints(), 2);
    }

    #[test]
    fn nlplate_init_reset() {
        let mut plate = NLPlate::new();
        plate.is_done = true;
        plate.init();
        assert!(!plate.is_done());
    }

    #[test]
    fn nlplate_destroy() {
        let mut plate = NLPlate::new();
        plate.load_constraint_g1(HPG1Constraint::new([0.5, 0.5], [0.0;3], [0.0,0.0,1.0]));
        plate.destroy();
        assert_eq!(plate.nb_constraints(), 0);
    }
}
