extern crate ironstream;
use ironstream::caf_document::*;

#[test]
fn test_caf_document_basic() {
    let mut doc = CafDocument::new("MyDoc");
    assert!(!doc.modified);

    let label = doc.new_label();
    assert!(doc.modified);
    assert!(!label.is_null());
    assert_eq!(label.depth(), 2); // "0:1" -> depth 2
    assert_eq!(doc.labels.len(), 1);
}

#[test]
fn test_caf_label_basic() {
    let l = CafLabel::new("0:1:2", 2);
    assert!(!l.is_null());
    assert_eq!(l.depth(), 3);
    assert_eq!(l.tag, 2);

    let null = CafLabel::new("", 0);
    assert!(null.is_null());
    assert_eq!(null.depth(), 0);
}

#[test]
fn test_caf_application_lifecycle() {
    let mut app = CafApplication::new();
    let doc = CafDocument::new("TestDoc");
    app.documents.push(doc);
    assert_eq!(app.documents.len(), 1);
}
