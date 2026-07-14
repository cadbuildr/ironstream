// FILE: bop_algo.rs

// occt: BOPAlgo_Operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopAlgoOperation {
    Fuse,
    Common,
    Cut,
    Cut21,
    Section,
}

// occt: BOPAlgo_CheckStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopAlgoCheckStatus {
    NoFault,
    BadType,
    SelfIntersect,
    TooSmallEdge,
    NonRecoverableFace,
    GeomAbs_C0,
    InvalidCurveOnSurface,
}

// occt: BOPAlgo_CheckerSI
pub struct BopAlgoCheckerSI {
    level_of_check: u8,
    nb_interferences: u32,
    is_done: bool,
}

impl BopAlgoCheckerSI {
    pub fn new() -> Self {
        Self {
            level_of_check: 2,
            nb_interferences: 0,
            is_done: false,
        }
    }

    pub fn set_level_of_check(&mut self, level: u8) {
        self.level_of_check = level;
    }

    pub fn level_of_check(&self) -> u8 {
        self.level_of_check
    }

    pub fn perform(&mut self) {
        self.is_done = true;
        self.nb_interferences = 0;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn has_interferences(&self) -> bool {
        self.nb_interferences > 0
    }

    pub fn nb_interferences(&self) -> u32 {
        self.nb_interferences
    }
}

impl Default for BopAlgoCheckerSI {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: BOPAlgo_Builder
pub struct BopAlgoBuilder {
    operation: BopAlgoOperation,
    fuzzy_value: f64,
    non_destructive: bool,
    run_parallel: bool,
    is_done: bool,
    nb_warnings: u32,
    nb_errors: u32,
}

impl BopAlgoBuilder {
    pub fn new(operation: BopAlgoOperation) -> Self {
        Self {
            operation,
            fuzzy_value: 0.0,
            non_destructive: false,
            run_parallel: false,
            is_done: false,
            nb_warnings: 0,
            nb_errors: 0,
        }
    }

    pub fn operation(&self) -> BopAlgoOperation {
        self.operation
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.fuzzy_value = v;
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.fuzzy_value
    }

    pub fn set_non_destructive(&mut self, v: bool) {
        self.non_destructive = v;
    }

    pub fn non_destructive(&self) -> bool {
        self.non_destructive
    }

    pub fn set_run_parallel(&mut self, v: bool) {
        self.run_parallel = v;
    }

    pub fn run_parallel(&self) -> bool {
        self.run_parallel
    }

    pub fn build(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn has_warnings(&self) -> bool {
        self.nb_warnings > 0
    }

    pub fn has_errors(&self) -> bool {
        self.nb_errors > 0
    }

    pub fn nb_warnings(&self) -> u32 {
        self.nb_warnings
    }

    pub fn nb_errors(&self) -> u32 {
        self.nb_errors
    }
}

// occt: BOPAlgo_BOP
pub struct BopAlgoBop {
    builder: BopAlgoBuilder,
}

impl BopAlgoBop {
    pub fn new(op: BopAlgoOperation) -> Self {
        Self {
            builder: BopAlgoBuilder::new(op),
        }
    }

    pub fn operation(&self) -> BopAlgoOperation {
        self.builder.operation()
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.builder.set_fuzzy_value(v);
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.builder.fuzzy_value()
    }

    pub fn set_non_destructive(&mut self, v: bool) {
        self.builder.set_non_destructive(v);
    }

    pub fn non_destructive(&self) -> bool {
        self.builder.non_destructive()
    }

    pub fn set_run_parallel(&mut self, v: bool) {
        self.builder.set_run_parallel(v);
    }

    pub fn run_parallel(&self) -> bool {
        self.builder.run_parallel()
    }

    pub fn build(&mut self) {
        self.builder.build();
    }

    pub fn is_done(&self) -> bool {
        self.builder.is_done()
    }

    pub fn has_warnings(&self) -> bool {
        self.builder.has_warnings()
    }

    pub fn has_errors(&self) -> bool {
        self.builder.has_errors()
    }

    pub fn nb_warnings(&self) -> u32 {
        self.builder.nb_warnings()
    }

