// FILE: rust/ironstream/crates/ironstream/src/brep_algo_check.rs

// occt: BOPAlgo_CheckStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepAlgoCheckErrorType {
    NoError,
    BOPNotValid,
    IncompatibleVertex,
    IncompatibleEdge,
    IncompatibleFace,
    SelfInterferingShape,
    BadType,
    NonRecoverableFace,
    NotImplemented,
}

// occt: BOPAlgo_CheckResult
#[derive(Clone, Debug)]
pub struct BrepAlgoCheckError {
    error_type: BrepAlgoCheckErrorType,
    shape_index: usize,
    message: String,
}

impl BrepAlgoCheckError {
    pub fn new(error_type: BrepAlgoCheckErrorType, shape_index: usize, message: String) -> Self {
        Self {
            error_type,
            shape_index,
            message,
        }
    }

    pub fn error_type(&self) -> BrepAlgoCheckErrorType {
        self.error_type
    }

    pub fn shape_index(&self) -> usize {
        self.shape_index
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// occt: BRepAlgoAPI_Check
pub struct BrepAlgoCheck {
    errors: Vec<BrepAlgoCheckError>,
    is_valid: bool,
}

impl BrepAlgoCheck {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            is_valid: true,
        }
    }

    pub fn check_shape(&mut self, valid: bool, errors: Vec<BrepAlgoCheckError>) {
        self.is_valid = valid;
        self.errors = errors;
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn nb_errors(&self) -> usize {
        self.errors.len()
    }

    pub fn errors(&self) -> &[BrepAlgoCheckError] {
        &self.errors
    }

    pub fn has_faulty_shapes(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for BrepAlgoCheck {
    fn default() -> Self {
        Self::new()
    }
}
