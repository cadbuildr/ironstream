// FILE: caf_label.rs
// occt: TDF_TagSource
// occt-ref: TDF_Label, TDF_LabelSequence, TDF_Attribute

/// Status of a CAF label operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelStatus {
    Ok,
    Null,
    NotFound,
    AlreadyExists,
}

impl Default for LabelStatus {
    fn default() -> Self { Self::Ok }
}

/// Tag path identifying a label in the document tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagPath(pub Vec<u32>);

impl TagPath {
    pub fn root() -> Self { Self(vec![]) }
    pub fn new(tags: Vec<u32>) -> Self { Self(tags) }
    pub fn child(&self, tag: u32) -> Self {
        let mut t = self.0.clone();
        t.push(tag);
        Self(t)
    }
    pub fn depth(&self) -> usize { self.0.len() }
    pub fn tag(&self) -> Option<u32> { self.0.last().copied() }
    pub fn father_path(&self) -> Option<Self> {
        if self.0.is_empty() { return None; }
        let mut t = self.0.clone();
        t.pop();
        Some(Self(t))
    }
}

// occt-ref: TDF_Label // — a node in the label tree
#[derive(Clone, Debug)]
pub struct TdfLabel {
    pub path: TagPath,
    pub is_null: bool,
    pub attr_ids: Vec<u32>,
}

impl TdfLabel {
    pub fn new(path: TagPath) -> Self {
        Self { path, is_null: false, attr_ids: Vec::new() }
    }

    pub fn null() -> Self {
        Self { path: TagPath::root(), is_null: true, attr_ids: Vec::new() }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn depth(&self) -> usize { self.path.depth() }
    pub fn tag(&self) -> Option<u32> { self.path.tag() }
    pub fn path(&self) -> &TagPath { &self.path }

    pub fn find_child(&self, tag: u32) -> TagPath { self.path.child(tag) }

    pub fn is_equal(&self, other: &TdfLabel) -> bool { self.path == other.path }
    pub fn is_descendant_of(&self, ancestor: &TdfLabel) -> bool {
        self.path.0.starts_with(&ancestor.path.0) && self.path.depth() > ancestor.path.depth()
    }

    pub fn has_attribute(&self, attr_id: u32) -> bool { self.attr_ids.contains(&attr_id) }
    pub fn add_attribute(&mut self, attr_id: u32) { if !self.attr_ids.contains(&attr_id) { self.attr_ids.push(attr_id); } }
    pub fn remove_attribute(&mut self, attr_id: u32) { self.attr_ids.retain(|&a| a != attr_id); }
    pub fn nb_attributes(&self) -> usize { self.attr_ids.len() }
}

// occt-ref: TDF_LabelSequence // — ordered sequence of labels
#[derive(Clone, Debug, Default)]
pub struct TdfLabelSequence {
    pub labels: Vec<TdfLabel>,
}

impl TdfLabelSequence {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, label: TdfLabel) { self.labels.push(label); }
    pub fn length(&self) -> usize { self.labels.len() }
    pub fn is_empty(&self) -> bool { self.labels.is_empty() }
    pub fn clear(&mut self) { self.labels.clear(); }
    pub fn value(&self, i: usize) -> Option<&TdfLabel> {
        if i == 0 { None } else { self.labels.get(i - 1) }
    }
    pub fn first(&self) -> Option<&TdfLabel> { self.labels.first() }
    pub fn last(&self) -> Option<&TdfLabel> { self.labels.last() }
}

// occt: TDF_TagSource // — source for generating unique tags on a label
#[derive(Clone, Debug)]
pub struct TdfTagSource {
    pub label: TagPath,
    pub last_tag: u32,
}

impl TdfTagSource {
    pub fn new(label: TagPath) -> Self {
        Self { label, last_tag: 0 }
    }

    pub fn new_child_label(&mut self) -> TagPath {
        self.last_tag += 1;
        self.label.child(self.last_tag)
    }

    pub fn set_last_tag(&mut self, tag: u32) {
        if tag > self.last_tag { self.last_tag = tag; }
    }

    pub fn last_tag(&self) -> u32 { self.last_tag }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_path_hierarchy() {
        let root = TagPath::root();
        let child = root.child(1);
        let grandchild = child.child(2);
        assert_eq!(grandchild.depth(), 2);
        assert_eq!(grandchild.tag(), Some(2));
        let father = grandchild.father_path().unwrap();
        assert_eq!(father, child);
    }

    #[test]
    fn label_attributes() {
        let mut l = TdfLabel::new(TagPath::root().child(1));
        l.add_attribute(100);
        l.add_attribute(200);
        assert!(l.has_attribute(100));
        l.remove_attribute(100);
        assert!(!l.has_attribute(100));
        assert_eq!(l.nb_attributes(), 1);
    }

    #[test]
    fn label_null() {
        let l = TdfLabel::null();
        assert!(l.is_null());
        assert_eq!(l.depth(), 0);
    }

    #[test]
    fn label_descendant_check() {
        let root = TdfLabel::new(TagPath::root());
        let child = TdfLabel::new(TagPath::root().child(1));
        let grandchild = TdfLabel::new(TagPath::root().child(1).child(2));
        assert!(grandchild.is_descendant_of(&root));
        assert!(grandchild.is_descendant_of(&child));
        assert!(!child.is_descendant_of(&grandchild));
    }

    #[test]
    fn label_sequence() {
        let mut seq = TdfLabelSequence::new();
        seq.append(TdfLabel::new(TagPath::new(vec![1])));
        seq.append(TdfLabel::new(TagPath::new(vec![2])));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1).unwrap().tag(), Some(1));
        assert!(seq.value(0).is_none()); // 1-based
    }

    #[test]
    fn tag_source_new_child_labels() {
        let mut ts = TdfTagSource::new(TagPath::root());
        let c1 = ts.new_child_label();
        let c2 = ts.new_child_label();
        assert_eq!(c1.tag(), Some(1));
        assert_eq!(c2.tag(), Some(2));
        assert_eq!(ts.last_tag(), 2);
    }
}
