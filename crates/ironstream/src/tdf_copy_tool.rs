// FILE: tdf_copy_tool.rs
// occt: TDF_CopyTool, TDF_RelocationTable, TDF_CopyLabel

/// Maps old label/attribute ids to new ones during a copy.
/// occt: TDF_RelocationTable
#[derive(Clone, Debug, Default)]
pub struct TdfRelocationTable {
    pub label_map: Vec<(u32, u32)>,     // (source_id, target_id)
    pub attr_map: Vec<(u32, u32)>,      // (source_attr_id, target_attr_id)
    pub self_relocation: bool,
}

impl TdfRelocationTable {
    pub fn new() -> Self { Self::default() }

    pub fn set_self_relocation(&mut self, v: bool) { self.self_relocation = v; }

    pub fn bind_label(&mut self, src: u32, tgt: u32) {
        if let Some(e) = self.label_map.iter_mut().find(|(s,_)| *s == src) {
            e.1 = tgt;
        } else {
            self.label_map.push((src, tgt));
        }
    }

    pub fn bind_attr(&mut self, src: u32, tgt: u32) {
        if let Some(e) = self.attr_map.iter_mut().find(|(s,_)| *s == src) {
            e.1 = tgt;
        } else {
            self.attr_map.push((src, tgt));
        }
    }

    pub fn find_label(&self, src: u32) -> Option<u32> {
        self.label_map.iter().find(|(s,_)| *s == src).map(|(_,t)| *t)
    }

    pub fn find_attr(&self, src: u32) -> Option<u32> {
        self.attr_map.iter().find(|(s,_)| *s == src).map(|(_,t)| *t)
    }

    pub fn is_bound_label(&self, src: u32) -> bool {
        self.label_map.iter().any(|(s,_)| *s == src)
    }

    pub fn nb_labels(&self) -> usize { self.label_map.len() }
    pub fn nb_attrs(&self) -> usize { self.attr_map.len() }

    pub fn clear(&mut self) {
        self.label_map.clear();
        self.attr_map.clear();
    }
}

/// A label entry in a copy operation.
/// occt: TDF_CopyLabel
#[derive(Clone, Debug)]
pub struct TdfCopyLabel {
    pub source_id: u32,
    pub target_id: u32,
    pub is_done: bool,
}

impl TdfCopyLabel {
    pub fn new(source_id: u32, target_id: u32) -> Self {
        Self { source_id, target_id, is_done: false }
    }

    /// Stub perform: marks done and registers in the relocation table.
    pub fn perform(&mut self, reloc: &mut TdfRelocationTable) {
        reloc.bind_label(self.source_id, self.target_id);
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
}

/// Copies a sub-tree of labels with attributes.
/// occt: TDF_CopyTool
#[derive(Clone, Debug, Default)]
pub struct TdfCopyTool {
    pub copies: Vec<TdfCopyLabel>,
}

impl TdfCopyTool {
    pub fn new() -> Self { Self::default() }

    /// Queue a label copy: source → target.
    pub fn add(&mut self, source_id: u32, target_id: u32) {
        self.copies.push(TdfCopyLabel::new(source_id, target_id));
    }

    /// Perform all queued copies and populate the relocation table.
    pub fn perform(&mut self, reloc: &mut TdfRelocationTable) {
        for copy in &mut self.copies {
            copy.perform(reloc);
        }
    }

    pub fn nb_copies(&self) -> usize { self.copies.len() }

    pub fn all_done(&self) -> bool {
        self.copies.iter().all(|c| c.is_done)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reloc_table_bind_find() {
        let mut r = TdfRelocationTable::new();
        r.bind_label(10, 100);
        r.bind_attr(20, 200);
        assert_eq!(r.find_label(10), Some(100));
        assert_eq!(r.find_attr(20), Some(200));
        assert!(!r.is_bound_label(99));
    }

    #[test]
    fn reloc_table_override() {
        let mut r = TdfRelocationTable::new();
        r.bind_label(5, 50);
        r.bind_label(5, 55);
        assert_eq!(r.find_label(5), Some(55));
        assert_eq!(r.nb_labels(), 1);
    }

    #[test]
    fn copy_label_perform() {
        let mut r = TdfRelocationTable::new();
        let mut c = TdfCopyLabel::new(1, 100);
        assert!(!c.is_done());
        c.perform(&mut r);
        assert!(c.is_done());
        assert_eq!(r.find_label(1), Some(100));
    }

    #[test]
    fn copy_tool_all_done() {
        let mut r = TdfRelocationTable::new();
        let mut t = TdfCopyTool::new();
        t.add(1, 10);
        t.add(2, 20);
        assert_eq!(t.nb_copies(), 2);
        assert!(!t.all_done());
        t.perform(&mut r);
        assert!(t.all_done());
        assert_eq!(r.nb_labels(), 2);
    }

    #[test]
    fn reloc_clear() {
        let mut r = TdfRelocationTable::new();
        r.bind_label(1, 10);
        r.bind_attr(2, 20);
        r.clear();
        assert_eq!(r.nb_labels(), 0);
        assert_eq!(r.nb_attrs(), 0);
    }
}
