// FILE: xcaf_doc_assembly_item_ref.rs
// occt: XCAFDoc_AssemblyItemRef
//
// An attribute that describes a weak reference to an assembly item,
// or to a subshape of an item, or to an item's label attribute.
// TDF plumbing (labels, document) is modeled locally.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// GUID of the XCAFDoc_AssemblyItemRef attribute (from OCCT).
pub const ASSEMBLY_ITEM_REF_GUID: &str = "3F2E4CD6-169B-4747-A321-5670E4291F5D";

/// Standard_GUID::CheckGUIDFormat analogue: 36 chars,
/// hyphens at positions 8, 13, 18, 23, hex digits elsewhere.
pub fn check_guid_format(s: &str) -> bool {
    if s.len() != 36 {
        return false;
    }
    for (i, c) in s.chars().enumerate() {
        match i {
            8 | 13 | 18 | 23 => {
                if c != '-' {
                    return false;
                }
            }
            _ => {
                if !c.is_ascii_hexdigit() {
                    return false;
                }
            }
        }
    }
    true
}

/// Local model of XCAFDoc_AssemblyItemId: a full path of label entries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XCAFDocAssemblyItemId {
    path: Vec<String>,
}

impl XCAFDocAssemblyItemId {
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs from a formatted path with '/'-separated entries.
    pub fn from_string(s: &str) -> Self {
        let mut id = Self::new();
        id.init_string(s);
        id
    }

    pub fn from_path(path: &[&str]) -> Self {
        XCAFDocAssemblyItemId {
            path: path.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn init_string(&mut self, s: &str) {
        self.path = s
            .split('/')
            .filter(|p| !p.is_empty())
            .map(|p| p.to_string())
            .collect();
    }

    pub fn is_null(&self) -> bool {
        self.path.is_empty()
    }

    pub fn nullify(&mut self) {
        self.path.clear();
    }

    pub fn get_path(&self) -> &[String] {
        &self.path
    }

    pub fn to_string_path(&self) -> String {
        self.path.join("/")
    }

    /// True if this item is a (transitive) child of the other item.
    pub fn is_child(&self, other: &XCAFDocAssemblyItemId) -> bool {
        self.path.len() > other.path.len()
            && self.path[..other.path.len()] == other.path[..]
    }

    /// True if this item is a direct child of the other item.
    pub fn is_direct_child(&self, other: &XCAFDocAssemblyItemId) -> bool {
        self.path.len() == other.path.len() + 1 && self.is_child(other)
    }
}

/// Extra-reference discriminator (matches the anonymous enum in OCCT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExtraRef {
    None,
    AttrGuid,
    SubshapeIndex,
}

/// Local model of a document label: attribute GUIDs present on it and
/// the number of subshapes of its stored shape.
#[derive(Debug, Clone, Default)]
pub struct DocLabel {
    pub attribute_guids: HashSet<String>,
    pub nb_subshapes: usize,
    pub has_shape: bool,
}

/// Local model of the XDE document: labels addressed by entry string.
#[derive(Default)]
pub struct TdfDocument {
    labels: HashMap<String, DocLabel>,
}

impl TdfDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_label(&mut self, entry: &str, label: DocLabel) {
        self.labels.insert(entry.to_string(), label);
    }

    pub fn find_label(&self, entry: &str) -> Option<&DocLabel> {
        self.labels.get(entry)
    }
}

/// XCAFDoc_AssemblyItemRef: weak reference to an assembly item.
#[derive(Debug, Clone, PartialEq)]
pub struct XCAFDocAssemblyItemRef {
    item_id: XCAFDocAssemblyItemId,
    extra_ref: ExtraRef,
    extra_id: String,
}

