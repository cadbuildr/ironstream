use ironstream::bin_ocaf::*;

#[test]
fn bin_ocaf_version_as_u32() {
    let v = BinOcafVersion::new(2, 3);
    assert_eq!(v.as_u32(), (2u32 << 16) | 3u32);

    let v2 = BinOcafVersion::new(0, 0);
    assert_eq!(v2.as_u32(), 0);

    let v3 = BinOcafVersion::new(1, 0);
    assert_eq!(v3.as_u32(), 65536);
}

#[test]
fn bin_mdf_relocation_table_bind_find_label() {
    let mut t = BinMdfRelocationTable::new();
    t.bind_label(1, 100);
    t.bind_label(2, 200);
    // duplicate binding should be ignored
    t.bind_label(1, 999);

    assert_eq!(t.find_label(1), Some(100));
    assert_eq!(t.find_label(2), Some(200));
    assert!(t.find_label(3).is_none());
}

#[test]
fn bin_mdf_relocation_table_bind_attr_and_clear() {
    let mut t = BinMdfRelocationTable::new();
    t.bind_attr(10, 100);
    t.bind_attr(20, 200);
    assert_eq!(t.find_attr(10), Some(100));

    t.clear();
    assert!(t.find_label(1).is_none());
    assert!(t.find_attr(10).is_none());
}

#[test]
fn bin_mdf_adriver_table_add_find_dedup() {
    let mut dt = BinMdfADriverTable::new();
    dt.add_driver(BinMdfAttrType::Integer, "IntDriver");
    dt.add_driver(BinMdfAttrType::Real, "RealDriver");
    // duplicate type ignored
    dt.add_driver(BinMdfAttrType::Integer, "AnotherInt");
    assert_eq!(dt.nb_drivers(), 2);
    assert_eq!(dt.find_driver(BinMdfAttrType::Real), Some("RealDriver"));
    assert!(dt.find_driver(BinMdfAttrType::Color).is_none());
}

#[test]
fn bin_ocaf_document_store_retrieve() {
    let mut doc = BinOcafDocument::new();
    assert!(!doc.is_stored);
    doc.add_record(BinMdfAttributeRecord::new(
        BinMdfAttrType::Integer, "0:1", vec![1, 0, 0, 0],
    ));
    doc.add_record(BinMdfAttributeRecord::new(
        BinMdfAttrType::Real, "0:2", vec![0, 0, 0, 0, 0, 0, 0xf0, 0x3f],
    ));
    assert_eq!(doc.nb_records(), 2);

    let buf = doc.store();
    assert!(doc.is_stored);
    assert!(buf.len() >= 8);

    let (ver, count) = BinOcafDocument::retrieve(&buf).unwrap();
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(count, 2);
}

#[test]
fn bin_ocaf_document_retrieve_short_buf() {
    assert!(BinOcafDocument::retrieve(&[]).is_none());
    assert!(BinOcafDocument::retrieve(&[0u8; 4]).is_none());
    // exactly 8 bytes should succeed
    let buf = [0u8; 8];
    let result = BinOcafDocument::retrieve(&buf);
    assert!(result.is_some());
    let (ver, count) = result.unwrap();
    assert_eq!(ver.major, 0);
    assert_eq!(count, 0);
}
