// FILE: occ_transfer.rs
// occt: Transfer_TransientProcess, Transfer_Finder, XSControl_TransferReader

/// Outcome of a transfer/import operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferStatus {
    NotStarted,
    Running,
    Done,
    DoneWithWarning,
    Error,
    Fail,
}

impl Default for TransferStatus {
    fn default() -> Self { Self::NotStarted }
}

impl TransferStatus {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done | Self::DoneWithWarning)
    }
}

/// A single transfer result entry.
#[derive(Clone, Debug)]
pub struct TransferResult {
    pub entity_id: u32,
    pub result_shape_id: u32,
    pub status: TransferStatus,
    pub message: String,
}

impl TransferResult {
    pub fn ok(entity_id: u32, shape_id: u32) -> Self {
        Self { entity_id, result_shape_id: shape_id, status: TransferStatus::Done, message: String::new() }
    }
    pub fn failed(entity_id: u32, msg: &str) -> Self {
        Self { entity_id, result_shape_id: 0, status: TransferStatus::Fail, message: msg.to_owned() }
    }
    pub fn is_done(&self) -> bool { self.status.is_done() }
}

// occt: Transfer_TransientProcess — manages entity-to-result mapping during import
#[derive(Clone, Debug, Default)]
pub struct TransientProcess {
    pub results: Vec<TransferResult>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub check_mode: u8,
}

impl TransientProcess {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, entity_id: u32, shape_id: u32) {
        self.results.push(TransferResult::ok(entity_id, shape_id));
    }

    pub fn find_result(&self, entity_id: u32) -> Option<u32> {
        self.results.iter()
            .find(|r| r.entity_id == entity_id && r.is_done())
            .map(|r| r.result_shape_id)
    }

    pub fn add_error(&mut self, msg: String) { self.errors.push(msg); }
    pub fn add_warning(&mut self, msg: String) { self.warnings.push(msg); }

    pub fn nb_mapped(&self) -> usize { self.results.iter().filter(|r| r.is_done()).count() }
    pub fn nb_errors(&self) -> usize { self.errors.len() }
    pub fn nb_warnings(&self) -> usize { self.warnings.len() }
    pub fn clear(&mut self) { self.results.clear(); self.errors.clear(); self.warnings.clear(); }
}

// occt: Transfer_Finder — locates a result associated with a source entity
#[derive(Clone, Debug)]
pub struct TransferFinder {
    pub finder_id: u32,
    pub entity_id: u32,
    pub result_id: u32,
    pub is_found: bool,
}

impl TransferFinder {
    pub fn new(finder_id: u32) -> Self {
        Self { finder_id, entity_id: 0, result_id: 0, is_found: false }
    }

    pub fn set_entity(&mut self, entity_id: u32) { self.entity_id = entity_id; }
    pub fn set_result(&mut self, result_id: u32) { self.result_id = result_id; self.is_found = true; }
    pub fn is_found(&self) -> bool { self.is_found }
    pub fn result(&self) -> Option<u32> { if self.is_found { Some(self.result_id) } else { None } }
}

// occt: XSControl_TransferReader — high-level reader that orchestrates transfer
#[derive(Clone, Debug, Default)]
pub struct XSControlTransferReader {
    pub model_id: u32,
    pub process: TransientProcess,
    pub root_shape_ids: Vec<u32>,
    pub status: TransferStatus,
}

impl XSControlTransferReader {
    pub fn new(model_id: u32) -> Self {
        Self { model_id, status: TransferStatus::NotStarted, ..Default::default() }
    }

    pub fn transfer_root(&mut self, entity_id: u32) -> bool {
        if self.model_id == 0 { return false; }
        let shape_id = entity_id + 60000;
        self.process.bind(entity_id, shape_id);
        self.root_shape_ids.push(shape_id);
        self.status = TransferStatus::Done;
        true
    }

    pub fn transfer_roots(&mut self, entity_ids: Vec<u32>) -> usize {
        let mut count = 0;
        for id in entity_ids {
            if self.transfer_root(id) { count += 1; }
        }
        count
    }

    pub fn one_shape(&self) -> Option<u32> { self.root_shape_ids.first().copied() }
    pub fn nb_shapes(&self) -> usize { self.root_shape_ids.len() }
    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn clear(&mut self) {
        self.process.clear();
        self.root_shape_ids.clear();
        self.status = TransferStatus::NotStarted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transient_process_bind_find() {
        let mut p = TransientProcess::new();
        p.bind(1, 1001);
        p.bind(2, 1002);
        assert_eq!(p.find_result(1), Some(1001));
        assert_eq!(p.find_result(99), None);
        assert_eq!(p.nb_mapped(), 2);
    }

    #[test]
    fn transient_process_errors() {
        let mut p = TransientProcess::new();
        p.add_error("bad entity".into());
        p.add_warning("possible issue".into());
        assert_eq!(p.nb_errors(), 1);
        assert_eq!(p.nb_warnings(), 1);
        p.clear();
        assert_eq!(p.nb_errors(), 0);
    }

    #[test]
    fn transfer_finder_result() {
        let mut f = TransferFinder::new(42);
        assert!(!f.is_found());
        f.set_result(999);
        assert!(f.is_found());
        assert_eq!(f.result(), Some(999));
    }

    #[test]
    fn transfer_reader_transfer_root() {
        let mut r = XSControlTransferReader::new(1);
        let ok = r.transfer_root(10);
        assert!(ok);
        assert!(r.is_done());
        assert_eq!(r.one_shape(), Some(60010));
    }

    #[test]
    fn transfer_reader_transfer_roots() {
        let mut r = XSControlTransferReader::new(1);
        let n = r.transfer_roots(vec![1, 2, 3]);
        assert_eq!(n, 3);
        assert_eq!(r.nb_shapes(), 3);
    }

    #[test]
    fn transfer_reader_no_model() {
        let mut r = XSControlTransferReader::new(0);
        let ok = r.transfer_root(5);
        assert!(!ok);
        assert_eq!(r.status, TransferStatus::NotStarted);
    }
}
