// FILE: rust/ironstream/crates/ironstream/tests/occt_graphic3d_camera.rs
//
// Ported / derived unit tests for Graphic3d_Camera (OCCT).
// Reference: OpenCASCADE Technology — Graphic3d_Camera.hxx / .cxx
// All expected values computed analytically or cross-checked against gluLookAt.

extern crate ironstream;
use ironstream::graphic3d_camera::*;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}), diff = {}",
        (a - b).abs()
    );
}

const EPS: f64 = 1e-12;
const LOOSE: f64 = 1e-9;

// ---------------------------------------------------------------------------
// Graphic3dProjectionType
// ---------------------------------------------------------------------------

#[test]
fn projection_type_variants_are_distinct() {
    assert_ne!(Graphic3dProjectionType::Orthographic, Graphic3dProjectionType::Perspective);
    assert_ne!(Graphic3dProjectionType::Stereo, Graphic3dProjectionType::MonoLeftEye);
    assert_ne!(Graphic3dProjectionType::MonoLeftEye, Graphic3dProjectionType::MonoRightEye);
}

#[test]
fn projection_type_copy_and_eq() {
    let p = Graphic3dProjectionType::Perspective;
    let q = p; // Copy
    assert_eq!(p, q);
}

// ---------------------------------------------------------------------------
// Graphic3dCamera::new — default values (occt spec)
// ---------------------------------------------------------------------------

#[test]
fn default_eye() {
    let cam = Graphic3dCamera::new();
    assert_eq!(cam.eye(), [0.0, 0.0, 1.0]);
}

#[test]
fn default_center() {
    let cam = Graphic3dCamera::new();
    assert_eq!(cam.center(), [0.0, 0.0, 0.0]);
}

#[test]
fn default_up() {
    let cam = Graphic3dCamera::new();
    assert_eq!(cam.up(), [0.0, 1.0, 0.0]);
}

#[test]
fn default_fov_y() {
    let cam = Graphic3dCamera::new();
    near(cam.fov_y(), 45.0, EPS);
}

#[test]
fn default_z_near() {
    let cam = Graphic3dCamera::new();
    near(cam.z_near(), 0.01, EPS);
}

#[test]
fn default_z_far() {
    let cam = Graphic3dCamera::new();
    near(cam.z_far(), 1000.0, EPS);
}

#[test]
fn default_projection() {
    let cam = Graphic3dCamera::new();
    assert_eq!(cam.projection(), Graphic3dProjectionType::Perspective);
}

#[test]
fn default_scale() {
    let cam = Graphic3dCamera::new();
    near(cam.scale(), 1.0, EPS);
}

// ---------------------------------------------------------------------------
// Setters / getters round-trip
// ---------------------------------------------------------------------------

#[test]
fn set_and_get_eye() {
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([1.0, 2.0, 3.0]);
    assert_eq!(cam.eye(), [1.0, 2.0, 3.0]);
}

#[test]
fn set_and_get_center() {
    let mut cam = Graphic3dCamera::new();
    cam.set_center([4.0, 5.0, 6.0]);
    assert_eq!(cam.center(), [4.0, 5.0, 6.0]);
}

#[test]
fn set_and_get_up() {
    let mut cam = Graphic3dCamera::new();
    cam.set_up([0.0, 0.0, 1.0]);
    assert_eq!(cam.up(), [0.0, 0.0, 1.0]);
}

#[test]
fn set_and_get_fov_y() {
    let mut cam = Graphic3dCamera::new();
    cam.set_fov_y(60.0);
    near(cam.fov_y(), 60.0, EPS);
}

#[test]
fn set_and_get_z_near() {
    let mut cam = Graphic3dCamera::new();
    cam.set_z_near(0.5);
    near(cam.z_near(), 0.5, EPS);
}

#[test]
fn set_and_get_z_far() {
    let mut cam = Graphic3dCamera::new();
    cam.set_z_far(500.0);
    near(cam.z_far(), 500.0, EPS);
}

#[test]
fn set_and_get_projection() {
    let mut cam = Graphic3dCamera::new();
    cam.set_projection(Graphic3dProjectionType::Orthographic);
    assert_eq!(cam.projection(), Graphic3dProjectionType::Orthographic);
}

#[test]
fn set_and_get_scale() {
    let mut cam = Graphic3dCamera::new();
    cam.set_scale(3.5);
    near(cam.scale(), 3.5, EPS);
}

// ---------------------------------------------------------------------------
// distance()
// ---------------------------------------------------------------------------

#[test]
fn distance_default() {
    // eye=[0,0,1], center=[0,0,0] => distance = 1.0
    let cam = Graphic3dCamera::new();
    near(cam.distance(), 1.0, EPS);
}

