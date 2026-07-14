// FILE: tdoc_std.rs
// occt: TDocStd_Application, TDocStd_Modified
// occt-ref: TDocStd_Document

use std::collections::HashMap;

/// Format identifier for a document type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocFormat(pub String);

impl DocFormat {
    pub fn new(name: &str) -> Self { Self(name.to_owned()) }
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn is_null(&self) -> bool { self.0.is_empty() }
}

/// Status of a document open/save operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocStatus {
    Ok,
    NoDriver,
    DriverFailure,
    ExtensionFailure,
    OpenError,
    SaveError,
    NotFound,
    AlreadyOpen,
    DocNullOrUnknown,
    NotStored,
}

impl Default for DocStatus {
    fn default() -> Self { Self::Ok }
}

impl DocStatus {
    pub fn is_ok(&self) -> bool { *self == DocStatus::Ok }
    pub fn message(&self) -> &'static str {
        match self {
            Self::Ok => "Ok",
            Self::NoDriver => "No driver",
            Self::DriverFailure => "Driver failure",
            Self::ExtensionFailure => "Extension failure",
            Self::OpenError => "Open error",
            Self::SaveError => "Save error",
            Self::NotFound => "Not found",
            Self::AlreadyOpen => "Already open",
            Self::DocNullOrUnknown => "Document null or unknown",
            Self::NotStored => "Not stored",
        }
    }
}

/// A document's storage resource path information.
#[derive(Clone, Debug, Default)]
pub struct DocPath {
    pub folder: String,
    pub name: String,
    pub extension: String,
}

impl DocPath {
    pub fn new(folder: &str, name: &str, ext: &str) -> Self {
        Self { folder: folder.to_owned(), name: name.to_owned(), extension: ext.to_owned() }
    }

    pub fn full_path(&self) -> String {
        if self.extension.is_empty() {
            format!("{}/{}", self.folder, self.name)
        } else {
            format!("{}/{}.{}", self.folder, self.name, self.extension)
        }
    }
}

// occt-ref: TDocStd_Document // — a TDocStd document (wraps a TDF_Data with XCAF tools)
#[derive(Clone, Debug)]
pub struct TDocStdDocument {
    pub id: u32,
    pub format: DocFormat,
    pub path: Option<DocPath>,
    pub is_modified: bool,
    pub is_stored: bool,
    pub undo_limit: usize,
    pub undo_count: usize,
    pub redo_count: usize,
    pub main_label: u32,
    pub attributes: HashMap<String, String>,
}

impl TDocStdDocument {
    pub fn new(id: u32, format: DocFormat) -> Self {
        Self {
            id,
            format,
            path: None,
            is_modified: false,
            is_stored: false,
            undo_limit: 10,
            undo_count: 0,
            redo_count: 0,
            main_label: 0,
            attributes: HashMap::new(),
        }
    }

    pub fn set_undo_limit(&mut self, n: usize) { self.undo_limit = n; }
    pub fn undo_limit(&self) -> usize { self.undo_limit }

    pub fn new_command(&mut self) { self.undo_count += 1; self.redo_count = 0; self.is_modified = true; }
    pub fn undo(&mut self) -> bool {
        if self.undo_count > 0 { self.undo_count -= 1; self.redo_count += 1; true } else { false }
    }
    pub fn redo(&mut self) -> bool {
        if self.redo_count > 0 { self.redo_count -= 1; self.undo_count += 1; true } else { false }
    }
    pub fn can_undo(&self) -> bool { self.undo_count > 0 }
    pub fn can_redo(&self) -> bool { self.redo_count > 0 }
    pub fn clear_undo_redo(&mut self) { self.undo_count = 0; self.redo_count = 0; }

    pub fn set_path(&mut self, path: DocPath) { self.path = Some(path); }
    pub fn is_saved(&self) -> bool { self.is_stored && !self.is_modified }
    pub fn mark_saved(&mut self) { self.is_stored = true; self.is_modified = false; }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_owned(), value.to_owned());
    }
    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(|s| s.as_str())
    }
}

// occt: TDocStd_Application // — manages document creation, open, close, save
#[derive(Clone, Debug, Default)]
pub struct TDocStdApplication {
    pub documents: HashMap<u32, TDocStdDocument>,
    pub resources_name: String,
    next_id: u32,
}