impl XCAFDocAssemblyItemRef {
    /// OCCT GetID.
    pub fn get_id() -> &'static str {
        ASSEMBLY_ITEM_REF_GUID
    }

    /// OCCT ID (dynamic).
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// OCCT default ctor: empty reference, no extra ref.
    pub fn new() -> Self {
        XCAFDocAssemblyItemRef {
            item_id: XCAFDocAssemblyItemId::new(),
            extra_ref: ExtraRef::None,
            extra_id: String::new(),
        }
    }

    /// OCCT HasExtraRef.
    pub fn has_extra_ref(&self) -> bool {
        self.extra_ref != ExtraRef::None
    }

    /// OCCT IsGUID: extra ref is an attribute GUID with valid format.
    pub fn is_guid(&self) -> bool {
        self.extra_ref == ExtraRef::AttrGuid && check_guid_format(&self.extra_id)
    }

    /// OCCT IsSubshapeIndex: extra ref is an integer subshape index.
    pub fn is_subshape_index(&self) -> bool {
        self.extra_ref == ExtraRef::SubshapeIndex && self.extra_id.parse::<i32>().is_ok()
    }

    /// OCCT GetGUID: the attribute GUID or None if not a GUID ref.
    pub fn get_guid(&self) -> Option<&str> {
        if self.is_guid() {
            Some(&self.extra_id)
        } else {
            None
        }
    }

    /// OCCT GetSubshapeIndex: the subshape index, or 0 if not one.
    pub fn get_subshape_index(&self) -> i32 {
        if self.is_subshape_index() {
            self.extra_id.parse::<i32>().unwrap()
        } else {
            0
        }
    }

    /// OCCT GetItem.
    pub fn get_item(&self) -> &XCAFDocAssemblyItemId {
        &self.item_id
    }

    /// OCCT SetItem: sets item ID and clears any extra reference data.
    pub fn set_item(&mut self, item_id: XCAFDocAssemblyItemId) {
        self.item_id = item_id;
        self.clear_extra_ref();
    }

    /// OCCT SetItem(TCollection_AsciiString).
    pub fn set_item_from_string(&mut self, s: &str) {
        self.item_id.init_string(s);
        self.clear_extra_ref();
    }

    /// OCCT SetGUID: point to an item's label attribute.
    pub fn set_guid(&mut self, attr_guid: &str) {
        self.extra_ref = ExtraRef::AttrGuid;
        self.extra_id = attr_guid.to_string();
    }

    /// OCCT SetSubshapeIndex: point to an item's subshape.
    pub fn set_subshape_index(&mut self, subshape_index: i32) {
        self.extra_ref = ExtraRef::SubshapeIndex;
        self.extra_id = subshape_index.to_string();
    }

    /// OCCT ClearExtraRef.
    pub fn clear_extra_ref(&mut self) {
        self.extra_ref = ExtraRef::None;
        self.extra_id.clear();
    }

    /// OCCT IsOrphan: checks the reference against the (locally modeled)
    /// document. True if the target item / attribute / subshape is gone.
    pub fn is_orphan(&self, doc: &TdfDocument) -> bool {
        if self.item_id.is_null() {
            return true;
        }
        let last_entry = match self.item_id.get_path().last() {
            Some(e) => e,
            None => return true,
        };
        let label = match doc.find_label(last_entry) {
            Some(l) => l,
            None => return true,
        };
        if self.has_extra_ref() {
            if self.is_guid() {
                if !label.attribute_guids.contains(&self.extra_id) {
                    return true;
                }
            } else if self.is_subshape_index() {
                if !label.has_shape {
                    return true;
                }
                let idx = self.get_subshape_index();
                if idx < 1 || (label.nb_subshapes as i32) < idx {
                    return true;
                }
            }
        }
        false
    }

    /// OCCT Restore.
    pub fn restore(&mut self, other: &XCAFDocAssemblyItemRef) {
        self.item_id = other.item_id.clone();
        self.extra_ref = other.extra_ref;
        self.extra_id = other.extra_id.clone();
    }
}

impl Default for XCAFDocAssemblyItemRef {
    fn default() -> Self {
        Self::new()
    }
}

/// Local model of a TDF_Label holding AssemblyItemRef attributes.
#[derive(Default, Clone)]
pub struct TdfLabel {
    attrs: Rc<RefCell<HashMap<String, Rc<RefCell<XCAFDocAssemblyItemRef>>>>>,
}

