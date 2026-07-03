extern crate ironstream;
use ironstream::geom_intersect_ext::*;

#[test]
fn test_geom_intersect_ext_basic() {
    let mut si = ShapeIntersector::new("TestShape", 1e-6);
    assert_eq!(si.nb_points(), 0);
    assert!(!si.is_done());
    si.perform([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(si.is_done());
    assert_eq!(si.nb_points(), 1);
    assert_eq!(si.point(0), Some([0.0, 0.0, 0.0]));

    let mut pi = PatchIntersection::new(1e-6);
    pi.perform("Surface1", "Surface2");
    assert!(pi.is_done());
    assert_eq!(pi.nb_points(), 1);

    let hits = ray_shape_intersect([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], "Solid");
    assert_eq!(hits.len(), 1);
}
