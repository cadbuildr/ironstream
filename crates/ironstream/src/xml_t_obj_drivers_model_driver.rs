// FILE: xml_t_obj_drivers_model_driver.rs
// occt: XmlTObjDrivers_ModelDriver
//
// Port of OCCT XmlTObjDrivers_ModelDriver (TObj XML drivers).
// The driver stores the model GUID as the element's string content;
// on retrieval it checks the GUID against the current model provided
// by TObj_Assistant, then binds the model to the TObj_TModel attribute
// and its label. TObj/TDF plumbing is modeled locally.

use std::cell::RefCell;
use std::rc::Rc;

/// Local model of a TDF label (entry string is enough here).
pub type Label = String;

/// Local model of TObj_Model: GUID identity plus the label it is
/// attached to.
#[derive(Debug)]
pub struct TObjModel {
    guid: String,
    label: Option<Label>,
}

impl TObjModel {
    pub fn new(guid: &str) -> Rc<RefCell<TObjModel>> {
        Rc::new(RefCell::new(TObjModel {
            guid: guid.to_string(),
            label: None,
        }))
    }

    pub fn get_guid(&self) -> &str {
        &self.guid
    }

    pub fn set_label(&mut self, label: Label) {
        self.label = Some(label);
    }

    pub fn label(&self) -> Option<&Label> {
        self.label.as_ref()
    }
}

/// Local model of the TObj_TModel attribute: lives on a label and
/// points at the model object.
#[derive(Debug, Default)]
pub struct TObjTModel {
    label: Label,
    model: Option<Rc<RefCell<TObjModel>>>,
}

impl TObjTModel {
    /// OCCT NewEmpty equivalent: an empty attribute at a label.
    pub fn new(label: &str) -> Self {
        TObjTModel {
            label: label.to_string(),
            model: None,
        }
    }

    pub fn label(&self) -> &Label {
        &self.label
    }

    /// TObj_TModel::Set.
    pub fn set(&mut self, model: Rc<RefCell<TObjModel>>) {
        self.model = Some(model);
    }

    /// TObj_TModel::Model.
    pub fn model(&self) -> Option<Rc<RefCell<TObjModel>>> {
        self.model.clone()
    }
}

/// Local model of TObj_Assistant: supplies the model being restored.
pub struct TObjAssistant {
    current_model: Option<Rc<RefCell<TObjModel>>>,
}

impl TObjAssistant {
    pub fn new() -> Self {
        TObjAssistant {
            current_model: None,
        }
    }

    pub fn set_current_model(&mut self, model: Rc<RefCell<TObjModel>>) {
        self.current_model = Some(model);
    }

    pub fn get_current_model(&self) -> Option<Rc<RefCell<TObjModel>>> {
        self.current_model.clone()
    }
}

impl Default for TObjAssistant {
    fn default() -> Self {
        Self::new()
    }
}

/// Local model of XmlObjMgt_Persistent: the element's string content.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XmlPersistent {
    extended_string: Option<String>,
}

impl XmlPersistent {
    pub fn new() -> Self {
        Self::default()
    }

    /// XmlObjMgt::GetExtendedString.
    pub fn get_extended_string(&self) -> Option<&str> {
        self.extended_string.as_deref()
    }

    /// XmlObjMgt::SetExtendedString.
    pub fn set_extended_string(&mut self, s: &str) {
        self.extended_string = Some(s.to_string());
    }
}

/// XmlMDF_ADriver for TObj_TModel.
pub struct XmlTObjDriversModelDriver {
    messages: RefCell<Vec<String>>,
}

impl XmlTObjDriversModelDriver {
    pub fn new() -> Self {
        XmlTObjDriversModelDriver {
            messages: RefCell::new(Vec::new()),
        }
    }

