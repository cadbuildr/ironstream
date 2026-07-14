// FILE: transfer_b_rep_reader.rs
// occt: TransferBRep_Reader

//! Faithful port of OCCT `TransferBRep_Reader` (DataExchange/TKXSBase):
//! a simple, easy-to-call way of transferring data from interface files to
//! shapes. Specialized per norm/protocol by defining how to read a file and
//! providing an actor that performs the entity → shape transfer.
//!
//! Semantics mirrored from TransferBRep_Reader.cxx:
//! - `FileStatus`: 0 = OK, < 0 = file not found, > 0 = syntax/read error.
//! - `SetModel` clears results and Done state (but keeps protocol/actor).
//! - `BeginTransfer` makes a new process when `ModeNewTransfer` is set or
//!   none exists yet, else clears the existing one; then `PrepareTransfer`.
//! - `EndTransfer` collects produced shapes and transient root results.
//! - `OneShape`: void shape if empty, the single shape if one, else compound.

/// Minimal shape result carried by the reader (stands in for TopoDS_Shape).
#[derive(Debug, Clone, PartialEq)]
pub enum ReaderShape {
    /// Null shape — returned when nothing is bound to an entity.
    Null,
    /// A simple shape produced from one entity (tagged by entity id).
    Simple(i32),
    /// A compound of several shapes (OneShape over multiple results).
    Compound(Vec<ReaderShape>),
}

impl ReaderShape {
    pub fn is_null(&self) -> bool {
        matches!(self, ReaderShape::Null)
    }
}

/// The transfer actor: decides whether an entity is Geom-Topol and, if so,
/// produces a shape for it (mirrors Transfer_ActorOfTransientProcess).
pub type ReaderActor = fn(entity: i32) -> Option<ReaderShape>;

/// Stands in for the Interface_InterfaceModel: an ordered set of entities
/// (1-based ranks as in OCCT) with a subset marked as roots.
#[derive(Debug, Clone, Default)]
pub struct ReaderModel {
    entities: Vec<i32>,
    roots: Vec<usize>, // 1-based ranks of root entities
}

impl ReaderModel {
    pub fn new(entities: Vec<i32>, roots: Vec<usize>) -> Self {
        ReaderModel { entities, roots }
    }

    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn value(&self, num: usize) -> Option<i32> {
        if num >= 1 && num <= self.entities.len() {
            Some(self.entities[num - 1])
        } else {
            None
        }
    }
}

/// The last-transfer process record (stands in for
/// Transfer_TransientProcess): roots transferred and their bound results.
#[derive(Debug, Default)]
struct TransientProcess {
    roots: Vec<i32>,
    bound: Vec<(i32, ReaderShape)>,
}

impl TransientProcess {
    fn clear(&mut self) {
        self.roots.clear();
        self.bound.clear();
    }

    fn find(&self, ent: i32) -> Option<&ReaderShape> {
        self.bound.iter().find(|(e, _)| *e == ent).map(|(_, s)| s)
    }
}

// occt: TransferBRep_Reader // — the reader itself
pub struct TransferBRepReader {
    done: bool,
    proc: Option<TransientProcess>,
    actor: Option<ReaderActor>,
    model: Option<ReaderModel>,
    file_status: i32,
    new_process_mode: bool,
    shapes: Vec<ReaderShape>,
    transients: Vec<i32>,
}

impl Default for TransferBRepReader {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferBRepReader {
    /// Initializes a non-specialised reader (theDone=false, theFilest=0,
    /// theNewpr=false, empty result sequences — as the OCCT ctor).
    pub fn new() -> Self {
        TransferBRepReader {
            done: false,
            proc: None,
            actor: None,
            model: None,
            file_status: 0,
            new_process_mode: false,
            shapes: Vec::new(),
            transients: Vec::new(),
        }
    }

    /// Records the actor to be used for transfers.
    pub fn set_actor(&mut self, actor: ReaderActor) {
        self.actor = Some(actor);
    }

    pub fn actor(&self) -> Option<ReaderActor> {
        self.actor
    }

    /// File status: 0 OK, < 0 file not found, > 0 read error.
    pub fn set_file_status(&mut self, status: i32) {
        self.file_status = status;
    }

    pub fn file_status(&self) -> i32 {
        self.file_status
    }

    pub fn file_not_found(&self) -> bool {
        self.file_status < 0
    }

