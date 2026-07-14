// FILE: rust/ironstream/crates/ironstream/tests/occt_poly_polygon.rs
//! Integration tests for `poly_polygon` — covers `PolyPolygon3D`,
//! `PolyPolygon2D`, and `PolyPolygonOnTriangulation`.

extern crate ironstream;
use ironstream::poly_polygon::*;

const TOL: f64 = 1e-12;

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= TOL,
        "expected {a} ≈ {b} (diff {})",
        (a - b).abs()
    );
}

// ─── PolyPolygon3D ───────────────────────────────────────────────────────────

#[test]
fn poly3d_new_basic() {
    let nodes = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]];
    let p = PolyPolygon3D::new(nodes);
    assert_eq!(p.nb_nodes(), 3);
    near(p.deflection(), 0.0);
    assert!(!p.has_parameters());
}

#[test]
fn poly3d_node_values() {
    let nodes = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    let p = PolyPolygon3D::new(nodes);
    assert_eq!(p.node(0), [1.0, 2.0, 3.0]);
    assert_eq!(p.node(1), [4.0, 5.0, 6.0]);
}

#[test]
fn poly3d_set_deflection() {
    let mut p = PolyPolygon3D::new(vec![[0.0, 0.0, 0.0]]);
    p.set_deflection(0.001);
    near(p.deflection(), 0.001);
}

#[test]
fn poly3d_with_parameters() {
    let nodes = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let params = vec![0.0, 0.5, 1.0];
    let p = PolyPolygon3D::with_parameters(nodes, params);
    assert!(p.has_parameters());
    assert_eq!(p.nb_nodes(), 3);
    near(p.parameter(0), 0.0);
    near(p.parameter(1), 0.5);
    near(p.parameter(2), 1.0);
}

#[test]
fn poly3d_with_parameters_node_access() {
    let nodes = vec![[10.0, 20.0, 30.0], [40.0, 50.0, 60.0]];
    let params = vec![0.0, 1.0];
    let p = PolyPolygon3D::with_parameters(nodes, params);
    assert_eq!(p.node(0), [10.0, 20.0, 30.0]);
    assert_eq!(p.node(1), [40.0, 50.0, 60.0]);
}

#[test]
fn poly3d_single_node() {
    let p = PolyPolygon3D::new(vec![[7.0, 8.0, 9.0]]);
    assert_eq!(p.nb_nodes(), 1);
    assert_eq!(p.node(0), [7.0, 8.0, 9.0]);
}

#[test]
fn poly3d_empty_nodes() {
    let p = PolyPolygon3D::new(vec![]);
    assert_eq!(p.nb_nodes(), 0);
    assert!(!p.has_parameters());
}

