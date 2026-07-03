// FILE: ocaf_framework.rs

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// TDF_Label
// ---------------------------------------------------------------------------

// occt: TDF_Label
/// A handle/path into the data tree, represented as a sequence of child tags
/// from the root.  An empty path is the root label.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TdfLabel {
    path: Vec<u32>,
}

impl TdfLabel {
    /// The root label (empty path).
    pub fn new_root() -> Self {
        Self { path: Vec::new() }
    }

    /// Return a child label under this one with the given tag.
    pub fn child(&self, tag: u32) -> Self {
        let mut path = self.path.clone();
        path.push(tag);
        Self { path }
    }

    /// Return the parent label, or `None` if this is the root.
    pub fn parent(&self) -> Option<Self> {
        if self.path.is_empty() {
            None
        } else {
            let path = self.path[..self.path.len() - 1].to_vec();
            Some(Self { path })
        }
    }

    /// The last tag in the path.  Panics if called on the root label.
    pub fn tag(&self) -> u32 {
        *self.path.last().expect("root label has no tag")
    }

    /// `true` when this is the root label (empty path).
    pub fn is_root(&self) -> bool {
        self.path.is_empty()
    }

    /// Depth in the tree (0 = root, 1 = direct child of root, …).
    pub fn depth(&self) -> usize {
        self.path.len()
    }

    /// A valid TdfLabel is never null; always returns `false`.
    pub fn is_null(&self) -> bool {
        false
    }

    /// `true` when `self` is a (strict) descendant of `other`.
    pub fn is_descendant_of(&self, other: &TdfLabel) -> bool {
        if other.path.len() >= self.path.len() {
            return false;
        }
        self.path.starts_with(&other.path)
    }
}

// ---------------------------------------------------------------------------
// TDF_Attribute trait
// ---------------------------------------------------------------------------

// occt: TDF_Attribute
pub trait TdfAttribute: std::fmt::Debug + Send + Sync {
    fn attribute_type_name(&self) -> &'static str;
}

// ---------------------------------------------------------------------------
// TDF_Data
// ---------------------------------------------------------------------------

// occt: TDF_Data
/// The data framework container.  Holds a tree of labels and their attached
/// attributes (boxed trait objects, stored by type name per label).
pub struct TdfData {
    /// label → (type_name → attribute)
    store: HashMap<TdfLabel, HashMap<&'static str, Box<dyn TdfAttribute>>>,
}

impl TdfData {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// The root label of this data framework.
    pub fn root(&self) -> TdfLabel {
        TdfLabel::new_root()
    }

    /// Attach an attribute to the given label, replacing any previous
    /// attribute of the same type name.
    pub fn set_attribute<A: TdfAttribute + 'static>(&mut self, label: &TdfLabel, attr: A) {
        self.store
            .entry(label.clone())
            .or_default()
            .insert(attr.attribute_type_name(), Box::new(attr));
    }

    /// Retrieve an attribute from a label by its type name.
    pub fn get_attribute(
        &self,
        label: &TdfLabel,
        type_name: &'static str,
    ) -> Option<&dyn TdfAttribute> {
        self.store
            .get(label)?
            .get(type_name)
            .map(|b| b.as_ref())
    }

    /// `true` when the label has any attribute attached.
    pub fn has_attribute(&self, label: &TdfLabel, type_name: &'static str) -> bool {
        self.store
            .get(label)
            .and_then(|m| m.get(type_name))
            .is_some()
    }
}

impl Default for TdfData {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// TDataStd_Name
// ---------------------------------------------------------------------------

// occt: TDataStd_Name
#[derive(Debug)]
pub struct TDataStdName {
    pub value: String,
}

impl TDataStdName {
    pub fn new(s: &str) -> Self {
        Self { value: s.to_owned() }
    }
}

impl TdfAttribute for TDataStdName {
    fn attribute_type_name(&self) -> &'static str {
        "TDataStd_Name"
    }
}

// ---------------------------------------------------------------------------
// TDataStd_Integer
// ---------------------------------------------------------------------------

// occt: TDataStd_Integer
#[derive(Debug)]
pub struct TDataStdInteger {
    pub value: i32,
}

