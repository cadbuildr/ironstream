// FILE: rust/ironstream/crates/ironstream/tests/occt_rw_obj.rs
//! Integration tests for the `rw_obj` module — OBJ material, mesh, and scene
//! types mirroring OpenCascade's `RWObj` package.

extern crate ironstream;
use ironstream::rw_obj::*;

// ===========================================================================
// RwObjMaterial
// ===========================================================================

#[test]
fn material_new_name_is_stored() {
    let m = RwObjMaterial::new("Copper");
    assert_eq!(m.name(), "Copper");
}

#[test]
fn material_new_diffuse_default_is_white() {
    let m = RwObjMaterial::new("x");
    assert_eq!(m.diffuse(), [1.0, 1.0, 1.0]);
}

#[test]
fn material_new_specular_default_is_black() {
    let m = RwObjMaterial::new("x");
    assert_eq!(m.specular(), [0.0, 0.0, 0.0]);
}

#[test]
fn material_new_shininess_default_is_zero() {
    let m = RwObjMaterial::new("x");
    assert_eq!(m.shininess(), 0.0_f32);
}

#[test]
fn material_new_transparency_default_is_zero() {
    let m = RwObjMaterial::new("x");
    assert_eq!(m.transparency(), 0.0_f32);
}

#[test]
fn material_new_texture_map_default_is_empty() {
    let m = RwObjMaterial::new("x");
    assert_eq!(m.texture_map(), "");
}

#[test]
fn material_set_and_get_diffuse() {
    let mut m = RwObjMaterial::new("red");
    m.set_diffuse([1.0, 0.0, 0.0]);
    assert_eq!(m.diffuse(), [1.0, 0.0, 0.0]);
}

#[test]
fn material_set_diffuse_overwrites_previous() {
    let mut m = RwObjMaterial::new("c");
    m.set_diffuse([0.1, 0.2, 0.3]);
    m.set_diffuse([0.9, 0.8, 0.7]);
    assert_eq!(m.diffuse(), [0.9, 0.8, 0.7]);
}

#[test]
fn material_set_and_get_specular() {
    let mut m = RwObjMaterial::new("shiny");
    m.set_specular([0.8, 0.8, 0.8]);
    assert_eq!(m.specular(), [0.8, 0.8, 0.8]);
}

#[test]
fn material_set_specular_overwrites_previous() {
    let mut m = RwObjMaterial::new("c");
    m.set_specular([0.1, 0.2, 0.3]);
    m.set_specular([0.4, 0.5, 0.6]);
    assert_eq!(m.specular(), [0.4, 0.5, 0.6]);
}

#[test]
fn material_set_and_get_shininess() {
    let mut m = RwObjMaterial::new("s");
    m.set_shininess(256.0);
    assert_eq!(m.shininess(), 256.0_f32);
}

#[test]
fn material_shininess_extremes() {
    let mut m = RwObjMaterial::new("s");
    m.set_shininess(0.0);
    assert_eq!(m.shininess(), 0.0_f32);
    m.set_shininess(1000.0);
    assert_eq!(m.shininess(), 1000.0_f32);
}

#[test]
fn material_set_and_get_transparency() {
    let mut m = RwObjMaterial::new("glass");
    m.set_transparency(0.75);
    assert!((m.transparency() - 0.75_f32).abs() < 1e-6);
}

#[test]
fn material_transparency_fully_opaque() {
    let mut m = RwObjMaterial::new("t");
    m.set_transparency(0.0);
    assert_eq!(m.transparency(), 0.0_f32);
}

#[test]
fn material_transparency_fully_transparent() {
    let mut m = RwObjMaterial::new("t");
    m.set_transparency(1.0);
    assert_eq!(m.transparency(), 1.0_f32);
}

#[test]
fn material_set_and_get_texture_map() {
    let mut m = RwObjMaterial::new("wood");
    m.set_texture_map("textures/wood_diffuse.png");
    assert_eq!(m.texture_map(), "textures/wood_diffuse.png");
}

#[test]
fn material_set_texture_map_empty_clears() {
    let mut m = RwObjMaterial::new("t");
    m.set_texture_map("some_file.png");
    m.set_texture_map("");
    assert_eq!(m.texture_map(), "");
}

#[test]
fn material_set_texture_map_overwrites() {
    let mut m = RwObjMaterial::new("t");
    m.set_texture_map("first.png");
    m.set_texture_map("second.png");
    assert_eq!(m.texture_map(), "second.png");
}

#[test]
fn material_default_name_is_empty() {
    let m = RwObjMaterial::default();
    assert_eq!(m.name(), "");
}

#[test]
fn material_default_diffuse_is_white() {
    let m = RwObjMaterial::default();
    assert_eq!(m.diffuse(), [1.0, 1.0, 1.0]);
}

