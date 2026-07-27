extern crate ironstream;
use ironstream::select3d_extra::*;

#[test]
fn sensitive_point_new_and_accessors() {
    let p = Select3dSensitivePoint::new([1.0, 2.0, 3.0], 0.5);
    assert_eq!(p.point(), [1.0, 2.0, 3.0]);
    assert!((p.sensitivity() - 0.5).abs() < 1e-12);
    assert_eq!(p.sensitive_type(), Select3dSensitiveType::Point);
}

#[test]
fn sensitive_point_fields_public() {
    let p = Select3dSensitivePoint::new([0.0, 0.0, 0.0], 1.0);
    assert_eq!(p.point, [0.0, 0.0, 0.0]);
    assert!((p.sensitivity - 1.0).abs() < 1e-12);
}

#[test]
fn sensitive_point_type_is_point() {
    let p = Select3dSensitivePoint::new([5.0, -3.0, 2.5], 0.1);
    assert_eq!(p.sensitive_type(), Select3dSensitiveType::Point);
}

#[test]
fn sensitive_segment_new_and_accessors() {
    let s = Select3dSensitiveSegment::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0], 1.0);
    assert_eq!(s.start(), [0.0, 0.0, 0.0]);
    assert_eq!(s.end(), [3.0, 4.0, 0.0]);
    assert!((s.sensitivity - 1.0).abs() < 1e-12);
    assert_eq!(s.sensitive_type(), Select3dSensitiveType::Segment);
}

#[test]
fn sensitive_segment_length_3_4_0() {
    let s = Select3dSensitiveSegment::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0], 1.0);
    assert!((s.length() - 5.0).abs() < 1e-10);
}

#[test]
fn sensitive_segment_length_axis_aligned() {
    let s = Select3dSensitiveSegment::new([1.0, 1.0, 1.0], [1.0, 1.0, 4.0], 0.5);
    assert!((s.length() - 3.0).abs() < 1e-10);
}

#[test]
fn sensitive_segment_length_zero() {
    let s = Select3dSensitiveSegment::new([2.0, 3.0, 4.0], [2.0, 3.0, 4.0], 0.1);
    assert!(s.length().abs() < 1e-12);
}

#[test]
fn sensitive_segment_type_is_segment() {
    let s = Select3dSensitiveSegment::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0);
    assert_eq!(s.sensitive_type(), Select3dSensitiveType::Segment);
}

#[test]
fn sensitive_sphere_new_and_accessors() {
    let sp = Select3dSensitiveSphere::new([1.0, 2.0, 3.0], 7.5);
    assert_eq!(sp.center(), [1.0, 2.0, 3.0]);
    assert!((sp.radius() - 7.5).abs() < 1e-12);
    assert_eq!(sp.sensitive_type(), Select3dSensitiveType::Sphere);
}

#[test]
fn sensitive_sphere_fields_public() {
    let sp = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 1.0);
    assert_eq!(sp.center, [0.0, 0.0, 0.0]);
    assert!((sp.radius - 1.0).abs() < 1e-12);
}

#[test]
fn sensitive_sphere_type_is_sphere() {
    let sp = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 5.0);
    assert_eq!(sp.sensitive_type(), Select3dSensitiveType::Sphere);
}

#[test]
fn owner_new_and_accessors() {
    let o = Select3dOwner::new("my_shape", 5);
    assert_eq!(o.label(), "my_shape");
    assert_eq!(o.priority(), 5);
}

#[test]
fn owner_set_priority() {
    let mut o = Select3dOwner::new("shape_a", 1);
    o.set_priority(10);
    assert_eq!(o.priority(), 10);
}

#[test]
fn owner_label_empty() {
    let o = Select3dOwner::new("", 0);
    assert_eq!(o.label(), "");
    assert_eq!(o.priority(), 0);
}

#[test]
fn owner_fields_public() {
    let o = Select3dOwner::new("test", 3);
    assert_eq!(o.label, "test");
    assert_eq!(o.priority, 3);
}

#[test]
fn sensitive_type_enum_variants() {
    let variants = [
        Select3dSensitiveType::Point,
        Select3dSensitiveType::Segment,
        Select3dSensitiveType::Triangle,
        Select3dSensitiveType::Box,
        Select3dSensitiveType::Circle,
        Select3dSensitiveType::Cylinder,
        Select3dSensitiveType::Sphere,
        Select3dSensitiveType::Polyhedron,
    ];
    assert_eq!(variants.len(), 8);
    assert_ne!(Select3dSensitiveType::Point, Select3dSensitiveType::Sphere);
    assert_eq!(Select3dSensitiveType::Triangle, Select3dSensitiveType::Triangle);
}
