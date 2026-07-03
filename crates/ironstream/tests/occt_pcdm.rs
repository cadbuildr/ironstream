extern crate ironstream;
use ironstream::pcdm::*;

#[test]
fn test_pcdm_error_variants() {
    assert!(PcdmError::Ok.is_ok());
    assert!(!PcdmError::WriteError.is_ok());
    assert!(!PcdmError::NoDriver.message().is_empty());
}

#[test]
fn test_pcdm_document_modify_and_store() {
    let mut doc = PcdmDocument::new("MyDoc", "XmlOcaf");
    assert!(!doc.is_modified);
    doc.set_attribute("Created", "2026-01-01");
    assert!(doc.is_modified);
    doc.set_stored("/data/model.xml");
    assert!(!doc.is_modified);
    assert!(doc.is_stored);
    assert_eq!(doc.version, 2);
}

#[test]
fn test_retrieval_driver_xml_accepts_xml_format() {
    let mut drv = RetrievalDriver::xml();
    assert!(drv.can_read("XmlOcaf"));
    assert!(drv.can_read("xml"));
    assert!(!drv.can_read("BinOcaf"));
    let result = drv.read("model.xml", "XmlOcaf");
    assert!(result.is_ok());
    assert_eq!(drv.nb_documents_read, 1);
}

#[test]
fn test_retrieval_driver_rejects_wrong_format() {
    let mut drv = RetrievalDriver::xml();
    let err = drv.read("model.cbf", "BinOcaf");
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), PcdmError::UnknownFileDriver);
}

#[test]
fn test_storage_driver_write_increments_counter() {
    let mut drv = StorageDriver::xml();
    let mut doc = PcdmDocument::new("D", "XmlOcaf");
    let err = drv.write(&mut doc, "/out/file.xml");
    assert!(err.is_ok());
    assert_eq!(drv.nb_documents_written, 1);
}

#[test]
fn test_read_writer_xml_round_trip() {
    let mut rw = ReadWriter::xml();
    let mut doc = PcdmDocument::new("RW", "XmlOcaf");
    let w_err = rw.write(&mut doc, "/tmp/rw.xml");
    assert!(w_err.is_ok());
    assert!(doc.is_stored);
}
