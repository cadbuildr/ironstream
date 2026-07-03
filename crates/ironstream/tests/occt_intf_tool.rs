// FILE: tests/occt_intf_tool.rs
extern crate ironstream;
use ironstream::intf_tool::*;

#[test]
fn test_section_point_boundary() {
    let sp = SectionPoint::new([0.0, 0.0, 0.0], 0.0, 0.5, 1e-6);
    assert!(sp.is_on_boundary_of_g1());
    assert!(!sp.is_on_boundary_of_g2());
}

#[test]
fn test_section_line_ops() {
    let mut sl = SectionLine::new();
    assert!(sl.is_empty());
    sl.append(SectionPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 1e-6));
    sl.append(SectionPoint::new([1.0, 0.0, 0.0], 1.0, 1.0, 1e-6));
    assert_eq!(sl.nb_points(), 2);
    let (mn, mx) = sl.bounding_box().unwrap();
    assert!(mn[0].abs() < 1e-12);
    assert!((mx[0] - 1.0).abs() < 1e-12);
    sl.close();
    assert!(sl.is_closed);
}

#[test]
fn test_tangent_zone_extent() {
    let mut tz = TangentZone::new();
    tz.append(SectionPoint::new([0.0; 3], 0.1, 0.2, 1e-6));
    tz.append(SectionPoint::new([1.0; 3], 0.9, 0.8, 1e-6));
    assert_eq!(tz.nb_points(), 2);
    assert!((tz.extent_g1() - 0.8).abs() < 1e-12);
    assert!((tz.extent_g2() - 0.6).abs() < 1e-12);
}

#[test]
fn test_interference_accumulate() {
    let mut intf = Interference::new(1e-7);
    assert!(intf.is_empty());

    let mut line = SectionLine::new();
    line.append(SectionPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 1e-7));
    intf.add_section_line(line);
    assert_eq!(intf.nb_section_lines(), 1);
    assert_eq!(intf.nb_total_section_points(), 1);

    let mut tz = TangentZone::new();
    tz.append(SectionPoint::new([0.5; 3], 0.5, 0.5, 1e-7));
    intf.add_tangent_zone(tz);
    assert_eq!(intf.nb_tangent_zones(), 1);

    intf.clear();
    assert!(intf.is_empty());
}

#[test]
fn test_intf_tool_boxes_overlap() {
    let tool = IntfTool::new();
    assert!(tool.boxes_overlap([0.0; 3], [1.0; 3], [0.5; 3], [2.0; 3]));
    assert!(!tool.boxes_overlap([0.0; 3], [1.0; 3], [2.0; 3], [3.0; 3]));
}

#[test]
fn test_intf_tool_segment_segment_hit() {
    let tool = IntfTool::new();
    let result = tool.segment_segment(
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [0.5, -0.5, 0.0], [0.5, 0.5, 0.0],
        1.0,
    );
    assert!(!result.is_empty());
}
