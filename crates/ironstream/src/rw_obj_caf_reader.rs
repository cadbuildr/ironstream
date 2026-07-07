// FILE: rw_obj_caf_reader.rs
// occt: RWObj_CafReader

//! The OBJ mesh reader into XDE document.
//! Faithful port of `RWObj_CafReader` (.hxx + .cxx): the class plugs an
//! `RWObj_TriangulationReader` context into the `RWMesh_CafReader`
//! pipeline and implements `RWObj_IShapeReceiver::BindNamedShape` to
//! record shape attributes and cache XCAF visualization materials by
//! their unique OBJ material name.
//!
//! Local plumbing: document/shape/material handles are small in-module
//! records; the reader context is a compact but genuine OBJ statement
//! scanner (comments, mtllib, o/g/usemtl, v/f with probe counters)
//! sufficient to drive every CafReader-owned behavior.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `TopoDS_Shape` produced by the reader context.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshShapeIdCaf {
    /// Unique id of the produced shape within one read.
    pub shape_index: u32,
}

/// Local stand-in for `RWObj_Material` (MTL definition).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RWObjMaterialCaf {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    pub diffuse_texture: String,
}

/// Local stand-in for `XCAFDoc_VisMaterial` (handle-managed).
#[derive(Debug, PartialEq)]
pub struct XcafVisMaterialCaf {
    pub raw_name: String,
    pub is_defined: bool,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    /// Image_Texture stand-in: texture path when non-empty in MTL.
    pub diffuse_texture: Option<String>,
}

pub type HandleVisMaterialCaf = Rc<XcafVisMaterialCaf>;

/// Local stand-in for `RWMesh_NodeAttributes`.
#[derive(Clone, Debug, Default)]
pub struct MeshNodeAttributesCaf {
    pub name: String,
    /// Style material assigned from the OBJ material map.
    pub style_material: Option<HandleVisMaterialCaf>,
}

/// The OBJ mesh reader into XDE document.
pub struct RWObjCafReader {
    /// `myObjMaterialMap`: material names are unique keys in OBJ.
    obj_material_map: HashMap<String, HandleVisMaterialCaf>,
    /// `myIsSinglePrecision`: FALSE by default.
    is_single_precision: bool,
    /// RWMesh_CafReader plumbing: attributes per shape.
    attrib_map: HashMap<MeshShapeIdCaf, MeshNodeAttributesCaf>,
    /// RWMesh_CafReader plumbing: root (free) shapes.
    root_shapes: Vec<MeshShapeIdCaf>,
    /// RWMesh_CafReader plumbing: metadata key/value pairs.
    metadata: Vec<(String, String)>,
    /// RWMesh_CafReader plumbing: external file references.
    external_files: Vec<String>,
    /// Memory limit in MiB (-1 = unlimited), forwarded to the context.
    memory_limit_mib: i64,
    /// Materials known from MTL libraries (fed to the context).
    known_materials: HashMap<String, RWObjMaterialCaf>,
    next_shape_index: u32,
}

impl Default for RWObjCafReader {
    fn default() -> Self {
        RWObjCafReader::new()
    }
}

impl RWObjCafReader {
    /// Empty constructor: input coordinate system defaults to glTF (Y-up).
    pub fn new() -> Self {
        RWObjCafReader {
            obj_material_map: HashMap::new(),
            is_single_precision: false,
            attrib_map: HashMap::new(),
            root_shapes: Vec::new(),
            metadata: Vec::new(),
            external_files: Vec::new(),
            memory_limit_mib: -1,
            known_materials: HashMap::new(),
            next_shape_index: 0,
        }
    }

    /// Return single precision flag; FALSE by default.
    pub fn is_single_precision(&self) -> bool {
        self.is_single_precision
    }

    /// Setup single/double precision flag for reading vertex data.
    pub fn set_single_precision(&mut self, is_single: bool) {
        self.is_single_precision = is_single;
    }

    /// SetMemoryLimitMiB plumbing from RWMesh_CafReader.
    pub fn set_memory_limit_mib(&mut self, limit: i64) {
        self.memory_limit_mib = limit;
    }

    /// Registers an MTL material as if parsed from a material library.
    pub fn define_material(&mut self, material: RWObjMaterialCaf) {
        self.known_materials.insert(material.name.clone(), material);
    }

    pub fn root_shapes(&self) -> &[MeshShapeIdCaf] {
        &self.root_shapes
    }

