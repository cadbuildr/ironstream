// FILE: tests/occt_graphic3d_buffer.rs
extern crate ironstream;
use ironstream::graphic3d_buffer::*;

#[test]
fn type_bytes() {
    assert_eq!(Graphic3dTypeOfData::Float.bytes(), 4);
    assert_eq!(Graphic3dTypeOfData::Vec3.bytes(), 12);
    assert_eq!(Graphic3dTypeOfData::Vec4d.bytes(), 32);
}

#[test]
fn buffer_init_and_access() {
    let mut b = Graphic3dBuffer::new();
    // 4 elements, stride 12 (vec3 of f32)
    b.init(4, 12);
    b.set_f32(0, 0, 1.5);
    b.set_f32(0, 4, 2.5);
    assert!((b.get_f32(0, 0).unwrap() - 1.5).abs() < 1e-6);
    assert!((b.get_f32(0, 4).unwrap() - 2.5).abs() < 1e-6);
    assert_eq!(b.size_bytes(), 48);
}

#[test]
fn index_buffer_quads() {
    let mut ib = Graphic3dIndexBuffer::new();
    // 2 quads → 2×6 = 12 indices → 4 triangles
    ib.init_quads(2);
    assert_eq!(ib.nb_elements(), 12);
    assert_eq!(ib.nb_triangles(), 4);
    // First tri of first quad: 0, 1, 2
    assert_eq!(ib.value(0), Some(0));
    assert_eq!(ib.value(2), Some(2));
}

#[test]
fn index_buffer_strip() {
    let mut ib = Graphic3dIndexBuffer::new();
    // 5 vertices → 3 triangles
    ib.init_triangle_strip(5);
    assert_eq!(ib.nb_triangles(), 3);
}

#[test]
fn bound_buffer_set_get() {
    let mut bb = Graphic3dBoundBuffer::new();
    bb.init(3, true);
    bb.set_bound(0, 4);
    bb.set_color(0, [1.0, 0.0, 0.0, 1.0]);
    bb.set_bound(1, 3);
    bb.set_bound(2, 6);
    assert_eq!(bb.bound(0), Some(4));
    assert_eq!(bb.nb_bounds(), 3);
}
