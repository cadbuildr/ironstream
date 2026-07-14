extern crate ironstream;
use ironstream::brep_gmap::*;

#[test]
fn test_brep_gmap_basic() {
    let mut img = ShapeImage::new();
    assert!(!img.has_image("face_1"));

    img.bind("face_1", vec!["face_1a".to_string(), "face_1b".to_string()]);
    assert!(img.has_image("face_1"));
    assert_eq!(img.image("face_1").unwrap().len(), 2);
    assert_eq!(img.roots(), &["face_1".to_string()]);
}

#[test]
fn test_shape_map_basic() {
    let mut sm = ShapeMap::new();
    sm.bind("s1", "s2");
    assert!(sm.is_bound("s1"));
    assert_eq!(sm.find("s1"), Some("s2"));

    sm.unbind("s1");
    assert!(!sm.is_bound("s1"));
}

#[test]
fn test_build_shape_map() {
    let shapes: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let sm = build_shape_map(&shapes);
    assert_eq!(sm.find("a"), Some("a"));
    assert_eq!(sm.find("b"), Some("b"));
}
