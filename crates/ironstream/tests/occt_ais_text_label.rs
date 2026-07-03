use ironstream::ais_text_label::*;

#[test]
fn ais_text_label_display_erase() {
    let mut lbl = AisTextLabel::new(1, "Hello", [0.0, 0.0, 0.0]);
    assert!(!lbl.is_displayed);
    lbl.display();
    assert!(lbl.is_displayed);
    lbl.erase();
    assert!(!lbl.is_displayed);
}

#[test]
fn ais_text_label_text_and_position() {
    let mut lbl = AisTextLabel::new(2, "World", [1.0, 2.0, 3.0]);
    assert_eq!(lbl.text, "World");
    assert_eq!(lbl.position, [1.0, 2.0, 3.0]);
    lbl.set_text("Updated");
    assert_eq!(lbl.text, "Updated");
    lbl.set_position([4.0, 5.0, 6.0]);
    assert_eq!(lbl.position, [4.0, 5.0, 6.0]);
}

#[test]
fn ais_text_attributes_color_sets_has_own_color() {
    let mut attr = AisTextAttributes::new();
    assert!(!attr.has_own_color);
    attr.set_color(1.0, 0.0, 0.0);
    assert!(attr.has_own_color);
    assert!((attr.color[0] - 1.0).abs() < 1e-6);
}

#[test]
fn font_slant_is_bold_is_italic() {
    assert!(FontSlant::Bold.is_bold());
    assert!(!FontSlant::Bold.is_italic());
    assert!(FontSlant::Italic.is_italic());
    assert!(!FontSlant::Italic.is_bold());
    assert!(FontSlant::BoldItalic.is_bold());
    assert!(FontSlant::BoldItalic.is_italic());
    assert!(!FontSlant::Regular.is_bold());
}

#[test]
fn ais_text_label_pool_add_find_remove() {
    let mut pool = AisTextLabelPool::new();
    pool.add(AisTextLabel::new(1, "A", [0.0; 3]));
    pool.add(AisTextLabel::new(2, "B", [1.0, 0.0, 0.0]));
    assert_eq!(pool.nb_labels(), 2);
    pool.find_mut(1).unwrap().display();
    assert_eq!(pool.displayed().len(), 1);
    pool.remove(2);
    assert_eq!(pool.nb_labels(), 1);
    assert!(pool.find_mut(2).is_none());
}

#[test]
fn font_text_formatter_wrap_and_newline() {
    let fmt = FontTextFormatter::new(50.0);
    // word-wrap: 50/10 = 5 chars per line, "ABCDEFGHIJ" -> 2 lines
    let lines = fmt.format("ABCDEFGHIJ", 10.0);
    assert_eq!(lines.len(), 2);

    // newline splitting
    let fmt2 = FontTextFormatter::new(1000.0);
    let lines2 = fmt2.format("line1\nline2\nline3", 10.0);
    assert_eq!(lines2.len(), 3);
}
