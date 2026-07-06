// FILE: xml_t_obj_drivers_reference_driver.rs
// occt: XmlTObjDrivers_ReferenceDriver
//
// Port of OCCT XmlTObjDrivers_ReferenceDriver (TObj XML drivers).
// Stores a TObj_TReference as XML attributes:
//   "entry"      - entry of the referred object's label
//   "master"     - entry of the master object's label
//   "modelentry" - name of the referred model, written only for
//                  cross-document references
// TObj/TDF plumbing (labels, documents, assistant) is modeled locally.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// DOM attribute names (IMPLEMENT_DOMSTRING in OCCT).
pub const ATTR_MASTER_ENTRY: &str = "master";
pub const ATTR_REFERRED_ENTRY: &str = "entry";
pub const ATTR_REFERRED_MODEL_ENTRY: &str = "modelentry";

/// Local model of an XmlObjMgt_Element.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XmlElement {
    attributes: HashMap<String, String>,
}

impl XmlElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }
}

/// A TDF label: identified by the document it belongs to and its entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub doc: String,
    pub entry: String,
}

/// Local model of TDF_Data: the set of existing label entries of one
/// document. TDF_Tool::Label(..., create) is find_label.
pub struct TdfData {
    doc: String,
    labels: RefCell<HashSet<String>>,
}

impl TdfData {
    pub fn new(doc: &str, entries: &[&str]) -> Rc<TdfData> {
        Rc::new(TdfData {
            doc: doc.to_string(),
            labels: RefCell::new(entries.iter().map(|e| e.to_string()).collect()),
        })
    }

    /// TDF_Tool::Label analogue: returns None for a missing entry unless
    /// `create` is set, in which case the label is created.
    pub fn find_label(&self, entry: &str, create: bool) -> Option<Label> {
        if entry.is_empty() {
            return None;
        }
        let exists = self.labels.borrow().contains(entry);
        if !exists {
            if !create {
                return None;
            }
            self.labels.borrow_mut().insert(entry.to_string());
        }
        Some(Label {
            doc: self.doc.clone(),
            entry: entry.to_string(),
        })
    }

    pub fn has_label(&self, entry: &str) -> bool {
        self.labels.borrow().contains(entry)
    }
}

/// Local model of TObj_Model: named model owning a TDF data.
pub struct TObjModel {
    name: String,
    data: Rc<TdfData>,
}

impl TObjModel {
    pub fn new(name: &str, data: Rc<TdfData>) -> Rc<TObjModel> {
        Rc::new(TObjModel {
            name: name.to_string(),
            data,
        })
    }

    pub fn get_model_name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &Rc<TdfData> {
        &self.data
    }
}

/// Local model of TObj_Assistant::FindModel.
#[derive(Default)]
pub struct TObjAssistant {
    models: HashMap<String, Rc<TObjModel>>,
}

impl TObjAssistant {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_model(&mut self, model: Rc<TObjModel>) {
        self.models.insert(model.get_model_name().to_string(), model);
    }

    pub fn find_model(&self, name: &str) -> Option<Rc<TObjModel>> {
        self.models.get(name).cloned()
    }
}

/// Local model of the referred TObj_Object (label + owning model name).
#[derive(Debug, Clone, PartialEq)]
pub struct TObjObject {
    pub label: Label,
    pub model_name: String,
}

/// Local model of the TObj_TReference attribute.
#[derive(Debug, Default)]
pub struct TObjTReference {
    referred: Option<Label>,
    master: Option<Label>,
    object: Option<TObjObject>,
}

impl TObjTReference {
    pub fn new() -> Self {
        Self::default()
    }

    /// The referred TObj_Object (used on the write side).
    pub fn set_object(&mut self, object: TObjObject, master: Label) {
        self.referred = Some(object.label.clone());
        self.master = Some(master);
        self.object = Some(object);
    }

    /// TObj_TReference::Get.
    pub fn get(&self) -> Option<&TObjObject> {
        self.object.as_ref()
    }

    /// TObj_TReference::Set(label, masterLabel) as done on retrieval.
    pub fn set(&mut self, label: Option<Label>, master: Option<Label>) {
        self.referred = label;
        self.master = master;
    }

    pub fn referred_label(&self) -> Option<&Label> {
        self.referred.as_ref()
    }

