// FILE: cdm_application.rs
// occt: CDM_Application

//! Base application class of the CDM framework: manages document
//! version/reference-counter propagation, update notifications through a
//! message driver, and the metadata look-up table.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for Message_Messenger: records sent messages.
#[derive(Debug, Default)]
pub struct Messenger {
    messages: RefCell<Vec<String>>,
}

impl Messenger {
    pub fn new() -> Self {
        Messenger { messages: RefCell::new(Vec::new()) }
    }
    pub fn send(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }
    pub fn messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }
}

/// Local stand-in for CDM_Document: version (modifications) and reference counter.
#[derive(Debug, Default)]
pub struct CDM_Document {
    modifications: i32,
    reference_counter: i32,
}

impl CDM_Document {
    pub fn new() -> Self {
        CDM_Document { modifications: 0, reference_counter: 0 }
    }
    pub fn set_modifications(&mut self, m: i32) {
        self.modifications = m;
    }
    pub fn modifications(&self) -> i32 {
        self.modifications
    }
    pub fn set_reference_counter(&mut self, c: i32) {
        self.reference_counter = c;
    }
    pub fn reference_counter(&self) -> i32 {
        self.reference_counter
    }
}

/// Local stand-in for CDM_MetaData: stores the document version.
#[derive(Debug, Clone)]
pub struct CDM_MetaData {
    document_version: i32,
}

impl CDM_MetaData {
    pub fn new(document_version: i32) -> Self {
        CDM_MetaData { document_version }
    }
    pub fn document_version(&self) -> i32 {
        self.document_version
    }
}

/// CDM_Application: base class for applications managing CDM documents.
pub struct CDM_Application {
    messenger: Rc<Messenger>,
    meta_data_lookup_table: HashMap<String, CDM_MetaData>,
}

impl CDM_Application {
    /// Creates a new application; the constructor installs a messenger.
    pub fn new() -> Self {
        CDM_Application {
            messenger: Rc::new(Messenger::new()),
            meta_data_lookup_table: HashMap::new(),
        }
    }

    /// SetDocumentVersion: copies the metadata document version into the document.
    pub fn set_document_version(&self, document: &mut CDM_Document, meta_data: &CDM_MetaData) {
        document.set_modifications(meta_data.document_version());
    }

    /// SetReferenceCounter.
    pub fn set_reference_counter(&self, document: &mut CDM_Document, reference_counter: i32) {
        document.set_reference_counter(reference_counter);
    }

    /// MessageDriver: returns the default message driver.
    pub fn message_driver(&self) -> Rc<Messenger> {
        Rc::clone(&self.messenger)
    }

    /// Write: sends a message through the message driver.
    pub fn write(&self, string: &str) {
        self.message_driver().send(string);
    }

    /// BeginOfUpdate: informs that a document is going to be updated.
    pub fn begin_of_update(&self, _document: &CDM_Document) {
        let mut updating = String::from("Updating: ");
        updating += "Document";
        self.write(&updating);
    }

    /// EndOfUpdate: informs whether the update succeeded.
    pub fn end_of_update(&self, _document: &CDM_Document, status: bool, _error_string: &str) {
        let mut message =
            String::from(if status { "Updated: " } else { "Error during updating: " });
        message += "Document";
        self.write(&message);
    }

    /// Name: default is empty.
    pub fn name(&self) -> String {
        String::new()
    }

    /// Version: default is empty.
    pub fn version(&self) -> String {
        String::new()
    }

    /// MetaDataLookUpTable: table mapping folder/name strings to metadata.
    pub fn meta_data_lookup_table(&mut self) -> &mut HashMap<String, CDM_MetaData> {
        &mut self.meta_data_lookup_table
    }
}

impl Default for CDM_Application {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdm_application_creation() {
        let mut app = CDM_Application::new();
        assert!(app.name().is_empty());
        assert!(app.version().is_empty());
        assert!(app.meta_data_lookup_table().is_empty());
        assert!(app.message_driver().messages().is_empty());
    }

    #[test]
    fn test_set_document_version() {
        let app = CDM_Application::new();
        let mut doc = CDM_Document::new();
        let meta = CDM_MetaData::new(7);
        app.set_document_version(&mut doc, &meta);
        assert_eq!(doc.modifications(), 7);
    }

    #[test]
    fn test_set_reference_counter() {
        let app = CDM_Application::new();
        let mut doc = CDM_Document::new();
        app.set_reference_counter(&mut doc, 3);
        assert_eq!(doc.reference_counter(), 3);
    }

    #[test]
    fn test_update_notifications() {
        let app = CDM_Application::new();
        let doc = CDM_Document::new();
        app.begin_of_update(&doc);
        app.end_of_update(&doc, true, "");
        app.end_of_update(&doc, false, "boom");
        let msgs = app.message_driver().messages();
        assert_eq!(
            msgs,
            vec![
                "Updating: Document".to_string(),
                "Updated: Document".to_string(),
                "Error during updating: Document".to_string(),
            ]
        );
    }

    #[test]
    fn test_meta_data_lookup_table() {
        let mut app = CDM_Application::new();
        app.meta_data_lookup_table()
            .insert("dir|file".to_string(), CDM_MetaData::new(2));
        assert_eq!(app.meta_data_lookup_table().len(), 1);
        assert_eq!(
            app.meta_data_lookup_table()
                .get("dir|file")
                .map(|m| m.document_version()),
            Some(2)
        );
    }
}
