// FILE: ocaf_transaction.rs
// occt: TDocStd_Application, TDocStd_Document, CDM_Document, TDocStd_Modified

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocStatus {
    Current,        // open and current
    Closed,
    Modified,
    StorageError,
    RetrievalError,
}

/// Application-level format descriptor.
#[derive(Clone, Debug)]
pub struct DocFormat {
    pub name: String,
    pub extension: String,
    pub version: u32,
}

impl DocFormat {
    pub fn new(name: impl Into<String>, ext: impl Into<String>) -> Self {
        Self { name: name.into(), extension: ext.into(), version: 1 }
    }
}

// occt: TDocStd_Document (single document with undo/redo transactions)
#[derive(Clone, Debug)]
pub struct StdDocument {
    pub id: u64,
    pub name: String,
    pub format: DocFormat,
    pub status: DocStatus,
    pub is_modified: bool,
    pub undo_limit: usize,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub comment: String,
}

impl StdDocument {
    pub fn new(id: u64, name: impl Into<String>, format: DocFormat) -> Self {
        Self {
            id, name: name.into(), format, status: DocStatus::Current,
            is_modified: false, undo_limit: 10,
            undo_stack: Vec::new(), redo_stack: Vec::new(), comment: String::new(),
        }
    }

    pub fn open_transaction(&mut self, description: impl Into<String>) {
        self.undo_stack.push(description.into());
        if self.undo_stack.len() > self.undo_limit { self.undo_stack.remove(0); }
        self.redo_stack.clear();
        self.is_modified = true;
    }

    pub fn commit_transaction(&mut self) { /* transaction already pushed */ }

    pub fn abort_transaction(&mut self) {
        self.undo_stack.pop();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(op) = self.undo_stack.pop() {
            self.redo_stack.push(op);
            true
        } else { false }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(op) = self.redo_stack.pop() {
            self.undo_stack.push(op);
            true
        } else { false }
    }

    pub fn nb_undos(&self) -> usize { self.undo_stack.len() }
    pub fn nb_redos(&self) -> usize { self.redo_stack.len() }

    pub fn close(&mut self) { self.status = DocStatus::Closed; }
    pub fn is_closed(&self) -> bool { self.status == DocStatus::Closed }
    pub fn clear_modified(&mut self) { self.is_modified = false; }
}

// occt: TDocStd_Application — manager of documents
#[derive(Clone, Debug, Default)]
pub struct StdApplication {
    pub name: String,
    pub documents: Vec<StdDocument>,
    next_id: u64,
}

impl StdApplication {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), documents: Vec::new(), next_id: 1 }
    }

    pub fn new_document(&mut self, format: DocFormat) -> u64 {
        let id = self.next_id; self.next_id += 1;
        self.documents.push(StdDocument::new(id, format.name.clone(), format));
        id
    }

    pub fn close(&mut self, id: u64) {
        if let Some(d) = self.documents.iter_mut().find(|d| d.id == id) { d.close(); }
    }

    pub fn find(&self, id: u64) -> Option<&StdDocument> { self.documents.iter().find(|d| d.id == id) }
    pub fn find_mut(&mut self, id: u64) -> Option<&mut StdDocument> { self.documents.iter_mut().find(|d| d.id == id) }
    pub fn nb_documents(&self) -> usize { self.documents.len() }
    pub fn nb_open(&self) -> usize { self.documents.iter().filter(|d| !d.is_closed()).count() }
    pub fn nb_modified(&self) -> usize { self.documents.iter().filter(|d| d.is_modified).count() }

    pub fn formats(&self) -> Vec<&str> {
        // return unique format names
        let mut seen = Vec::new();
        for d in &self.documents {
            if !seen.contains(&d.format.name.as_str()) { seen.push(&d.format.name); }
        }
        seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc_transactions() {
        let fmt = DocFormat::new("XCAF", "xbf");
        let mut doc = StdDocument::new(1, "test", fmt);
        doc.open_transaction("Add shape");
        assert_eq!(doc.nb_undos(), 1);
        doc.open_transaction("Color shape");
        assert_eq!(doc.nb_undos(), 2);
        assert!(doc.undo());
        assert_eq!(doc.nb_redos(), 1);
        assert!(doc.redo());
        assert_eq!(doc.nb_undos(), 2);
    }

    #[test]
    fn doc_abort() {
        let fmt = DocFormat::new("XCAF", "xbf");
        let mut doc = StdDocument::new(1, "test", fmt);
        doc.open_transaction("tmp");
        doc.abort_transaction();
        assert_eq!(doc.nb_undos(), 0);
    }

    #[test]
    fn application_documents() {
        let mut app = StdApplication::new("MyApp");
        let id1 = app.new_document(DocFormat::new("XDE", "xde"));
        let id2 = app.new_document(DocFormat::new("XDE", "xde"));
        assert_eq!(app.nb_documents(), 2);
        assert_ne!(id1, id2);
        app.close(id1);
        assert_eq!(app.nb_open(), 1);
    }

    #[test]
    fn application_modified_count() {
        let mut app = StdApplication::new("MyApp");
        let id = app.new_document(DocFormat::new("XDE", "xde"));
        let doc = app.find_mut(id).unwrap();
        doc.open_transaction("edit");
        assert_eq!(app.nb_modified(), 1);
    }
}