impl TDocStdApplication {
    pub fn new() -> Self { Self { next_id: 1, resources_name: "XCAFApp".to_owned(), ..Default::default() } }

    pub fn new_document(&mut self, format: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.documents.insert(id, TDocStdDocument::new(id, DocFormat::new(format)));
        id
    }

    pub fn open(&mut self, _path: &str, format: &str) -> (u32, DocStatus) {
        let id = self.new_document(format);
        if let Some(doc) = self.documents.get_mut(&id) {
            doc.is_stored = true;
        }
        (id, DocStatus::Ok)
    }

    pub fn save_as(&mut self, id: u32, folder: &str, name: &str, ext: &str) -> DocStatus {
        if let Some(doc) = self.documents.get_mut(&id) {
            doc.set_path(DocPath::new(folder, name, ext));
            doc.mark_saved();
            DocStatus::Ok
        } else {
            DocStatus::NotFound
        }
    }

    pub fn close(&mut self, id: u32) -> bool { self.documents.remove(&id).is_some() }

    pub fn find_document(&self, id: u32) -> Option<&TDocStdDocument> { self.documents.get(&id) }
    pub fn find_document_mut(&mut self, id: u32) -> Option<&mut TDocStdDocument> { self.documents.get_mut(&id) }

    pub fn nb_documents(&self) -> usize { self.documents.len() }
    pub fn is_driver_defined(&self, format: &str) -> bool { !format.is_empty() }
    pub fn formats(&self) -> Vec<String> {
        let mut fmts: Vec<String> = self.documents.values().map(|d| d.format.0.clone()).collect();
        fmts.dedup();
        fmts
    }
}

// occt: TDocStd_Modified // — tracks which labels were changed in a transaction
#[derive(Clone, Debug, Default)]
pub struct TDocStdModified {
    pub modified_labels: Vec<u32>,
    pub transaction_depth: i32,
}

impl TDocStdModified {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, label: u32) {
        if !self.modified_labels.contains(&label) {
            self.modified_labels.push(label);
        }
    }

    pub fn remove(&mut self, label: u32) { self.modified_labels.retain(|&l| l != label); }
    pub fn contains(&self, label: u32) -> bool { self.modified_labels.contains(&label) }
    pub fn is_empty(&self) -> bool { self.modified_labels.is_empty() }
    pub fn clear(&mut self) { self.modified_labels.clear(); }
    pub fn nb_modified(&self) -> usize { self.modified_labels.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_new_document() {
        let mut app = TDocStdApplication::new();
        let id = app.new_document("BinXCAF");
        assert!(app.find_document(id).is_some());
        assert_eq!(app.nb_documents(), 1);
    }

    #[test]
    fn document_undo_redo() {
        let mut app = TDocStdApplication::new();
        let id = app.new_document("XmlXCAF");
        let doc = app.find_document_mut(id).unwrap();
        doc.new_command();
        doc.new_command();
        assert!(doc.can_undo());
        assert!(doc.undo());
        assert!(doc.can_redo());
        doc.clear_undo_redo();
        assert!(!doc.can_undo());
    }

    #[test]
    fn application_save_close() {
        let mut app = TDocStdApplication::new();
        let id = app.new_document("XmlXCAF");
        let st = app.save_as(id, "/tmp", "model", "xml");
        assert!(st.is_ok());
        let doc = app.find_document(id).unwrap();
        assert!(doc.is_saved());
        assert!(app.close(id));
        assert_eq!(app.nb_documents(), 0);
    }

    #[test]
    fn document_attributes() {
        let mut doc = TDocStdDocument::new(1, DocFormat::new("XmlXCAF"));
        doc.set_attribute("author", "Alice");
        assert_eq!(doc.get_attribute("author"), Some("Alice"));
        assert!(doc.get_attribute("missing").is_none());
    }

    #[test]
    fn doc_status_messages() {
        assert_eq!(DocStatus::Ok.message(), "Ok");
        assert!(DocStatus::Ok.is_ok());
        assert!(!DocStatus::NoDriver.is_ok());
    }

    #[test]
    fn modified_tracker() {
        let mut m = TDocStdModified::new();
        m.add(1); m.add(2); m.add(1);
        assert_eq!(m.nb_modified(), 2);
        m.remove(1);
        assert!(!m.contains(1));
        assert!(m.contains(2));
    }
}