impl TdfAttribute for TDataStdInteger {
    fn attribute_type_name(&self) -> &'static str {
        "TDataStd_Integer"
    }
}

// ---------------------------------------------------------------------------
// TDataStd_Real
// ---------------------------------------------------------------------------

// occt: TDataStd_Real
#[derive(Debug)]
pub struct TDataStdReal {
    pub value: f64,
}

impl TdfAttribute for TDataStdReal {
    fn attribute_type_name(&self) -> &'static str {
        "TDataStd_Real"
    }
}

// ---------------------------------------------------------------------------
// TDocStd_Document
// ---------------------------------------------------------------------------

// occt: TDocStd_Document
/// A document wrapping a `TdfData` tree with an optional format tag.
pub struct TDocStdDocument {
    data: TdfData,
    format: String,
}

impl TDocStdDocument {
    pub fn new(format: &str) -> Self {
        Self {
            data: TdfData::new(),
            format: format.to_owned(),
        }
    }

    /// The conventional "main" label for user data: child 1 of root.
    pub fn main_label(&self) -> TdfLabel {
        TdfLabel::new_root().child(1)
    }

    pub fn data(&self) -> &TdfData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut TdfData {
        &mut self.data
    }

    pub fn format(&self) -> &str {
        &self.format
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_label_is_root() {
        let r = TdfLabel::new_root();
        assert!(r.is_root());
        assert_eq!(r.depth(), 0);
        assert!(!r.is_null());
    }

    #[test]
    fn child_label_depth_and_tag() {
        let r = TdfLabel::new_root();
        let c = r.child(3);
        assert!(!c.is_root());
        assert_eq!(c.depth(), 1);
        assert_eq!(c.tag(), 3);
    }

    #[test]
    fn parent_of_child_is_root() {
        let child = TdfLabel::new_root().child(7);
        let parent = child.parent().unwrap();
        assert!(parent.is_root());
    }

    #[test]
    fn root_parent_is_none() {
        let r = TdfLabel::new_root();
        assert!(r.parent().is_none());
    }

    #[test]
    fn is_descendant_of_direct_parent() {
        let root = TdfLabel::new_root();
        let child = root.child(1);
        let grandchild = child.child(2);
        assert!(grandchild.is_descendant_of(&root));
        assert!(grandchild.is_descendant_of(&child));
        assert!(!child.is_descendant_of(&grandchild));
        assert!(!root.is_descendant_of(&root));
    }

    #[test]
    fn tdata_std_name_attribute() {
        let attr = TDataStdName::new("part");
        assert_eq!(attr.value, "part");
        assert_eq!(attr.attribute_type_name(), "TDataStd_Name");
    }

    #[test]
    fn tdata_std_integer_attribute() {
        let attr = TDataStdInteger { value: 42 };
        assert_eq!(attr.value, 42);
        assert_eq!(attr.attribute_type_name(), "TDataStd_Integer");
    }

    #[test]
    fn tdata_std_real_attribute() {
        let attr = TDataStdReal { value: 3.14 };
        assert!((attr.value - 3.14).abs() < 1e-12);
        assert_eq!(attr.attribute_type_name(), "TDataStd_Real");
    }

    #[test]
    fn tdf_data_set_and_get() {
        let mut data = TdfData::new();
        let label = data.root().child(1);
        data.set_attribute(&label, TDataStdName::new("MyPart"));
        let got = data.get_attribute(&label, "TDataStd_Name");
        assert!(got.is_some());
        assert_eq!(got.unwrap().attribute_type_name(), "TDataStd_Name");
    }

    #[test]
    fn tdf_data_has_attribute() {
        let mut data = TdfData::new();
        let label = data.root().child(2);
        assert!(!data.has_attribute(&label, "TDataStd_Integer"));
        data.set_attribute(&label, TDataStdInteger { value: 1 });
        assert!(data.has_attribute(&label, "TDataStd_Integer"));
    }

    #[test]
    fn tdoc_std_document_main_label() {
        let doc = TDocStdDocument::new("BinOCAF");
        assert_eq!(doc.format(), "BinOCAF");
        let ml = doc.main_label();
        assert_eq!(ml.depth(), 1);
        assert_eq!(ml.tag(), 1);
    }
}
