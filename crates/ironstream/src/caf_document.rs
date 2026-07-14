// FILE: rust/ironstream/crates/ironstream/src/caf_document.rs

// occt-ref: TDocStd_Document
/// A label (node) inside a CAF document, identified by a dot-separated entry
/// path (e.g. `"0:1:2"`) and a numeric tag.
///
/// Mirrors `TDF_Label` as used within `TDocStd_Document`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CafLabel {
    pub entry: String,
    pub tag: u32,
}

impl CafLabel {
    /// Create a new label with the given entry path and tag.
    pub fn new(entry: &str, tag: u32) -> Self {
        Self {
            entry: entry.to_owned(),
            tag,
        }
    }

    /// Return `true` when the entry path is empty, indicating a null / unset
    /// label (mirrors `TDF_Label::IsNull`).
    pub fn is_null(&self) -> bool {
        self.entry.is_empty()
    }

    /// Return the depth of this label in the tree, computed from the number of
    /// colon-separated components in the entry path.
    ///
    /// A root label `"0"` has depth 1; `"0:1:2"` has depth 3; an empty entry
    /// has depth 0.
    pub fn depth(&self) -> usize {
        if self.entry.is_empty() {
            0
        } else {
            self.entry.split(':').count()
        }
    }
}

// occt-ref: TDocStd_Document
/// An OCAF-style CAF document that owns a flat list of labels.
///
/// Mirrors `TDocStd_Document`.
pub struct CafDocument {
    pub name: String,
    pub labels: Vec<CafLabel>,
    pub modified: bool,
}

impl CafDocument {
    /// Create a new, empty document with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            labels: Vec::new(),
            modified: false,
        }
    }

    /// Allocate a new label inside this document.
    ///
    /// The tag is assigned sequentially starting at 1 and the entry path is
    /// formatted as `"0:<tag>"`, which places every label as a direct child of
    /// the root node — the simplest allocation strategy that matches the
    /// `TDocStd_Document::NewLabel` contract.
    pub fn new_label(&mut self) -> CafLabel {
        let tag = self.labels.len() as u32 + 1;
        let entry = format!("0:{}", tag);
        let label = CafLabel::new(&entry, tag);
        self.labels.push(label.clone());
        self.modified = true;
        label
    }

    /// Look up a label by its entry path.
    ///
    /// Returns `None` when no label with that entry exists (mirrors
    /// `TDF_Label::Find`).
    pub fn find_label(&self, entry: &str) -> Option<&CafLabel> {
        self.labels.iter().find(|l| l.entry == entry)
    }

    /// Return `true` when the document contains no labels.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Persist the document to `path` in a minimal text format.
    ///
    /// Each label is written as one line:  `<entry>\t<tag>`.  The `modified`
    /// flag is cleared on success.
    ///
    /// Mirrors `TDocStd_Application::SaveAs`.
    pub fn save(&mut self, path: &str) -> Result<(), String> {
        use std::fmt::Write as FmtWrite;

        let mut buf = String::new();
        writeln!(buf, "# CafDocument: {}", self.name)
            .map_err(|e| e.to_string())?;
        for label in &self.labels {
            writeln!(buf, "{}\t{}", label.entry, label.tag)
                .map_err(|e| e.to_string())?;
        }

        std::fs::write(path, buf.as_bytes())
            .map_err(|e| format!("save failed for '{}': {}", path, e))?;

        self.modified = false;
        Ok(())
    }
}

// occt-ref: TDocStd_Application
/// Application-level manager that owns a collection of open CAF documents.
///
/// Mirrors `TDocStd_Application`.
pub struct CafApplication {
    pub documents: Vec<CafDocument>,
}

impl CafApplication {
    /// Create a new application with no open documents.
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    /// Create a new empty document, register it with this application, and
    /// return a mutable reference to it.
    ///
    /// Mirrors `TDocStd_Application::NewDocument`.
    pub fn new_document(&mut self, name: &str) -> &mut CafDocument {
        self.documents.push(CafDocument::new(name));
        self.documents.last_mut().unwrap()
    }

