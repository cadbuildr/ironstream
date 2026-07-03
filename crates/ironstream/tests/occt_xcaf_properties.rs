extern crate ironstream;
use ironstream::xcaf_properties::*;

#[test]
fn test_xcaf_properties_basic() {
    let mut style = PrsStyle::new();
    assert!(!style.has_color());
    style.set_color(1.0, 0.0, 0.0);
    assert!(style.has_color());
    style.set_transparency(0.5);
    assert!((style.transparency - 0.5).abs() < 1e-12);

    let prop = XcafProperty::new("author", "Alice");
    assert_eq!(prop.name, "author");
    assert_eq!(prop.value, "Alice");

    let mut tool = PropertyTool::new();
    tool.add(XcafProperty::new("title", "Part A"));
    assert_eq!(tool.count(), 1);
    assert!(tool.find("title").is_some());
    assert!(tool.find("missing").is_none());

    let ds = default_style();
    assert!(ds.has_color()); // default_style sets white (1,1,1)
}
