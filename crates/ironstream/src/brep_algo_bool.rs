// FILE: brep_algo_bool.rs

// occt: BOPAlgo_Operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoolOpType {
    Fuse,
    Common,
    Cut,
    Section,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoolOpError {
    NotDone,
    EmptyShape,
    SelfIntersect,
    InvalidInput,
}

// occt: BRepAlgoAPI_BooleanOperation
pub struct BRepAlgoBoolOp {
    operation: BoolOpType,
    is_done: bool,
    nb_warnings: u32,
    fuzzy_value: f64,
}

impl BRepAlgoBoolOp {
    pub fn new(op: BoolOpType) -> Self {
        Self {
            operation: op,
            is_done: false,
            nb_warnings: 0,
            fuzzy_value: 0.0,
        }
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.fuzzy_value = v;
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.fuzzy_value
    }

    pub fn build(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn operation(&self) -> BoolOpType {
        self.operation
    }

    pub fn nb_warnings(&self) -> u32 {
        self.nb_warnings
    }

    pub fn has_warnings(&self) -> bool {
        self.nb_warnings > 0
    }

    pub fn has_errors(&self) -> bool {
        false
    }
}

// occt: BRepAlgoAPI_Fuse
pub struct BRepAlgoFuse {
    inner: BRepAlgoBoolOp,
}

impl BRepAlgoFuse {
    pub fn new() -> Self {
        Self {
            inner: BRepAlgoBoolOp::new(BoolOpType::Fuse),
        }
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.inner.set_fuzzy_value(v);
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.inner.fuzzy_value()
    }

    pub fn build(&mut self) {
        self.inner.build();
    }

    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    pub fn operation(&self) -> BoolOpType {
        self.inner.operation()
    }

    pub fn nb_warnings(&self) -> u32 {
        self.inner.nb_warnings()
    }

    pub fn has_warnings(&self) -> bool {
        self.inner.has_warnings()
    }

    pub fn has_errors(&self) -> bool {
        self.inner.has_errors()
    }
}

impl Default for BRepAlgoFuse {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgoAPI_Common
pub struct BRepAlgoCommon {
    inner: BRepAlgoBoolOp,
}

impl BRepAlgoCommon {
    pub fn new() -> Self {
        Self {
            inner: BRepAlgoBoolOp::new(BoolOpType::Common),
        }
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.inner.set_fuzzy_value(v);
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.inner.fuzzy_value()
    }

    pub fn build(&mut self) {
        self.inner.build();
    }

    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    pub fn operation(&self) -> BoolOpType {
        self.inner.operation()
    }

    pub fn nb_warnings(&self) -> u32 {
        self.inner.nb_warnings()
    }

    pub fn has_warnings(&self) -> bool {
        self.inner.has_warnings()
    }

    pub fn has_errors(&self) -> bool {
        self.inner.has_errors()
    }
}

impl Default for BRepAlgoCommon {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgoAPI_Cut
pub struct BRepAlgoCut {
    inner: BRepAlgoBoolOp,
}

impl BRepAlgoCut {
    pub fn new() -> Self {
        Self {
            inner: BRepAlgoBoolOp::new(BoolOpType::Cut),
        }
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.inner.set_fuzzy_value(v);
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.inner.fuzzy_value()
    }

    pub fn build(&mut self) {
        self.inner.build();
    }

    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    pub fn operation(&self) -> BoolOpType {
        self.inner.operation()
    }

    pub fn nb_warnings(&self) -> u32 {
        self.inner.nb_warnings()
    }

    pub fn has_warnings(&self) -> bool {
        self.inner.has_warnings()
    }

    pub fn has_errors(&self) -> bool {
        self.inner.has_errors()
    }
}

impl Default for BRepAlgoCut {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepAlgoAPI_Section
pub struct BRepAlgoSection {
    inner: BRepAlgoBoolOp,
}

impl BRepAlgoSection {
    pub fn new() -> Self {
        Self {
            inner: BRepAlgoBoolOp::new(BoolOpType::Section),
        }
    }

    pub fn set_fuzzy_value(&mut self, v: f64) {
        self.inner.set_fuzzy_value(v);
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.inner.fuzzy_value()
    }

    pub fn build(&mut self) {
        self.inner.build();
    }

    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    pub fn operation(&self) -> BoolOpType {
        self.inner.operation()
    }

    pub fn nb_warnings(&self) -> u32 {
        self.inner.nb_warnings()
    }

    pub fn has_warnings(&self) -> bool {
        self.inner.has_warnings()
    }

    pub fn has_errors(&self) -> bool {
        self.inner.has_errors()
    }
}

impl Default for BRepAlgoSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_op_new_not_done() {
        let op = BRepAlgoBoolOp::new(BoolOpType::Fuse);
        assert!(!op.is_done());
    }

    #[test]
    fn bool_op_build_sets_done() {
        let mut op = BRepAlgoBoolOp::new(BoolOpType::Cut);
        op.build();
        assert!(op.is_done());
    }

    #[test]
    fn bool_op_operation_roundtrip() {
        for kind in [BoolOpType::Fuse, BoolOpType::Common, BoolOpType::Cut, BoolOpType::Section] {
            let op = BRepAlgoBoolOp::new(kind);
            assert_eq!(op.operation(), kind);
        }
    }

    #[test]
    fn bool_op_fuzzy_default_zero() {
        let op = BRepAlgoBoolOp::new(BoolOpType::Fuse);
        assert_eq!(op.fuzzy_value(), 0.0);
    }

    #[test]
    fn bool_op_set_fuzzy() {
        let mut op = BRepAlgoBoolOp::new(BoolOpType::Common);
        op.set_fuzzy_value(1e-5);
        assert!((op.fuzzy_value() - 1e-5).abs() < 1e-15);
    }

    #[test]
    fn bool_op_has_no_errors() {
        let mut op = BRepAlgoBoolOp::new(BoolOpType::Cut);
        op.build();
        assert!(!op.has_errors());
    }

    #[test]
    fn bool_op_no_warnings_initially() {
        let op = BRepAlgoBoolOp::new(BoolOpType::Section);
        assert_eq!(op.nb_warnings(), 0);
        assert!(!op.has_warnings());
    }

    #[test]
    fn fuse_op_type() {
        let f = BRepAlgoFuse::new();
        assert_eq!(f.operation(), BoolOpType::Fuse);
    }

    #[test]
    fn common_op_type() {
        let c = BRepAlgoCommon::new();
        assert_eq!(c.operation(), BoolOpType::Common);
    }

    #[test]
    fn cut_op_type() {
        let c = BRepAlgoCut::new();
        assert_eq!(c.operation(), BoolOpType::Cut);
    }

    #[test]
    fn section_op_type() {
        let s = BRepAlgoSection::new();
        assert_eq!(s.operation(), BoolOpType::Section);
    }

    #[test]
    fn fuse_build_and_done() {
        let mut f = BRepAlgoFuse::new();
        assert!(!f.is_done());
        f.build();
        assert!(f.is_done());
    }

    #[test]
    fn bool_error_variants_debug() {
        let e = BoolOpError::NotDone;
        let s = format!("{:?}", e);
        assert_eq!(s, "NotDone");
    }
}