#[test]
fn distance_with_custom_eye_and_center() {
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([3.0, 4.0, 0.0]);
    cam.set_center([0.0, 0.0, 0.0]);
    near(cam.distance(), 5.0, EPS);
}

#[test]
fn distance_eye_equals_center() {
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([1.0, 1.0, 1.0]);
    cam.set_center([1.0, 1.0, 1.0]);
    near(cam.distance(), 0.0, EPS);
}

#[test]
fn distance_negative_coordinates() {
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([-1.0, 0.0, 0.0]);
    cam.set_center([1.0, 0.0, 0.0]);
    near(cam.distance(), 2.0, EPS);
}

// ---------------------------------------------------------------------------
// view_matrix() — look-at matrix
// ---------------------------------------------------------------------------

// Multiply 4x4 row-major matrix by a 4-vector (homogeneous).
fn mat4_mul_vec4(m: [[f64; 4]; 4], v: [f64; 4]) -> [f64; 4] {
    let mut out = [0.0f64; 4];
    for r in 0..4 {
        for c in 0..4 {
            out[r] += m[r][c] * v[c];
        }
    }
    out
}

#[test]
fn view_matrix_maps_center_to_negative_z() {
    // Default camera: eye=[0,0,1], center=[0,0,0], up=[0,1,0].
    // Center should map to (0, 0, -1, 1) in view space (distance = 1).
    let cam = Graphic3dCamera::new();
    let m = cam.view_matrix();
    let v = mat4_mul_vec4(m, [0.0, 0.0, 0.0, 1.0]);
    near(v[0], 0.0, LOOSE);
    near(v[1], 0.0, LOOSE);
    near(v[2], -1.0, LOOSE);
    near(v[3], 1.0, EPS);
}

#[test]
fn view_matrix_maps_eye_to_view_origin() {
    // The eye point always maps to (0, 0, 0, 1) in view space.
    let cam = Graphic3dCamera::new();
    let m = cam.view_matrix();
    let eye = cam.eye();
    let v = mat4_mul_vec4(m, [eye[0], eye[1], eye[2], 1.0]);
    near(v[0], 0.0, LOOSE);
    near(v[1], 0.0, LOOSE);
    near(v[2], 0.0, LOOSE);
    near(v[3], 1.0, EPS);
}

#[test]
fn view_matrix_last_row_is_homogeneous() {
    let cam = Graphic3dCamera::new();
    let m = cam.view_matrix();
    near(m[3][0], 0.0, EPS);
    near(m[3][1], 0.0, EPS);
    near(m[3][2], 0.0, EPS);
    near(m[3][3], 1.0, EPS);
}

#[test]
fn view_matrix_rotation_rows_are_unit_vectors() {
    // The upper-left 3x3 rows of a look-at matrix must be unit length.
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([2.0, 3.0, 5.0]);
    cam.set_center([1.0, 1.0, 1.0]);
    cam.set_up([0.0, 1.0, 0.0]);
    let m = cam.view_matrix();
    for row in 0..3 {
        let len_sq = m[row][0] * m[row][0] + m[row][1] * m[row][1] + m[row][2] * m[row][2];
        near(len_sq, 1.0, LOOSE);
    }
}

#[test]
fn view_matrix_rotation_rows_are_orthogonal() {
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([2.0, 3.0, 5.0]);
    cam.set_center([0.0, 0.0, 0.0]);
    cam.set_up([0.0, 1.0, 0.0]);
    let m = cam.view_matrix();
    let dot01 = m[0][0] * m[1][0] + m[0][1] * m[1][1] + m[0][2] * m[1][2];
    let dot02 = m[0][0] * m[2][0] + m[0][1] * m[2][1] + m[0][2] * m[2][2];
    let dot12 = m[1][0] * m[2][0] + m[1][1] * m[2][1] + m[1][2] * m[2][2];
    near(dot01, 0.0, LOOSE);
    near(dot02, 0.0, LOOSE);
    near(dot12, 0.0, LOOSE);
}

#[test]
fn view_matrix_eye_along_x_axis() {
    // eye=[1,0,0], center=[0,0,0], up=[0,1,0] — eye maps to view origin.
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([1.0, 0.0, 0.0]);
    cam.set_center([0.0, 0.0, 0.0]);
    cam.set_up([0.0, 1.0, 0.0]);
    let m = cam.view_matrix();
    near(m[3][0], 0.0, EPS);
    near(m[3][1], 0.0, EPS);
    near(m[3][2], 0.0, EPS);
    near(m[3][3], 1.0, EPS);
    let v = mat4_mul_vec4(m, [1.0, 0.0, 0.0, 1.0]);
    near(v[0], 0.0, LOOSE);
    near(v[1], 0.0, LOOSE);
    near(v[2], 0.0, LOOSE);
}

