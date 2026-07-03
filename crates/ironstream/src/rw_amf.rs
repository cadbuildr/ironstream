// FILE: rw_amf.rs
// occt: RWAmf (Additive Manufacturing Format reader/writer stubs)

/// AMF constellation instance (positioned object).
#[derive(Clone, Debug)]
pub struct AmfInstance {
    pub object_id: u32,
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}

impl Default for AmfInstance {
    fn default() -> Self {
        Self { object_id: 0, delta_x: 0.0, delta_y: 0.0, delta_z: 0.0, rx: 0.0, ry: 0.0, rz: 0.0 }
    }
}

impl AmfInstance {
    pub fn new(object_id: u32) -> Self { Self { object_id, ..Default::default() } }

    pub fn set_translation(&mut self, dx: f64, dy: f64, dz: f64) {
        self.delta_x = dx; self.delta_y = dy; self.delta_z = dz;
    }

    pub fn set_rotation(&mut self, rx: f64, ry: f64, rz: f64) {
        self.rx = rx; self.ry = ry; self.rz = rz;
    }
}

/// AMF material definition.
#[derive(Clone, Debug)]
pub struct AmfMaterial {
    pub id: u32,
    pub name: String,
    pub color: [f64; 4],    // RGBA
    pub density: f64,
}

impl Default for AmfMaterial {
    fn default() -> Self {
        Self { id: 0, name: String::new(), color: [0.8, 0.8, 0.8, 1.0], density: 1.0 }
    }
}

impl AmfMaterial {
    pub fn new(id: u32, name: &str) -> Self {
        Self { id, name: name.to_string(), ..Default::default() }
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.color = [r.clamp(0.0,1.0), g.clamp(0.0,1.0), b.clamp(0.0,1.0), a.clamp(0.0,1.0)];
    }
}

/// AMF volume (triangulated solid region with material reference).
#[derive(Clone, Debug)]
pub struct AmfVolume {
    pub material_id: Option<u32>,
    pub triangles: Vec<[u32; 3]>,   // vertex indices
}

impl Default for AmfVolume {
    fn default() -> Self { Self { material_id: None, triangles: Vec::new() } }
}

impl AmfVolume {
    pub fn new() -> Self { Self::default() }

    pub fn set_material(&mut self, id: u32) { self.material_id = Some(id); }

    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.triangles.push([a, b, c]);
    }

    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
}

/// An AMF object (mesh + volumes).
/// occt: RWAmf (object representation)
#[derive(Clone, Debug)]
pub struct AmfObject {
    pub id: u32,
    pub name: String,
    pub vertices: Vec<[f64; 3]>,
    pub volumes: Vec<AmfVolume>,
}

impl AmfObject {
    pub fn new(id: u32, name: &str) -> Self {
        Self { id, name: name.to_string(), vertices: Vec::new(), volumes: Vec::new() }
    }

    pub fn add_vertex(&mut self, p: [f64; 3]) -> u32 {
        let idx = self.vertices.len() as u32;
        self.vertices.push(p);
        idx
    }

    pub fn add_volume(&mut self, v: AmfVolume) { self.volumes.push(v); }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn nb_volumes(&self) -> usize { self.volumes.len() }
    pub fn nb_triangles(&self) -> usize { self.volumes.iter().map(|v| v.nb_triangles()).sum() }
}

/// AMF document (top-level container).
/// occt: RWAmf_Reader / RWAmf_Writer target model
#[derive(Clone, Debug, Default)]
pub struct AmfDocument {
    pub unit: String,
    pub objects: Vec<AmfObject>,
    pub materials: Vec<AmfMaterial>,
    pub constellation: Vec<AmfInstance>,
}

impl AmfDocument {
    pub fn new(unit: &str) -> Self {
        Self { unit: unit.to_string(), ..Default::default() }
    }

    pub fn add_object(&mut self, obj: AmfObject) { self.objects.push(obj); }
    pub fn add_material(&mut self, mat: AmfMaterial) { self.materials.push(mat); }
    pub fn add_instance(&mut self, inst: AmfInstance) { self.constellation.push(inst); }

    pub fn find_material(&self, id: u32) -> Option<&AmfMaterial> {
        self.materials.iter().find(|m| m.id == id)
    }

    pub fn nb_objects(&self) -> usize { self.objects.len() }
    pub fn nb_materials(&self) -> usize { self.materials.len() }

    /// Stub read: populate from a path string (does nothing, returns true if path non-empty).
    pub fn read_stub(path: &str) -> Option<Self> {
        if path.is_empty() { return None; }
        Some(Self::new("millimeter"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amf_object_add_vertices() {
        let mut obj = AmfObject::new(1, "part");
        let i0 = obj.add_vertex([0.0, 0.0, 0.0]);
        let i1 = obj.add_vertex([1.0, 0.0, 0.0]);
        let i2 = obj.add_vertex([0.0, 1.0, 0.0]);
        assert_eq!(i0, 0);
        assert_eq!(i2, 2);
        assert_eq!(obj.nb_vertices(), 3);

        let mut vol = AmfVolume::new();
        vol.add_triangle(i0, i1, i2);
        assert_eq!(vol.nb_triangles(), 1);
        obj.add_volume(vol);
        assert_eq!(obj.nb_triangles(), 1);
    }

    #[test]
    fn amf_material_color() {
        let mut m = AmfMaterial::new(1, "Steel");
        m.set_color(0.5, 0.5, 0.5, 1.0);
        assert!((m.color[0] - 0.5).abs() < 1e-10);
        m.set_color(2.0, -1.0, 0.5, 0.5); // clamp
        assert!((m.color[0] - 1.0).abs() < 1e-10);
        assert!((m.color[1]).abs() < 1e-10);
    }

    #[test]
    fn amf_document_find_material() {
        let mut doc = AmfDocument::new("mm");
        doc.add_material(AmfMaterial::new(1, "Plastic"));
        doc.add_material(AmfMaterial::new(2, "Metal"));
        assert!(doc.find_material(1).is_some());
        assert!(doc.find_material(99).is_none());
    }

    #[test]
    fn amf_instance_translation() {
        let mut inst = AmfInstance::new(1);
        inst.set_translation(10.0, 20.0, 30.0);
        assert!((inst.delta_x - 10.0).abs() < 1e-10);
    }

    #[test]
    fn amf_read_stub() {
        assert!(AmfDocument::read_stub("model.amf").is_some());
        assert!(AmfDocument::read_stub("").is_none());
    }
}
