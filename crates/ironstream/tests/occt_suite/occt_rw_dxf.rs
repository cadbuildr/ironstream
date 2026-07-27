extern crate ironstream;
use ironstream::rw_dxf::*;

#[test]
fn test_dxf_reader_parse_entities() {
    let dxf = "0\nSECTION\n8\nLayer1\n0\nLINE\n8\nLayer1\n0\nCIRCLE\n8\nLayer1\n0\nARC\n8\nLayer1\n";
    let mut r = DxfReader::new();
    r.parse(dxf);
    assert!(r.is_done());
    assert_eq!(r.nb_entities(), 3);
}

#[test]
fn test_dxf_entities_of_type() {
    let dxf = "0\nLINE\n0\nLINE\n0\nCIRCLE\n";
    let mut r = DxfReader::new();
    r.parse(dxf);
    assert_eq!(r.entities_of_type("LINE").len(), 2);
    assert_eq!(r.entities_of_type("CIRCLE").len(), 1);
    assert_eq!(r.entities_of_type("ARC").len(), 0);
}

#[test]
fn test_dxf_writer_line() {
    let mut w = DxfWriter::new();
    w.write_header();
    w.write_line(&DxfLine {
        start: [0.0; 3],
        end: [1.0, 0.0, 0.0],
        layer: "0".to_string(),
        color: 7,
    });
    w.finish();
    let s = w.content_str();
    assert!(s.contains("LINE"));
    assert!(s.contains("EOF"));
    assert!(w.byte_count() > 0);
}

#[test]
fn test_dxf_writer_circle() {
    let mut w = DxfWriter::new();
    w.write_header();
    w.write_circle(&DxfCircle {
        center: [1.0, 2.0, 0.0],
        radius: 5.0,
        layer: "GEOM".to_string(),
        color: 1,
    });
    w.finish();
    assert!(w.content_str().contains("CIRCLE"));
    assert!(w.content_str().contains("GEOM"));
}

#[test]
fn test_dxf_reader_nb_layers() {
    let mut r = DxfReader::new();
    r.parse("8\nMyLayer\n0\nLINE\n");
    assert!(r.nb_layers() >= 1);
}
