// FILE: tests/occt_rw_gltf.rs
//! Integration tests for `ironstream::rw_gltf` — the glTF I/O framework
//! ported from OpenCascade's `RWGltf` package.
//!
//! Tests exercise the public API from an external-crate perspective, verifying
//! that enums, mesh primitives, and the writer behave as documented.

extern crate ironstream;

use ironstream::rw_gltf::{
    GltfAccessorLayout, GltfMeshPrimitive, GltfPrimitiveMode, RwGltfWriter,
};

// ---------------------------------------------------------------------------
// GltfAccessorLayout
// ---------------------------------------------------------------------------

#[test]
fn accessor_layout_scalar_str_and_components() {
    assert_eq!(GltfAccessorLayout::Scalar.as_str(), "SCALAR");
    assert_eq!(GltfAccessorLayout::Scalar.nb_components(), 1);
}

#[test]
fn accessor_layout_vec_variants() {
    assert_eq!(GltfAccessorLayout::Vec2.as_str(), "VEC2");
    assert_eq!(GltfAccessorLayout::Vec2.nb_components(), 2);

    assert_eq!(GltfAccessorLayout::Vec3.as_str(), "VEC3");
    assert_eq!(GltfAccessorLayout::Vec3.nb_components(), 3);

    assert_eq!(GltfAccessorLayout::Vec4.as_str(), "VEC4");
    assert_eq!(GltfAccessorLayout::Vec4.nb_components(), 4);
}

#[test]
fn accessor_layout_mat_variants() {
    assert_eq!(GltfAccessorLayout::Mat2.as_str(), "MAT2");
    assert_eq!(GltfAccessorLayout::Mat2.nb_components(), 4);

    assert_eq!(GltfAccessorLayout::Mat3.as_str(), "MAT3");
    assert_eq!(GltfAccessorLayout::Mat3.nb_components(), 9);

    assert_eq!(GltfAccessorLayout::Mat4.as_str(), "MAT4");
    assert_eq!(GltfAccessorLayout::Mat4.nb_components(), 16);
}

#[test]
fn accessor_layout_display_matches_as_str() {
    let layouts = [
        GltfAccessorLayout::Scalar,
        GltfAccessorLayout::Vec2,
        GltfAccessorLayout::Vec3,
        GltfAccessorLayout::Vec4,
        GltfAccessorLayout::Mat2,
        GltfAccessorLayout::Mat3,
        GltfAccessorLayout::Mat4,
    ];
    for layout in layouts {
        assert_eq!(layout.to_string(), layout.as_str());
    }
}

#[test]
fn accessor_layout_debug_is_available() {
    let _ = format!("{:?}", GltfAccessorLayout::Vec3);
}

#[test]
fn accessor_layout_clone_eq() {
    let a = GltfAccessorLayout::Mat4;
    let b = a;
    assert_eq!(a, b);
    let c = a.clone();
    assert_eq!(a, c);
}

// ---------------------------------------------------------------------------
// GltfPrimitiveMode
// ---------------------------------------------------------------------------

#[test]
fn primitive_mode_numeric_values() {
    assert_eq!(GltfPrimitiveMode::Points.as_u8(),        0);
    assert_eq!(GltfPrimitiveMode::Lines.as_u8(),         1);
    assert_eq!(GltfPrimitiveMode::LineLoop.as_u8(),      2);
    assert_eq!(GltfPrimitiveMode::LineStrip.as_u8(),     3);
    assert_eq!(GltfPrimitiveMode::Triangles.as_u8(),     4);
    assert_eq!(GltfPrimitiveMode::TriangleStrip.as_u8(), 5);
    assert_eq!(GltfPrimitiveMode::TriangleFan.as_u8(),   6);
}

#[test]
fn primitive_mode_from_u8_all_valid() {
    for v in 0u8..=6 {
        let mode = GltfPrimitiveMode::from_u8(v);
        assert!(mode.is_some(), "from_u8({v}) should be Some");
        assert_eq!(mode.unwrap().as_u8(), v);
    }
}

#[test]
fn primitive_mode_from_u8_invalid_returns_none() {
    assert!(GltfPrimitiveMode::from_u8(7).is_none());
    assert!(GltfPrimitiveMode::from_u8(100).is_none());
    assert!(GltfPrimitiveMode::from_u8(255).is_none());
}

#[test]
fn primitive_mode_default_triangles() {
    assert_eq!(GltfPrimitiveMode::default(), GltfPrimitiveMode::Triangles);
}

