// FILE: xml_t_obj_drivers_object_driver.rs
// occt: XmlTObjDrivers_ObjectDriver
//
// Port of OCCT XmlTObjDrivers_ObjectDriver (TObj XML drivers).
// On storage the driver writes the object's dynamic type name as the
// element's string content; on retrieval it re-creates the object via
// the TObj_Persistence factory using that type name and binds it to
// the TObj_TObject attribute. TObj/TDF plumbing is modeled locally.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Local model of a TDF label (entry string).
pub type Label = String;

/// Local model of TObj_Object: a typed object attached to a label.
#[derive(Debug, Clone, PartialEq)]
pub struct TObjObject {
    type_name: String,
    label: Label,
}

impl TObjObject {
    pub fn new(type_name: &str, label: &str) -> Rc<TObjObject> {
        Rc::new(TObjObject {
            type_name: type_name.to_string(),
            label: label.to_string(),
        })
    }

    /// DynamicType()->Name() analogue.
    pub fn dynamic_type_name(&self) -> &str {
        &self.type_name
    }

    pub fn get_label(&self) -> &Label {
        &self.label
    }
}

/// Local model of the TObj_Persistence factory: maps registered type
/// names to constructors creating objects on a label.
pub struct TObjPersistence {
    types: HashMap<String, fn(&str, &str) -> Rc<TObjObject>>,
}

impl TObjPersistence {
    pub fn new() -> Self {
        TObjPersistence {
            types: HashMap::new(),
        }
    }

    pub fn register(&mut self, type_name: &str, ctor: fn(&str, &str) -> Rc<TObjObject>) {
        self.types.insert(type_name.to_string(), ctor);
    }

    /// TObj_Persistence::CreateNewObject: None for unknown types
    /// (OCCT returns a null handle).
    pub fn create_new_object(&self, type_name: &str, label: &str) -> Option<Rc<TObjObject>> {
        self.types.get(type_name).map(|ctor| ctor(type_name, label))
    }
}

impl Default for TObjPersistence {
    fn default() -> Self {
        Self::new()
    }
}

/// Local model of the TObj_TObject attribute.
#[derive(Debug, Default)]
pub struct TObjTObject {
    label: Label,
    object: Option<Rc<TObjObject>>,
}

impl TObjTObject {
    pub fn new(label: &str) -> Self {
        TObjTObject {
            label: label.to_string(),
            object: None,
        }
    }

    pub fn label(&self) -> &Label {
        &self.label
    }

    /// TObj_TObject::Set.
    pub fn set(&mut self, object: Option<Rc<TObjObject>>) {
        self.object = object;
    }

    /// TObj_TObject::Get.
    pub fn get(&self) -> Option<Rc<TObjObject>> {
        self.object.clone()
    }
}

/// Local model of XmlObjMgt_Persistent: element string content.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XmlPersistent {
    extended_string: Option<String>,
}

impl XmlPersistent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_extended_string(&self) -> Option<&str> {
        self.extended_string.as_deref()
    }

    pub fn set_extended_string(&mut self, s: &str) {
        self.extended_string = Some(s.to_string());
    }
}

/// XmlMDF_ADriver for TObj_TObject.
pub struct XmlTObjDriversObjectDriver {
    messages: RefCell<Vec<String>>,
}

