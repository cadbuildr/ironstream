extern crate ironstream;
use ironstream::prs3d_point_aspect::*;

#[test]
fn test_prs3d_point_aspect_basic() {
    // MarkerType enum
    assert_eq!(MarkerType::Point, MarkerType::Point);
    assert_ne!(MarkerType::Star, MarkerType::X);

    // PointAspect construction and accessors
    let mut pa = PointAspect::new(MarkerType::O, 0.0, 1.0, 0.0, 2.5);
    assert_eq!(pa.color, [0.0, 1.0, 0.0]);
    assert_eq!(pa.marker_type, MarkerType::O);
    assert_eq!(pa.scale, 2.5);
    assert_eq!(pa.color(), [0.0, 1.0, 0.0]);

    pa.set_color(1.0, 0.0, 0.0);
    assert_eq!(pa.color, [1.0, 0.0, 0.0]);

    pa.set_scale(3.0);
    assert_eq!(pa.scale, 3.0);

    pa.set_type(MarkerType::Square);
    assert_eq!(pa.marker_type, MarkerType::Square);

    // Default trait
    let pd: PointAspect = Default::default();
    assert_eq!(pd.marker_type, MarkerType::Point);
    assert_eq!(pd.scale, 1.0);

    // AspectMarker3d
    let am = AspectMarker3d::new();
    assert_eq!(am.color, [1.0, 1.0, 1.0]);
    assert_eq!(am.marker_type, MarkerType::Point);
    assert_eq!(am.scale, 1.0);

    let amd = AspectMarker3d::default_point();
    assert_eq!(amd.marker_type, MarkerType::Point);

    // Default trait
    let amtd: AspectMarker3d = Default::default();
    assert_eq!(amtd.scale, 1.0);
}
