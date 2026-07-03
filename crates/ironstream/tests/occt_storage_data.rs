extern crate ironstream;
use ironstream::storage_data::*;

#[test]
fn test_storage_error_ok_and_message() {
    assert!(StorageError::Ok.is_ok());
    assert!(!StorageError::ReadError.is_ok());
    assert_eq!(StorageError::Ok.message(), "Ok");
    assert!(!StorageError::WriteError.message().is_empty());
}

#[test]
fn test_storage_header_application_and_user_info() {
    let mut h = StorageHeaderData::new();
    h.set_application("IronStream", "0.1.0");
    h.add_to_user_info("license", "MIT");
    assert_eq!(h.application_name, "IronStream");
    assert_eq!(h.find_user_info("license"), Some("MIT"));
    assert_eq!(h.find_user_info("missing"), None);
}

#[test]
fn test_storage_data_roots_no_duplicates() {
    let mut d = StorageData::new();
    d.add_root("A");
    d.add_root("B");
    d.add_root("A");
    assert_eq!(d.nb_roots(), 2);
    assert!(d.is_root("B"));
    d.remove_root("B");
    assert!(!d.is_root("B"));
    assert_eq!(d.nb_roots(), 1);
}

#[test]
fn test_storage_base_driver_open_write_close() {
    let mut drv = StorageBaseDriver::new();
    let err = drv.open("/tmp/test.bin", StorageAccessMode::Write);
    assert!(err.is_ok());
    assert!(drv.is_open);
    drv.write_real(1.23);
    drv.write_integer(7);
    drv.write_string("hello");
    let err2 = drv.close();
    assert!(err2.is_ok());
    assert!(!drv.is_open);
    assert!(drv.written_bytes > 0);
}

#[test]
fn test_storage_base_driver_close_when_not_open() {
    let mut drv = StorageBaseDriver::new();
    let err = drv.close();
    assert!(!err.is_ok());
    assert_eq!(err, StorageError::CloseError);
}

#[test]
fn test_storage_schema_register_and_dedup() {
    let mut schema = StorageSchema::new("Standard", "2");
    schema.register("Geom_BSplineSurface");
    schema.register("gp_Pnt");
    schema.register("Geom_BSplineSurface"); // duplicate
    assert_eq!(schema.nb_types(), 2);
    assert!(schema.is_registered("gp_Pnt"));
    assert!(!schema.is_registered("TopoDS_Shape"));
}