    /// Messages sent to the message driver (for inspection in tests).
    pub fn messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }

    fn send(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }

    /// OCCT NewEmpty.
    pub fn new_empty(&self, label: &str) -> TObjTModel {
        TObjTModel::new(label)
    }

    /// OCCT Paste (persistent -> transient): reads the stored GUID,
    /// checks it against the current model from the assistant, then
    /// binds the model to the target attribute and its label.
    pub fn paste_from_xml(
        &self,
        source: &XmlPersistent,
        target: &mut TObjTModel,
        assistant: &TObjAssistant,
    ) -> bool {
        let guid = match source.get_extended_string() {
            Some(s) => s.to_string(),
            None => {
                self.send("error retrieving ExtendedString for type TObj_TModel");
                return false;
            }
        };
        let current_model = match assistant.get_current_model() {
            Some(m) => m,
            None => {
                self.send("TObj_TModel retrieval: wrong model GUID");
                return false;
            }
        };
        if guid == current_model.borrow().get_guid() {
            current_model
                .borrow_mut()
                .set_label(target.label().clone());
            target.set(current_model);
            return true;
        }
        self.send("TObj_TModel retrieval: wrong model GUID");
        false
    }

    /// OCCT Paste (transient -> persistent): stores the model GUID.
    pub fn paste_to_xml(&self, source: &TObjTModel, target: &mut XmlPersistent) {
        let model = source
            .model()
            .expect("TObj_TModel must reference a model when stored");
        target.set_extended_string(model.borrow().get_guid());
    }
}

impl Default for XmlTObjDriversModelDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GUID: &str = "3bbefb47-e618-11d4-bd1a-0060b0ee18ea";

    #[test]
    fn test_new_empty() {
        let driver = XmlTObjDriversModelDriver::new();
        let tmodel = driver.new_empty("0:1");
        assert_eq!(tmodel.label(), "0:1");
        assert!(tmodel.model().is_none());
    }

    #[test]
    fn test_write_stores_guid() {
        let driver = XmlTObjDriversModelDriver::new();
        let model = TObjModel::new(GUID);
        let mut tmodel = TObjTModel::new("0:1");
        tmodel.set(model);

        let mut p = XmlPersistent::new();
        driver.paste_to_xml(&tmodel, &mut p);
        assert_eq!(p.get_extended_string(), Some(GUID));
    }

    #[test]
    fn test_read_model_valid() {
        let driver = XmlTObjDriversModelDriver::new();
        let model = TObjModel::new(GUID);
        let mut assistant = TObjAssistant::new();
        assistant.set_current_model(model.clone());

        let mut p = XmlPersistent::new();
        p.set_extended_string(GUID);

        let mut tmodel = TObjTModel::new("0:1:5");
        assert!(driver.paste_from_xml(&p, &mut tmodel, &assistant));
        // Model got bound to the attribute and its label.
        assert!(Rc::ptr_eq(&tmodel.model().unwrap(), &model));
        assert_eq!(model.borrow().label(), Some(&"0:1:5".to_string()));
        assert!(driver.messages().is_empty());
    }

    #[test]
    fn test_read_model_wrong_guid() {
        let driver = XmlTObjDriversModelDriver::new();
        let model = TObjModel::new(GUID);
        let mut assistant = TObjAssistant::new();
        assistant.set_current_model(model);

        let mut p = XmlPersistent::new();
        p.set_extended_string("00000000-0000-0000-0000-000000000000");

        let mut tmodel = TObjTModel::new("0:1");
        assert!(!driver.paste_from_xml(&p, &mut tmodel, &assistant));
        assert!(tmodel.model().is_none());
        assert_eq!(
            driver.messages(),
            vec!["TObj_TModel retrieval: wrong model GUID".to_string()]
        );
    }

    #[test]
    fn test_read_model_missing_string() {
        let driver = XmlTObjDriversModelDriver::new();
        let mut assistant = TObjAssistant::new();
        assistant.set_current_model(TObjModel::new(GUID));

        let p = XmlPersistent::new(); // no string content
        let mut tmodel = TObjTModel::new("0:1");
        assert!(!driver.paste_from_xml(&p, &mut tmodel, &assistant));
        assert_eq!(
            driver.messages(),
            vec!["error retrieving ExtendedString for type TObj_TModel".to_string()]
        );
    }

    #[test]
    fn test_roundtrip_model() {
        let driver = XmlTObjDriversModelDriver::new();
        let model = TObjModel::new(GUID);

        // Store.
        let mut src = TObjTModel::new("0:2");
        src.set(model.clone());
        let mut p = XmlPersistent::new();
        driver.paste_to_xml(&src, &mut p);

        // Retrieve into a fresh attribute with the same current model.
        let mut assistant = TObjAssistant::new();
        assistant.set_current_model(model.clone());
        let mut dst = TObjTModel::new("0:2");
        assert!(driver.paste_from_xml(&p, &mut dst, &assistant));
        assert!(Rc::ptr_eq(&dst.model().unwrap(), &model));
    }
}