#[test]
fn view_matrix_distance_preserved_along_z() {
    // eye=[0,0,5], center=[0,0,0] — origin maps to z=-5 in view space.
    let mut cam = Graphic3dCamera::new();
    cam.set_eye([0.0, 0.0, 5.0]);
    cam.set_center([0.0, 0.0, 0.0]);
    cam.set_up([0.0, 1.0, 0.0]);
    let m = cam.view_matrix();
    let v = mat4_mul_vec4(m, [0.0, 0.0, 0.0, 1.0]);
    near(v[2], -5.0, LOOSE);
}

// ---------------------------------------------------------------------------
// ortho_projection_matrix()
// ---------------------------------------------------------------------------

#[test]
fn ortho_projection_matrix_bottom_row() {
    let cam = Graphic3dCamera::new();
    let m = cam.ortho_projection_matrix();
    near(m[3][0], 0.0, EPS);
    near(m[3][1], 0.0, EPS);
    near(m[3][2], 0.0, EPS);
    near(m[3][3], 1.0, EPS);
}

#[test]
fn ortho_projection_matrix_scale2_near1_far3() {
    // scale=2, z_near=1, z_far=3:
    //   A = 2/(f-n) = 2/2 = 1.0       => m[2][2] = 1.0
    //   B = -(f+n)/(f-n) = -4/2 = -2.0 => m[2][3] = -2.0
    //   x,y scale = 2/s = 2/2 = 1.0
    let mut cam = Graphic3dCamera::new();
    cam.set_scale(2.0);
    cam.set_z_near(1.0);
    cam.set_z_far(3.0);
    let m = cam.ortho_projection_matrix();
    near(m[2][2], 1.0, EPS);
    near(m[2][3], -2.0, EPS);
    near(m[0][0], 1.0, EPS);
    near(m[1][1], 1.0, EPS);
}

#[test]
fn ortho_projection_matrix_default_scale_xy() {
    // Default scale=1.0 => x,y scale = 2/1 = 2.0
    let cam = Graphic3dCamera::new();
    let m = cam.ortho_projection_matrix();
    near(m[0][0], 2.0, EPS);
    near(m[1][1], 2.0, EPS);
}

#[test]
fn ortho_projection_matrix_no_xy_shear() {
    let cam = Graphic3dCamera::new();
    let m = cam.ortho_projection_matrix();
    near(m[0][1], 0.0, EPS);
    near(m[0][2], 0.0, EPS);
    near(m[1][0], 0.0, EPS);
    near(m[1][2], 0.0, EPS);
    near(m[2][0], 0.0, EPS);
    near(m[2][1], 0.0, EPS);
}

#[test]
fn ortho_near_maps_to_minus_one() {
    // glOrtho convention: z_view = z_near -> NDC z = -1.
    let mut cam = Graphic3dCamera::new();
    let n = 2.0_f64;
    let f = 10.0_f64;
    cam.set_z_near(n);
    cam.set_z_far(f);
    let m = cam.ortho_projection_matrix();
    let ndc_near = m[2][2] * n + m[2][3];
    near(ndc_near, -1.0, LOOSE);
}

#[test]
fn ortho_far_maps_to_plus_one() {
    let mut cam = Graphic3dCamera::new();
    let n = 2.0_f64;
    let f = 10.0_f64;
    cam.set_z_near(n);
    cam.set_z_far(f);
    let m = cam.ortho_projection_matrix();
    let ndc_far = m[2][2] * f + m[2][3];
    near(ndc_far, 1.0, LOOSE);
}

// ---------------------------------------------------------------------------
// Default trait
// ---------------------------------------------------------------------------

#[test]
fn default_trait_matches_new() {
    let cam_new = Graphic3dCamera::new();
    let cam_def = Graphic3dCamera::default();
    assert_eq!(cam_new.eye(), cam_def.eye());
    assert_eq!(cam_new.center(), cam_def.center());
    assert_eq!(cam_new.up(), cam_def.up());
    near(cam_new.fov_y(), cam_def.fov_y(), EPS);
    near(cam_new.z_near(), cam_def.z_near(), EPS);
    near(cam_new.z_far(), cam_def.z_far(), EPS);
    assert_eq!(cam_new.projection(), cam_def.projection());
    near(cam_new.scale(), cam_def.scale(), EPS);
}

// ---------------------------------------------------------------------------
// Projection type round-trips
// ---------------------------------------------------------------------------

#[test]
fn set_all_projection_variants() {
    let variants = [
        Graphic3dProjectionType::Orthographic,
        Graphic3dProjectionType::Perspective,
        Graphic3dProjectionType::Stereo,
        Graphic3dProjectionType::MonoLeftEye,
        Graphic3dProjectionType::MonoRightEye,
    ];
    let mut cam = Graphic3dCamera::new();
    for p in variants {
        cam.set_projection(p);
        assert_eq!(cam.projection(), p);
    }
}
