// FILE: rust/ironstream/crates/ironstream/tests/occt_poly_mesh_tools.rs
//! Integration tests for `ironstream::poly_mesh_tools`, mirroring the
//! attribute-enriched triangulation side of OpenCascade's `Poly` package:
//! `Poly_MeshPurpose`, `Poly_ArrayOfNodes` (normals), `Poly_ArrayOfUVNodes`,
//! and `Poly_Triangulation` (with attributes).

extern crate ironstream;

use ironstream::poly_mesh_tools::*;

// ─── PolyMeshPurpose ─────────────────────────────────────────────────────────

#[test]
fn purpose_all_variants_distinct() {
    let variants = [
        PolyMeshPurpose::Active,
        PolyMeshPurpose::Loaded,
        PolyMeshPurpose::Computed,
        PolyMeshPurpose::User,
    ];
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j]);
            } else {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

#[test]
fn purpose_is_copy() {
    let a = PolyMeshPurpose::Active;
    let b = a; // Copy
    assert_eq!(a, b);
}

#[test]
fn purpose_debug_contains_variant_name() {
    assert!(format!("{:?}", PolyMeshPurpose::Active).contains("Active"));
    assert!(format!("{:?}", PolyMeshPurpose::Loaded).contains("Loaded"));
    assert!(format!("{:?}", PolyMeshPurpose::Computed).contains("Computed"));
    assert!(format!("{:?}", PolyMeshPurpose::User).contains("User"));
}

// ─── PolyNormals ─────────────────────────────────────────────────────────────

#[test]
fn normals_starts_empty() {
    let n = PolyNormals::new();
    assert_eq!(n.nb_normals(), 0);
    assert!(n.iter().is_empty());
}

#[test]
fn normals_add_increments_count() {
    let mut n = PolyNormals::new();
    n.add([0.0, 0.0, 1.0]);
    assert_eq!(n.nb_normals(), 1);
    n.add([0.0, 1.0, 0.0]);
    assert_eq!(n.nb_normals(), 2);
}

#[test]
fn normals_get_returns_correct_entry() {
    let mut n = PolyNormals::new();
    n.add([1.0, 0.0, 0.0]);
    n.add([0.0, 1.0, 0.0]);
    n.add([0.0, 0.0, 1.0]);
    assert_eq!(n.get(0), [1.0_f32, 0.0, 0.0]);
    assert_eq!(n.get(1), [0.0_f32, 1.0, 0.0]);
    assert_eq!(n.get(2), [0.0_f32, 0.0, 1.0]);
}

#[test]
fn normals_set_overwrites_entry() {
    let mut n = PolyNormals::new();
    n.add([0.0, 0.0, 1.0]);
    n.set(0, [1.0, 0.0, 0.0]);
    assert_eq!(n.get(0), [1.0_f32, 0.0, 0.0]);
}

#[test]
fn normals_iter_is_contiguous_slice() {
    let mut n = PolyNormals::new();
    n.add([1.0, 0.0, 0.0]);
    n.add([0.0, 1.0, 0.0]);
    let s: &[[f32; 3]] = n.iter();
    assert_eq!(s.len(), 2);
    assert_eq!(s[0], [1.0_f32, 0.0, 0.0]);
    assert_eq!(s[1], [0.0_f32, 1.0, 0.0]);
}

#[test]
fn normals_multiple_entries_roundtrip() {
    let mut n = PolyNormals::new();
    let data: &[[f32; 3]] = &[
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [-1.0, 0.0, 0.0],
    ];
    for &entry in data {
        n.add(entry);
    }
    assert_eq!(n.nb_normals(), data.len());
    for (i, &expected) in data.iter().enumerate() {
        assert_eq!(n.get(i), expected, "mismatch at index {i}");
    }
}

#[test]
#[should_panic]
fn normals_get_out_of_bounds_panics() {
    let n = PolyNormals::new();
    let _ = n.get(0);
}

#[test]
#[should_panic]
fn normals_set_out_of_bounds_panics() {
    let mut n = PolyNormals::new();
    n.set(0, [0.0, 0.0, 1.0]);
}

// ─── PolyUVNodes ─────────────────────────────────────────────────────────────

#[test]
fn uvnodes_starts_empty() {
    let uv = PolyUVNodes::new();
    assert_eq!(uv.nb_uvs(), 0);
}

#[test]
fn uvnodes_add_increments_count() {
    let mut uv = PolyUVNodes::new();
    uv.add([0.0, 0.0]);
    assert_eq!(uv.nb_uvs(), 1);
    uv.add([1.0, 1.0]);
    assert_eq!(uv.nb_uvs(), 2);
}

#[test]
fn uvnodes_get_returns_correct_entry() {
    let mut uv = PolyUVNodes::new();
    uv.add([0.0, 0.5]);
    uv.add([1.0, 0.0]);
    assert_eq!(uv.get(0), [0.0_f32, 0.5]);
    assert_eq!(uv.get(1), [1.0_f32, 0.0]);
}

#[test]
fn uvnodes_many_entries_roundtrip() {
    let mut uv = PolyUVNodes::new();
    let data: &[[f32; 2]] = &[[0.0, 0.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]];
    for &entry in data {
        uv.add(entry);
    }
    assert_eq!(uv.nb_uvs(), data.len());
    for (i, &expected) in data.iter().enumerate() {
        assert_eq!(uv.get(i), expected, "mismatch at index {i}");
    }
}

#[test]
#[should_panic]
fn uvnodes_get_out_of_bounds_panics() {
    let uv = PolyUVNodes::new();
    let _ = uv.get(0);
}

// ─── PolyMeshData ────────────────────────────────────────────────────────────

/// Build a simple unit-triangle mesh for reuse across tests.
///
/// Nodes (0-based):
///   0 = (0, 0, 0)
///   1 = (1, 0, 0)
///   2 = (0, 1, 0)
/// Triangle: [0, 1, 2]
fn unit_triangle_mesh() -> PolyMeshData {
    let mut m = PolyMeshData::new(0.01);
    m.add_node([0.0, 0.0, 0.0]);
    m.add_node([1.0, 0.0, 0.0]);
    m.add_node([0.0, 1.0, 0.0]);
    m.add_triangle([0, 1, 2]);
    m
}

#[test]
fn mesh_data_new_initialises_to_computed() {
    let m = PolyMeshData::new(0.05);
    assert_eq!(m.purpose(), PolyMeshPurpose::Computed);
}

#[test]
fn mesh_data_new_deflection_roundtrip() {
    let deflection = 0.123_f64;
    let m = PolyMeshData::new(deflection);
    assert!((m.deflection() - deflection).abs() < 1e-15, "deflection mismatch");
}

#[test]
fn mesh_data_empty_on_construction() {
    let m = PolyMeshData::new(0.0);
    assert_eq!(m.nb_nodes(), 0);
    assert_eq!(m.nb_triangles(), 0);
    assert_eq!(m.normals.nb_normals(), 0);
    assert_eq!(m.uvs.nb_uvs(), 0);
}

#[test]
fn mesh_data_add_node_grows_all_parallel_arrays() {
    let mut m = PolyMeshData::new(0.0);
    m.add_node([1.0, 2.0, 3.0]);
    assert_eq!(m.nb_nodes(), 1);
    assert_eq!(m.normals.nb_normals(), 1, "normals should grow with nodes");
    assert_eq!(m.uvs.nb_uvs(), 1, "uvs should grow with nodes");
}

#[test]
fn mesh_data_add_node_default_normal_is_zero() {
    let mut m = PolyMeshData::new(0.0);
    m.add_node([0.0, 0.0, 0.0]);
    assert_eq!(m.normals.get(0), [0.0_f32, 0.0, 0.0]);
}

#[test]
fn mesh_data_add_node_default_uv_is_zero() {
    let mut m = PolyMeshData::new(0.0);
    m.add_node([0.0, 0.0, 0.0]);
    assert_eq!(m.uvs.get(0), [0.0_f32, 0.0]);
}

#[test]
fn mesh_data_add_triangle_increments_count() {
    let mut m = unit_triangle_mesh();
    assert_eq!(m.nb_triangles(), 1);
    m.add_triangle([0, 2, 1]); // add reversed winding
    assert_eq!(m.nb_triangles(), 2);
}

#[test]
fn mesh_data_triangle_indices_preserved() {
    let m = unit_triangle_mesh();
    assert_eq!(m.triangles[0], [0, 1, 2]);
}

#[test]
fn mesh_data_node_positions_preserved() {
    let m = unit_triangle_mesh();
    assert_eq!(m.nodes[0], [0.0_f64, 0.0, 0.0]);
    assert_eq!(m.nodes[1], [1.0_f64, 0.0, 0.0]);
    assert_eq!(m.nodes[2], [0.0_f64, 1.0, 0.0]);
}

#[test]
fn mesh_data_set_normal_overwrites_default() {
    let mut m = unit_triangle_mesh();
    m.set_normal(0, [0.0, 0.0, 1.0]);
    assert_eq!(m.normals.get(0), [0.0_f32, 0.0, 1.0]);
}

#[test]
fn mesh_data_set_uv_overwrites_default() {
    let mut m = unit_triangle_mesh();
    m.set_uv(1, [0.5, 0.25]);
    assert_eq!(m.uvs.get(1), [0.5_f32, 0.25]);
}

#[test]
fn mesh_data_set_all_normals_and_uvs() {
    let mut m = unit_triangle_mesh();
    let normals: &[[f32; 3]] = &[
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let uvs: &[[f32; 2]] = &[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    for i in 0..3 {
        m.set_normal(i, normals[i]);
        m.set_uv(i, uvs[i]);
    }
    for i in 0..3 {
        assert_eq!(m.normals.get(i), normals[i], "normal mismatch at {i}");
        assert_eq!(m.uvs.get(i), uvs[i], "uv mismatch at {i}");
    }
}

#[test]
fn mesh_data_purpose_field_assignable() {
    let mut m = PolyMeshData::new(0.0);
    m.purpose = PolyMeshPurpose::Active;
    assert_eq!(m.purpose(), PolyMeshPurpose::Active);
    m.purpose = PolyMeshPurpose::User;
    assert_eq!(m.purpose(), PolyMeshPurpose::User);
    m.purpose = PolyMeshPurpose::Loaded;
    assert_eq!(m.purpose(), PolyMeshPurpose::Loaded);
}

#[test]
fn mesh_data_nb_nodes_and_nb_triangles_are_consistent() {
    let mut m = PolyMeshData::new(0.0);
    for i in 0..10 {
        m.add_node([i as f64, 0.0, 0.0]);
    }
    for i in 0..8 {
        m.add_triangle([i, i + 1, 9]);
    }
    assert_eq!(m.nb_nodes(), 10);
    assert_eq!(m.nb_triangles(), 8);
}

#[test]
#[should_panic]
fn mesh_data_set_normal_out_of_range_panics() {
    let mut m = PolyMeshData::new(0.0);
    m.set_normal(0, [0.0, 0.0, 1.0]);
}

#[test]
#[should_panic]
fn mesh_data_set_uv_out_of_range_panics() {
    let mut m = PolyMeshData::new(0.0);
    m.set_uv(0, [0.0, 0.0]);
}
