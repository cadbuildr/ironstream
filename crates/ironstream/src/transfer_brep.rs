// FILE: transfer_brep.rs
// occt: TransferBRep_ShapeBinder, TransferBRep_ShapeMapper
// occt-ref: Transfer_Binder, Transfer_FinderProcess

use std::collections::HashMap;

/// Status of a transfer operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferStatus {
    Initial,
    Run,
    Done,
    Error,
    Void,
}

impl Default for TransferStatus {
    fn default() -> Self { Self::Initial }
}

impl TransferStatus {
    pub fn is_done(&self) -> bool { *self == TransferStatus::Done }
}

// occt-ref: Transfer_Binder // — wraps a result entity after transfer
#[derive(Clone, Debug)]
pub struct TransferBinder {
    pub status: TransferStatus,
    pub result_id: u32,
    pub check_messages: Vec<String>,
    pub has_result: bool,
}

impl Default for TransferBinder {
    fn default() -> Self {
        Self { status: TransferStatus::Initial, result_id: 0, check_messages: Vec::new(), has_result: false }
    }
}

impl TransferBinder {
    pub fn new() -> Self { Self::default() }

    pub fn set_result(&mut self, id: u32) {
        self.result_id = id;
        self.has_result = true;
        self.status = TransferStatus::Done;
    }

    pub fn add_message(&mut self, msg: &str) { self.check_messages.push(msg.to_owned()); }

    pub fn status(&self) -> TransferStatus { self.status }
    pub fn has_result(&self) -> bool { self.has_result }
    pub fn result(&self) -> u32 { self.result_id }
    pub fn nb_messages(&self) -> usize { self.check_messages.len() }
    pub fn is_done(&self) -> bool { self.status.is_done() }
}

// occt: TransferBRep_ShapeBinder // — binder that carries a shape (by ID)
#[derive(Clone, Debug, Default)]
pub struct ShapeBinder {
    pub binder: TransferBinder,
    pub shape_id: u32,
    pub shape_type: u8,   // 0=unknown, 1=vertex, 2=edge, 3=wire, 4=face, 5=shell, 6=solid, 7=compound
}

impl ShapeBinder {
    pub fn new() -> Self { Self::default() }

    pub fn set_shape(&mut self, shape_id: u32, shape_type: u8) {
        self.shape_id = shape_id;
        self.shape_type = shape_type;
        self.binder.set_result(shape_id);
    }

    pub fn result_shape(&self) -> u32 { self.shape_id }
    pub fn shape_type(&self) -> u8 { self.shape_type }
    pub fn is_done(&self) -> bool { self.binder.is_done() }
}

// occt: TransferBRep_ShapeMapper // — maps a source entity to a shape
#[derive(Clone, Debug)]
pub struct ShapeMapper {
    pub source_id: u32,
    pub target_shape_id: u32,
    pub is_mapped: bool,
}

impl ShapeMapper {
    pub fn new(source_id: u32) -> Self {
        Self { source_id, target_shape_id: 0, is_mapped: false }
    }

    pub fn map_to(&mut self, shape_id: u32) {
        self.target_shape_id = shape_id;
        self.is_mapped = true;
    }

    pub fn result(&self) -> Option<u32> {
        if self.is_mapped { Some(self.target_shape_id) } else { None }
    }

    pub fn source_id(&self) -> u32 { self.source_id }
    pub fn is_mapped(&self) -> bool { self.is_mapped }
}

// occt-ref: Transfer_FinderProcess // — manages a collection of finders during transfer
#[derive(Clone, Debug, Default)]
pub struct FinderProcess {
    pub binders: HashMap<u32, ShapeBinder>,
    pub messages: Vec<String>,
    pub nb_roots: usize,
}

impl FinderProcess {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, source_id: u32, binder: ShapeBinder) {
        self.binders.insert(source_id, binder);
    }

    pub fn find(&self, source_id: u32) -> Option<&ShapeBinder> {
        self.binders.get(&source_id)
    }

    pub fn is_bound(&self, source_id: u32) -> bool {
        self.binders.contains_key(&source_id)
    }

    pub fn set_nb_mapped_roots(&mut self, n: usize) { self.nb_roots = n; }
    pub fn nb_mapped(&self) -> usize { self.binders.len() }
    pub fn nb_roots(&self) -> usize { self.nb_roots }

    pub fn add_warning(&mut self, msg: &str) { self.messages.push(format!("WARN: {}", msg)); }
    pub fn add_fail(&mut self, msg: &str) { self.messages.push(format!("FAIL: {}", msg)); }
    pub fn nb_messages(&self) -> usize { self.messages.len() }
    pub fn clear(&mut self) { self.binders.clear(); self.messages.clear(); self.nb_roots = 0; }

    pub fn shape_result(&self, source_id: u32) -> Option<u32> {
        self.binders.get(&source_id).map(|b| b.shape_id)
    }
}

// occt: TransferBRep // — utility functions for BRep transfers
pub struct TransferBrep;

impl TransferBrep {
    pub fn shape_result(process: &FinderProcess, source_id: u32) -> Option<u32> {
        process.shape_result(source_id)
    }

    pub fn shape_results(process: &FinderProcess) -> Vec<u32> {
        process.binders.values().filter(|b| b.is_done()).map(|b| b.shape_id).collect()
    }

    pub fn nb_results(process: &FinderProcess) -> usize {
        process.binders.values().filter(|b| b.is_done()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binder_set_result() {
        let mut b = TransferBinder::new();
        assert!(!b.is_done());
        b.set_result(42);
        assert!(b.is_done());
        assert_eq!(b.result(), 42);
    }

    #[test]
    fn binder_messages() {
        let mut b = TransferBinder::new();
        b.add_message("warning: gap detected");
        assert_eq!(b.nb_messages(), 1);
    }

    #[test]
    fn shape_binder_basic() {
        let mut sb = ShapeBinder::new();
        sb.set_shape(100, 4);
        assert!(sb.is_done());
        assert_eq!(sb.result_shape(), 100);
        assert_eq!(sb.shape_type(), 4);
    }

    #[test]
    fn shape_mapper_basic() {
        let mut m = ShapeMapper::new(10);
        assert!(!m.is_mapped());
        m.map_to(99);
        assert!(m.is_mapped());
        assert_eq!(m.result(), Some(99));
    }

    #[test]
    fn finder_process_bind_find() {
        let mut fp = FinderProcess::new();
        let mut sb = ShapeBinder::new();
        sb.set_shape(200, 6);
        fp.bind(1, sb);
        assert!(fp.is_bound(1));
        assert_eq!(fp.shape_result(1), Some(200));
        assert_eq!(fp.nb_mapped(), 1);
    }

    #[test]
    fn transfer_brep_shape_results() {
        let mut fp = FinderProcess::new();
        let mut b1 = ShapeBinder::new(); b1.set_shape(10, 4);
        let mut b2 = ShapeBinder::new(); b2.set_shape(20, 5);
        fp.bind(1, b1);
        fp.bind(2, b2);
        assert_eq!(TransferBrep::nb_results(&fp), 2);
        let shapes = TransferBrep::shape_results(&fp);
        assert!(shapes.contains(&10) && shapes.contains(&20));
    }
}
