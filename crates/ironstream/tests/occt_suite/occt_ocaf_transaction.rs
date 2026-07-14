// FILE: tests/occt_ocaf_transaction.rs
extern crate ironstream;
use ironstream::ocaf_transaction::*;

#[test]
fn doc_open_and_undo() {
    let fmt = DocFormat::new("XCAF", "xbf");
    let mut doc = StdDocument::new(1, "test", fmt);
    doc.open_transaction("Add shape");
    doc.open_transaction("Color shape");
    assert_eq!(doc.nb_undos(), 2);
    assert!(doc.undo());
    assert_eq!(doc.nb_redos(), 1);
}

#[test]
fn doc_redo_after_undo() {
    let fmt = DocFormat::new("XCAF", "xbf");
    let mut doc = StdDocument::new(1, "doc", fmt);
    doc.open_transaction("op1");
    assert!(doc.undo());
    assert!(doc.redo());
    assert_eq!(doc.nb_undos(), 1);
    assert_eq!(doc.nb_redos(), 0);
}

#[test]
fn doc_abort_removes_last_transaction() {
    let fmt = DocFormat::new("XDE", "xde");
    let mut doc = StdDocument::new(2, "doc2", fmt);
    doc.open_transaction("temp");
    doc.abort_transaction();
    assert_eq!(doc.nb_undos(), 0);
}

#[test]
fn doc_close_and_status() {
    let fmt = DocFormat::new("XDE", "xde");
    let mut doc = StdDocument::new(3, "doc3", fmt);
    assert!(!doc.is_closed());
    doc.close();
    assert!(doc.is_closed());
    assert_eq!(doc.status, DocStatus::Closed);
}

#[test]
fn application_new_document_unique_ids() {
    let mut app = StdApplication::new("TestApp");
    let id1 = app.new_document(DocFormat::new("XDE", "xde"));
    let id2 = app.new_document(DocFormat::new("XDE", "xde"));
    assert_ne!(id1, id2);
    assert_eq!(app.nb_documents(), 2);
    assert_eq!(app.nb_open(), 2);
}

#[test]
fn application_close_reduces_open_count() {
    let mut app = StdApplication::new("App");
    let id = app.new_document(DocFormat::new("XDE", "xde"));
    app.close(id);
    assert_eq!(app.nb_open(), 0);
}

#[test]
fn application_modified_count() {
    let mut app = StdApplication::new("App2");
    let id = app.new_document(DocFormat::new("XDE", "xde"));
    {
        let doc = app.find_mut(id).unwrap();
        doc.open_transaction("edit");
    }
    assert_eq!(app.nb_modified(), 1);
}