#[test]
fn material_all_fields_independently_settable() {
    let mut m = RwObjMaterial::new("full");
    m.set_diffuse([0.2, 0.4, 0.6]);
    m.set_specular([0.7, 0.8, 0.9]);
    m.set_shininess(64.0);
    m.set_transparency(0.1);
    m.set_texture_map("tex.png");
    assert_eq!(m.diffuse(), [0.2, 0.4, 0.6]);
    assert_eq!(m.specular(), [0.7, 0.8, 0.9]);
    assert_eq!(m.shininess(), 64.0_f32);
    assert!((m.transparency() - 0.1_f32).abs() < 1e-6);
    assert_eq!(m.texture_map(), "tex.png");
}

// ===========================================================================
// RwObjMesh
// ===========================================================================

#[test]
fn mesh_new_name_is_stored() {
    let m = RwObjMesh::new("Body");
    assert_eq!(m.name(), "Body");
}

#[test]
fn mesh_new_has_zero_vertices() {
    let m = RwObjMesh::new("x");
    assert_eq!(m.nb_vertices(), 0);
}

#[test]
fn mesh_new_has_zero_triangles() {
    let m = RwObjMesh::new("x");
    assert_eq!(m.nb_triangles(), 0);
}

#[test]
fn mesh_new_material_name_is_empty() {
    let m = RwObjMesh::new("x");
    assert_eq!(m.material_name(), "");
}

#[test]
fn mesh_add_vertex_returns_index() {
    let mut m = RwObjMesh::new("x");
    assert_eq!(m.add_vertex([0.0, 0.0, 0.0]), 0);
    assert_eq!(m.add_vertex([1.0, 0.0, 0.0]), 1);
    assert_eq!(m.add_vertex([0.0, 1.0, 0.0]), 2);
}

#[test]
fn mesh_nb_vertices_matches_adds() {
    let mut m = RwObjMesh::new("x");
    for i in 0..5 {
        m.add_vertex([i as f32, 0.0, 0.0]);
    }
    assert_eq!(m.nb_vertices(), 5);
}

#[test]
fn mesh_vertices_slice_is_correct() {
    let mut m = RwObjMesh::new("x");
    m.add_vertex([1.0, 2.0, 3.0]);
    m.add_vertex([4.0, 5.0, 6.0]);
    assert_eq!(m.vertices(), &[[1.0_f32, 2.0, 3.0], [4.0, 5.0, 6.0]]);
}

#[test]
fn mesh_add_normal_returns_index() {
    let mut m = RwObjMesh::new("n");
    assert_eq!(m.add_normal([0.0, 0.0, 1.0]), 0);
    assert_eq!(m.add_normal([0.0, 1.0, 0.0]), 1);
}

#[test]
fn mesh_normals_slice_is_correct() {
    let mut m = RwObjMesh::new("n");
    m.add_normal([0.0, 0.0, 1.0]);
    assert_eq!(m.normals(), &[[0.0_f32, 0.0, 1.0]]);
}

#[test]
fn mesh_add_tex_coord_returns_index() {
    let mut m = RwObjMesh::new("t");
    assert_eq!(m.add_tex_coord([0.0, 0.0]), 0);
    assert_eq!(m.add_tex_coord([1.0, 1.0]), 1);
}

#[test]
fn mesh_tex_coords_slice_is_correct() {
    let mut m = RwObjMesh::new("t");
    m.add_tex_coord([0.25, 0.75]);
    assert_eq!(m.tex_coords(), &[[0.25_f32, 0.75]]);
}

#[test]
fn mesh_add_triangle_increments_count() {
    let mut m = RwObjMesh::new("t");
    m.add_vertex([0.0, 0.0, 0.0]);
    m.add_vertex([1.0, 0.0, 0.0]);
    m.add_vertex([0.0, 1.0, 0.0]);
    m.add_triangle([0, 1, 2]);
    assert_eq!(m.nb_triangles(), 1);
}

#[test]
fn mesh_indices_slice_is_correct() {
    let mut m = RwObjMesh::new("t");
    m.add_triangle([0, 1, 2]);
    m.add_triangle([1, 2, 3]);
    assert_eq!(m.indices(), &[[0_u32, 1, 2], [1, 2, 3]]);
}

#[test]
fn mesh_set_material_stores_name() {
    let mut m = RwObjMesh::new("x");
    m.set_material("Chrome");
    assert_eq!(m.material_name(), "Chrome");
}

#[test]
fn mesh_set_material_overwrites() {
    let mut m = RwObjMesh::new("x");
    m.set_material("Chrome");
    m.set_material("Rubber");
    assert_eq!(m.material_name(), "Rubber");
}

