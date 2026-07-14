use ironstream::tdoc_std::*;

#[test]
fn doc_format_null_check() {
    let f = DocFormat::new("");
    assert!(f.is_null());
    let f2 = DocFormat::new("BinXCAF");
    assert!(!f2.is_null());
    assert_eq!(f2.as_str(), "BinXCAF");
}

#[test]
fn doc_path_full_path_with_and_without_extension() {
    let p = DocPath::new("/tmp", "model", "brep");
    assert_eq!(p.full_path(), "/tmp/model.brep");
    let p2 = DocPath::new("/data", "archive", "");
    assert_eq!(p2.full_path(), "/data/archive");
}

#[test]
fn tdoc_std_document_undo_limit() {
    let mut doc = TDocStdDocument::new(1, DocFormat::new("XmlXCAF"));
    doc.set_undo_limit(20);
    assert_eq!(doc.undo_limit(), 20);
    assert!(!doc.can_undo());
    assert!(!doc.can_redo());
}

#[test]
fn tdoc_std_document_is_saved_lifecycle() {
    let mut doc = TDocStdDocument::new(2, DocFormat::new("BinXCAF"));
    assert!(!doc.is_saved()); // not stored yet
    doc.mark_saved();
    assert!(doc.is_saved());
    doc.new_command(); // marks modified
    assert!(!doc.is_saved());
}

#[test]
fn tdoc_std_application_open_returns_ok() {
    let mut app = TDocStdApplication::new();
    let (id, status) = app.open("/path/to/model.xml", "XmlXCAF");
    assert!(status.is_ok());
    let doc = app.find_document(id).unwrap();
    assert!(doc.is_stored);
    assert_eq!(doc.format.as_str(), "XmlXCAF");
}

#[test]
fn tdoc_std_modified_no_duplicates() {
    let mut m = TDocStdModified::new();
    m.add(10);
    m.add(10);
    m.add(20);
    assert_eq!(m.nb_modified(), 2);
    assert!(m.contains(10));
    assert!(m.contains(20));
    m.remove(10);
    assert!(!m.contains(10));
    assert_eq!(m.nb_modified(), 1);
}
