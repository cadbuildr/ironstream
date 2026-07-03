extern crate ironstream;
use ironstream::xcaf_view::*;

#[test]
fn test_xcaf_view_basic() {
    // XcafView
    let mut v = XcafView::new("front");
    assert_eq!(v.name, "front");
    assert!(!v.is_perspective);
    v.set_camera([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]);
    assert_eq!(v.camera_position, [1.0, 2.0, 3.0]);
    v.set_perspective(true);
    assert!(v.is_perspective);
    let cloned = v.clone_view();
    assert_eq!(cloned.name, "front");

    // XcafViewTool
    let mut tool = XcafViewTool::new();
    assert_eq!(tool.view_count(), 0);
    tool.add_view(XcafView::new("top"));
    tool.add_view(XcafView::new("side"));
    assert_eq!(tool.view_count(), 2);
    let found = tool.find_view("top");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "top");
    assert!(tool.find_view("missing").is_none());

    // ProjectionType
    assert_eq!(ProjectionType::Parallel, ProjectionType::Parallel);
    assert_ne!(ProjectionType::Parallel, ProjectionType::Perspective);
}
