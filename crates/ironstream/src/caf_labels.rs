// FILE: caf_labels.rs
// occt: TDF_LabelMap, TDF_ChildIterator
// occt-ref: TDF_Label, TDF_TagSource

// occt-ref: TDF_Label
/// A node in the CAF label tree, identified by a tag path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    pub tag: Vec<u32>,
    pub is_null: bool,
}

impl Label {
    pub fn null() -> Self { Self { tag: Vec::new(), is_null: true } }
    pub fn root() -> Self { Self { tag: vec![0], is_null: false } }
    pub fn new(tag: Vec<u32>) -> Self { Self { tag, is_null: false } }

    pub fn find_child(&self, child_tag: u32, create: bool) -> Option<Label> {
        if self.is_null { return None; }
        let _ = create;
        let mut child_path = self.tag.clone();
        child_path.push(child_tag);
        Some(Label { tag: child_path, is_null: false })
    }

    pub fn father(&self) -> Option<Label> {
        if self.is_null || self.tag.len() <= 1 { return None; }
        let mut parent = self.tag.clone();
        parent.pop();
        Some(Label::new(parent))
    }

    pub fn tag(&self) -> u32 { self.tag.last().copied().unwrap_or(0) }
    pub fn depth(&self) -> usize { self.tag.len().saturating_sub(1) }
    pub fn is_root(&self) -> bool { self.tag == vec![0] }
    pub fn is_descendant_of(&self, other: &Label) -> bool {
        self.tag.len() > other.tag.len() && self.tag.starts_with(&other.tag)
    }
    pub fn entry(&self) -> String {
        self.tag.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(":")
    }
}

// occt: TDF_LabelMap
#[derive(Clone, Debug, Default)]
pub struct LabelMap {
    labels: Vec<Label>,
}

impl LabelMap {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, l: Label) -> bool {
        if self.contains(&l) { return false; }
        self.labels.push(l);
        true
    }
    pub fn contains(&self, l: &Label) -> bool {
        self.labels.iter().any(|x| x == l)
    }
    pub fn extent(&self) -> usize { self.labels.len() }
    pub fn clear(&mut self) { self.labels.clear(); }
}

// occt: TDF_ChildIterator
pub struct ChildIterator<'a> {
    parent: &'a Label,
    tags: Vec<Label>,
    index: usize,
}

impl<'a> ChildIterator<'a> {
    pub fn new(parent: &'a Label, children: Vec<Label>) -> Self {
        let tags: Vec<Label> = children.into_iter().filter(|c| {
            c.tag.len() == parent.tag.len() + 1 && c.tag.starts_with(&parent.tag)
        }).collect();
        Self { parent, tags, index: 0 }
    }

    pub fn more(&self) -> bool { self.index < self.tags.len() }
    pub fn next(&mut self) { self.index += 1; }
    pub fn value(&self) -> Option<&Label> { self.tags.get(self.index) }
}

// occt-ref: TDF_TagSource
#[derive(Clone, Debug, Default)]
pub struct TagSource {
    pub last_tag: u32,
}

impl TagSource {
    pub fn new() -> Self { Self::default() }
    pub fn new_child_label(&mut self, parent: &Label) -> Label {
        self.last_tag += 1;
        parent.find_child(self.last_tag, true).unwrap_or_else(Label::null)
    }
    pub fn set(&mut self, tag: u32) { self.last_tag = tag; }
    pub fn get(&self) -> u32 { self.last_tag }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_tree() {
        let root = Label::root();
        assert!(root.is_root());
        let child = root.find_child(1, true).unwrap();
        assert_eq!(child.depth(), 1);
        assert_eq!(child.tag(), 1);
        let parent = child.father().unwrap();
        assert_eq!(parent, root);
    }

    #[test]
    fn label_entry() {
        let l = Label::new(vec![0, 1, 3]);
        assert_eq!(l.entry(), "0:1:3");
    }

    #[test]
    fn label_descendant() {
        let root = Label::root();
        let child = root.find_child(2, true).unwrap();
        let grandchild = child.find_child(5, true).unwrap();
        assert!(grandchild.is_descendant_of(&root));
        assert!(grandchild.is_descendant_of(&child));
        assert!(!child.is_descendant_of(&grandchild));
    }

    #[test]
    fn label_map() {
        let mut m = LabelMap::new();
        let a = Label::new(vec![0, 1]);
        let b = Label::new(vec![0, 2]);
        m.add(a.clone());
        m.add(b);
        assert!(m.contains(&a));
        assert_eq!(m.extent(), 2);
    }

    #[test]
    fn tag_source() {
        let root = Label::root();
        let mut ts = TagSource::new();
        let c1 = ts.new_child_label(&root);
        let c2 = ts.new_child_label(&root);
        assert_ne!(c1, c2);
        assert_eq!(ts.get(), 2);
    }
}
