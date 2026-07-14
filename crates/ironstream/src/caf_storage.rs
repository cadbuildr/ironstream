// FILE: caf_storage.rs
// occt: CDF_Store, CDF_Directory
// occt-ref: CDF_MetaData, CDF_Session

use std::collections::HashMap;

/// Mode used when storing a document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdfStoreMode {
    NoCurrentData,        // never been stored
    StorageData,          // storage info available
    RetrievalData,        // read from storage
    StorageAndRetrievalData,
}

impl Default for CdfStoreMode {
    fn default() -> Self { Self::NoCurrentData }
}

/// Format of stored document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdfStorageFormat {
    Unknown,
    XmlDocument,
    BinaryDocument,
    StepDocument,
    IgesDocument,
}

impl Default for CdfStorageFormat {
    fn default() -> Self { Self::Unknown }
}

impl CdfStorageFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::XmlDocument => ".xml",
            Self::BinaryDocument => ".cbf",
            Self::StepDocument => ".stp",
            Self::IgesDocument => ".igs",
            Self::Unknown => "",
        }
    }
}

// occt-note: CDF_MetaData — identity record for a document in a repository
#[derive(Clone, Debug, Default)]
pub struct CdfMetaData {
    pub folder: String,
    pub name: String,
    pub version: String,
    pub document_name: String,
    pub format: CdfStorageFormat,
    pub is_read_only: bool,
    pub is_stored: bool,
}

impl CdfMetaData {
    pub fn new(folder: &str, name: &str) -> Self {
        Self {
            folder: folder.to_owned(),
            name: name.to_owned(),
            version: "1.0".to_owned(),
            ..Default::default()
        }
    }

    pub fn full_path(&self) -> String {
        format!("{}/{}{}", self.folder, self.name, self.format.extension())
    }

    pub fn is_null(&self) -> bool { self.name.is_empty() }
    pub fn folder(&self) -> &str { &self.folder }
    pub fn name(&self) -> &str { &self.name }
    pub fn version(&self) -> &str { &self.version }
    pub fn document_name(&self) -> &str { &self.document_name }
}

// occt: CDF_Store // — manages store/retrieve for one document
#[derive(Clone, Debug)]
pub struct CdfStore {
    pub doc_id: u32,
    pub meta: CdfMetaData,
    pub mode: CdfStoreMode,
    pub is_modified: bool,
    pub nb_stored: u32,
}

impl CdfStore {
    pub fn new(doc_id: u32) -> Self {
        Self {
            doc_id,
            meta: CdfMetaData::default(),
            mode: CdfStoreMode::NoCurrentData,
            is_modified: false,
            nb_stored: 0,
        }
    }

    pub fn set_meta(&mut self, meta: CdfMetaData) {
        self.meta = meta;
        self.mode = CdfStoreMode::StorageData;
    }

    pub fn store(&mut self) -> bool {
        if self.meta.is_null() { return false; }
        self.meta.is_stored = true;
        self.mode = CdfStoreMode::StorageAndRetrievalData;
        self.is_modified = false;
        self.nb_stored += 1;
        true
    }

    pub fn mark_modified(&mut self) { self.is_modified = true; }
    pub fn current_mode(&self) -> CdfStoreMode { self.mode }
    pub fn has_been_stored(&self) -> bool { self.nb_stored > 0 }
    pub fn path(&self) -> String { self.meta.full_path() }
}

// occt: CDF_Directory // — list of documents in a repository
#[derive(Clone, Debug, Default)]
pub struct CdfDirectory {
    pub entries: Vec<CdfMetaData>,
}

impl CdfDirectory {
    pub fn new() -> Self { Self::default() }

    pub fn new_meta(&mut self, folder: &str, name: &str) -> usize {
        let meta = CdfMetaData::new(folder, name);
        self.entries.push(meta);
        self.entries.len() - 1
    }

    pub fn is_stored(&self, folder: &str, name: &str) -> bool {
        self.entries.iter().any(|m| m.folder == folder && m.name == name && m.is_stored)
    }

    pub fn find(&self, folder: &str, name: &str) -> Option<&CdfMetaData> {
        self.entries.iter().find(|m| m.folder == folder && m.name == name)
    }

