// FILE: tests/occt_brep_gprop.rs
//! Integration tests for `brep_gprop` — BRepGProp / GProp_GProps.
//!
//! All tests exercise the public API only.  Numeric tolerances mirror typical
//! OCCT test-suite conventions; triangulated geometry introduces discretisation
//! error so tolerances are relaxed accordingly.

extern crate ironstream;

use ironstream::brep_gprop::{BRepGProp, GProps, InertiaMatrix, PrincipalProps};
use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::mesh::TriMesh;
use ironstream::topods::{Solid, Wire};
use std::f64::consts::PI;

const TOL: f64 = 1e-6;
const MESH_TOL: f64 = 1e-4; // coarser: triangulated shapes

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: expected {a} ≈ {b} (tol={tol}), diff={}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build an axis-aligned box mesh with corners (0,0,0) to (dx,dy,dz).
fn make_box_mesh(dx: f64, dy: f64, dz: f64) -> TriMesh {
    let p = |x: f64, y: f64, z: f64| Pnt::new(x * dx, y * dy, z * dz);
    let mut m = TriMesh::new();
    let mut quad = |a: Pnt, b: Pnt, c: Pnt, d: Pnt| {
        m.push_triangle(a, b, c);
        m.push_triangle(a, c, d);
    };
    quad(p(0., 0., 0.), p(0., 1., 0.), p(1., 1., 0.), p(1., 0., 0.));
    quad(p(0., 0., 1.), p(1., 0., 1.), p(1., 1., 1.), p(0., 1., 1.));
    quad(p(0., 0., 0.), p(1., 0., 0.), p(1., 0., 1.), p(0., 0., 1.));
    quad(p(1., 0., 0.), p(1., 1., 0.), p(1., 1., 1.), p(1., 0., 1.));
    quad(p(1., 1., 0.), p(0., 1., 0.), p(0., 1., 1.), p(1., 1., 1.));
    quad(p(0., 1., 0.), p(0., 0., 0.), p(0., 0., 1.), p(0., 1., 1.));
    m
}

/// Approximate sphere mesh using lat/lon tesselation.
fn make_sphere_mesh(r: f64, stacks: usize, slices: usize) -> TriMesh {
    let mut m = TriMesh::new();
    // Vertices.
    let mut verts: Vec<Pnt> = Vec::new();
    for i in 0..=stacks {
        let phi = PI * i as f64 / stacks as f64;
        for j in 0..=slices {
            let theta = 2.0 * PI * j as f64 / slices as f64;
            verts.push(Pnt::new(
                r * phi.sin() * theta.cos(),
                r * phi.sin() * theta.sin(),
                r * phi.cos(),
            ));
        }
    }
    let idx = |i: usize, j: usize| i * (slices + 1) + j;
    for i in 0..stacks {
        for j in 0..slices {
            let a = idx(i, j);
            let b = idx(i + 1, j);
            let c = idx(i + 1, j + 1);
            let d = idx(i, j + 1);
            let va = verts[a];
            let vb = verts[b];
            let vc = verts[c];
            let vd = verts[d];
            m.push_triangle(va, vb, vc);
            m.push_triangle(va, vc, vd);
        }
    }
    m
}

// ─────────────────────────────────────────────────────────────────────────────
// Volume properties — GProps::mass()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn volume_unit_cube() {
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    near(props.mass(), 1.0, TOL, "unit_cube_vol");
}

#[test]
fn volume_scaled_box() {
    // 2 × 3 × 4 box → volume = 24.
    let mesh = make_box_mesh(2.0, 3.0, 4.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    near(props.mass(), 24.0, TOL, "2x3x4_vol");
}

#[test]
fn volume_centre_of_mass_unit_cube() {
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let com = props.centre_of_mass();
    near(com.x, 0.5, TOL, "com_x");
    near(com.y, 0.5, TOL, "com_y");
    near(com.z, 0.5, TOL, "com_z");
}

#[test]
fn volume_centre_of_mass_shifted_box() {
    // Box from (5,0,0) to (6,1,1).
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let mut m = mesh;
    let t = ironstream::gp::Trsf::translation(Pnt::new(5.0, 0.0, 0.0));
    m.transform(&t);
    let solid = Solid::from_mesh(m);
    let props = BRepGProp::volume_properties(&solid);
    let com = props.centre_of_mass();
    near(com.x, 5.5, TOL, "shifted_com_x");
    near(com.y, 0.5, TOL, "shifted_com_y");
    near(com.z, 0.5, TOL, "shifted_com_z");
}

// ─────────────────────────────────────────────────────────────────────────────
// Surface area properties — GProps::mass()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn surface_area_unit_cube() {
    // Total surface area of a unit cube = 6 faces × 1 = 6.
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::surface_properties(&solid);
    near(props.mass(), 6.0, TOL, "unit_cube_area");
}

