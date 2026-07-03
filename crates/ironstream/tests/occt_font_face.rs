use ironstream::font_face::*;

#[test]
fn font_rect_dimensions() {
    let r = FontRect::new(10.0, 20.0, 110.0, 70.0);
    assert!((r.width() - 100.0).abs() < 1e-6);
    assert!((r.height() - 50.0).abs() < 1e-6);
    assert!(!r.is_empty());
}

#[test]
fn font_rect_empty() {
    let r = FontRect::default();
    assert!(r.is_empty());
}

#[test]
fn formatter_bounding_box_and_nb_lines() {
    let mut f = FontTextFormatter::new();
    f.set_text("Hello\nWorld");
    f.set_font_size(12.0);
    let bb = f.bounding_box();
    assert!(bb.width() > 0.0, "bounding box width should be positive");
    assert!(bb.height() > 0.0, "bounding box height should be positive");
    assert_eq!(f.nb_lines(), 2);
}

#[test]
fn formatter_line_at() {
    let mut f = FontTextFormatter::new();
    f.set_text("line0\nline1\nline2");
    assert_eq!(f.line_at(0), Some("line0"));
    assert_eq!(f.line_at(2), Some("line2"));
    assert!(f.line_at(5).is_none());
}

#[test]
fn brep_font_load_and_render() {
    let mut bf = FontBRepFont::new();
    assert!(!bf.is_loaded());
    assert!(bf.load("Courier", 14.0));
    assert!(bf.is_loaded());
    assert!((bf.font_size() - 14.0).abs() < 1e-6);
    let id = bf.render_glyph('A');
    assert!(id > 0, "rendered glyph id should be nonzero");
    assert!(bf.glyph_advance('A') > 0.0);
}

#[test]
fn brep_font_empty_name_fails() {
    let mut bf = FontBRepFont::new();
    assert!(!bf.load("", 12.0), "empty font name should fail");
    assert!(!bf.is_loaded());
    assert_eq!(bf.glyph_advance('A'), 0.0, "unloaded font should return 0 advance");
}
