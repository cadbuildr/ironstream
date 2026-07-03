// FILE: tdf_data.rs
// occt: TDF_Data, TDF_Transaction, TDF_Label, TDF_LabelSequence,
//       TDF_Attribute, TDF_Delta, TDF_RelocationTable

use std::collections::HashMap;

/// Unique label identifier (simplified path).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabelTag(pub Vec<u32>);

impl LabelTag {
    pub fn root() -> Self { Self(vec![]) }
    pub fn child(&self, tag: u32) -> Self {
        let mut v = self.0.clone();
        v.push(tag);
        Self(v)
    }
    pub fn depth(&self) -> usize { self.0.len() }
    pub fn parent(&self) -> Option<Self> {
        if self.0.is_empty() { return None; }
        let mut v = self.0.clone();
        v.pop();
        Some(Self(v))
    }
    pub fn tag(&self) -> Option<u32> { self.0.last().copied() }
}

impl std::fmt::Display for LabelTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0:{}", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(":"))
    }
}

// occt: TDF_Attribute (simplified)
pub trait TdfAttribute: std::fmt::Debug {
    fn id(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct GenericAttribute {
    pub attr_id: String,
    pub payload: String,
}

impl TdfAttribute for GenericAttribute {
    fn id(&self) -> &'static str { "GenericAttribute" }
}

// occt: TDF_Label
#[derive(Clone, Debug)]
pub struct TdfLabel {
    pub tag: LabelTag,
    pub attrs: HashMap<String, String>,
    pub children: Vec<LabelTag>,
    pub is_free: bool,
}

impl TdfLabel {
    pub fn new(tag: LabelTag) -> Self {
        Self { tag, attrs: HashMap::new(), is_free: false, children: Vec::new() }
    }

    pub fn set_attr(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.attrs.insert(key.into(), val.into());
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|s| s.as_str())
    }

    pub fn has_attr(&self, key: &str) -> bool { self.attrs.contains_key(key) }
    pub fn nb_attrs(&self) -> usize { self.attrs.len() }
    pub fn is_root(&self) -> bool { self.tag.0.is_empty() }
    pub fn depth(&self) -> usize { self.tag.depth() }
    pub fn nb_children(&self) -> usize { self.children.len() }
}

// occt: TDF_Delta (change record)
#[derive(Clone, Debug, Default)]
pub struct TdfDelta {
    pub name: String,
    pub changed_labels: Vec<LabelTag>,
    pub is_undo: bool,
}

impl TdfDelta {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Default::default() }
    }
    pub fn nb_changes(&self) -> usize { self.changed_labels.len() }
}

// occt: TDF_Transaction
#[derive(Clone, Debug)]
pub struct TdfTransaction {
    pub name: String,
    pub depth: u32,
    pub modified_labels: Vec<LabelTag>,
    pub is_open: bool,
}

impl TdfTransaction {
    pub fn new(name: impl Into<String>, depth: u32) -> Self {
        Self { name: name.into(), depth, modified_labels: Vec::new(), is_open: true }
    }

    pub fn commit(&mut self) -> TdfDelta {
        self.is_open = false;
        let mut delta = TdfDelta::new(self.name.clone());
        delta.changed_labels = self.modified_labels.clone();
        delta
    }

    pub fn abort(&mut self) {
        self.is_open = false;
        self.modified_labels.clear();
    }

    pub fn mark_modified(&mut self, label: LabelTag) {
        if !self.modified_labels.contains(&label) {
            self.modified_labels.push(label);
        }
    }

    pub fn nb_modified(&self) -> usize { self.modified_labels.len() }
}

// occt: TDF_Data
#[derive(Clone, Debug, Default)]
pub struct TdfData {
    pub labels: HashMap<String, TdfLabel>,
    pub transaction_depth: u32,
    pub nb_labels: usize,
    pub deltas: Vec<TdfDelta>,
}

impl TdfData {
    pub fn new() -> Self { Self::default() }

    fn tag_key(tag: &LabelTag) -> String { tag.to_string() }

    pub fn root(&mut self) -> &TdfLabel {
        let root_tag = LabelTag::root();
        let key = Self::tag_key(&root_tag);
        self.labels.entry(key).or_insert_with(|| TdfLabel::new(root_tag))
    }

