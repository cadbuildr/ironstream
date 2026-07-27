extern crate ironstream;
use ironstream::caf_storage::*;

#[test]
fn cdf_session_current_document_after_opening_multiple() {
    let mut session = CdfSession::new();
    let _id1 = session.open_document();
    let _id2 = session.open_document();
    let id3 = session.open_document();
    // current_document should point to the last opened document
    let current = session.current_document().expect("should have current doc");
    assert_eq!(current.doc_id, id3);
    assert_eq!(session.nb_documents(), 3);
}

#[test]
fn cdf_store_has_been_stored_multiple_times() {
    let mut store = CdfStore::new(10);
    let meta = CdfMetaData::new("/repo", "mypart");
    store.set_meta(meta);
    assert!(!store.has_been_stored());
    store.store();
    assert!(store.has_been_stored());
    assert_eq!(store.nb_stored, 1);
    store.store();
    assert_eq!(store.nb_stored, 2);
    store.store();
    assert_eq!(store.nb_stored, 3);
}

#[test]
fn cdf_directory_nb_stored_documents() {
    let mut dir = CdfDirectory::new();
    let i1 = dir.new_meta("/r", "doc1");
    let i2 = dir.new_meta("/r", "doc2");
    let _i3 = dir.new_meta("/r", "doc3");
    assert_eq!(dir.nb_stored_documents(), 0);
    dir.entries[i1].is_stored = true;
    assert_eq!(dir.nb_stored_documents(), 1);
    dir.entries[i2].is_stored = true;
    assert_eq!(dir.nb_stored_documents(), 2);
    assert_eq!(dir.nb_documents(), 3);
}

#[test]
fn cdf_meta_data_null_check_empty_name() {
    let meta_empty = CdfMetaData::new("/folder", "");
    assert!(meta_empty.is_null());
    let meta_named = CdfMetaData::new("/folder", "part");
    assert!(!meta_named.is_null());
}

#[test]
fn cdf_store_mode_retrieval_data_variant_is_not_no_current_data() {
    let v = CdfStoreMode::RetrievalData;
    assert_ne!(v, CdfStoreMode::NoCurrentData);
    assert_ne!(CdfStoreMode::StorageData, CdfStoreMode::NoCurrentData);
    assert_ne!(CdfStoreMode::StorageAndRetrievalData, CdfStoreMode::NoCurrentData);
    assert_eq!(CdfStoreMode::default(), CdfStoreMode::NoCurrentData);
}

#[test]
fn cdf_directory_find_returns_correct_metadata() {
    let mut dir = CdfDirectory::new();
    dir.new_meta("/project", "assembly");
    dir.new_meta("/project", "part_a");
    dir.new_meta("/other", "part_b");
    let found = dir.find("/project", "assembly").expect("should find assembly");
    assert_eq!(found.folder(), "/project");
    assert_eq!(found.name(), "assembly");
    let found2 = dir.find("/other", "part_b").expect("should find part_b");
    assert_eq!(found2.name(), "part_b");
    assert!(dir.find("/project", "nonexistent").is_none());
}
