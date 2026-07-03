extern crate ironstream;
use ironstream::geom_surface_extrema::*;

#[test]
fn test_geom_surface_extrema_basic() {
    let mut eps = ExtremaPtSurface::new([0.0, 0.0, 0.0], "plane_1", 1e-7);
    assert!(!eps.is_done());
    eps.perform();
    assert!(eps.is_done());
    assert!(eps.nb_ext() > 0);

    let uv = eps.point(0);
    assert!(uv.is_some());

    let dist = eps.distance(0);
    assert!(dist.is_some());

    assert!(eps.point(999).is_none());
    assert!(eps.distance(999).is_none());

    let mut ecs = ExtremaCurveSurface::new("line_1", "plane_1", 1e-7);
    assert!(!ecs.is_done());
    ecs.perform();
    assert!(ecs.is_done());
    assert!(ecs.nb_ext() > 0);

    let (uv2, d2) = nearest_point_on_surface([1.0, 2.0, 3.0], "sphere_1");
    assert!(d2 >= 0.0);
    let _ = uv2;
}