    pub fn nb_errors(&self) -> u32 {
        self.builder.nb_errors()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_new_defaults() {
        let b = BopAlgoBuilder::new(BopAlgoOperation::Fuse);
        assert_eq!(b.operation(), BopAlgoOperation::Fuse);
        assert_eq!(b.fuzzy_value(), 0.0);
        assert!(!b.non_destructive());
        assert!(!b.run_parallel());
        assert!(!b.is_done());
        assert!(!b.has_warnings());
        assert!(!b.has_errors());
    }

    #[test]
    fn builder_build_sets_done() {
        let mut b = BopAlgoBuilder::new(BopAlgoOperation::Cut);
        assert!(!b.is_done());
        b.build();
        assert!(b.is_done());
    }

    #[test]
    fn builder_fuzzy_value_roundtrip() {
        let mut b = BopAlgoBuilder::new(BopAlgoOperation::Common);
        b.set_fuzzy_value(1e-5);
        assert!((b.fuzzy_value() - 1e-5).abs() < f64::EPSILON);
    }

    #[test]
    fn builder_non_destructive_flag() {
        let mut b = BopAlgoBuilder::new(BopAlgoOperation::Section);
        b.set_non_destructive(true);
        assert!(b.non_destructive());
        b.set_non_destructive(false);
        assert!(!b.non_destructive());
    }

    #[test]
    fn builder_run_parallel_flag() {
        let mut b = BopAlgoBuilder::new(BopAlgoOperation::Cut21);
        b.set_run_parallel(true);
        assert!(b.run_parallel());
    }

    #[test]
    fn builder_no_errors_no_warnings_by_default() {
        let b = BopAlgoBuilder::new(BopAlgoOperation::Fuse);
        assert_eq!(b.nb_warnings(), 0);
        assert_eq!(b.nb_errors(), 0);
    }

    #[test]
    fn checker_si_new_defaults() {
        let c = BopAlgoCheckerSI::new();
        assert_eq!(c.level_of_check(), 2);
        assert!(!c.is_done());
        assert!(!c.has_interferences());
        assert_eq!(c.nb_interferences(), 0);
    }

    #[test]
    fn checker_si_perform_sets_done() {
        let mut c = BopAlgoCheckerSI::new();
        c.perform();
        assert!(c.is_done());
        assert!(!c.has_interferences());
    }

    #[test]
    fn checker_si_level_of_check() {
        let mut c = BopAlgoCheckerSI::new();
        c.set_level_of_check(0);
        assert_eq!(c.level_of_check(), 0);
        c.set_level_of_check(1);
        assert_eq!(c.level_of_check(), 1);
    }

    #[test]
    fn bop_operation_variants_are_distinct() {
        assert_ne!(BopAlgoOperation::Fuse, BopAlgoOperation::Cut);
        assert_ne!(BopAlgoOperation::Common, BopAlgoOperation::Section);
        assert_ne!(BopAlgoOperation::Cut, BopAlgoOperation::Cut21);
    }

    #[test]
    fn check_status_variants_are_distinct() {
        assert_ne!(BopAlgoCheckStatus::NoFault, BopAlgoCheckStatus::BadType);
        assert_ne!(
            BopAlgoCheckStatus::SelfIntersect,
            BopAlgoCheckStatus::TooSmallEdge
        );
        assert_ne!(
            BopAlgoCheckStatus::GeomAbs_C0,
            BopAlgoCheckStatus::InvalidCurveOnSurface
        );
    }

    #[test]
    fn bop_algo_bop_delegates_to_builder() {
        let mut bop = BopAlgoBop::new(BopAlgoOperation::Fuse);
        assert_eq!(bop.operation(), BopAlgoOperation::Fuse);
        bop.set_fuzzy_value(0.001);
        assert!((bop.fuzzy_value() - 0.001).abs() < f64::EPSILON);
        bop.set_non_destructive(true);
        assert!(bop.non_destructive());
        assert!(!bop.is_done());
        bop.build();
        assert!(bop.is_done());
        assert!(!bop.has_errors());
        assert!(!bop.has_warnings());
    }
}