    /// TObj_TReference::GetMasterLabel.
    pub fn get_master_label(&self) -> Option<&Label> {
        self.master.as_ref()
    }
}

/// XmlMDF_ADriver for TObj_TReference.
#[derive(Debug, Default)]
pub struct XmlTObjDriversReferenceDriver;

impl XmlTObjDriversReferenceDriver {
    pub fn new() -> Self {
        Self
    }

    /// OCCT NewEmpty.
    pub fn new_empty(&self) -> TObjTReference {
        TObjTReference::new()
    }

    /// OCCT Paste (persistent -> transient). `data` is the document
    /// data of the target attribute's label. Returns true only when
    /// both referred and master labels were resolved.
    pub fn paste_from_xml(
        &self,
        source: &XmlElement,
        target: &mut TObjTReference,
        data: &TdfData,
        assistant: &TObjAssistant,
    ) -> bool {
        let ref_entry = source.get_attribute(ATTR_REFERRED_ENTRY).unwrap_or("");
        let master_entry = source.get_attribute(ATTR_MASTER_ENTRY).unwrap_or("");
        let in_holder_entry = source
            .get_attribute(ATTR_REFERRED_MODEL_ENTRY)
            .unwrap_or("");

        // Master label: resolved in this document, never created.
        let master = data.find_label(master_entry, false);

        // Referred label: same document (created if absent) or in the
        // referred model's document.
        let referred = if in_holder_entry.is_empty() {
            data.find_label(ref_entry, true)
        } else {
            assistant
                .find_model(in_holder_entry)
                .and_then(|m| m.data().find_label(ref_entry, true))
        };

        let ok = referred.is_some() && master.is_some();
        target.set(referred, master);
        ok
    }

    /// OCCT Paste (transient -> persistent): writes nothing when the
    /// reference holds no object.
    pub fn paste_to_xml(&self, source: &TObjTReference, target: &mut XmlElement) {
        let object = match source.get() {
            Some(o) => o,
            None => return,
        };

        target.set_attribute(ATTR_REFERRED_ENTRY, &object.label.entry);

        let master = source
            .get_master_label()
            .expect("stored reference has a master label");
        target.set_attribute(ATTR_MASTER_ENTRY, &master.entry);

        // Cross-document reference: also store the referred model name.
        if object.label.doc == master.doc {
            return;
        }
        target.set_attribute(ATTR_REFERRED_MODEL_ENTRY, &object.model_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let r = driver.new_empty();
        assert!(r.get().is_none());
        assert!(r.referred_label().is_none());
        assert!(r.get_master_label().is_none());
    }

    #[test]
    fn test_write_same_document_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let mut r = TObjTReference::new();
        r.set_object(
            TObjObject {
                label: Label {
                    doc: "doc1".into(),
                    entry: "0:1:5".into(),
                },
                model_name: "main".into(),
            },
            Label {
                doc: "doc1".into(),
                entry: "0:1:2".into(),
            },
        );

        let mut el = XmlElement::new();
        driver.paste_to_xml(&r, &mut el);
        assert_eq!(el.get_attribute(ATTR_REFERRED_ENTRY), Some("0:1:5"));
        assert_eq!(el.get_attribute(ATTR_MASTER_ENTRY), Some("0:1:2"));
        // Same document: no model entry attribute.
        assert_eq!(el.get_attribute(ATTR_REFERRED_MODEL_ENTRY), None);
    }

    #[test]
    fn test_write_cross_document_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let mut r = TObjTReference::new();
        r.set_object(
            TObjObject {
                label: Label {
                    doc: "doc2".into(),
                    entry: "0:3".into(),
                },
                model_name: "other-model".into(),
            },
            Label {
                doc: "doc1".into(),
                entry: "0:1:2".into(),
            },
        );

