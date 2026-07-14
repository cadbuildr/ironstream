// FILE: caf_session.rs
// occt: CDF_Application, CDM_Document
// occt-ref: CDF_Session, CDF_Directory

/// Document state in the application session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum CdfDocumentState {
    #[default]
    Unknown,
    Modified,
    Stored,
    Closed,
}

/// Storage format for persisting documents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum CdfStorageFormat {
    #[default]
    Binary,
    Xml,
    Ascii,
}

impl CdfStorageFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Binary => "cbf",
            Self::Xml    => "xbf",
            Self::Ascii  => "acf",
        }
    }
}

/// A document entry in the CDF session.
/// occt: CDM_Document
#[derive(Clone, Debug)]
pub struct CdfDocumentEntry {
    pub doc_id: u32,
    pub name: String,
    pub format: CdfStorageFormat,
    pub state: CdfDocumentState,
    pub path: Option<String>,
    pub is_modified: bool,
}

impl CdfDocumentEntry {
    pub fn new(doc_id: u32, name: impl Into<String>, format: CdfStorageFormat) -> Self {
        Self {
            doc_id,
            name: name.into(),
            format,
            state: CdfDocumentState::Unknown,
            path: None,
            is_modified: false,
        }
    }

    pub fn set_path(&mut self, p: impl Into<String>) {
        self.path = Some(p.into());
        self.state = CdfDocumentState::Stored;
    }

    pub fn mark_modified(&mut self) {
        self.is_modified = true;
        self.state = CdfDocumentState::Modified;
    }

    pub fn mark_stored(&mut self) {
        self.is_modified = false;
        self.state = CdfDocumentState::Stored;
    }

    pub fn close(&mut self) {
        self.state = CdfDocumentState::Closed;
    }

    pub fn is_stored(&self) -> bool { self.state == CdfDocumentState::Stored }
}

/// Directory in the CDF session (a named set of documents).
// occt-ref: CDF_Directory
#[derive(Clone, Debug, Default)]
pub struct CdfDirectory {
    pub name: String,
    pub doc_ids: Vec<u32>,
}

impl CdfDirectory {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), doc_ids: Vec::new() }
    }

    pub fn add(&mut self, doc_id: u32) {
        if !self.doc_ids.contains(&doc_id) { self.doc_ids.push(doc_id); }
    }

    pub fn remove(&mut self, doc_id: u32) -> bool {
        let before = self.doc_ids.len();
        self.doc_ids.retain(|&d| d != doc_id);
        self.doc_ids.len() < before
    }

    pub fn contains(&self, doc_id: u32) -> bool { self.doc_ids.contains(&doc_id) }
    pub fn nb_documents(&self) -> usize { self.doc_ids.len() }
    pub fn is_empty(&self) -> bool { self.doc_ids.is_empty() }
}

/// CDF application: manages one or more document formats.
/// occt: CDF_Application
#[derive(Clone, Debug)]
pub struct CdfApplication {
    pub app_id: u32,
    pub name: String,
    pub supported_formats: Vec<CdfStorageFormat>,
    pub version: String,
}

impl CdfApplication {
    pub fn new(app_id: u32, name: impl Into<String>) -> Self {
        Self {
            app_id,
            name: name.into(),
            supported_formats: vec![CdfStorageFormat::Binary, CdfStorageFormat::Xml],
            version: "1.0".into(),
        }
    }

    pub fn set_version(&mut self, v: impl Into<String>) { self.version = v.into(); }

    pub fn supports_format(&self, fmt: CdfStorageFormat) -> bool {
        self.supported_formats.contains(&fmt)
    }

    pub fn add_format(&mut self, fmt: CdfStorageFormat) {
        if !self.supported_formats.contains(&fmt) {
            self.supported_formats.push(fmt);
        }
    }

    pub fn resources_name(&self) -> String { format!("{}.res", self.name) }
}