impl TdfLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_attribute(&self, guid: &str) -> Option<Rc<RefCell<XCAFDocAssemblyItemRef>>> {
        self.attrs.borrow().get(guid).cloned()
    }

    pub fn add_attribute(&self, attr: Rc<RefCell<XCAFDocAssemblyItemRef>>) {
        let guid = attr.borrow().id().to_string();
        self.attrs.borrow_mut().insert(guid, attr);
    }
}

/// OCCT static Get.
pub fn get(label: &TdfLabel) -> Option<Rc<RefCell<XCAFDocAssemblyItemRef>>> {
    label.find_attribute(XCAFDocAssemblyItemRef::get_id())
}

/// OCCT static Set(label, itemId): create if not present.
pub fn set_item_ref(
    label: &TdfLabel,
    item_id: &XCAFDocAssemblyItemId,
) -> Rc<RefCell<XCAFDocAssemblyItemRef>> {
    if let Some(existing) = get(label) {
        return existing;
    }
    let mut a = XCAFDocAssemblyItemRef::new();
    a.set_item(item_id.clone());
    let a = Rc::new(RefCell::new(a));
    label.add_attribute(a.clone());
    a
}

/// OCCT static Set(label, itemId, GUID).
pub fn set_item_ref_guid(
    label: &TdfLabel,
    item_id: &XCAFDocAssemblyItemId,
    attr_guid: &str,
) -> Rc<RefCell<XCAFDocAssemblyItemRef>> {
    if let Some(existing) = get(label) {
        return existing;
    }
    let mut a = XCAFDocAssemblyItemRef::new();
    a.set_item(item_id.clone());
    a.set_guid(attr_guid);
    let a = Rc::new(RefCell::new(a));
    label.add_attribute(a.clone());
    a
}

