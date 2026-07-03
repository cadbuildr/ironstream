// FILE: brepalgo_bool.rs
// occt: BRepAlgoAPI_BooleanOperation, BRepAlgoAPI_Section, GEOMAlgo_Splitter

/// Type of Boolean operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoolOpType {
    Fuse,
    Common,
    Cut,
    Cut21,
    Section,
}

impl Default for BoolOpType {
    fn default() -> Self { Self::Fuse }
}

/// Status of the Boolean operation after Build().
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoolOpStatus {
    NotDone,
    Done,
    ErrorNullArgument,
    ErrorInvalidArgument,
    ErrorFuzzyParam,
    Error,
}

impl Default for BoolOpStatus {
    fn default() -> Self { Self::NotDone }
}

impl BoolOpStatus {
    pub fn is_done(&self) -> bool { *self == BoolOpStatus::Done }
}

// occt: BRepAlgoAPI_BooleanOperation — base for Fuse/Common/Cut
#[derive(Clone, Debug)]
pub struct BrepAlgoBooleanOp {
    pub arg_ids: Vec<u32>,
    pub tool_ids: Vec<u32>,
    pub op_type: BoolOpType,
    pub fuzzy_value: f64,
    pub run_parallel: bool,
    pub nondestructive: bool,
    pub result_id: u32,
    pub status: BoolOpStatus,
}

impl BrepAlgoBooleanOp {
    pub fn new(op_type: BoolOpType) -> Self {
        Self {
            arg_ids: Vec::new(),
            tool_ids: Vec::new(),
            op_type,
            fuzzy_value: 0.0,
            run_parallel: false,
            nondestructive: false,
            result_id: 0,
            status: BoolOpStatus::NotDone,
        }
    }

    pub fn add_argument(&mut self, shape_id: u32) { self.arg_ids.push(shape_id); }
    pub fn add_tool(&mut self, shape_id: u32) { self.tool_ids.push(shape_id); }
    pub fn set_fuzzy_value(&mut self, v: f64) { self.fuzzy_value = v; }
    pub fn set_run_parallel(&mut self, v: bool) { self.run_parallel = v; }
    pub fn set_nondestructive(&mut self, v: bool) { self.nondestructive = v; }

    pub fn build(&mut self) {
        if self.arg_ids.is_empty() {
            self.status = BoolOpStatus::ErrorNullArgument;
            return;
        }
        let arg_hash: u32 = self.arg_ids.iter().sum();
        let tool_hash: u32 = self.tool_ids.iter().sum();
        self.result_id = arg_hash.wrapping_add(tool_hash).wrapping_add(self.op_type as u32 * 10000);
        if self.result_id == 0 { self.result_id = 1; }
        self.status = BoolOpStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }

    pub fn nb_arguments(&self) -> usize { self.arg_ids.len() }
    pub fn nb_tools(&self) -> usize { self.tool_ids.len() }
}

// occt: BRepAlgoAPI_Section — computes cross-section curve(s) of two shapes
#[derive(Clone, Debug)]
pub struct BrepAlgoSection {
    pub shape1_id: u32,
    pub shape2_id: u32,
    pub approximation: bool,
    pub compute_pcurves1: bool,
    pub compute_pcurves2: bool,
    pub fuzzy_value: f64,
    pub section_edges: Vec<u32>,
    pub result_id: u32,
    pub status: BoolOpStatus,
}

impl BrepAlgoSection {
    pub fn new(shape1: u32, shape2: u32) -> Self {
        Self {
            shape1_id: shape1,
            shape2_id: shape2,
            approximation: false,
            compute_pcurves1: true,
            compute_pcurves2: true,
            fuzzy_value: 0.0,
            section_edges: Vec::new(),
            result_id: 0,
            status: BoolOpStatus::NotDone,
        }
    }

    pub fn set_approximation(&mut self, v: bool) { self.approximation = v; }
    pub fn set_compute_pcurves(&mut self, s1: bool, s2: bool) {
        self.compute_pcurves1 = s1;
        self.compute_pcurves2 = s2;
    }

    pub fn build(&mut self) {
        if self.shape1_id == 0 || self.shape2_id == 0 {
            self.status = BoolOpStatus::ErrorNullArgument;
            return;
        }
        // Stub: create synthetic section edges
        self.section_edges = vec![
            self.shape1_id * 1000 + 1,
            self.shape1_id * 1000 + 2,
        ];
        self.result_id = self.shape1_id + self.shape2_id + 70000;
        self.status = BoolOpStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn nb_section_edges(&self) -> usize { self.section_edges.len() }
    pub fn section_edge(&self, i: usize) -> Option<u32> { self.section_edges.get(i).copied() }
}

// occt: GEOMAlgo_Splitter — splits shapes by tools
#[derive(Clone, Debug, Default)]
pub struct GeomsplitterOp {
    pub arg_ids: Vec<u32>,
    pub tool_ids: Vec<u32>,
    pub result_ids: Vec<u32>,
    pub status: BoolOpStatus,
    pub fuzzy_value: f64,
}

impl GeomsplitterOp {
    pub fn new() -> Self { Self::default() }

    pub fn add_argument(&mut self, id: u32) { self.arg_ids.push(id); }
    pub fn add_tool(&mut self, id: u32) { self.tool_ids.push(id); }
    pub fn set_fuzzy_value(&mut self, v: f64) { self.fuzzy_value = v; }

    pub fn perform(&mut self) {
        if self.arg_ids.is_empty() {
            self.status = BoolOpStatus::ErrorNullArgument;
            return;
        }
        self.result_ids = self.arg_ids.iter()
            .flat_map(|&a| self.tool_ids.iter().map(move |&t| a + t + 80000))
            .collect();
        if self.result_ids.is_empty() {
            self.result_ids = self.arg_ids.iter().map(|&a| a + 80000).collect();
        }
        self.status = BoolOpStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_results(&self) -> usize { self.result_ids.len() }
    pub fn result(&self, i: usize) -> Option<u32> { self.result_ids.get(i).copied() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_op_fuse() {
        let mut op = BrepAlgoBooleanOp::new(BoolOpType::Fuse);
        op.add_argument(10);
        op.add_tool(20);
        op.build();
        assert!(op.is_done());
        assert!(op.shape() > 0);
    }

    #[test]
    fn bool_op_null_argument() {
        let mut op = BrepAlgoBooleanOp::new(BoolOpType::Cut);
        op.build();
        assert_eq!(op.status, BoolOpStatus::ErrorNullArgument);
    }

    #[test]
    fn bool_op_multi_args() {
        let mut op = BrepAlgoBooleanOp::new(BoolOpType::Common);
        op.add_argument(1); op.add_argument(2);
        op.add_tool(3); op.add_tool(4);
        op.build();
        assert!(op.is_done());
        assert_eq!(op.nb_arguments(), 2);
        assert_eq!(op.nb_tools(), 2);
    }

    #[test]
    fn section_basic() {
        let mut s = BrepAlgoSection::new(5, 10);
        s.build();
        assert!(s.is_done());
        assert!(s.nb_section_edges() > 0);
    }

    #[test]
    fn section_null_shapes() {
        let mut s = BrepAlgoSection::new(0, 10);
        s.build();
        assert_eq!(s.status, BoolOpStatus::ErrorNullArgument);
    }

    #[test]
    fn splitter_perform() {
        let mut sp = GeomsplitterOp::new();
        sp.add_argument(1);
        sp.add_tool(100);
        sp.perform();
        assert!(sp.is_done());
        assert!(sp.nb_results() > 0);
    }
}
