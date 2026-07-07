// FILE: cdm_reference.rs
// occt: CDM_Reference

//! A reference from one CDM document to another, either resolved in session
//! (to-document present) or stored (metadata only, retrieved on demand).

use std::cell::RefCell;
use std::rc::Rc;

/// Local stand-in for CDM_Document.
#[derive(Debug)]
pub struct CDM_Document {
    pub modifications: i32,
    pub is_opened: bool,
    pub is_read_only: bool,
}

impl CDM_Document {
    pub fn new(modifications: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CDM_Document {
            modifications,
            is_opened: true,
            is_read_only: false,
        }))
    }
}

/// Local stand-in for CDM_MetaData.
#[derive(Debug)]
pub struct CDM_MetaData {
    pub document_version: i32,
    pub is_read_only: bool,
}

impl CDM_MetaData {
    pub fn new(document_version: i32, is_read_only: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CDM_MetaData {
            document_version,
            is_read_only,
        }))
    }
}

pub type DocumentHandle = Rc<RefCell<CDM_Document>>;
pub type MetaDataHandle = Rc<RefCell<CDM_MetaData>>;

/// CDM_Reference port.
pub struct CDM_Reference {
    to_document: Option<DocumentHandle>,
    from_document: DocumentHandle,
    reference_identifier: i32,
    meta_data: Option<MetaDataHandle>,
    document_version: i32,
    use_storage_configuration: bool,
}

impl CDM_Reference {
    /// In-session constructor: both documents are known.
    pub fn new_in_session(
        from_document: DocumentHandle,
        to_document: DocumentHandle,
        reference_identifier: i32,
        to_document_version: i32,
    ) -> Self {
        CDM_Reference {
            to_document: Some(to_document),
            from_document,
            reference_identifier,
            meta_data: None,
            document_version: to_document_version,
            use_storage_configuration: false,
        }
    }

    /// Stored constructor: the to-document is described by metadata.
    pub fn new_stored(
        from_document: DocumentHandle,
        meta_data: MetaDataHandle,
        reference_identifier: i32,
        to_document_version: i32,
        use_storage_configuration: bool,
    ) -> Self {
        CDM_Reference {
            to_document: None,
            from_document,
            reference_identifier,
            meta_data: Some(meta_data),
            document_version: to_document_version,
            use_storage_configuration,
        }
    }

    pub fn from_document(&self) -> DocumentHandle {
        Rc::clone(&self.from_document)
    }

    pub fn reference_identifier(&self) -> i32 {
        self.reference_identifier
    }

    /// DocumentVersion recorded at reference creation.
    pub fn document_version(&self) -> i32 {
        self.document_version
    }

    fn actual_document_version(&self) -> i32 {
        match &self.to_document {
            Some(doc) => doc.borrow().modifications,
            None => self
                .meta_data
                .as_ref()
                .expect("stored reference has metadata")
                .borrow()
                .document_version,
        }
    }

    /// IsUpToDate: compares the actual document version with the version
    /// recorded when the reference was created.
    pub fn is_up_to_date(&self) -> bool {
        self.document_version == self.actual_document_version()
    }

    /// SetIsUpToDate: records the actual version (unless it is -1).
    pub fn set_is_up_to_date(&mut self) {
        let actual = self.actual_document_version();
        if actual != -1 {
            self.document_version = actual;
        }
    }

    /// UnsetToDocument: forgets the in-session document, keeping metadata.
    pub fn unset_to_document(&mut self, meta_data: MetaDataHandle) {
        self.to_document = None;
        self.meta_data = Some(meta_data);
    }

    /// IsOpened: true if the to-document is retrieved and opened.
    pub fn is_opened(&self) -> bool {
        match &self.to_document {
            None => false,
            Some(doc) => doc.borrow().is_opened,
        }
    }

    /// IsReadOnly: from the document when in session, otherwise the metadata.
    pub fn is_read_only(&self) -> bool {
        match &self.to_document {
            Some(doc) => doc.borrow().is_read_only,
            None => self
                .meta_data
                .as_ref()
                .expect("stored reference has metadata")
                .borrow()
                .is_read_only,
        }
    }

    pub fn document(&self) -> Option<DocumentHandle> {
        self.to_document.clone()
    }

    pub fn meta_data(&self) -> Option<MetaDataHandle> {
        self.meta_data.clone()
    }

    pub fn use_storage_configuration(&self) -> bool {
        self.use_storage_configuration
    }

    /// IsInSession: the to-document is present.
    pub fn is_in_session(&self) -> bool {
        self.to_document.is_some()
    }

    /// IsStored: metadata is present.
    pub fn is_stored(&self) -> bool {
        self.meta_data.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_session_reference() {
        let from = CDM_Document::new(0);
        let to = CDM_Document::new(3);
        let r = CDM_Reference::new_in_session(Rc::clone(&from), Rc::clone(&to), 1, 3);
        assert_eq!(r.reference_identifier(), 1);
        assert_eq!(r.document_version(), 3);
        assert!(r.is_in_session());
        assert!(!r.is_stored());
        assert!(r.is_opened());
        assert!(Rc::ptr_eq(&r.from_document(), &from));
        assert!(Rc::ptr_eq(&r.document().unwrap(), &to));
    }

    #[test]
    fn test_stored_reference() {
        let from = CDM_Document::new(0);
        let md = CDM_MetaData::new(5, true);
        let r = CDM_Reference::new_stored(from, Rc::clone(&md), 2, 5, true);
        assert!(!r.is_in_session());
        assert!(r.is_stored());
        assert!(!r.is_opened());
        assert!(r.is_read_only());
        assert!(r.use_storage_configuration());
        assert!(Rc::ptr_eq(&r.meta_data().unwrap(), &md));
    }

    #[test]
    fn test_is_up_to_date_in_session() {
        let from = CDM_Document::new(0);
        let to = CDM_Document::new(3);
        let mut r = CDM_Reference::new_in_session(from, Rc::clone(&to), 1, 3);
        assert!(r.is_up_to_date());
        // Modify the target document: reference becomes stale.
        to.borrow_mut().modifications = 4;
        assert!(!r.is_up_to_date());
        r.set_is_up_to_date();
        assert!(r.is_up_to_date());
        assert_eq!(r.document_version(), 4);
    }

    #[test]
    fn test_is_up_to_date_stored() {
        let from = CDM_Document::new(0);
        let md = CDM_MetaData::new(7, false);
        let r = CDM_Reference::new_stored(from, md, 1, 6, false);
        assert!(!r.is_up_to_date());
        assert!(!r.is_read_only());
    }

    #[test]
    fn test_unset_to_document() {
        let from = CDM_Document::new(0);
        let to = CDM_Document::new(2);
        let mut r = CDM_Reference::new_in_session(from, to, 1, 2);
        assert!(r.is_in_session());
        let md = CDM_MetaData::new(2, false);
        r.unset_to_document(md);
        assert!(!r.is_in_session());
        assert!(r.is_stored());
        assert!(!r.is_opened());
    }
}