    pub fn attributes(&self, shape: &MeshShapeIdCaf) -> Option<&MeshNodeAttributesCaf> {
        self.attrib_map.get(shape)
    }

    pub fn metadata(&self) -> &[(String, String)] {
        &self.metadata
    }

    pub fn external_files(&self) -> &[String] {
        &self.external_files
    }

    /// `RWObj_IShapeReceiver::BindNamedShape` implementation.
    pub fn bind_named_shape(
        &mut self,
        shape: Option<&MeshShapeIdCaf>,
        name: &str,
        material: Option<&RWObjMaterialCaf>,
        is_root_shape: bool,
    ) {
        let shape = match shape {
            Some(s) => s.clone(),
            None => return, // null shape ignored
        };
        let mut attribs = MeshNodeAttributesCaf {
            name: name.to_string(),
            style_material: None,
        };
        if let Some(mat) = material {
            let handle = if let Some(existing) = self.obj_material_map.get(&mat.name) {
                existing.clone()
            } else {
                let vis = Rc::new(XcafVisMaterialCaf {
                    raw_name: mat.name.clone(),
                    is_defined: true,
                    ambient_color: mat.ambient_color,
                    diffuse_color: mat.diffuse_color,
                    specular_color: mat.specular_color,
                    shininess: mat.shininess,
                    transparency: mat.transparency,
                    diffuse_texture: if mat.diffuse_texture.is_empty() {
                        None
                    } else {
                        Some(mat.diffuse_texture.clone())
                    },
                });
                self.obj_material_map.insert(mat.name.clone(), vis.clone());
                vis
            };
            attribs.style_material = Some(handle);
        }
        self.attrib_map.insert(shape.clone(), attribs);
        if is_root_shape {
            self.root_shapes.push(shape);
        }
    }

