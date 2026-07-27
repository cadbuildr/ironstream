use ironstream::gprop_ext::*;

#[test]
fn gprops_defaults_mass_zero() {
    let g = GPropGProps::default();
    assert!((g.mass() - 0.0).abs() < 1e-12);
    assert_eq!(g.centroid(), [0.0; 3]);
}

#[test]
fn gprops_add_mass_sums_centroid_weighted() {
    let mut g1 = GPropGProps::new(GPropSystemType::VolumeBased);
    g1.set_mass(2.0);
    g1.set_centroid([1.0, 0.0, 0.0]);
    let mut g2 = GPropGProps::new(GPropSystemType::VolumeBased);
    g2.set_mass(2.0);
    g2.set_centroid([3.0, 0.0, 0.0]);
    g1.add(&g2);
    assert!((g1.mass() - 4.0).abs() < 1e-10);
    // weighted average centroid: (2*1 + 2*3) / 4 = 2.0
    assert!((g1.centroid()[0] - 2.0).abs() < 1e-10);
}

#[test]
fn gprops_static_moments() {
    let mut g = GPropGProps::default();
    g.set_mass(3.0);
    g.set_centroid([2.0, 1.0, 0.5]);
    let sm = g.static_moments();
    assert!((sm[0] - 6.0).abs() < 1e-10);
    assert!((sm[1] - 3.0).abs() < 1e-10);
    assert!((sm[2] - 1.5).abs() < 1e-10);
}

#[test]
fn pgprops_centroid_weighted() {
    let mut pg = GPropPGProps::new();
    pg.add_point([0.0, 0.0, 0.0], 1.0);
    pg.add_point([2.0, 0.0, 0.0], 1.0);
    pg.add_point([1.0, 2.0, 0.0], 2.0);
    let c = pg.centroid();
    // weighted: (0*1+2*1+1*2)/4=1.0, (0*1+0*1+2*2)/4=1.0
    assert!((c[0] - 1.0).abs() < 1e-10);
    assert!((c[1] - 1.0).abs() < 1e-10);
}

#[test]
fn pgprops_classify_plane_coplanar_points() {
    let mut pg = GPropPGProps::new();
    pg.add_point([0.0, 0.0, 0.0], 1.0);
    pg.add_point([1.0, 0.0, 0.0], 1.0);
    pg.add_point([0.0, 1.0, 0.0], 1.0);
    pg.add_point([1.0, 1.0, 0.0], 1.0);
    assert_eq!(pg.classify(1e-6), GPropPEquationType::Plane);
}

#[test]
fn pgprops_classify_point_all_same() {
    let mut pg = GPropPGProps::new();
    pg.add_point([0.0, 0.0, 0.0], 1.0);
    pg.add_point([0.0, 0.0, 0.0], 1.0);
    assert_eq!(pg.classify(1e-6), GPropPEquationType::Point);
}
