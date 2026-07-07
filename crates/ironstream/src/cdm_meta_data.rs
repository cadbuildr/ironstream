// FILE: cdm_meta_data.rs
// occt: CDM_MetaData

//! Meta-data describing where a stored document lives (folder, name, path,
//! optional version, file name) plus retrieval state and read-only flag.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for CDM_Document (only identity is needed here).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CDM_Document {
    pub id: u32,
}

/// CDM_MetaData port. Handles are modeled with Rc<RefCell<...>> so that the
/// static LookUp table can share instances like OCCT handles do.
#[derive(Debug)]
pub struct CDM_MetaData {
    is_retrieved: bool,
    document: Option<CDM_Document>,
    folder: String,
    name: String,
    version: Option<String>,
    file_name: String,
    path: String,
    document_version: i32,
    is_read_only: bool,
}

pub type MetaDataHandle = Rc<RefCell<CDM_MetaData>>;

impl CDM_MetaData {
    /// Constructor without version.
    pub fn new(folder: &str, name: &str, path: &str, file_name: &str, read_only: bool) -> Self {
        CDM_MetaData {
            is_retrieved: false,
            document: None,
            folder: folder.to_string(),
            name: name.to_string(),
            version: None,
            file_name: file_name.to_string(),
            path: path.to_string(),
            document_version: 0,
            is_read_only: read_only,
        }
    }

    /// Constructor with version.
    pub fn new_with_version(
        folder: &str,
        name: &str,
        path: &str,
        version: &str,
        file_name: &str,
        read_only: bool,
    ) -> Self {
        let mut md = Self::new(folder, name, path, file_name, read_only);
        md.version = Some(version.to_string());
        md
    }

    /// LookUp: finds or creates the metadata bound to the conventional path
    /// (backslashes replaced by slashes) in the look-up table.
    pub fn look_up(
        look_up_table: &mut HashMap<String, MetaDataHandle>,
        folder: &str,
        name: &str,
        path: &str,
        file_name: &str,
        read_only: bool,
    ) -> MetaDataHandle {
        let conventional_path = path.replace('\\', "/");
        if let Some(existing) = look_up_table.get(&conventional_path) {
            return Rc::clone(existing);
        }
        let md = Rc::new(RefCell::new(CDM_MetaData::new(
            folder, name, path, file_name, read_only,
        )));
        look_up_table.insert(conventional_path, Rc::clone(&md));
        md
    }

    /// LookUp with version.
    pub fn look_up_with_version(
        look_up_table: &mut HashMap<String, MetaDataHandle>,
        folder: &str,
        name: &str,
        path: &str,
        version: &str,
        file_name: &str,
        read_only: bool,
    ) -> MetaDataHandle {
        let conventional_path = path.replace('\\', "/");
        if let Some(existing) = look_up_table.get(&conventional_path) {
            return Rc::clone(existing);
        }
        let md = Rc::new(RefCell::new(CDM_MetaData::new_with_version(
            folder, name, path, version, file_name, read_only,
        )));
        look_up_table.insert(conventional_path, Rc::clone(&md));
        md
    }

    pub fn is_retrieved(&self) -> bool {
        self.is_retrieved
    }

    pub fn document(&self) -> Option<&CDM_Document> {
        self.document.as_ref()
    }

    /// SetDocument: marks the metadata as retrieved.
    pub fn set_document(&mut self, document: CDM_Document) {
        self.is_retrieved = true;
        self.document = Some(document);
    }

    /// UnsetDocument: clears the retrieved flag (the pointer is kept in OCCT).
    pub fn unset_document(&mut self) {
        self.is_retrieved = false;
    }

    /// Folder in which the meta-data has to be created or found.
    pub fn folder(&self) -> &str {
        &self.folder
    }