#[test]
fn mesh_set_material_to_empty() {
    let mut m = RwObjMesh::new("x");
    m.set_material("Chrome");
    m.set_material("");
    assert_eq!(m.material_name(), "");
}

#[test]
fn mesh_tetrahedron_geometry() {
    let mut m = RwObjMesh::new("tetra");
    m.add_vertex([0.0, 0.0, 0.0]);
    m.add_vertex([1.0, 0.0, 0.0]);
    m.add_vertex([0.5, 1.0, 0.0]);
    m.add_vertex([0.5, 0.5, 1.0]);
    m.add_triangle([0, 1, 2]);
    m.add_triangle([0, 1, 3]);
    m.add_triangle([0, 2, 3]);
    m.add_triangle([1, 2, 3]);
    assert_eq!(m.nb_vertices(), 4);
    assert_eq!(m.nb_triangles(), 4);
}

#[test]
fn mesh_normals_independent_of_vertices() {
    let mut m = RwObjMesh::new("x");
    m.add_vertex([0.0, 0.0, 0.0]);
    m.add_normal([0.0, 0.0, 1.0]);
    m.add_normal([0.0, 1.0, 0.0]);
    assert_eq!(m.nb_vertices(), 1);
    assert_eq!(m.normals().len(), 2);
}

// ===========================================================================
// RwObjScene
// ===========================================================================

#[test]
fn scene_new_nb_meshes_is_zero() {
    let s = RwObjScene::new();
    assert_eq!(s.nb_meshes(), 0);
}

#[test]
fn scene_new_nb_materials_is_zero() {
    let s = RwObjScene::new();
    assert_eq!(s.nb_materials(), 0);
}

#[test]
fn scene_default_is_empty() {
    let s = RwObjScene::default();
    assert_eq!(s.nb_meshes(), 0);
    assert_eq!(s.nb_materials(), 0);
}

#[test]
fn scene_add_mesh_increments_count() {
    let mut s = RwObjScene::new();
    s.add_mesh(RwObjMesh::new("A"));
    assert_eq!(s.nb_meshes(), 1);
    s.add_mesh(RwObjMesh::new("B"));
    assert_eq!(s.nb_meshes(), 2);
}

#[test]
fn scene_add_material_increments_count() {
    let mut s = RwObjScene::new();
    s.add_material(RwObjMaterial::new("M1"));
    assert_eq!(s.nb_materials(), 1);
    s.add_material(RwObjMaterial::new("M2"));
    assert_eq!(s.nb_materials(), 2);
}

#[test]
fn scene_mesh_returns_correct_ref() {
    let mut s = RwObjScene::new();
    s.add_mesh(RwObjMesh::new("Alpha"));
    s.add_mesh(RwObjMesh::new("Beta"));
    assert_eq!(s.mesh(0).name(), "Alpha");
    assert_eq!(s.mesh(1).name(), "Beta");
}

#[test]
fn scene_material_returns_correct_ref() {
    let mut s = RwObjScene::new();
    s.add_material(RwObjMaterial::new("Iron"));
    s.add_material(RwObjMaterial::new("Gold"));
    assert_eq!(s.material(0).name(), "Iron");
    assert_eq!(s.material(1).name(), "Gold");
}

#[test]
#[should_panic]
fn scene_mesh_oob_panics() {
    let s = RwObjScene::new();
    let _ = s.mesh(0);
}

#[test]
#[should_panic]
fn scene_material_oob_panics() {
    let s = RwObjScene::new();
    let _ = s.material(0);
}

#[test]
fn scene_mesh_geometry_survives_insertion() {
    let mut s = RwObjScene::new();
    let mut m = RwObjMesh::new("Cube");
    m.add_vertex([0.0, 0.0, 0.0]);
    m.add_vertex([1.0, 0.0, 0.0]);
    m.add_vertex([1.0, 1.0, 0.0]);
    m.add_triangle([0, 1, 2]);
    s.add_mesh(m);
    let stored = s.mesh(0);
    assert_eq!(stored.nb_vertices(), 3);
    assert_eq!(stored.nb_triangles(), 1);
    assert_eq!(stored.vertices()[0], [0.0_f32, 0.0, 0.0]);
}

#[test]
fn scene_material_properties_survive_insertion() {
    let mut s = RwObjScene::new();
    let mut mat = RwObjMaterial::new("Jade");
    mat.set_diffuse([0.0, 0.8, 0.2]);
    mat.set_shininess(50.0);
    mat.set_transparency(0.05);
    mat.set_texture_map("jade.png");
    s.add_material(mat);
    let stored = s.material(0);
    assert_eq!(stored.diffuse(), [0.0, 0.8, 0.2]);
    assert_eq!(stored.shininess(), 50.0_f32);
    assert!((stored.transparency() - 0.05_f32).abs() < 1e-6);
    assert_eq!(stored.texture_map(), "jade.png");
}

