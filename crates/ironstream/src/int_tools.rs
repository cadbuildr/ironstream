// FILE: int_tools.rs

// occt: IntTools_CommonPrt // kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntToolsEdgeEdgeStatus {
    NotDone,
    Parallel,
    Coincide,
    Intersect,
}

// occt: IntTools_CommonPrt
pub struct IntToolsCommonPrt {
    pub param1_start: f64,
    pub param1_end: f64,
    pub param2_start: f64,
    pub param2_end: f64,
    pub status: IntToolsEdgeEdgeStatus,
}

impl IntToolsCommonPrt {
    pub fn new(
        p1s: f64,
        p1e: f64,
        p2s: f64,
        p2e: f64,
        st: IntToolsEdgeEdgeStatus,
    ) -> Self {
        Self {
            param1_start: p1s,
            param1_end: p1e,
            param2_start: p2s,
            param2_end: p2e,
            status: st,
        }
    }
}

// occt: IntTools_FClass2d
pub struct IntToolsFClass2d {
    tolerance: f64,
    initialized: bool,
}

impl IntToolsFClass2d {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance, initialized: false }
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn perform(&self, u: f64, v: f64) -> bool {
        let _ = self.tolerance;
        u >= 0.0 && v >= 0.0
    }
}

// occt: IntTools_EdgeEdge
pub struct IntToolsEdgeEdge {
    parts: Vec<IntToolsCommonPrt>,
    is_done: bool,
    tolerance: f64,
}

impl IntToolsEdgeEdge {
    pub fn new(tolerance: f64) -> Self {
        Self { parts: Vec::new(), is_done: false, tolerance }
    }

    pub fn perform(&mut self) {
        let _ = self.tolerance;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn common_parts(&self) -> &[IntToolsCommonPrt] {
        &self.parts
    }

    pub fn add_part(&mut self, part: IntToolsCommonPrt) {
        self.parts.push(part);
    }

    pub fn nb_common_parts(&self) -> usize {
        self.parts.len()
    }
}

// occt: IntTools_Context
pub struct IntToolsContext {
    tolerance: f64,
    parallel: bool,
}

impl IntToolsContext {
    pub fn new() -> Self {
        Self { tolerance: 1e-7, parallel: false }
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_parallel(&mut self, v: bool) {
        self.parallel = v;
    }

    pub fn is_parallel(&self) -> bool {
        self.parallel
    }
}

impl Default for IntToolsContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_prt_fields_round_trip() {
        let p = IntToolsCommonPrt::new(0.1, 0.9, 0.2, 0.8, IntToolsEdgeEdgeStatus::Intersect);
        assert!((p.param1_start - 0.1).abs() < 1e-15);
        assert!((p.param1_end - 0.9).abs() < 1e-15);
        assert!((p.param2_start - 0.2).abs() < 1e-15);
        assert!((p.param2_end - 0.8).abs() < 1e-15);
        assert_eq!(p.status, IntToolsEdgeEdgeStatus::Intersect);
    }

    #[test]
    fn fclass2d_initialize_flag() {
        let mut fc = IntToolsFClass2d::new(1e-6);
        assert!(!fc.is_initialized());
        fc.initialize();
        assert!(fc.is_initialized());
    }

    #[test]
    fn fclass2d_perform_positive_quadrant() {
        let fc = IntToolsFClass2d::new(1e-6);
        assert!(fc.perform(1.0, 1.0));
        assert!(fc.perform(0.0, 0.0));
    }

    #[test]
    fn fclass2d_perform_outside() {
        let fc = IntToolsFClass2d::new(1e-6);
        assert!(!fc.perform(-0.1, 1.0));
        assert!(!fc.perform(1.0, -0.1));
        assert!(!fc.perform(-1.0, -1.0));
    }

    #[test]
    fn edge_edge_perform_sets_done() {
        let mut ee = IntToolsEdgeEdge::new(1e-7);
        assert!(!ee.is_done());
        ee.perform();
        assert!(ee.is_done());
    }

    #[test]
    fn edge_edge_add_and_count_parts() {
        let mut ee = IntToolsEdgeEdge::new(1e-7);
        ee.perform();
        assert_eq!(ee.nb_common_parts(), 0);
        ee.add_part(IntToolsCommonPrt::new(0.0, 1.0, 0.0, 1.0, IntToolsEdgeEdgeStatus::Coincide));
        assert_eq!(ee.nb_common_parts(), 1);
        ee.add_part(IntToolsCommonPrt::new(0.5, 0.5, 0.5, 0.5, IntToolsEdgeEdgeStatus::Intersect));
        assert_eq!(ee.nb_common_parts(), 2);
    }

    #[test]
    fn edge_edge_common_parts_slice() {
        let mut ee = IntToolsEdgeEdge::new(1e-7);
        ee.add_part(IntToolsCommonPrt::new(0.0, 1.0, 0.0, 1.0, IntToolsEdgeEdgeStatus::Parallel));
        let parts = ee.common_parts();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].status, IntToolsEdgeEdgeStatus::Parallel);
    }

    #[test]
    fn context_default_tolerance() {
        let ctx = IntToolsContext::new();
        assert!((ctx.tolerance() - 1e-7).abs() < 1e-15);
    }

    #[test]
    fn context_set_tolerance() {
        let mut ctx = IntToolsContext::new();
        ctx.set_tolerance(1e-4);
        assert!((ctx.tolerance() - 1e-4).abs() < 1e-15);
    }

    #[test]
    fn context_parallel_flag() {
        let mut ctx = IntToolsContext::new();
        assert!(!ctx.is_parallel());
        ctx.set_parallel(true);
        assert!(ctx.is_parallel());
        ctx.set_parallel(false);
        assert!(!ctx.is_parallel());
    }

    #[test]
    fn status_enum_variants_eq() {
        assert_eq!(IntToolsEdgeEdgeStatus::NotDone, IntToolsEdgeEdgeStatus::NotDone);
        assert_ne!(IntToolsEdgeEdgeStatus::Parallel, IntToolsEdgeEdgeStatus::Coincide);
    }
}
