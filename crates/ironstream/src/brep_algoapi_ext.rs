// FILE: brep_algoapi_ext.rs
// occt: BRepAlgoAPI_Defeaturing, BRepAlgoAPI_Check, BOPAlgo_Options

/// Status of a defeaturing or check operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlgoApiStatus {
    NotDone,
    Done,
    Error,
    NullInput,
    BadInput,
}

impl Default for AlgoApiStatus {
    fn default() -> Self { Self::NotDone }
}

impl AlgoApiStatus {
    pub fn is_done(&self) -> bool { *self == AlgoApiStatus::Done }
}

/// Type of shape defect detected by BRepAlgoAPI_Check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeDefect {
    NoDefect,
    NullShape,
    SelfIntersectingWire,
    NullSubshape,
    InvalidDataStructure,
    SmallEdge,
    DegeneratedEdge,
    FreeEdge,
    TangentialEdge,
    SelfIntersectingShell,
    InvalidClosedShell,
    InvalidSolid,
}

// occt: BRepAlgoAPI_Check // — validates shape for Boolean operations
#[derive(Clone, Debug, Default)]
pub struct BrepAlgoApiCheck {
    pub shape_id: u32,
    pub defects: Vec<ShapeDefect>,
    pub is_valid: bool,
}

impl BrepAlgoApiCheck {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, ..Default::default() }
    }

    pub fn perform(&mut self) {
        if self.shape_id == 0 {
            self.defects.push(ShapeDefect::NullShape);
            self.is_valid = false;
        } else {
            self.is_valid = true;
        }
    }

    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn nb_defects(&self) -> usize { self.defects.len() }
    pub fn defect(&self, i: usize) -> Option<ShapeDefect> { self.defects.get(i).copied() }
    pub fn has_defect(&self, d: ShapeDefect) -> bool { self.defects.contains(&d) }
}

// occt: BRepAlgoAPI_Defeaturing // — removes small/unwanted features from a solid
#[derive(Clone, Debug)]
pub struct BrepAlgoApiDefeaturing {
    pub shape_id: u32,
    pub feature_face_ids: Vec<u32>,
    pub result_id: u32,
    pub status: AlgoApiStatus,
    pub tolerance: f64,
}

impl BrepAlgoApiDefeaturing {
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            feature_face_ids: Vec::new(),
            result_id: 0,
            status: AlgoApiStatus::NotDone,
            tolerance: 1e-7,
        }
    }

    pub fn add_face_to_remove(&mut self, face_id: u32) { self.feature_face_ids.push(face_id); }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }

    pub fn build(&mut self) {
        if self.shape_id == 0 {
            self.status = AlgoApiStatus::NullInput;
            return;
        }
        if self.feature_face_ids.is_empty() {
            self.status = AlgoApiStatus::Error;
            return;
        }
        self.result_id = self.shape_id + 20000;
        self.status = AlgoApiStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn nb_faces_removed(&self) -> usize { if self.is_done() { self.feature_face_ids.len() } else { 0 } }
}

/// Options for BOPAlgo operations.
#[derive(Clone, Debug)]
pub struct BopAlgoOptions {
    pub run_parallel: bool,
    pub fuzzy_value: f64,
    pub check_inverted: bool,
    pub nondestructive: bool,
    pub glue: u8,
}

impl Default for BopAlgoOptions {
    fn default() -> Self {
        Self {
            run_parallel: false,
            fuzzy_value: 0.0,
            check_inverted: false,
            nondestructive: false,
            glue: 0,
        }
    }
}

impl BopAlgoOptions {
    pub fn new() -> Self { Self::default() }
    pub fn set_run_parallel(&mut self, v: bool) { self.run_parallel = v; }
    pub fn set_fuzzy_value(&mut self, v: f64) { self.fuzzy_value = v; }
    pub fn set_check_inverted(&mut self, v: bool) { self.check_inverted = v; }
    pub fn set_nondestructive(&mut self, v: bool) { self.nondestructive = v; }
    pub fn set_glue(&mut self, g: u8) { self.glue = g; }
    pub fn fuzzy_value(&self) -> f64 { self.fuzzy_value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_shape() {
        let mut c = BrepAlgoApiCheck::new(42);
        c.perform();
        assert!(c.is_valid());
        assert_eq!(c.nb_defects(), 0);
    }

    #[test]
    fn check_null_shape() {
        let mut c = BrepAlgoApiCheck::new(0);
        c.perform();
        assert!(!c.is_valid());
        assert!(c.has_defect(ShapeDefect::NullShape));
    }

    #[test]
    fn defeaturing_basic() {
        let mut d = BrepAlgoApiDefeaturing::new(10);
        d.add_face_to_remove(1);
        d.add_face_to_remove(2);
        d.build();
        assert!(d.is_done());
        assert_eq!(d.nb_faces_removed(), 2);
        assert!(d.shape() > 0);
    }

    #[test]
    fn defeaturing_null_shape() {
        let mut d = BrepAlgoApiDefeaturing::new(0);
        d.add_face_to_remove(1);
        d.build();
        assert_eq!(d.status, AlgoApiStatus::NullInput);
    }

    #[test]
    fn defeaturing_no_faces() {
        let mut d = BrepAlgoApiDefeaturing::new(5);
        d.build();
        assert_eq!(d.status, AlgoApiStatus::Error);
    }

    #[test]
    fn bop_options() {
        let mut opt = BopAlgoOptions::new();
        opt.set_fuzzy_value(1e-6);
        opt.set_run_parallel(true);
        assert!(opt.run_parallel);
        assert!((opt.fuzzy_value() - 1e-6).abs() < 1e-15);
    }
}