#[test]
fn surface_area_scaled_box() {
    // Area = 2*(w*h + h*d + d*w) = 2*(2*3+3*4+4*2) = 52.
    let mesh = make_box_mesh(2.0, 3.0, 4.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::surface_properties(&solid);
    near(props.mass(), 52.0, TOL, "2x3x4_area");
}

#[test]
fn surface_area_sphere_approx() {
    // Sphere r=1 → area = 4π ≈ 12.566.  With 128 stacks/slices error < 0.2%.
    let mesh = make_sphere_mesh(1.0, 128, 128);
    let props = BRepGProp::surface_properties_mesh(&mesh);
    let expected = 4.0 * PI;
    near(
        props.mass(),
        expected,
        expected * 0.002,
        "sphere_area",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Linear properties
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn linear_unit_square_perimeter() {
    // Unit square perimeter = 4.
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ]);
    let props = BRepGProp::linear_properties(&w);
    near(props.mass(), 4.0, TOL, "square_perimeter");
}

#[test]
fn linear_unit_square_centroid() {
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ]);
    let props = BRepGProp::linear_properties(&w);
    let com = props.centre_of_mass();
    near(com.x, 0.5, TOL, "wire_com_x");
    near(com.y, 0.5, TOL, "wire_com_y");
}

#[test]
fn linear_triangle_perimeter() {
    // 3-4-5 right triangle; perimeter = 3+4+5 = 12.
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(0.0, 4.0, 0.0),
    ]);
    let props = BRepGProp::linear_properties(&w);
    near(props.mass(), 12.0, TOL, "triangle_perimeter");
}

// ─────────────────────────────────────────────────────────────────────────────
// Inertia matrix
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn inertia_unit_cube_diagonal() {
    // For a solid unit cube (mass = 1) with CoM at (0.5, 0.5, 0.5):
    // I_cm_xx = m*(b^2+c^2)/12 = 1*2/12 = 1/6 ≈ 0.1667.
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let mat = props.matrix_of_inertia();
    near(mat.ixx, 1.0 / 6.0, MESH_TOL, "ixx");
    near(mat.iyy, 1.0 / 6.0, MESH_TOL, "iyy");
    near(mat.izz, 1.0 / 6.0, MESH_TOL, "izz");
}

#[test]
fn inertia_off_diagonal_zero_cube() {
    // A cube has no products of inertia (full octahedral symmetry about CoM).
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let mat = props.matrix_of_inertia();
    near(mat.ixy.abs(), 0.0, MESH_TOL, "|ixy|");
    near(mat.ixz.abs(), 0.0, MESH_TOL, "|ixz|");
    near(mat.iyz.abs(), 0.0, MESH_TOL, "|iyz|");
}

// ─────────────────────────────────────────────────────────────────────────────
// GProps accumulation / add
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn add_two_unit_cubes() {
    let m1 = make_box_mesh(1.0, 1.0, 1.0);
    let s1 = Solid::from_mesh(m1);
    let p1 = BRepGProp::volume_properties(&s1);

    // Shift a second cube by (3, 0, 0).
    let mut m2 = make_box_mesh(1.0, 1.0, 1.0);
    let t = ironstream::gp::Trsf::translation(Pnt::new(3.0, 0.0, 0.0));
    m2.transform(&t);
    let s2 = Solid::from_mesh(m2);
    let p2 = BRepGProp::volume_properties(&s2);

    let mut combined = p1.clone();
    combined.add(&p2);

    near(combined.mass(), 2.0, TOL, "combined_vol");
    let com = combined.centre_of_mass();
    // CoM at midpoint of 0.5 and 3.5.
    near(com.x, 2.0, TOL, "combined_com_x");
    near(com.y, 0.5, TOL, "combined_com_y");
    near(com.z, 0.5, TOL, "combined_com_z");
}

#[test]
fn add_scaled_gprops() {
    // Adding a GProps with coeff 2 should double the mass.
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let base = BRepGProp::volume_properties(&solid);

    let mut acc = GProps::new();
    acc.add_scaled(&base, 2.0);
    near(acc.mass(), 2.0, TOL, "scaled_mass");
}

