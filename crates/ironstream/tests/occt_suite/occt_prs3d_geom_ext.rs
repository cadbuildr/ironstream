use ironstream::prs3d_geom_ext::*;

#[test]
fn sphere_all_vertices_on_unit_sphere() {
    let s = Prs3dToolSphere::new(1.0, 16, 8);
    let verts = s.vertices();
    assert_eq!(verts.len(), s.nb_vertices());
    for v in &verts {
        let r = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
        assert!((r - 1.0).abs() < 1e-10, "vertex radius={r} not on unit sphere");
    }
}

#[test]
fn sphere_nb_triangles() {
    let s = Prs3dToolSphere::new(2.0, 8, 4);
    assert_eq!(s.nb_triangles(), 2 * 8 * 4);
}

#[test]
fn cylinder_ring_vertices_z_equals_zero_and_height() {
    let c = Prs3dToolCylinder::new(1.0, 1.0, 5.0, 12);
    let bottom = c.ring_vertices(0);
    assert_eq!(bottom.len(), 13); // nb_u + 1
    for v in &bottom {
        assert!(v[2].abs() < 1e-10, "bottom ring z should be 0, got {}", v[2]);
    }
    let top = c.ring_vertices(1);
    for v in &top {
        assert!((v[2] - 5.0).abs() < 1e-10, "top ring z should be 5, got {}", v[2]);
    }
    assert_eq!(c.nb_side_triangles(), 2 * 12);
}

#[test]
fn torus_is_full_and_vertex_at_major_plus_minor() {
    let t = Prs3dToolTorus::new(3.0, 1.0, 16, 8);
    assert!(t.is_full());
    // At (iu=0, iv=0): theta=0, phi=0 => x = R+r = 4, y=0, z=0
    let v = t.vertex(0, 0);
    assert!((v[0] - 4.0).abs() < 1e-10);
    assert!(v[1].abs() < 1e-10);
    assert!(v[2].abs() < 1e-10);
}

#[test]
fn disk_width_and_nb_triangles() {
    let d = Prs3dToolDisk::new(1.0, 4.0, 8);
    assert!((d.width() - 3.0).abs() < 1e-10);
    assert_eq!(d.nb_triangles(), 16);
}

#[test]
fn mesh_resolution_nb_triangles_and_vertices() {
    let r = PrsMeshResolution::new(10, 5);
    assert_eq!(r.nb_triangles(), 2 * 10 * 5);
    assert_eq!(r.nb_vertices(), 11 * 6);
}