    pub fn nb_documents(&self) -> usize { self.entries.len() }
    pub fn nb_stored_documents(&self) -> usize { self.entries.iter().filter(|m| m.is_stored).count() }
}

// occt-note: CDF_Session — manages the active set of open documents
#[derive(Clone, Debug, Default)]
pub struct CdfSession {
    pub documents: HashMap<u32, CdfStore>,
    pub current_doc_id: Option<u32>,
    next_doc_id: u32,
}

impl CdfSession {
    pub fn new() -> Self { Self { next_doc_id: 1, ..Default::default() } }

    pub fn open_document(&mut self) -> u32 {
        let id = self.next_doc_id;
        self.next_doc_id += 1;
        self.documents.insert(id, CdfStore::new(id));
        self.current_doc_id = Some(id);
        id
    }

    pub fn close_document(&mut self, id: u32) -> bool {
        let removed = self.documents.remove(&id).is_some();
        if self.current_doc_id == Some(id) {
            self.current_doc_id = self.documents.keys().next().copied();
        }
        removed
    }

    pub fn find_document(&self, id: u32) -> Option<&CdfStore> {
        self.documents.get(&id)
    }

    pub fn find_document_mut(&mut self, id: u32) -> Option<&mut CdfStore> {
        self.documents.get_mut(&id)
    }

    pub fn current_document(&self) -> Option<&CdfStore> {
        self.current_doc_id.and_then(|id| self.documents.get(&id))
    }

    pub fn nb_documents(&self) -> usize { self.documents.len() }
    pub fn has_documents(&self) -> bool { !self.documents.is_empty() }

    pub fn store_all(&mut self) -> usize {
        let mut count = 0;
        for doc in self.documents.values_mut() {
            if doc.is_modified && doc.store() { count += 1; }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cdf_meta_data_path() {
        let mut meta = CdfMetaData::new("/tmp", "test");
        meta.format = CdfStorageFormat::XmlDocument;
        assert_eq!(meta.full_path(), "/tmp/test.xml");
        assert!(!meta.is_null());
    }

    #[test]
    fn cdf_store_workflow() {
        let mut store = CdfStore::new(1);
        let mut meta = CdfMetaData::new("/repo", "part1");
        meta.format = CdfStorageFormat::BinaryDocument;
        store.set_meta(meta);
        assert_eq!(store.current_mode(), CdfStoreMode::StorageData);
        store.mark_modified();
        assert!(store.is_modified);
        assert!(store.store());
        assert!(!store.is_modified);
        assert!(store.has_been_stored());
    }

    #[test]
    fn cdf_directory() {
        let mut dir = CdfDirectory::new();
        let idx = dir.new_meta("/r", "doc1");
        assert_eq!(dir.nb_documents(), 1);
        assert!(!dir.is_stored("/r", "doc1"));
        dir.entries[idx].is_stored = true;
        assert!(dir.is_stored("/r", "doc1"));
    }

    #[test]
    fn cdf_session_open_close() {
        let mut session = CdfSession::new();
        let id1 = session.open_document();
        let id2 = session.open_document();
        assert_eq!(session.nb_documents(), 2);
        assert!(session.close_document(id1));
        assert_eq!(session.nb_documents(), 1);
        assert!(session.find_document(id2).is_some());
        assert!(session.find_document(id1).is_none());
    }

    #[test]
    fn cdf_session_store_all() {
        let mut session = CdfSession::new();
        let id = session.open_document();
        {
            let doc = session.find_document_mut(id).unwrap();
            let mut meta = CdfMetaData::new("/r", "d1");
            meta.format = CdfStorageFormat::XmlDocument;
            doc.set_meta(meta);
            doc.mark_modified();
        }
        let stored = session.store_all();
        assert_eq!(stored, 1);
    }

    #[test]
    fn cdf_format_extensions() {
        assert_eq!(CdfStorageFormat::XmlDocument.extension(), ".xml");
        assert_eq!(CdfStorageFormat::BinaryDocument.extension(), ".cbf");
        assert_eq!(CdfStorageFormat::StepDocument.extension(), ".stp");
    }
}
