use ironstream::rw_amf::*;

#[test]
fn amf_object_add_vertex_indices_and_volume_nb_triangles() {
    let mut obj = AmfObject::new(1, "cube");
    let i0 = obj.add_vertex([0.0, 0.0, 0.0]);
    let i1 = obj.add_vertex([1.0, 0.0, 0.0]);
    let i2 = obj.add_vertex([0.0, 1.0, 0.0]);
    let i3 = obj.add_vertex([0.0, 0.0, 1.0]);
    assert_eq!(i0, 0);
    assert_eq!(i3, 3);
    assert_eq!(obj.nb_vertices(), 4);

    let mut vol = AmfVolume::new();
    vol.add_triangle(i0, i1, i2);
    vol.add_triangle(i0, i1, i3);
    assert_eq!(vol.nb_triangles(), 2);
    obj.add_volume(vol);
    assert_eq!(obj.nb_triangles(), 2);
}

#[test]
fn amf_material_set_color_clamp() {
    let mut m = AmfMaterial::new(5, "Titanium");
    m.set_color(0.7, 0.3, 0.1, 1.0);
    assert!((m.color[0] - 0.7).abs() < 1e-10);
    // values out of [0,1] get clamped
    m.set_color(2.0, -0.5, 0.5, 1.5);
    assert!((m.color[0] - 1.0).abs() < 1e-10);
    assert!((m.color[1] - 0.0).abs() < 1e-10);
    assert!((m.color[3] - 1.0).abs() < 1e-10);
}

#[test]
fn amf_document_find_material_and_nb_objects() {
    let mut doc = AmfDocument::new("millimeter");
    doc.add_material(AmfMaterial::new(1, "Steel"));
    doc.add_material(AmfMaterial::new(2, "Plastic"));
    doc.add_object(AmfObject::new(10, "body"));
    assert_eq!(doc.nb_objects(), 1);
    assert_eq!(doc.nb_materials(), 2);
    assert!(doc.find_material(1).is_some());
    assert!(doc.find_material(99).is_none());
    assert_eq!(doc.find_material(2).unwrap().name, "Plastic");
}

#[test]
fn amf_instance_set_translation() {
    let mut inst = AmfInstance::new(42);
    inst.set_translation(5.0, -3.0, 10.0);
    assert!((inst.delta_x - 5.0).abs() < 1e-10);
    assert!((inst.delta_y + 3.0).abs() < 1e-10);
    assert!((inst.delta_z - 10.0).abs() < 1e-10);
}

#[test]
fn amf_document_read_stub() {
    let doc = AmfDocument::read_stub("model.amf");
    assert!(doc.is_some());
    assert_eq!(doc.unwrap().unit, "millimeter");
    assert!(AmfDocument::read_stub("").is_none());
}

#[test]
fn amf_document_add_instance_in_constellation() {
    let mut doc = AmfDocument::new("mm");
    let mut inst = AmfInstance::new(1);
    inst.set_translation(1.0, 2.0, 3.0);
    doc.add_instance(inst);
    assert_eq!(doc.constellation.len(), 1);
    assert!((doc.constellation[0].delta_z - 3.0).abs() < 1e-10);
}