#[test]
fn primitive_mode_display() {
    assert_eq!(GltfPrimitiveMode::Points.to_string(),        "POINTS");
    assert_eq!(GltfPrimitiveMode::Lines.to_string(),         "LINES");
    assert_eq!(GltfPrimitiveMode::LineLoop.to_string(),      "LINE_LOOP");
    assert_eq!(GltfPrimitiveMode::LineStrip.to_string(),     "LINE_STRIP");
    assert_eq!(GltfPrimitiveMode::Triangles.to_string(),     "TRIANGLES");
    assert_eq!(GltfPrimitiveMode::TriangleStrip.to_string(), "TRIANGLE_STRIP");
    assert_eq!(GltfPrimitiveMode::TriangleFan.to_string(),   "TRIANGLE_FAN");
}

#[test]
fn primitive_mode_debug_clone_eq() {
    let a = GltfPrimitiveMode::TriangleFan;
    let b = a;
    assert_eq!(a, b);
    let c = a.clone();
    assert_eq!(a, c);
    let _ = format!("{:?}", a);
}

// ---------------------------------------------------------------------------
// GltfMeshPrimitive
// ---------------------------------------------------------------------------

#[test]
fn mesh_primitive_new_name_and_mode() {
    let prim = GltfMeshPrimitive::new("body", GltfPrimitiveMode::Triangles);
    assert_eq!(prim.name(), "body");
    assert_eq!(prim.mode(), GltfPrimitiveMode::Triangles);
}

#[test]
fn mesh_primitive_new_accessors_start_unset() {
    let prim = GltfMeshPrimitive::new("p", GltfPrimitiveMode::Triangles);
    assert_eq!(prim.index_accessor(),    GltfMeshPrimitive::UNSET);
    assert_eq!(prim.position_accessor(), GltfMeshPrimitive::UNSET);
    assert_eq!(prim.normal_accessor(),   GltfMeshPrimitive::UNSET);
    assert!(!prim.has_index_accessor());
    assert!(!prim.has_position_accessor());
    assert!(!prim.has_normal_accessor());
}

#[test]
fn mesh_primitive_set_index_accessor() {
    let mut prim = GltfMeshPrimitive::new("p", GltfPrimitiveMode::Triangles);
    prim.set_index_accessor(3);
    assert_eq!(prim.index_accessor(), 3);
    assert!(prim.has_index_accessor());
}

#[test]
fn mesh_primitive_set_position_accessor() {
    let mut prim = GltfMeshPrimitive::new("p", GltfPrimitiveMode::Triangles);
    prim.set_position_accessor(7);
    assert_eq!(prim.position_accessor(), 7);
    assert!(prim.has_position_accessor());
}

#[test]
fn mesh_primitive_set_normal_accessor() {
    let mut prim = GltfMeshPrimitive::new("p", GltfPrimitiveMode::Triangles);
    prim.set_normal_accessor(9);
    assert_eq!(prim.normal_accessor(), 9);
    assert!(prim.has_normal_accessor());
}

#[test]
fn mesh_primitive_all_accessors_set() {
    let mut prim = GltfMeshPrimitive::new("full", GltfPrimitiveMode::Triangles);
    prim.set_index_accessor(0);
    prim.set_position_accessor(1);
    prim.set_normal_accessor(2);
    assert_eq!(prim.index_accessor(),    0);
    assert_eq!(prim.position_accessor(), 1);
    assert_eq!(prim.normal_accessor(),   2);
    assert!(prim.has_index_accessor());
    assert!(prim.has_position_accessor());
    assert!(prim.has_normal_accessor());
}

#[test]
fn mesh_primitive_mode_points() {
    let prim = GltfMeshPrimitive::new("cloud", GltfPrimitiveMode::Points);
    assert_eq!(prim.mode(), GltfPrimitiveMode::Points);
    assert_eq!(prim.mode().as_u8(), 0);
}

#[test]
fn mesh_primitive_empty_name() {
    let prim = GltfMeshPrimitive::new("", GltfPrimitiveMode::Lines);
    assert_eq!(prim.name(), "");
    assert_eq!(prim.mode(), GltfPrimitiveMode::Lines);
}

#[test]
fn mesh_primitive_clone() {
    let mut prim = GltfMeshPrimitive::new("orig", GltfPrimitiveMode::Triangles);
    prim.set_position_accessor(5);
    let cloned = prim.clone();
    assert_eq!(cloned.name(), "orig");
    assert_eq!(cloned.position_accessor(), 5);
}

#[test]
fn mesh_primitive_debug() {
    let prim = GltfMeshPrimitive::new("d", GltfPrimitiveMode::TriangleFan);
    let s = format!("{:?}", prim);
    assert!(s.contains("TriangleFan") || s.contains("GltfMeshPrimitive"));
}

// ---------------------------------------------------------------------------
// RwGltfWriter
// ---------------------------------------------------------------------------

