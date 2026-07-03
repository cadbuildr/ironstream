extern crate ironstream;
use ironstream::geom_surface_props::*;

#[test]
fn test_geom_surface_props_basic() {
    let g = GProps::new();
    assert_eq!(g.mass(), 0.0);
    assert_eq!(g.centre_of_mass(), [0.0; 3]);

    let mut a = GProps { mass: 1.0, centre_of_mass: [0.0, 0.0, 0.0], inertia: [[0.0; 3]; 3] };
    let b = GProps { mass: 1.0, centre_of_mass: [2.0, 0.0, 0.0], inertia: [[0.0; 3]; 3] };
    a.add(&b, 1.0);
    assert_eq!(a.mass(), 2.0);

    let vp = VolumeProps::from_shape("box");
    assert!(!vp.is_valid());

    let sp = SurfaceProps::from_shape("sphere");
    assert_eq!(sp.area, 0.0);

    let lp = linear_props("wire");
    assert_eq!(lp.mass(), 0.0);
    let sfp = surface_props("face");
    assert_eq!(sfp.mass(), 0.0);
    let volp = volume_props("solid");
    assert_eq!(volp.mass(), 0.0);
}