    /// Load a document previously saved by [`CafDocument::save`].
    ///
    /// Parses the minimal text format (comment lines starting with `#`,
    /// then tab-separated `<entry>\t<tag>` label lines).
    ///
    /// Mirrors `TDocStd_Application::Open`.
    pub fn open(&self, path: &str) -> Result<CafDocument, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("open failed for '{}': {}", path, e))?;

        let mut doc_name = String::new();
        let mut labels: Vec<CafLabel> = Vec::new();

        for line in raw.lines() {
            if line.starts_with('#') {
                // Header: "# CafDocument: <name>"
                if let Some(rest) = line.strip_prefix("# CafDocument: ") {
                    doc_name = rest.trim().to_owned();
                }
                continue;
            }
            let mut parts = line.splitn(2, '\t');
            let entry = match parts.next() {
                Some(e) if !e.is_empty() => e,
                _ => continue,
            };
            let tag: u32 = parts
                .next()
                .and_then(|t| t.trim().parse().ok())
                .ok_or_else(|| {
                    format!("malformed label line in '{}': {:?}", path, line)
                })?;
            labels.push(CafLabel::new(entry, tag));
        }

        if doc_name.is_empty() {
            doc_name = path.to_owned();
        }

        Ok(CafDocument {
            name: doc_name,
            labels,
            modified: false,
        })
    }

    /// Remove and drop a document from this application.
    ///
    /// Mirrors `TDocStd_Application::Close`.  If the document is not found
    /// (matched by name), the call is a no-op.
    pub fn close(&mut self, doc: CafDocument) {
        if let Some(pos) = self.documents.iter().position(|d| d.name == doc.name) {
            self.documents.remove(pos);
        }
        // `doc` is dropped here — mirrors TDocStd_Application::Close releasing
        // the handle.
    }
}

impl Default for CafApplication {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_depth_and_null() {
        let null = CafLabel::new("", 0);
        assert!(null.is_null());
        assert_eq!(null.depth(), 0);

        let root = CafLabel::new("0", 0);
        assert!(!root.is_null());
        assert_eq!(root.depth(), 1);

        let child = CafLabel::new("0:1:2", 2);
        assert_eq!(child.depth(), 3);
    }

    #[test]
    fn document_new_label_and_find() {
        let mut doc = CafDocument::new("test");
        assert!(doc.is_empty());

        let l1 = doc.new_label();
        assert_eq!(l1.tag, 1);
        assert_eq!(l1.entry, "0:1");
        assert!(!doc.is_empty());
        assert!(doc.modified);

        let l2 = doc.new_label();
        assert_eq!(l2.tag, 2);

        assert!(doc.find_label("0:1").is_some());
        assert!(doc.find_label("0:99").is_none());
    }

    #[test]
    fn application_new_document_and_close() {
        let mut app = CafApplication::new();
        {
            let doc = app.new_document("alpha");
            doc.new_label();
        }
        assert_eq!(app.documents.len(), 1);
        assert_eq!(app.documents[0].name, "alpha");

        let doc = app.documents.remove(0);
        app.close(doc);
        assert!(app.documents.is_empty());
    }

    #[test]
    fn save_and_open_round_trip() {
        let dir = std::env::temp_dir();
        let path = dir.join("caf_document_test.caf");
        let path_str = path.to_str().unwrap();

        let mut app = CafApplication::new();
        {
            let doc = app.new_document("round-trip");
            doc.new_label();
            doc.new_label();
            doc.save(path_str).unwrap();
            assert!(!doc.modified);
        }

        let loaded = app.open(path_str).unwrap();
        assert_eq!(loaded.name, "round-trip");
        assert_eq!(loaded.labels.len(), 2);
        assert_eq!(loaded.labels[0].entry, "0:1");
        assert_eq!(loaded.labels[1].tag, 2);

        let _ = std::fs::remove_file(path_str);
    }
}