    /// Name under which the meta-data has to be created or found.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Version; panics like Standard_NoSuchObject if no version was defined.
    pub fn version(&self) -> &str {
        self.version
            .as_deref()
            .expect("Document has no version")
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub fn set_is_read_only(&mut self) {
        self.is_read_only = true;
    }

    pub fn unset_is_read_only(&mut self) {
        self.is_read_only = false;
    }

    /// DocumentVersion: lazily fetched from the application on first call.
    pub fn document_version(&mut self, application_version: impl Fn() -> i32) -> i32 {
        if self.document_version == 0 {
            self.document_version = application_version();
        }
        self.document_version
    }

    /// Print: mirrors CDM_MetaData::Print formatting.
    pub fn print(&self) -> String {
        let mut s = String::from("*CDM_MetaData*");
        s += &self.folder;
        s += ",";
        s += &self.name;
        if let Some(v) = &self.version {
            s += ",";
            s += v;
        }
        s += "; Physical situation: ";
        s += &self.file_name;
        s += "\n";
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdm_meta_data_creation() {
        let md = CDM_MetaData::new("folder", "name", "/tmp/doc", "doc.dat", false);
        assert!(!md.is_retrieved());
        assert!(md.document().is_none());
        assert_eq!(md.folder(), "folder");
        assert_eq!(md.name(), "name");
        assert_eq!(md.path(), "/tmp/doc");
        assert_eq!(md.file_name(), "doc.dat");
        assert!(!md.has_version());
        assert!(!md.is_read_only());
    }

    #[test]
    fn test_version() {
        let md = CDM_MetaData::new_with_version("f", "n", "p", "1.2", "fn", true);
        assert!(md.has_version());
        assert_eq!(md.version(), "1.2");
        assert!(md.is_read_only());
    }

    #[test]
    fn test_set_unset_document() {
        let mut md = CDM_MetaData::new("f", "n", "p", "fn", false);
        md.set_document(CDM_Document { id: 42 });
        assert!(md.is_retrieved());
        assert_eq!(md.document(), Some(&CDM_Document { id: 42 }));
        md.unset_document();
        assert!(!md.is_retrieved());
    }

    #[test]
    fn test_read_only_flags() {
        let mut md = CDM_MetaData::new("f", "n", "p", "fn", false);
        md.set_is_read_only();
        assert!(md.is_read_only());
        md.unset_is_read_only();
        assert!(!md.is_read_only());
    }

    #[test]
    fn test_look_up_creates_then_reuses() {
        let mut table: HashMap<String, MetaDataHandle> = HashMap::new();
        let a = CDM_MetaData::look_up(&mut table, "f", "n", "C:\\dir\\doc", "doc.dat", false);
        assert_eq!(table.len(), 1);
        // Backslashes are normalized: same document under conventional path.
        let b = CDM_MetaData::look_up(&mut table, "f2", "n2", "C:/dir/doc", "other.dat", true);
        assert_eq!(table.len(), 1);
        assert!(Rc::ptr_eq(&a, &b));
        // The reused entry keeps the original attributes.
        assert_eq!(b.borrow().name(), "n");
    }

    #[test]
    fn test_look_up_with_version() {
        let mut table: HashMap<String, MetaDataHandle> = HashMap::new();
        let a = CDM_MetaData::look_up_with_version(&mut table, "f", "n", "/p1", "2.0", "fn", false);
        assert!(a.borrow().has_version());
        let c = CDM_MetaData::look_up(&mut table, "f", "n", "/p2", "fn", false);
        assert_eq!(table.len(), 2);
        assert!(!Rc::ptr_eq(&a, &c));
    }

    #[test]
    fn test_document_version_lazy() {
        let mut md = CDM_MetaData::new("f", "n", "p", "fn", false);
        assert_eq!(md.document_version(|| 5), 5);
        // Cached: the application is not asked again.
        assert_eq!(md.document_version(|| 99), 5);
    }

    #[test]
    fn test_print() {
        let md = CDM_MetaData::new_with_version("fold", "nam", "p", "3", "file.d", false);
        assert_eq!(md.print(), "*CDM_MetaData*fold,nam,3; Physical situation: file.d\n");
        let md2 = CDM_MetaData::new("fold", "nam", "p", "file.d", false);
        assert_eq!(md2.print(), "*CDM_MetaData*fold,nam; Physical situation: file.d\n");
    }
}
