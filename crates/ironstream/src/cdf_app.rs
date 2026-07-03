// FILE: rust/ironstream/crates/ironstream/src/cdf_app.rs

// occt: CDF — OCAF application framework (document lifecycle management)

// ---------------------------------------------------------------------------
// CdfDocumentStatus
// ---------------------------------------------------------------------------

// occt: CDF_Document status
/// The lifecycle state of a CDF document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CdfDocumentStatus {
    /// Document has been created but never saved.
    Created,
    /// Document has been closed.
    Closed,
    /// Document has unsaved modifications.
    Modified,
    /// Document has been saved and is unmodified.
    Saved,
    /// Document has been notified of external changes.
    Notified,
}

// ---------------------------------------------------------------------------
// CdfDocument
// ---------------------------------------------------------------------------

// occt: CDF_Document
/// A document managed by a `CdfApplication`.
///
/// Tracks identity, format, and modification state analogous to OCCT's
/// `CDF_Document` / `TDocStd_Document`.
#[derive(Debug)]
pub struct CdfDocument {
    id: usize,
    name: String,
    format: String,
    status: CdfDocumentStatus,
    is_modified: bool,
}

impl CdfDocument {
    /// Create a new document with the given id, name and format string.
    /// Initial status is `CdfDocumentStatus::Created`.
    pub fn new(id: usize, name: &str, format: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
            format: format.to_owned(),
            status: CdfDocumentStatus::Created,
            is_modified: false,
        }
    }

    /// Unique numeric identifier assigned by the owning application.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Human-readable name of the document (e.g. file path or label).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Format tag (e.g. `"BinOCAF"`, `"XmlOCAF"`, `"STEP"`).
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Current lifecycle status.
    pub fn status(&self) -> CdfDocumentStatus {
        self.status
    }

    /// `true` when the document has been modified since the last save.
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// Transition to `Modified` state.
    pub fn mark_modified(&mut self) {
        self.is_modified = true;
        self.status = CdfDocumentStatus::Modified;
    }

    /// Transition to `Saved` state, clearing the modified flag.
    pub fn mark_saved(&mut self) {
        self.is_modified = false;
        self.status = CdfDocumentStatus::Saved;
    }
}

// ---------------------------------------------------------------------------
// CdfApplication
// ---------------------------------------------------------------------------

// occt: CDF_Application
/// Manages the lifecycle of a set of `CdfDocument` instances.
///
/// Mirrors the role of `CDF_Application` / `TDocStd_Application` in OCCT:
/// creating, opening, closing, and saving documents.
pub struct CdfApplication {
    documents: Vec<CdfDocument>,
    next_id: usize,
    name: String,
}

impl CdfApplication {
    /// Create a new application with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            documents: Vec::new(),
            next_id: 1,
            name: name.to_owned(),
        }
    }

    /// The name of this application.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Create a new document with the given name and format.
    ///
    /// Returns the unique document id that can be used to retrieve or
    /// manipulate the document later.
    pub fn new_document(&mut self, name: &str, format: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.documents.push(CdfDocument::new(id, name, format));
        id
    }

    /// Find a document by name and return its id.
    ///
    /// Returns `None` if no non-closed document with that name exists.
    pub fn open_document(&self, name: &str) -> Option<usize> {
        self.documents
            .iter()
            .find(|d| d.name() == name && d.status() != CdfDocumentStatus::Closed)
            .map(|d| d.id())
    }

    /// Close the document with the given id.
    ///
    /// Sets its status to `Closed`. Returns `false` if no such document exists.
    pub fn close_document(&mut self, id: usize) -> bool {
        match self.documents.iter_mut().find(|d| d.id() == id) {
            Some(doc) => {
                doc.status = CdfDocumentStatus::Closed;
                true
            }
            None => false,
        }
    }

    /// Save the document with the given id (marks it as `Saved`).
    ///
    /// Returns `false` if no such document exists.
    pub fn save_document(&mut self, id: usize) -> bool {
        match self.documents.iter_mut().find(|d| d.id() == id) {
            Some(doc) => {
                doc.mark_saved();
                true
            }
            None => false,
        }
    }

    /// Retrieve a shared reference to the document with the given id.
    ///
    /// Returns `None` if no document with that id exists.
    pub fn find_document(&self, id: usize) -> Option<&CdfDocument> {
        self.documents.iter().find(|d| d.id() == id)
    }

    /// Total number of documents known to this application (including closed).
    pub fn nb_documents(&self) -> usize {
        self.documents.len()
    }

    /// `true` when the document with the given id has unsaved modifications.
    ///
    /// Returns `false` if the document does not exist.
    pub fn is_modified(&self, id: usize) -> bool {
        self.documents
            .iter()
            .find(|d| d.id() == id)
            .map(|d| d.is_modified())
            .unwrap_or(false)
    }
}