/// CDF session: top-level manager for documents and applications.
// occt-ref: CDF_Session
#[derive(Clone, Debug, Default)]
pub struct CdfSession {
    pub session_id: u32,
    pub documents: Vec<CdfDocumentEntry>,
    pub applications: Vec<CdfApplication>,
    pub directories: Vec<CdfDirectory>,
    next_doc_id: u32,
}

impl CdfSession {
    pub fn new(session_id: u32) -> Self {
        Self { session_id, next_doc_id: 1, ..Self::default() }
    }

    pub fn new_document(&mut self, name: impl Into<String>, fmt: CdfStorageFormat) -> u32 {
        let id = self.next_doc_id;
        self.next_doc_id += 1;
        self.documents.push(CdfDocumentEntry::new(id, name, fmt));
        id
    }

    pub fn find_document(&self, doc_id: u32) -> Option<&CdfDocumentEntry> {
        self.documents.iter().find(|d| d.doc_id == doc_id)
    }

    pub fn find_document_mut(&mut self, doc_id: u32) -> Option<&mut CdfDocumentEntry> {
        self.documents.iter_mut().find(|d| d.doc_id == doc_id)
    }

    pub fn close_document(&mut self, doc_id: u32) -> bool {
        if let Some(d) = self.find_document_mut(doc_id) {
            d.close();
            true
        } else {
            false
        }
    }

    pub fn register_application(&mut self, app: CdfApplication) {
        if !self.applications.iter().any(|a| a.app_id == app.app_id) {
            self.applications.push(app);
        }
    }

    pub fn add_directory(&mut self, dir: CdfDirectory) {
        if !self.directories.iter().any(|d| d.name == dir.name) {
            self.directories.push(dir);
        }
    }

    pub fn nb_documents(&self) -> usize { self.documents.len() }
    pub fn nb_applications(&self) -> usize { self.applications.len() }

    pub fn has_opened_documents(&self) -> bool {
        self.documents.iter().any(|d| d.state != CdfDocumentState::Closed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_new_document() {
        let mut sess = CdfSession::new(1);
        let id1 = sess.new_document("Part1", CdfStorageFormat::Binary);
        let id2 = sess.new_document("Part2", CdfStorageFormat::Xml);
        assert_eq!(sess.nb_documents(), 2);
        assert_ne!(id1, id2);
        assert!(sess.has_opened_documents());
    }

    #[test]
    fn document_state_transitions() {
        let mut sess = CdfSession::new(1);
        let id = sess.new_document("Doc", CdfStorageFormat::Binary);
        {
            let d = sess.find_document_mut(id).unwrap();
            d.mark_modified();
            assert_eq!(d.state, CdfDocumentState::Modified);
            d.set_path("/tmp/doc.cbf");
            assert!(d.is_stored());
        }
        sess.close_document(id);
        assert_eq!(sess.find_document(id).unwrap().state, CdfDocumentState::Closed);
    }

    #[test]
    fn directory_add_remove() {
        let mut dir = CdfDirectory::new("default");
        dir.add(1);
        dir.add(2);
        dir.add(1); // dup
        assert_eq!(dir.nb_documents(), 2);
        assert!(dir.contains(2));
        dir.remove(1);
        assert!(!dir.contains(1));
        assert_eq!(dir.nb_documents(), 1);
    }

    #[test]
    fn application_formats() {
        let mut app = CdfApplication::new(1, "MyCAD");
        assert!(app.supports_format(CdfStorageFormat::Binary));
        assert!(!app.supports_format(CdfStorageFormat::Ascii));
        app.add_format(CdfStorageFormat::Ascii);
        assert!(app.supports_format(CdfStorageFormat::Ascii));
        assert_eq!(app.resources_name(), "MyCAD.res");
    }

    #[test]
    fn storage_format_extensions() {
        assert_eq!(CdfStorageFormat::Binary.extension(), "cbf");
        assert_eq!(CdfStorageFormat::Xml.extension(), "xbf");
        assert_eq!(CdfStorageFormat::Ascii.extension(), "acf");
    }
}
