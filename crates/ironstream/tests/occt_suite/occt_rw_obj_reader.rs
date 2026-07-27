extern crate ironstream;
use ironstream::rw_obj_reader::*;

const SIMPLE_OBJ: &str = "\
# a simple square\n\
v 0 0 0\n\
v 1 0 0\n\
v 1 1 0\n\
v 0 1 0\n\
vn 0 0 1\n\
f 1//1 2//1 3//1\n\
f 1//1 3//1 4//1\n\
";

#[test]
fn obj_reader_vertices_and_faces() {
    let mut r = ObjReader::new();
    r.parse(SIMPLE_OBJ);
    assert!(r.is_done());
    assert_eq!(r.nb_vertices(), 4);
    assert_eq!(r.nb_faces(), 2);
    assert_eq!(r.nb_normals(), 1);
}

#[test]
fn obj_reader_rejected_face() {
    let mut r = ObjReader::new();
    r.parse("v 0 0 0\nv 1 0 0\nf 1 2\n");
    assert_eq!(r.nb_rejected_faces, 1);
    assert_eq!(r.nb_faces(), 0);
}

#[test]
fn mtl_reader_materials() {
    let mtl = "newmtl Red\nKd 1 0 0\nnewmtl Blue\nKd 0 0 1\n";
    let mut r = MtlReader::new();
    r.parse(mtl);
    assert_eq!(r.nb_materials(), 2);
    let red = r.find("Red").unwrap();
    assert!((red.diffuse[0] - 1.0).abs() < 1e-6);
    assert_eq!(r.find("Green"), None);
}

#[test]
fn obj_material_transparency() {
    let m = ObjMaterial::new("glass")
        .with_diffuse(0.5, 0.5, 0.5)
        .with_transparency(0.8);
    assert!(m.is_transparent());
    assert!((m.diffuse[0] - 0.5).abs() < 1e-6);
}

#[test]
fn obj_material_default_opaque() {
    let m = ObjMaterial::default();
    assert!(!m.is_transparent());
    assert_eq!(m.name, "default");
}