        let mut el = XmlElement::new();
        driver.paste_to_xml(&r, &mut el);
        assert_eq!(
            el.get_attribute(ATTR_REFERRED_MODEL_ENTRY),
            Some("other-model")
        );
    }

    #[test]
    fn test_write_null_object_writes_nothing() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let r = TObjTReference::new();
        let mut el = XmlElement::new();
        driver.paste_to_xml(&r, &mut el);
        assert_eq!(el, XmlElement::new());
    }

    #[test]
    fn test_read_reference_valid() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let data = TdfData::new("doc1", &["0:1:2", "0:1:5"]);
        let assistant = TObjAssistant::new();

        let mut el = XmlElement::new();
        el.set_attribute(ATTR_REFERRED_ENTRY, "0:1:5");
        el.set_attribute(ATTR_MASTER_ENTRY, "0:1:2");

        let mut r = TObjTReference::new();
        assert!(driver.paste_from_xml(&el, &mut r, &data, &assistant));
        assert_eq!(r.referred_label().unwrap().entry, "0:1:5");
        assert_eq!(r.get_master_label().unwrap().entry, "0:1:2");
    }

    #[test]
    fn test_read_reference_creates_missing_referred_label() {
        // OCCT resolves the referred label with create=true.
        let driver = XmlTObjDriversReferenceDriver::new();
        let data = TdfData::new("doc1", &["0:1:2"]);
        let assistant = TObjAssistant::new();

        let mut el = XmlElement::new();
        el.set_attribute(ATTR_REFERRED_ENTRY, "0:9:9");
        el.set_attribute(ATTR_MASTER_ENTRY, "0:1:2");

        let mut r = TObjTReference::new();
        assert!(driver.paste_from_xml(&el, &mut r, &data, &assistant));
        assert!(data.has_label("0:9:9"));
    }

    #[test]
    fn test_read_reference_missing_master_fails() {
        // The master label is never created, so an unknown entry fails.
        let driver = XmlTObjDriversReferenceDriver::new();
        let data = TdfData::new("doc1", &["0:1:5"]);
        let assistant = TObjAssistant::new();

        let mut el = XmlElement::new();
        el.set_attribute(ATTR_REFERRED_ENTRY, "0:1:5");
        el.set_attribute(ATTR_MASTER_ENTRY, "0:404");

        let mut r = TObjTReference::new();
        assert!(!driver.paste_from_xml(&el, &mut r, &data, &assistant));
        assert!(r.get_master_label().is_none());
    }

    #[test]
    fn test_read_cross_document_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let data = TdfData::new("doc1", &["0:1:2"]);
        let other_data = TdfData::new("doc2", &["0:3"]);
        let mut assistant = TObjAssistant::new();
        assistant.add_model(TObjModel::new("other-model", other_data.clone()));

        let mut el = XmlElement::new();
        el.set_attribute(ATTR_REFERRED_ENTRY, "0:3");
        el.set_attribute(ATTR_MASTER_ENTRY, "0:1:2");
        el.set_attribute(ATTR_REFERRED_MODEL_ENTRY, "other-model");

        let mut r = TObjTReference::new();
        assert!(driver.paste_from_xml(&el, &mut r, &data, &assistant));
        assert_eq!(r.referred_label().unwrap().doc, "doc2");
        assert_eq!(r.get_master_label().unwrap().doc, "doc1");
    }

    #[test]
    fn test_read_cross_document_unknown_model_fails() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let data = TdfData::new("doc1", &["0:1:2"]);
        let assistant = TObjAssistant::new();

        let mut el = XmlElement::new();
        el.set_attribute(ATTR_REFERRED_ENTRY, "0:3");
        el.set_attribute(ATTR_MASTER_ENTRY, "0:1:2");
        el.set_attribute(ATTR_REFERRED_MODEL_ENTRY, "nope");

        let mut r = TObjTReference::new();
        assert!(!driver.paste_from_xml(&el, &mut r, &data, &assistant));
    }

    #[test]
    fn test_roundtrip_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let data = TdfData::new("doc1", &["0:1:2", "0:1:5"]);
        let assistant = TObjAssistant::new();

        let mut src = TObjTReference::new();
        src.set_object(
            TObjObject {
                label: Label {
                    doc: "doc1".into(),
                    entry: "0:1:5".into(),
                },
                model_name: "main".into(),
            },
            Label {
                doc: "doc1".into(),
                entry: "0:1:2".into(),
            },
        );

        let mut el = XmlElement::new();
        driver.paste_to_xml(&src, &mut el);

        let mut dst = TObjTReference::new();
        assert!(driver.paste_from_xml(&el, &mut dst, &data, &assistant));
        assert_eq!(dst.referred_label(), src.referred_label());
        assert_eq!(dst.get_master_label(), src.get_master_label());
    }
}