    /// `performMesh`: runs the reader context over the document.
    /// Probe mode only counts nodes/elements and reads metadata.
    pub fn perform_mesh(&mut self, content: &str, to_probe: bool) -> bool {
        // Reader context state (RWObj_TriangulationReader essentials).
        let mut comments = String::new();
        let mut ctx_external: Vec<String> = Vec::new();
        let mut nb_probe_nodes = 0usize;
        let mut nb_probe_elems = 0usize;
        let mut is_start = true;
        let mut active_object = String::new();
        let mut active_material = String::new();
        let mut has_elems_in_object = false;
        let mut is_done = !content.is_empty();

        let mut flush_object =
            |reader: &mut RWObjCafReader,
             active_object: &str,
             active_material: &str,
             has_elems: bool,
             next_index: &mut u32| {
                if !has_elems {
                    return;
                }
                let face = MeshShapeIdCaf { shape_index: *next_index };
                *next_index += 1;
                let mat = reader.known_materials.get(active_material).cloned();
                reader.bind_named_shape(Some(&face), "", mat.as_ref(), false);
                let root = MeshShapeIdCaf { shape_index: *next_index };
                *next_index += 1;
                reader.bind_named_shape(Some(&root), active_object, None, true);
            };

        let mut next_index = self.next_shape_index;
        for line in content.split('\n') {
            if let Some(comment) = line.strip_prefix('#') {
                if is_start {
                    let c = comment.trim();
                    if !c.is_empty() {
                        if !comments.is_empty() {
                            comments.push('\n');
                        }
                        comments.push_str(c);
                    }
                }
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }
            is_start = false;
            let b = line.as_bytes();
            if line.starts_with("mtllib") {
                let path = line[6..].trim();
                if !path.is_empty() && !ctx_external.contains(&path.to_string()) {
                    ctx_external.push(path.to_string());
                }
            } else if b[0] == b'v' && b.len() > 1 && (b[1] == b' ' || b[1] == b'\t') {
                nb_probe_nodes += 1;
            } else if b[0] == b'f' && b.len() > 1 && (b[1] == b' ' || b[1] == b'\t') {
                nb_probe_elems += 1;
                if !to_probe {
                    has_elems_in_object = true;
                }
            } else if !to_probe && b[0] == b'o' && b.len() > 1 && (b[1] as char).is_whitespace() {
                flush_object(
                    self,
                    &active_object,
                    &active_material,
                    has_elems_in_object,
                    &mut next_index,
                );
                has_elems_in_object = false;
                active_object = line[2..].trim().to_string();
            } else if !to_probe && line.starts_with("usemtl") {
                let mat = line[6..].trim().to_string();
                if mat.is_empty() || self.known_materials.contains_key(&mat) {
                    active_material = mat;
                }
            }
        }
        if !to_probe {
            flush_object(
                self,
                &active_object,
                &active_material,
                has_elems_in_object,
                &mut next_index,
            );
        }
        self.next_shape_index = next_index;

        if to_probe && nb_probe_nodes == 0 && nb_probe_elems == 0 && comments.is_empty() {
            is_done = false;
        }

        // performMesh tail: metadata and external files.
        if !comments.is_empty() {
            self.metadata.push(("Comments".to_string(), comments));
        }
        for f in ctx_external {
            if !self.external_files.contains(&f) {
                self.external_files.push(f);
            }
        }
        is_done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steel() -> RWObjMaterialCaf {
        RWObjMaterialCaf {
            name: "steel".into(),
            diffuse_color: [0.7, 0.7, 0.75],
            shininess: 0.5,
            diffuse_texture: "steel.png".into(),
            ..RWObjMaterialCaf::default()
        }
    }

    #[test]
    fn single_precision_defaults_false() {
        let mut reader = RWObjCafReader::new();
        assert!(!reader.is_single_precision());
        reader.set_single_precision(true);
        assert!(reader.is_single_precision());
    }

    #[test]
    fn bind_named_shape_caches_material_by_name() {
        let mut reader = RWObjCafReader::new();
        let s1 = MeshShapeIdCaf { shape_index: 1 };
        let s2 = MeshShapeIdCaf { shape_index: 2 };
        let mat = steel();
        reader.bind_named_shape(Some(&s1), "face1", Some(&mat), false);
        reader.bind_named_shape(Some(&s2), "face2", Some(&mat), false);
        let m1 = reader.attributes(&s1).unwrap().style_material.clone().unwrap();
        let m2 = reader.attributes(&s2).unwrap().style_material.clone().unwrap();
        assert!(Rc::ptr_eq(&m1, &m2), "material handle reused via myObjMaterialMap");
        assert_eq!(m1.raw_name, "steel");
        assert!(m1.is_defined);
        assert_eq!(m1.diffuse_texture.as_deref(), Some("steel.png"));
    }

    #[test]
    fn null_shape_is_ignored() {
        let mut reader = RWObjCafReader::new();
        reader.bind_named_shape(None, "ghost", None, true);
        assert!(reader.root_shapes().is_empty());
    }

    #[test]
    fn root_shapes_are_appended() {
        let mut reader = RWObjCafReader::new();
        let root = MeshShapeIdCaf { shape_index: 10 };
        reader.bind_named_shape(Some(&root), "assembly", None, true);
        assert_eq!(reader.root_shapes(), &[root.clone()]);
        assert_eq!(reader.attributes(&root).unwrap().name, "assembly");
    }

    #[test]
    fn perform_mesh_collects_comments_and_external_files() {
        let mut reader = RWObjCafReader::new();
        let obj = "# made by ironstream\n# unit: mm\nmtllib parts.mtl\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        assert!(reader.perform_mesh(obj, false));
        assert_eq!(
            reader.metadata(),
            &[("Comments".to_string(), "made by ironstream\nunit: mm".to_string())]
        );
        assert_eq!(reader.external_files(), &["parts.mtl".to_string()]);
        // One face + one root object shape were bound.
        assert_eq!(reader.root_shapes().len(), 1);
    }

    #[test]
    fn probe_mode_binds_no_shapes() {
        let mut reader = RWObjCafReader::new();
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        assert!(reader.perform_mesh(obj, true));
        assert!(reader.root_shapes().is_empty());
        assert!(reader.metadata().is_empty());
    }

    #[test]
    fn named_object_with_material() {
        let mut reader = RWObjCafReader::new();
        reader.define_material(steel());
        let obj = "o bracket\nusemtl steel\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        assert!(reader.perform_mesh(obj, false));
        assert_eq!(reader.root_shapes().len(), 1);
        let root = &reader.root_shapes()[0].clone();
        assert_eq!(reader.attributes(root).unwrap().name, "bracket");
        // The face shape (bound before the root) carries the material.
        let face_attribs: Vec<_> = reader
            .attrib_map
            .values()
            .filter(|a| a.style_material.is_some())
            .collect();
        assert_eq!(face_attribs.len(), 1);
        assert_eq!(
            face_attribs[0].style_material.as_ref().unwrap().raw_name,
            "steel"
        );
    }

    #[test]
    fn empty_document_fails() {
        let mut reader = RWObjCafReader::new();
        assert!(!reader.perform_mesh("", false));
    }
}