    pub fn new_label(&mut self, parent: &LabelTag, child_tag: u32) -> LabelTag {
        let tag = parent.child(child_tag);
        let key = Self::tag_key(&tag);
        self.labels.insert(key, TdfLabel::new(tag.clone()));
        // register as child of parent
        let parent_key = Self::tag_key(parent);
        if let Some(p) = self.labels.get_mut(&parent_key) {
            if !p.children.contains(&tag) {
                p.children.push(tag.clone());
            }
        }
        self.nb_labels = self.labels.len();
        tag
    }

    pub fn find_label(&self, tag: &LabelTag) -> Option<&TdfLabel> {
        self.labels.get(&Self::tag_key(tag))
    }

    pub fn find_label_mut(&mut self, tag: &LabelTag) -> Option<&mut TdfLabel> {
        self.labels.get_mut(&Self::tag_key(tag))
    }

    pub fn open_transaction(&mut self) -> u32 {
        self.transaction_depth += 1;
        self.transaction_depth
    }

    pub fn commit_transaction(&mut self, delta: TdfDelta) {
        if self.transaction_depth > 0 {
            self.transaction_depth -= 1;
            self.deltas.push(delta);
        }
    }

    pub fn abort_transaction(&mut self) {
        if self.transaction_depth > 0 { self.transaction_depth -= 1; }
    }

    pub fn nb_transactions(&self) -> u32 { self.transaction_depth }
    pub fn nb_deltas(&self) -> usize { self.deltas.len() }
}

// occt: TDF_RelocationTable — maps old labels to new labels (for copy/paste)
#[derive(Clone, Debug, Default)]
pub struct RelocationTable {
    pub label_map: HashMap<String, LabelTag>,
}

impl RelocationTable {
    pub fn new() -> Self { Self::default() }

    pub fn set_relocation(&mut self, old: &LabelTag, new_tag: LabelTag) {
        self.label_map.insert(old.to_string(), new_tag);
    }

    pub fn get_relocation(&self, old: &LabelTag) -> Option<&LabelTag> {
        self.label_map.get(&old.to_string())
    }

    pub fn has_relocation(&self, old: &LabelTag) -> bool {
        self.label_map.contains_key(&old.to_string())
    }

    pub fn nb_relocations(&self) -> usize { self.label_map.len() }
}

/// Ordered list of labels.
pub type LabelSequence = Vec<LabelTag>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_tag_path() {
        let root = LabelTag::root();
        let child = root.child(1);
        let grandchild = child.child(3);
        assert_eq!(grandchild.depth(), 2);
        assert_eq!(grandchild.tag(), Some(3));
        assert_eq!(child.parent(), Some(root.clone()));
        assert!(root.0.is_empty() || root.depth() == 0);
    }

    fn root_is_empty(root: &LabelTag) -> bool { root.0.is_empty() }

    #[test]
    fn tdf_label_attrs() {
        let mut lbl = TdfLabel::new(LabelTag::root());
        assert!(lbl.is_root());
        lbl.set_attr("Name", "Root");
        assert_eq!(lbl.get_attr("Name"), Some("Root"));
        assert!(lbl.has_attr("Name"));
        assert_eq!(lbl.nb_attrs(), 1);
    }

    #[test]
    fn tdf_data_new_label() {
        let mut data = TdfData::new();
        let root = LabelTag::root();
        let child = data.new_label(&root, 1);
        assert_eq!(child.depth(), 1);
        assert!(data.find_label(&child).is_some());
        assert_eq!(data.nb_labels, data.labels.len());
    }

    #[test]
    fn tdf_transaction_commit() {
        let mut tx = TdfTransaction::new("edit", 1);
        tx.mark_modified(LabelTag::root().child(1));
        let delta = tx.commit();
        assert!(!tx.is_open);
        assert_eq!(delta.nb_changes(), 1);
    }

    #[test]
    fn relocation_table() {
        let mut rt = RelocationTable::new();
        let old = LabelTag::root().child(1);
        let new_tag = LabelTag::root().child(2);
        rt.set_relocation(&old, new_tag.clone());
        assert!(rt.has_relocation(&old));
        assert_eq!(rt.get_relocation(&old), Some(&new_tag));
        assert_eq!(rt.nb_relocations(), 1);
    }
}