impl XmlTObjDriversObjectDriver {
    pub fn new() -> Self {
        XmlTObjDriversObjectDriver {
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }

    fn send(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }

    /// OCCT NewEmpty.
    pub fn new_empty(&self, label: &str) -> TObjTObject {
        TObjTObject::new(label)
    }

    /// OCCT Paste (persistent -> transient): reads the type name and
    /// re-creates the object at the target's label via the factory.
    pub fn paste_from_xml(
        &self,
        source: &XmlPersistent,
        target: &mut TObjTObject,
        persistence: &TObjPersistence,
    ) -> bool {
        let type_name = match source.get_extended_string() {
            Some(s) => s.to_string(),
            None => {
                // OCCT emits this exact (copy-pasted) message here.
                self.send("error retrieving ExtendedString for type TObj_TModel");
                return false;
            }
        };
        let label = target.label().clone();
        let object = persistence.create_new_object(&type_name, &label);
        // OCCT sets the (possibly null) object and returns true.
        target.set(object);
        true
    }

    /// OCCT Paste (transient -> persistent): stores the object's
    /// dynamic type name.
    pub fn paste_to_xml(&self, source: &TObjTObject, target: &mut XmlPersistent) {
        let object = source
            .get()
            .expect("TObj_TObject must hold an object when stored");
        target.set_extended_string(object.dynamic_type_name());
    }
}

impl Default for XmlTObjDriversObjectDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_persistence() -> TObjPersistence {
        let mut p = TObjPersistence::new();
        p.register("TObj_Object", TObjObject::new);
        p.register("MyApp_Part", TObjObject::new);
        p
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlTObjDriversObjectDriver::new();
        let t = driver.new_empty("0:1:3");
        assert_eq!(t.label(), "0:1:3");
        assert!(t.get().is_none());
    }

    #[test]
    fn test_write_stores_type_name() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut t = TObjTObject::new("0:1:3");
        t.set(Some(TObjObject::new("MyApp_Part", "0:1:3")));

        let mut p = XmlPersistent::new();
        driver.paste_to_xml(&t, &mut p);
        assert_eq!(p.get_extended_string(), Some("MyApp_Part"));
    }

    #[test]
    fn test_read_object_valid() {
        let driver = XmlTObjDriversObjectDriver::new();
        let persistence = make_persistence();
        let mut p = XmlPersistent::new();
        p.set_extended_string("MyApp_Part");

        let mut t = TObjTObject::new("0:1:7");
        assert!(driver.paste_from_xml(&p, &mut t, &persistence));
        let obj = t.get().expect("object must be created");
        assert_eq!(obj.dynamic_type_name(), "MyApp_Part");
        // Object is created at the target attribute's label.
        assert_eq!(obj.get_label(), "0:1:7");
        assert!(driver.messages().is_empty());
    }

    #[test]
    fn test_read_object_unknown_type_sets_null_but_succeeds() {
        // OCCT: CreateNewObject returns a null handle for unregistered
        // types; Paste still returns true.
        let driver = XmlTObjDriversObjectDriver::new();
        let persistence = make_persistence();
        let mut p = XmlPersistent::new();
        p.set_extended_string("Unknown_Type");

        let mut t = TObjTObject::new("0:1");
        assert!(driver.paste_from_xml(&p, &mut t, &persistence));
        assert!(t.get().is_none());
    }

    #[test]
    fn test_read_object_missing_type() {
        let driver = XmlTObjDriversObjectDriver::new();
        let persistence = make_persistence();
        let p = XmlPersistent::new(); // no string content

        let mut t = TObjTObject::new("0:1");
        assert!(!driver.paste_from_xml(&p, &mut t, &persistence));
        assert!(t.get().is_none());
        assert_eq!(
            driver.messages(),
            vec!["error retrieving ExtendedString for type TObj_TModel".to_string()]
        );
    }

    #[test]
    fn test_roundtrip_object() {
        let driver = XmlTObjDriversObjectDriver::new();
        let persistence = make_persistence();

        let mut src = TObjTObject::new("0:5");
        src.set(Some(TObjObject::new("TObj_Object", "0:5")));

        let mut p = XmlPersistent::new();
        driver.paste_to_xml(&src, &mut p);

        let mut dst = TObjTObject::new("0:5");
        assert!(driver.paste_from_xml(&p, &mut dst, &persistence));
        let restored = dst.get().unwrap();
        assert_eq!(restored.dynamic_type_name(), "TObj_Object");
        assert_eq!(restored.get_label(), "0:5");
    }
}