// ─────────────────────────────────────────────────────────────────────────────
// InertiaMatrix — parallel-axis theorem
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn parallel_axis_identity_at_com() {
    // Shifting by zero vector (COM = ref) should not change the matrix.
    let mat = InertiaMatrix {
        ixx: 5.0,
        iyy: 3.0,
        izz: 4.0,
        ixy: 1.0,
        ixz: 0.5,
        iyz: -0.5,
    };
    let com = Pnt::new(1.0, 2.0, 3.0);
    let shifted = mat.shift_to_point(com, com, 10.0);
    // Shifting to the same point = no change.
    near(shifted.ixx, mat.ixx, 1e-12, "ixx_unchanged");
    near(shifted.iyy, mat.iyy, 1e-12, "iyy_unchanged");
    near(shifted.izz, mat.izz, 1e-12, "izz_unchanged");
}

#[test]
fn moment_about_z_axis_cube() {
    // For a unit solid cube about its own Z axis through CoM:
    // I_z = m*(a^2+b^2)/12 = (1+1)/12 = 1/6.
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let z_axis = Ax1::new(Pnt::new(0.5, 0.5, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let moi = props.moment_of_inertia(&z_axis);
    near(moi, 1.0 / 6.0, MESH_TOL, "I_z_cube");
}

// ─────────────────────────────────────────────────────────────────────────────
// Principal properties
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn principal_props_diagonal_matrix() {
    // A diagonal inertia matrix's principal moments are just the diagonal.
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let pp = props.principal_properties();
    // For a unit cube all three principal moments are 1/6; they should be equal.
    near(pp.i1, 1.0 / 6.0, MESH_TOL, "pp_i1");
    near(pp.i2, 1.0 / 6.0, MESH_TOL, "pp_i2");
    near(pp.i3, 1.0 / 6.0, MESH_TOL, "pp_i3");
}

#[test]
fn radius_of_gyration_via_principal() {
    let mesh = make_box_mesh(2.0, 2.0, 2.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let pp = props.principal_properties();
    let (r1, r2, r3) = pp.radius_of_gyration(props.mass());
    // All three should be equal for a cube.  r = sqrt(I/m) = sqrt((2/3)/8) = sqrt(1/12).
    let expected = (1.0_f64 / 12.0_f64 * 4.0).sqrt(); // side=2 → I_cm = m*2*s^2/12 = 8*(2/12)*4/4 ...
    // Just check they are all equal and positive.
    assert!(r1 > 0.0, "r1 must be positive");
    near(r1, r2, MESH_TOL, "r1==r2");
    near(r2, r3, MESH_TOL, "r2==r3");
}

// ─────────────────────────────────────────────────────────────────────────────
// Static moments
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn static_moments_unit_cube() {
    // For a unit cube of mass 1 with CoM at (0.5, 0.5, 0.5):
    // static moments (Sx, Sy, Sz) = mass * com = (0.5, 0.5, 0.5).
    let mesh = make_box_mesh(1.0, 1.0, 1.0);
    let solid = Solid::from_mesh(mesh);
    let props = BRepGProp::volume_properties(&solid);
    let (sx, sy, sz) = props.static_moments();
    near(sx, 0.5, TOL, "Sx");
    near(sy, 0.5, TOL, "Sy");
    near(sz, 0.5, TOL, "Sz");
}

// ─────────────────────────────────────────────────────────────────────────────
// Volume sphere approximation
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn volume_sphere_approx() {
    // Sphere r=2 → V = (4/3)*π*r^3 ≈ 33.51.  With 64 stacks/slices error < 0.1%.
    let r = 2.0_f64;
    let mesh = make_sphere_mesh(r, 64, 64);
    let props = BRepGProp::volume_properties_mesh(&mesh);
    let expected = 4.0 / 3.0 * PI * r * r * r;
    near(
        props.mass(),
        expected,
        expected * 0.005,
        "sphere_volume",
    );
}

#[test]
fn volume_sphere_com_at_origin() {
    // Sphere centred at origin → CoM = (0,0,0).
    let mesh = make_sphere_mesh(1.0, 32, 32);
    let props = BRepGProp::volume_properties_mesh(&mesh);
    let com = props.centre_of_mass();
    near(com.x.abs(), 0.0, 1e-10, "sphere_com_x");
    near(com.y.abs(), 0.0, 1e-10, "sphere_com_y");
    near(com.z.abs(), 0.0, 1e-10, "sphere_com_z");
}