/// OCCT static Set(label, itemId, shapeIndex).
pub fn set_item_ref_subshape(
    label: &TdfLabel,
    item_id: &XCAFDocAssemblyItemId,
    shape_index: i32,
) -> Rc<RefCell<XCAFDocAssemblyItemRef>> {
    if let Some(existing) = get(label) {
        return existing;
    }
    let mut a = XCAFDocAssemblyItemRef::new();
    a.set_item(item_id.clone());
    a.set_subshape_index(shape_index);
    let a = Rc::new(RefCell::new(a));
    label.add_attribute(a.clone());
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_constant_and_format() {
        assert_eq!(
            XCAFDocAssemblyItemRef::get_id(),
            "3F2E4CD6-169B-4747-A321-5670E4291F5D"
        );
        assert!(check_guid_format(ASSEMBLY_ITEM_REF_GUID));
        assert!(!check_guid_format("not-a-guid"));
        assert!(!check_guid_format("3F2E4CD6-169B-4747-A321-5670E4291F5")); // 35 chars
        assert!(!check_guid_format("3F2E4CD6X169B-4747-A321-5670E4291F5D")); // bad hyphen
    }

    #[test]
    fn test_item_id_paths() {
        let id = XCAFDocAssemblyItemId::from_string("0:1/0:1:1:1/0:1:1:2");
        assert!(!id.is_null());
        assert_eq!(id.get_path().len(), 3);
        assert_eq!(id.to_string_path(), "0:1/0:1:1:1/0:1:1:2");

        let parent = XCAFDocAssemblyItemId::from_string("0:1/0:1:1:1");
        assert!(id.is_child(&parent));
        assert!(id.is_direct_child(&parent));
        let root = XCAFDocAssemblyItemId::from_string("0:1");
        assert!(id.is_child(&root));
        assert!(!id.is_direct_child(&root));
        assert!(!parent.is_child(&id));

        let mut n = id.clone();
        n.nullify();
        assert!(n.is_null());
    }

    #[test]
    fn test_new_ref_is_empty() {
        let r = XCAFDocAssemblyItemRef::new();
        assert!(r.get_item().is_null());
        assert!(!r.has_extra_ref());
        assert!(!r.is_guid());
        assert!(!r.is_subshape_index());
        assert_eq!(r.get_subshape_index(), 0);
        assert!(r.get_guid().is_none());
    }

    #[test]
    fn test_set_item_clears_extra_ref() {
        let mut r = XCAFDocAssemblyItemRef::new();
        r.set_item(XCAFDocAssemblyItemId::from_string("0:1/0:2"));
        r.set_subshape_index(5);
        assert!(r.is_subshape_index());
        assert_eq!(r.get_subshape_index(), 5);

        // OCCT SetItem clears extra reference data.
        r.set_item(XCAFDocAssemblyItemId::from_string("0:1/0:3"));
        assert!(!r.has_extra_ref());
        assert_eq!(r.get_subshape_index(), 0);
    }

    #[test]
    fn test_guid_extra_ref() {
        let mut r = XCAFDocAssemblyItemRef::new();
        r.set_item_from_string("0:1/0:2");
        r.set_guid("FDEA4C52-0F54-484c-B590-579E18F7B5D4");
        assert!(r.has_extra_ref());
        assert!(r.is_guid());
        assert!(!r.is_subshape_index());
        assert_eq!(r.get_guid(), Some("FDEA4C52-0F54-484c-B590-579E18F7B5D4"));

        // Invalid GUID format -> IsGUID false, GetGUID empty.
        r.set_guid("garbage");
        assert!(r.has_extra_ref());
        assert!(!r.is_guid());
        assert!(r.get_guid().is_none());
    }

    #[test]
    fn test_static_set_on_label() {
        let label = TdfLabel::new();
        assert!(get(&label).is_none());
        let id = XCAFDocAssemblyItemId::from_string("0:1/0:2");
        let a1 = set_item_ref_subshape(&label, &id, 3);
        assert_eq!(a1.borrow().get_subshape_index(), 3);
        // Second Set finds the existing attribute and does not overwrite.
        let a2 = set_item_ref_guid(&label, &id, "FDEA4C52-0F54-484c-B590-579E18F7B5D4");
        assert!(Rc::ptr_eq(&a1, &a2));
        assert!(a2.borrow().is_subshape_index());
    }

    #[test]
    fn test_is_orphan() {
        let mut doc = TdfDocument::new();
        doc.add_label(
            "0:1:1:2",
            DocLabel {
                attribute_guids: ["FDEA4C52-0F54-484c-B590-579E18F7B5D4".to_string()]
                    .into_iter()
                    .collect(),
                nb_subshapes: 4,
                has_shape: true,
            },
        );

        // Null item -> orphan.
        let r = XCAFDocAssemblyItemRef::new();
        assert!(r.is_orphan(&doc));

        // Existing label, no extra ref -> not orphan.
        let mut r = XCAFDocAssemblyItemRef::new();
        r.set_item_from_string("0:1/0:1:1:2");
        assert!(!r.is_orphan(&doc));

        // Missing label -> orphan.
        let mut r2 = XCAFDocAssemblyItemRef::new();
        r2.set_item_from_string("0:1/0:9:9:9");
        assert!(r2.is_orphan(&doc));

        // Attribute GUID present -> not orphan; absent -> orphan.
        r.set_guid("FDEA4C52-0F54-484c-B590-579E18F7B5D4");
        assert!(!r.is_orphan(&doc));
        r.set_guid("1127951D-87D5-4ecc-89D5-D1406576C43F");
        assert!(r.is_orphan(&doc));

        // Subshape index within range -> not orphan; out of range -> orphan.
        r.set_subshape_index(4);
        assert!(!r.is_orphan(&doc));
        r.set_subshape_index(5);
        assert!(r.is_orphan(&doc));
        r.set_subshape_index(0);
        assert!(r.is_orphan(&doc));
    }

    #[test]
    fn test_restore() {
        let mut src = XCAFDocAssemblyItemRef::new();
        src.set_item_from_string("0:1/0:2");
        src.set_subshape_index(7);

        let mut dst = XCAFDocAssemblyItemRef::new();
        dst.restore(&src);
        assert_eq!(dst, src);
        assert_eq!(dst.get_subshape_index(), 7);
    }
}
