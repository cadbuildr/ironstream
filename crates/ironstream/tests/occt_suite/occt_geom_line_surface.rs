extern crate ironstream;
use ironstream::geom_line_surface::*;

#[test]
fn test_geom_line_surface_basic() {
    // GeomLine
    let l = GeomLine::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let p = l.point_at(3.0);
    assert!((p[0] - 3.0).abs() < 1e-12);
    assert!((p[1]).abs() < 1e-12);

    // direction is normalized
    let l2 = GeomLine::new([0.0; 3], [3.0, 0.0, 0.0]);
    let d = l2.direction();
    assert!((d[0] - 1.0).abs() < 1e-12);

    // reverse
    let mut l3 = GeomLine::new([0.0; 3], [1.0, 0.0, 0.0]);
    l3.reverse();
    assert!((l3.direction()[0] + 1.0).abs() < 1e-12);

    // GeomPlane
    let plane = GeomPlane::new([0.0; 3], [0.0, 0.0, 1.0]);
    let pt = plane.evaluate(3.0, 4.0);
    // Any evaluated point must lie on the plane (zero signed distance)
    assert!((plane.distance(pt)).abs() < 1e-12);
    // project_point: project a point above the plane, foot has z=0
    let test_pt = [3.0, 4.0, 5.0];
    let [u, v] = plane.project_point(test_pt);
    // Foot in plane: re-evaluate and verify z=0
    let foot = plane.evaluate(u, v);
    assert!((foot[2]).abs() < 1e-12, "foot.z = {}", foot[2]);
    // distance from above point is 5.0
    assert!((plane.distance(test_pt) - 5.0).abs() < 1e-12);

    // line_plane_intersect
    let line = GeomLine::new([0.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    let ipt = line_plane_intersect(&line, &plane).expect("intersect");
    assert!((ipt[2]).abs() < 1e-12);

    // line_line_distance
    let la = GeomLine::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let lb = GeomLine::new([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]);
    assert!((line_line_distance(&la, &lb) - 1.0).abs() < 1e-12);
}