#[test]
#[should_panic(expected = "index 3 out of range")]
fn poly3d_node_out_of_range_panics() {
    let p = PolyPolygon3D::new(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let _ = p.node(3);
}

#[test]
#[should_panic(expected = "no parameters stored")]
fn poly3d_parameter_without_params_panics() {
    let p = PolyPolygon3D::new(vec![[0.0, 0.0, 0.0]]);
    let _ = p.parameter(0);
}

#[test]
#[should_panic(expected = "nodes.len() (2) != parameters.len() (3)")]
fn poly3d_mismatched_params_panics() {
    let _ = PolyPolygon3D::with_parameters(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        vec![0.0, 0.5, 1.0],
    );
}

#[test]
fn poly3d_deflection_roundtrip() {
    let mut p = PolyPolygon3D::new(vec![[0.0, 0.0, 0.0]]);
    near(p.deflection(), 0.0);
    p.set_deflection(1e-4);
    near(p.deflection(), 1e-4);
    p.set_deflection(0.0);
    near(p.deflection(), 0.0);
}

// ─── PolyPolygon2D ───────────────────────────────────────────────────────────

#[test]
fn poly2d_new_basic() {
    let nodes = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let p = PolyPolygon2D::new(nodes);
    assert_eq!(p.nb_nodes(), 4);
    near(p.deflection(), 0.0);
    assert!(!p.has_parameters());
}

#[test]
fn poly2d_node_values() {
    let nodes = vec![[3.0, 4.0], [5.0, 6.0]];
    let p = PolyPolygon2D::new(nodes);
    assert_eq!(p.node(0), [3.0, 4.0]);
    assert_eq!(p.node(1), [5.0, 6.0]);
}

#[test]
fn poly2d_set_deflection() {
    let mut p = PolyPolygon2D::new(vec![[0.0, 0.0]]);
    p.set_deflection(2.5);
    near(p.deflection(), 2.5);
}

#[test]
fn poly2d_with_parameters() {
    let nodes = vec![[0.0, 0.0], [0.5, 0.5], [1.0, 1.0]];
    let params = vec![0.0, 0.5, 1.0];
    let p = PolyPolygon2D::with_parameters(nodes, params);
    assert!(p.has_parameters());
    assert_eq!(p.nb_nodes(), 3);
    near(p.parameter(0), 0.0);
    near(p.parameter(1), 0.5);
    near(p.parameter(2), 1.0);
}

#[test]
fn poly2d_with_parameters_and_deflection() {
    let nodes = vec![[0.0, 0.0], [1.0, 0.0]];
    let params = vec![0.0, 1.0];
    let mut p = PolyPolygon2D::with_parameters(nodes, params);
    p.set_deflection(0.01);
    near(p.deflection(), 0.01);
    assert!(p.has_parameters());
}

#[test]
fn poly2d_single_node() {
    let p = PolyPolygon2D::new(vec![[99.0, -1.0]]);
    assert_eq!(p.nb_nodes(), 1);
    assert_eq!(p.node(0), [99.0, -1.0]);
}

#[test]
fn poly2d_empty_nodes() {
    let p = PolyPolygon2D::new(vec![]);
    assert_eq!(p.nb_nodes(), 0);
}

#[test]
#[should_panic(expected = "index 5 out of range")]
fn poly2d_node_out_of_range_panics() {
    let p = PolyPolygon2D::new(vec![[0.0, 0.0], [1.0, 1.0]]);
    let _ = p.node(5);
}

#[test]
#[should_panic(expected = "no parameters stored")]
fn poly2d_parameter_without_params_panics() {
    let p = PolyPolygon2D::new(vec![[0.0, 0.0]]);
    let _ = p.parameter(0);
}

#[test]
#[should_panic(expected = "nodes.len() (1) != parameters.len() (2)")]
fn poly2d_mismatched_params_panics() {
    let _ = PolyPolygon2D::with_parameters(vec![[0.0, 0.0]], vec![0.0, 1.0]);
}

// ─── PolyPolygonOnTriangulation ──────────────────────────────────────────────

#[test]
fn poly_on_tri_new_basic() {
    let indices = vec![0, 3, 7, 2];
    let p = PolyPolygonOnTriangulation::new(indices);
    assert_eq!(p.nb_nodes(), 4);
    near(p.deflection(), 0.0);
    assert!(!p.has_parameters());
}

#[test]
fn poly_on_tri_node_access() {
    let p = PolyPolygonOnTriangulation::new(vec![10, 20, 30]);
    assert_eq!(p.node(0), 10);
    assert_eq!(p.node(1), 20);
    assert_eq!(p.node(2), 30);
}

#[test]
fn poly_on_tri_set_deflection() {
    let mut p = PolyPolygonOnTriangulation::new(vec![0, 1, 2]);
    p.set_deflection(0.05);
    near(p.deflection(), 0.05);
}

#[test]
fn poly_on_tri_single_node() {
    let p = PolyPolygonOnTriangulation::new(vec![42]);
    assert_eq!(p.nb_nodes(), 1);
    assert_eq!(p.node(0), 42);
}

#[test]
fn poly_on_tri_empty() {
    let p = PolyPolygonOnTriangulation::new(vec![]);
    assert_eq!(p.nb_nodes(), 0);
    assert!(!p.has_parameters());
}

#[test]
#[should_panic(expected = "index 3 out of range")]
fn poly_on_tri_node_out_of_range_panics() {
    let p = PolyPolygonOnTriangulation::new(vec![0, 1, 2]);
    let _ = p.node(3);
}

#[test]
#[should_panic(expected = "no parameters stored")]
fn poly_on_tri_parameter_without_params_panics() {
    let p = PolyPolygonOnTriangulation::new(vec![0, 1]);
    let _ = p.parameter(0);
}

#[test]
fn poly_on_tri_deflection_zero_default() {
    let p = PolyPolygonOnTriangulation::new(vec![0]);
    near(p.deflection(), 0.0);
}

#[test]
fn poly_on_tri_has_parameters_false_by_default() {
    let p = PolyPolygonOnTriangulation::new(vec![0, 1, 2, 3]);
    assert!(!p.has_parameters());
}

// ─── Cross-type smoke tests ───────────────────────────────────────────────────

#[test]
fn poly3d_many_nodes_sequential_access() {
    let n = 100_usize;
    let nodes: Vec<[f64; 3]> = (0..n)
        .map(|i| [i as f64, (i * 2) as f64, (i * 3) as f64])
        .collect();
    let p = PolyPolygon3D::new(nodes);
    assert_eq!(p.nb_nodes(), n);
    for i in 0..n {
        let node = p.node(i);
        near(node[0], i as f64);
        near(node[1], (i * 2) as f64);
        near(node[2], (i * 3) as f64);
    }
}

#[test]
fn poly2d_many_nodes_with_params() {
    let n = 50_usize;
    let nodes: Vec<[f64; 2]> = (0..n).map(|i| [i as f64, -(i as f64)]).collect();
    let params: Vec<f64> = (0..n).map(|i| i as f64 / (n - 1) as f64).collect();
    let p = PolyPolygon2D::with_parameters(nodes, params.clone());
    assert!(p.has_parameters());
    for i in 0..n {
        let node = p.node(i);
        near(node[0], i as f64);
        near(node[1], -(i as f64));
        near(p.parameter(i), params[i]);
    }
}

#[test]
fn poly_on_tri_large_indices() {
    let indices: Vec<usize> = (1000..1010).collect();
    let p = PolyPolygonOnTriangulation::new(indices);
    assert_eq!(p.nb_nodes(), 10);
    for i in 0..10 {
        assert_eq!(p.node(i), 1000 + i);
    }
}