#[test]
fn writer_new_json_mode() {
    let w = RwGltfWriter::new(false);
    assert!(!w.is_binary());
    assert_eq!(w.nb_meshes(), 0);
}

#[test]
fn writer_new_binary_mode() {
    let w = RwGltfWriter::new(true);
    assert!(w.is_binary());
    assert_eq!(w.nb_meshes(), 0);
}

#[test]
fn writer_default_is_json() {
    let w = RwGltfWriter::default();
    assert!(!w.is_binary());
    assert_eq!(w.nb_meshes(), 0);
}

#[test]
fn writer_set_binary_toggles() {
    let mut w = RwGltfWriter::new(false);
    w.set_binary(true);
    assert!(w.is_binary());
    w.set_binary(false);
    assert!(!w.is_binary());
}

#[test]
fn writer_add_mesh_increments_nb_meshes() {
    let mut w = RwGltfWriter::new(false);
    assert_eq!(w.nb_meshes(), 0);
    w.add_mesh(GltfMeshPrimitive::new("m0", GltfPrimitiveMode::Triangles));
    assert_eq!(w.nb_meshes(), 1);
    w.add_mesh(GltfMeshPrimitive::new("m1", GltfPrimitiveMode::Points));
    assert_eq!(w.nb_meshes(), 2);
    w.add_mesh(GltfMeshPrimitive::new("m2", GltfPrimitiveMode::Lines));
    assert_eq!(w.nb_meshes(), 3);
}

#[test]
fn writer_nb_meshes_matches_primitives_len() {
    let mut w = RwGltfWriter::new(false);
    for i in 0..5 {
        w.add_mesh(GltfMeshPrimitive::new(
            format!("mesh{i}"),
            GltfPrimitiveMode::Triangles,
        ));
        assert_eq!(w.nb_meshes(), w.primitives().len());
    }
}

#[test]
fn writer_primitive_by_index() {
    let mut w = RwGltfWriter::new(false);
    w.add_mesh(GltfMeshPrimitive::new("alpha", GltfPrimitiveMode::Triangles));
    w.add_mesh(GltfMeshPrimitive::new("beta",  GltfPrimitiveMode::Points));
    assert_eq!(w.primitive(0).unwrap().name(), "alpha");
    assert_eq!(w.primitive(1).unwrap().name(), "beta");
    assert!(w.primitive(2).is_none());
}

#[test]
fn writer_primitive_out_of_bounds_is_none() {
    let w = RwGltfWriter::new(false);
    assert!(w.primitive(0).is_none());
}

#[test]
fn writer_clear_resets_count_and_primitives() {
    let mut w = RwGltfWriter::new(true);
    w.add_mesh(GltfMeshPrimitive::new("x", GltfPrimitiveMode::Triangles));
    w.add_mesh(GltfMeshPrimitive::new("y", GltfPrimitiveMode::Lines));
    w.clear();
    assert_eq!(w.nb_meshes(), 0);
    assert!(w.primitives().is_empty());
    assert!(w.primitive(0).is_none());
    // binary flag is preserved
    assert!(w.is_binary());
}

#[test]
fn writer_add_mesh_retains_accessor_data() {
    let mut w = RwGltfWriter::new(false);
    let mut prim = GltfMeshPrimitive::new("geo", GltfPrimitiveMode::Triangles);
    prim.set_index_accessor(10);
    prim.set_position_accessor(11);
    prim.set_normal_accessor(12);
    w.add_mesh(prim);
    let stored = w.primitive(0).unwrap();
    assert_eq!(stored.index_accessor(),    10);
    assert_eq!(stored.position_accessor(), 11);
    assert_eq!(stored.normal_accessor(),   12);
    assert!(stored.has_index_accessor());
    assert!(stored.has_position_accessor());
    assert!(stored.has_normal_accessor());
}

#[test]
fn writer_mixed_modes() {
    let mut w = RwGltfWriter::new(false);
    let modes = [
        GltfPrimitiveMode::Points,
        GltfPrimitiveMode::Lines,
        GltfPrimitiveMode::LineLoop,
        GltfPrimitiveMode::LineStrip,
        GltfPrimitiveMode::Triangles,
        GltfPrimitiveMode::TriangleStrip,
        GltfPrimitiveMode::TriangleFan,
    ];
    for mode in modes {
        w.add_mesh(GltfMeshPrimitive::new("prim", mode));
    }
    assert_eq!(w.nb_meshes(), modes.len());
    for (i, mode) in modes.iter().enumerate() {
        assert_eq!(w.primitive(i).unwrap().mode(), *mode);
    }
}

#[test]
fn writer_debug_is_available() {
    let w = RwGltfWriter::new(false);
    let _ = format!("{:?}", w);
}
