// Integration tests for intf_ext module
extern crate ironstream;

use ironstream::intf_ext::*;

#[test]
fn intf_status_predicates() {
    assert!(IntfStatus::Done.is_done());
    assert!(!IntfStatus::NotDone.is_done());
    assert!(!IntfStatus::Empty.is_done());
    assert!(!IntfStatus::Error.is_done());
    let s = IntfStatus::default();
    assert_eq!(s, IntfStatus::NotDone);
}

#[test]
fn section_point_accessors() {
    let pt = IntfSectionPoint::new([1.0, 2.0, 3.0], 0.5, 0.3, 1e-6);
    assert_eq!(pt.pnt(), [1.0, 2.0, 3.0]);
    assert!((pt.param_on_first() - 0.5).abs() < 1e-9);
    assert!((pt.param_on_second() - 0.3).abs() < 1e-9);
    assert!((pt.tolerance() - 1e-6).abs() < 1e-15);
}

#[test]
fn section_line_append_and_query() {
    let mut line = IntfSectionLine::new();
    assert!(line.is_empty());
    assert_eq!(line.nb_points(), 0);
    assert!(line.first().is_none());
    assert!(line.last().is_none());

    line.append(IntfSectionPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 1e-6));
    line.append(IntfSectionPoint::new([1.0, 0.0, 0.0], 1.0, 0.5, 1e-6));
    line.append(IntfSectionPoint::new([2.0, 0.0, 0.0], 2.0, 1.0, 1e-6));

    assert!(!line.is_empty());
    assert_eq!(line.nb_points(), 3);
    assert_eq!(line.get_point(0), Some([0.0, 0.0, 0.0]));
    assert_eq!(line.get_point(99), None);
    assert_eq!(line.first().map(|p| p.pnt()), Some([0.0, 0.0, 0.0]));
    assert_eq!(line.last().map(|p| p.pnt()), Some([2.0, 0.0, 0.0]));
    let v = line.value(1).unwrap();
    assert!((v.param_on_first() - 1.0).abs() < 1e-9);
}

#[test]
fn tangent_zone_param_range() {
    let mut tz = IntfTangentZone::new();
    assert!(!tz.has_points());
    assert!(tz.param_range_on_first().is_none());

    tz.append(IntfSectionPoint::new([0.0, 0.0, 0.0], 0.2, 0.1, 1e-5));
    tz.append(IntfSectionPoint::new([0.5, 0.0, 0.0], 0.7, 0.4, 1e-5));
    assert!(tz.has_points());
    assert_eq!(tz.nb_points(), 2);

    let (lo, hi) = tz.param_range_on_first().unwrap();
    assert!((lo - 0.2).abs() < 1e-9);
    assert!((hi - 0.7).abs() < 1e-9);

    let v = tz.value(0).unwrap();
    assert!((v.param_on_first() - 0.2).abs() < 1e-9);
}

#[test]
fn interference_add_and_query() {
    let mut intf = IntfInterference::new(1e-5);
    assert!(intf.is_empty());
    assert!(!intf.is_done());

    intf.add_section_point(IntfSectionPoint::new([0.0; 3], 0.0, 0.0, 1e-5));
    assert!(!intf.is_empty());
    assert_eq!(intf.nb_section_points(), 1);

    let mut line = IntfSectionLine::new();
    line.append(IntfSectionPoint::new([1.0; 3], 1.0, 1.0, 1e-5));
    intf.add_section_line(line);
    assert_eq!(intf.nb_section_lines(), 1);

    let mut tz = IntfTangentZone::new();
    tz.append(IntfSectionPoint::new([2.0; 3], 2.0, 2.0, 1e-5));
    intf.add_tangent_zone(tz);
    assert_eq!(intf.nb_tangent_zones(), 1);

    intf.mark_done();
    assert!(intf.is_done());

    // accessors
    let sp = intf.section_point(0).unwrap();
    assert_eq!(sp.pnt(), [0.0; 3]);
    let sl = intf.section_line(0).unwrap();
    assert_eq!(sl.nb_points(), 1);
    let tzone = intf.tangent_zone(0).unwrap();
    assert!(tzone.has_points());
    assert!(intf.section_point(99).is_none());
}

#[test]
fn intf_tool_overlap_tests() {
    // boxes
    assert!(IntfTool::boxes_overlap([0.0; 3], [1.0; 3], [0.5; 3], [1.5; 3], 0.0));
    assert!(!IntfTool::boxes_overlap([0.0; 3], [1.0; 3], [2.0; 3], [3.0; 3], 0.0));
    // with tolerance bringing them into contact
    assert!(IntfTool::boxes_overlap([0.0; 3], [1.0; 3], [1.1; 3], [2.0; 3], 0.2));

    // spheres
    assert!(IntfTool::spheres_overlap([0.0; 3], 1.0, [1.5, 0.0, 0.0], 1.0, 0.0));
    assert!(!IntfTool::spheres_overlap([0.0; 3], 0.4, [2.0, 0.0, 0.0], 0.4, 0.0));

    // segment-triangle stub returns None
    let result = IntfTool::segment_triangle(
        [0.0; 3], [1.0; 3],
        [0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, 0.0, 0.0],
        1e-6,
    );
    assert!(result.is_none());
}