    pub fn syntax_error(&self) -> bool {
        self.file_status > 0
    }

    /// Specifies a model to work on. Also clears the result and Done status.
    pub fn set_model(&mut self, model: ReaderModel) {
        self.model = Some(model);
        self.clear();
    }

    pub fn model(&self) -> Option<&ReaderModel> {
        self.model.as_ref()
    }

    /// Clears the result and Done status. But not the model.
    pub fn clear(&mut self) {
        self.done = false;
        self.shapes.clear();
        self.transients.clear();
    }

    /// Mode for new transfer: true means each new transfer produces a new
    /// process; else the original is kept but cleared per transfer.
    pub fn mode_new_transfer(&mut self) -> &mut bool {
        &mut self.new_process_mode
    }

    /// Initializes the reader for a transfer. Returns false if no model.
    pub fn begin_transfer(&mut self) -> bool {
        self.done = false;
        if self.model.is_none() {
            return false;
        }
        if self.new_process_mode || self.proc.is_none() {
            self.proc = Some(TransientProcess::default());
        } else if let Some(p) = self.proc.as_mut() {
            p.clear();
        }
        self.prepare_transfer();
        true
    }

    /// Ends a transfer by recording its result (shapes + transient roots).
    pub fn end_transfer(&mut self) {
        if let Some(p) = self.proc.as_ref() {
            for (_, shape) in &p.bound {
                self.shapes.push(shape.clone());
            }
            for root in &p.roots {
                if p.find(*root).is_some() {
                    self.transients.push(*root);
                }
            }
        }
        self.done = true;
    }

    /// The provided default does nothing (redefinable in OCCT subclasses).
    pub fn prepare_transfer(&mut self) {}

    /// Transfers all root entities recognized as Geom-Topol.
    /// Former result is cleared.
    pub fn transfer_roots(&mut self) {
        self.clear();
        if !self.begin_transfer() {
            return;
        }
        let (roots, actor) = match (self.model.as_ref(), self.actor) {
            (Some(m), Some(a)) => (m.roots.clone(), a),
            _ => {
                self.end_transfer();
                return;
            }
        };
        let entities: Vec<(usize, i32)> = {
            let m = self.model.as_ref().unwrap();
            roots
                .iter()
                .filter_map(|&r| m.value(r).map(|e| (r, e)))
                .collect()
        };
        if let Some(p) = self.proc.as_mut() {
            for (_, ent) in entities {
                p.roots.push(ent);
                if let Some(shape) = actor(ent) {
                    p.bound.push((ent, shape));
                }
            }
        }
        self.end_transfer();
    }

    /// Transfers an entity given its rank (root or not). Returns true if it
    /// is in range (recognition failure shows via `is_done`/empty result).
    pub fn transfer(&mut self, num: usize) -> bool {
        if !self.begin_transfer() {
            return false;
        }
        let ent = match self.model.as_ref().and_then(|m| m.value(num)) {
            Some(e) => e,
            None => return false,
        };
        if let (Some(p), Some(actor)) = (self.proc.as_mut(), self.actor) {
            if let Some(shape) = actor(ent) {
                p.bound.push((ent, shape));
            }
            p.roots.push(ent);
        }
        self.end_transfer();
        true
    }

    /// Transfers a list of entities (only the ones also in the model).
    /// Former result is cleared.
    pub fn transfer_list(&mut self, list: &[i32]) {
        self.clear();
        if !self.begin_transfer() {
            return;
        }
        let actor = match self.actor {
            Some(a) => a,
            None => {
                self.end_transfer();
                return;
            }
        };
        let in_model: Vec<i32> = {
            let m = self.model.as_ref().unwrap();
            list.iter().copied().filter(|e| m.entities.contains(e)).collect()
        };
        if let Some(p) = self.proc.as_mut() {
            for ent in in_model {
                p.roots.push(ent);
                if let Some(shape) = actor(ent) {
                    p.bound.push((ent, shape));
                }
            }
        }
        self.end_transfer();
    }

    /// True if the LAST transfer was a success.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Count of produced shapes (roots).
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    pub fn shapes(&self) -> &[ReaderShape] {
        &self.shapes
    }

    /// Returns a shape given its 1-based rank (default the first one);
    /// Null if out of range.
    pub fn shape(&self, num: usize) -> ReaderShape {
        if num >= 1 && num <= self.shapes.len() {
            self.shapes[num - 1].clone()
        } else {
            ReaderShape::Null
        }
    }

