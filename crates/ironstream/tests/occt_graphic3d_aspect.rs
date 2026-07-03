// FILE: tests/occt_graphic3d_aspect.rs
extern crate ironstream;
use ironstream::graphic3d_aspect::*;

// --- Graphic3dShaderType ---

#[test]
fn shader_type_copy_and_eq() {
    let t = Graphic3dShaderType::Vertex;
    let t2 = t;
    assert_eq!(t, t2);
    assert_ne!(t, Graphic3dShaderType::Fragment);
}

#[test]
fn shader_type_all_variants_distinct() {
    let all = [
        Graphic3dShaderType::Vertex,
        Graphic3dShaderType::Fragment,
        Graphic3dShaderType::Geometry,
        Graphic3dShaderType::TessControl,
        Graphic3dShaderType::TessEval,
        Graphic3dShaderType::Compute,
    ];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

// --- Graphic3dShaderObject ---

#[test]
fn shader_object_new_and_accessors() {
    let src = "void main() {}";
    let obj = Graphic3dShaderObject::new(Graphic3dShaderType::Fragment, src);
    assert_eq!(obj.shader_type(), Graphic3dShaderType::Fragment);
    assert_eq!(obj.source(), src);
}

#[test]
fn shader_object_vertex_type() {
    let obj = Graphic3dShaderObject::new(Graphic3dShaderType::Vertex, "// vert");
    assert_eq!(obj.shader_type(), Graphic3dShaderType::Vertex);
    assert_eq!(obj.source(), "// vert");
}

#[test]
fn shader_object_compute_empty_source() {
    let obj = Graphic3dShaderObject::new(Graphic3dShaderType::Compute, "");
    assert_eq!(obj.shader_type(), Graphic3dShaderType::Compute);
    assert_eq!(obj.source(), "");
}

// --- Graphic3dFillMethod ---

#[test]
fn fill_method_copy_and_eq() {
    let m = Graphic3dFillMethod::Solid;
    let m2 = m;
    assert_eq!(m, m2);
    assert_ne!(m, Graphic3dFillMethod::Hollow);
}

#[test]
fn fill_method_all_variants_distinct() {
    let all = [
        Graphic3dFillMethod::Solid,
        Graphic3dFillMethod::Hollow,
        Graphic3dFillMethod::HorizontalLine,
        Graphic3dFillMethod::VerticalLine,
        Graphic3dFillMethod::DiagonalLeft45,
        Graphic3dFillMethod::DiagonalRight45,
        Graphic3dFillMethod::DiagonalCross45,
        Graphic3dFillMethod::HorizontalVerticalCross,
    ];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

// --- Graphic3dInteriorStyle ---

#[test]
fn interior_style_copy_and_eq() {
    let s = Graphic3dInteriorStyle::Filled;
    let s2 = s;
    assert_eq!(s, s2);
    assert_ne!(s, Graphic3dInteriorStyle::Hollow);
}

#[test]
fn interior_style_all_variants_distinct() {
    let all = [
        Graphic3dInteriorStyle::Empty,
        Graphic3dInteriorStyle::Hollow,
        Graphic3dInteriorStyle::Hatch,
        Graphic3dInteriorStyle::Filled,
        Graphic3dInteriorStyle::Hiddenline,
    ];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

// --- Graphic3dAspectFillArea ---

#[test]
fn fill_area_default_front_color_white() {
    let a = Graphic3dAspectFillArea::new();
    assert_eq!(a.front_color(), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn fill_area_default_edge_color_black() {
    let a = Graphic3dAspectFillArea::new();
    assert_eq!(a.edge_color(), [0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn fill_area_default_interior_style_filled() {
    let a = Graphic3dAspectFillArea::new();
    assert_eq!(a.interior_style(), Graphic3dInteriorStyle::Filled);
}

#[test]
fn fill_area_default_edge_width_one() {
    let a = Graphic3dAspectFillArea::new();
    assert!((a.edge_width() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn fill_area_default_not_back_face_culled() {
    let a = Graphic3dAspectFillArea::new();
    assert!(!a.is_back_face_culled());
}

#[test]
fn fill_area_set_front_color() {
    let mut a = Graphic3dAspectFillArea::new();
    let color = [0.2, 0.4, 0.6, 0.8];
    a.set_front_color(color);
    assert_eq!(a.front_color(), color);
}

#[test]
fn fill_area_set_edge_color() {
    let mut a = Graphic3dAspectFillArea::new();
    let color = [1.0, 0.0, 0.5, 1.0];
    a.set_edge_color(color);
    assert_eq!(a.edge_color(), color);
}

#[test]
fn fill_area_set_interior_style_hatch() {
    let mut a = Graphic3dAspectFillArea::new();
    a.set_interior_style(Graphic3dInteriorStyle::Hatch);
    assert_eq!(a.interior_style(), Graphic3dInteriorStyle::Hatch);
}

#[test]
fn fill_area_set_edge_width() {
    let mut a = Graphic3dAspectFillArea::new();
    a.set_edge_width(2.5);
    assert!((a.edge_width() - 2.5).abs() < f32::EPSILON);
}

#[test]
fn fill_area_set_back_face_culled_true() {
    let mut a = Graphic3dAspectFillArea::new();
    a.set_back_face_culled(true);
    assert!(a.is_back_face_culled());
}

#[test]
fn fill_area_set_back_face_culled_toggle() {
    let mut a = Graphic3dAspectFillArea::new();
    a.set_back_face_culled(true);
    assert!(a.is_back_face_culled());
    a.set_back_face_culled(false);
    assert!(!a.is_back_face_culled());
}

#[test]
fn fill_area_default_impl() {
    let a = Graphic3dAspectFillArea::default();
    assert_eq!(a.front_color(), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(a.interior_style(), Graphic3dInteriorStyle::Filled);
    assert!(!a.is_back_face_culled());
}

#[test]
fn fill_area_interior_style_empty() {
    let mut a = Graphic3dAspectFillArea::new();
    a.set_interior_style(Graphic3dInteriorStyle::Empty);
    assert_eq!(a.interior_style(), Graphic3dInteriorStyle::Empty);
}

#[test]
fn fill_area_interior_style_hiddenline() {
    let mut a = Graphic3dAspectFillArea::new();
    a.set_interior_style(Graphic3dInteriorStyle::Hiddenline);
    assert_eq!(a.interior_style(), Graphic3dInteriorStyle::Hiddenline);
}