#[test]
fn scene_many_meshes_and_materials() {
    let mut s = RwObjScene::new();
    for i in 0..10_usize {
        let mut m = RwObjMesh::new(format!("Mesh{i}"));
        m.add_vertex([i as f32, 0.0, 0.0]);
        s.add_mesh(m);

        let mut mat = RwObjMaterial::new(format!("Mat{i}"));
        mat.set_shininess(i as f32 * 10.0);
        s.add_material(mat);
    }
    assert_eq!(s.nb_meshes(), 10);
    assert_eq!(s.nb_materials(), 10);
    for i in 0..10_usize {
        assert_eq!(s.mesh(i).name(), format!("Mesh{i}"));
        assert_eq!(s.mesh(i).nb_vertices(), 1);
        assert_eq!(s.material(i).name(), format!("Mat{i}"));
        assert_eq!(s.material(i).shininess(), i as f32 * 10.0);
    }
}

#[test]
fn scene_mesh_and_material_are_independent() {
    let mut s = RwObjScene::new();
    s.add_mesh(RwObjMesh::new("OnlyMesh"));
    // No materials added — counts stay independent.
    assert_eq!(s.nb_meshes(), 1);
    assert_eq!(s.nb_materials(), 0);

    s.add_material(RwObjMaterial::new("OnlyMat"));
    assert_eq!(s.nb_meshes(), 1);
    assert_eq!(s.nb_materials(), 1);
}

#[test]
fn scene_full_obj_asset() {
    // Simulate reading a simple OBJ + MTL pair into a scene.
    let mut scene = RwObjScene::new();

    // Two materials
    let mut mat_red = RwObjMaterial::new("Red");
    mat_red.set_diffuse([1.0, 0.0, 0.0]);
    mat_red.set_specular([0.5, 0.5, 0.5]);
    mat_red.set_shininess(16.0);
    scene.add_material(mat_red);

    let mut mat_tex = RwObjMaterial::new("Textured");
    mat_tex.set_diffuse([1.0, 1.0, 1.0]);
    mat_tex.set_texture_map("wall.png");
    mat_tex.set_transparency(0.0);
    scene.add_material(mat_tex);

    // First mesh — a triangle with the red material
    let mut tri = RwObjMesh::new("Triangle");
    tri.add_vertex([0.0, 0.0, 0.0]);
    tri.add_vertex([1.0, 0.0, 0.0]);
    tri.add_vertex([0.5, 1.0, 0.0]);
    tri.add_normal([0.0, 0.0, 1.0]);
    tri.add_tex_coord([0.0, 0.0]);
    tri.add_tex_coord([1.0, 0.0]);
    tri.add_tex_coord([0.5, 1.0]);
    tri.add_triangle([0, 1, 2]);
    tri.set_material("Red");
    scene.add_mesh(tri);

    // Second mesh — a quad (two triangles) with the textured material
    let mut quad = RwObjMesh::new("Quad");
    quad.add_vertex([0.0, 0.0, 0.0]);
    quad.add_vertex([2.0, 0.0, 0.0]);
    quad.add_vertex([2.0, 2.0, 0.0]);
    quad.add_vertex([0.0, 2.0, 0.0]);
    quad.add_triangle([0, 1, 2]);
    quad.add_triangle([0, 2, 3]);
    quad.set_material("Textured");
    scene.add_mesh(quad);

    // Assertions
    assert_eq!(scene.nb_meshes(), 2);
    assert_eq!(scene.nb_materials(), 2);

    let tri_out = scene.mesh(0);
    assert_eq!(tri_out.name(), "Triangle");
    assert_eq!(tri_out.nb_vertices(), 3);
    assert_eq!(tri_out.nb_triangles(), 1);
    assert_eq!(tri_out.normals().len(), 1);
    assert_eq!(tri_out.tex_coords().len(), 3);
    assert_eq!(tri_out.material_name(), "Red");

    let quad_out = scene.mesh(1);
    assert_eq!(quad_out.name(), "Quad");
    assert_eq!(quad_out.nb_vertices(), 4);
    assert_eq!(quad_out.nb_triangles(), 2);
    assert_eq!(quad_out.material_name(), "Textured");

    let red = scene.material(0);
    assert_eq!(red.name(), "Red");
    assert_eq!(red.diffuse(), [1.0, 0.0, 0.0]);
    assert_eq!(red.shininess(), 16.0_f32);

    let tex_mat = scene.material(1);
    assert_eq!(tex_mat.name(), "Textured");
    assert_eq!(tex_mat.texture_map(), "wall.png");
}