    /// Shape produced from a given entity in the last transfer, else Null.
    pub fn shape_result(&self, ent: i32) -> ReaderShape {
        self.proc
            .as_ref()
            .and_then(|p| p.find(ent).cloned())
            .unwrap_or(ReaderShape::Null)
    }

    /// A unique shape for the result: Null if empty, the single shape if
    /// one, a compound if more than one.
    pub fn one_shape(&self) -> ReaderShape {
        match self.shapes.len() {
            0 => ReaderShape::Null,
            1 => self.shapes[0].clone(),
            _ => ReaderShape::Compound(self.shapes.clone()),
        }
    }

    /// Count of produced transient results (roots).
    pub fn nb_transients(&self) -> usize {
        self.transients.len()
    }

    pub fn transients(&self) -> &[i32] {
        &self.transients
    }

    /// A transient root result given its 1-based rank (default first).
    pub fn transient(&self, num: usize) -> Option<i32> {
        if num >= 1 && num <= self.transients.len() {
            Some(self.transients[num - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Actor recognizing even entity ids as Geom-Topol.
    fn even_actor(ent: i32) -> Option<ReaderShape> {
        if ent % 2 == 0 {
            Some(ReaderShape::Simple(ent))
        } else {
            None
        }
    }

    fn model() -> ReaderModel {
        // entities ranks 1..=5, roots at ranks 1, 2, 4 (ids 10, 21, 42)
        ReaderModel::new(vec![10, 21, 30, 42, 55], vec![1, 2, 4])
    }

    #[test]
    fn file_status_semantics() {
        let mut r = TransferBRepReader::new();
        assert_eq!(r.file_status(), 0);
        assert!(!r.file_not_found() && !r.syntax_error());
        r.set_file_status(-1);
        assert!(r.file_not_found() && !r.syntax_error());
        r.set_file_status(3);
        assert!(!r.file_not_found() && r.syntax_error());
    }

    #[test]
    fn begin_transfer_requires_model() {
        let mut r = TransferBRepReader::new();
        assert!(!r.begin_transfer());
        r.set_model(model());
        assert!(r.begin_transfer());
    }

    #[test]
    fn transfer_roots_collects_recognized_shapes() {
        let mut r = TransferBRepReader::new();
        r.set_model(model());
        r.set_actor(even_actor);
        r.transfer_roots();
        assert!(r.is_done());
        // roots are ids 10, 21, 42; even_actor recognizes 10 and 42
        assert_eq!(r.nb_shapes(), 2);
        assert_eq!(r.shape(1), ReaderShape::Simple(10));
        assert_eq!(r.shape(2), ReaderShape::Simple(42));
        assert_eq!(r.nb_transients(), 2);
        assert_eq!(r.transient(1), Some(10));
    }

    #[test]
    fn transfer_single_entity_bounds_checked() {
        let mut r = TransferBRepReader::new();
        r.set_model(model());
        r.set_actor(even_actor);
        assert!(!r.transfer(0));
        assert!(!r.transfer(6));
        assert!(r.transfer(3)); // id 30, recognized
        assert!(r.is_done());
        assert_eq!(r.shape_result(30), ReaderShape::Simple(30));
        assert_eq!(r.shape_result(21), ReaderShape::Null);
    }

    #[test]
    fn one_shape_void_single_compound() {
        let mut r = TransferBRepReader::new();
        r.set_model(ReaderModel::new(vec![7], vec![1]));
        r.set_actor(even_actor);
        r.transfer_roots(); // 7 is odd → nothing produced
        assert!(r.one_shape().is_null());

        r.set_model(ReaderModel::new(vec![4], vec![1]));
        r.transfer_roots();
        assert_eq!(r.one_shape(), ReaderShape::Simple(4));

        r.set_model(model());
        r.transfer_roots();
        match r.one_shape() {
            ReaderShape::Compound(v) => assert_eq!(v.len(), 2),
            other => panic!("expected compound, got {:?}", other),
        }
    }

    #[test]
    fn set_model_clears_results_and_done() {
        let mut r = TransferBRepReader::new();
        r.set_model(model());
        r.set_actor(even_actor);
        r.transfer_roots();
        assert!(r.is_done() && r.nb_shapes() > 0);
        r.set_model(model());
        assert!(!r.is_done());
        assert_eq!(r.nb_shapes(), 0);
    }
}
