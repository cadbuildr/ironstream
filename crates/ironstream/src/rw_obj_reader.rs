// FILE: rw_obj_reader.rs
// occt: RWObj_Reader, RWObj_MtlReader, RWObj_Material

// occt: RWObj_Material
#[derive(Clone, Debug, PartialEq)]
pub struct ObjMaterial {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    pub texture_path: Option<String>,
}

impl ObjMaterial {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.8, 0.8, 0.8],
            specular: [0.0, 0.0, 0.0],
            shininess: 0.0,
            transparency: 0.0,
            texture_path: None,
        }
    }

    pub fn with_diffuse(mut self, r: f32, g: f32, b: f32) -> Self {
        self.diffuse = [r, g, b]; self
    }

    pub fn with_transparency(mut self, t: f32) -> Self {
        self.transparency = t; self
    }

    pub fn is_transparent(&self) -> bool { self.transparency > 0.0 }
}

impl Default for ObjMaterial {
    fn default() -> Self { Self::new("default") }
}

// occt: RWObj_MtlReader
#[derive(Clone, Debug, Default)]
pub struct MtlReader {
    pub materials: Vec<ObjMaterial>,
}

impl MtlReader {
    pub fn new() -> Self { Self::default() }

    pub fn parse(&mut self, content: &str) {
        let mut current: Option<ObjMaterial> = None;
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("newmtl ") {
                if let Some(m) = current.take() { self.materials.push(m); }
                current = Some(ObjMaterial::new(line[7..].trim()));
            } else if let Some(ref mut m) = current {
                if line.starts_with("Kd ") {
                    let parts: Vec<f32> = line[3..].split_whitespace()
                        .filter_map(|s| s.parse().ok()).collect();
                    if parts.len() >= 3 { m.diffuse = [parts[0], parts[1], parts[2]]; }
                } else if line.starts_with("d ") {
                    if let Ok(t) = line[2..].trim().parse::<f32>() { m.transparency = 1.0 - t; }
                } else if line.starts_with("map_Kd ") {
                    m.texture_path = Some(line[7..].trim().to_string());
                }
            }
        }
        if let Some(m) = current { self.materials.push(m); }
    }

    pub fn find(&self, name: &str) -> Option<&ObjMaterial> {
        self.materials.iter().find(|m| m.name == name)
    }

    pub fn nb_materials(&self) -> usize { self.materials.len() }
}

// occt: RWObj_Reader
#[derive(Clone, Debug, Default)]
pub struct ObjReader {
    pub vertices: Vec<[f64; 3]>,
    pub normals: Vec<[f64; 3]>,
    pub uvs: Vec<[f64; 2]>,
    pub faces: Vec<ObjFace>,
    pub materials: Vec<ObjMaterial>,
    pub done: bool,
    pub nb_rejected_faces: usize,
}

#[derive(Clone, Debug)]
pub struct ObjFace {
    pub vertex_indices: Vec<usize>,
    pub normal_indices: Vec<usize>,
    pub uv_indices: Vec<usize>,
    pub material_name: Option<String>,
}

impl ObjReader {
    pub fn new() -> Self { Self::default() }

    pub fn parse(&mut self, content: &str) {
        let mut current_material: Option<String> = None;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if line.starts_with("v ") {
                let parts: Vec<f64> = line[2..].split_whitespace()
                    .filter_map(|s| s.parse().ok()).collect();
                if parts.len() >= 3 { self.vertices.push([parts[0], parts[1], parts[2]]); }
            } else if line.starts_with("vn ") {
                let parts: Vec<f64> = line[3..].split_whitespace()
                    .filter_map(|s| s.parse().ok()).collect();
                if parts.len() >= 3 { self.normals.push([parts[0], parts[1], parts[2]]); }
            } else if line.starts_with("vt ") {
                let parts: Vec<f64> = line[3..].split_whitespace()
                    .filter_map(|s| s.parse().ok()).collect();
                if parts.len() >= 2 { self.uvs.push([parts[0], parts[1]]); }
            } else if line.starts_with("usemtl ") {
                current_material = Some(line[7..].trim().to_string());
            } else if line.starts_with("f ") {
                let mut vi = Vec::new(); let mut ni = Vec::new(); let mut ui = Vec::new();
                for tok in line[2..].split_whitespace() {
                    let ids: Vec<&str> = tok.split('/').collect();
                    if let Some(v) = ids.first().and_then(|s| s.parse::<usize>().ok()) {
                        vi.push(v.saturating_sub(1));
                    }
                    if let Some(u) = ids.get(1).and_then(|s| s.parse::<usize>().ok()) {
                        ui.push(u.saturating_sub(1));
                    }
                    if let Some(n) = ids.get(2).and_then(|s| s.parse::<usize>().ok()) {
                        ni.push(n.saturating_sub(1));
                    }
                }
                if vi.len() >= 3 {
                    self.faces.push(ObjFace { vertex_indices: vi, normal_indices: ni, uv_indices: ui, material_name: current_material.clone() });
                } else {
                    self.nb_rejected_faces += 1;
                }
            }
        }
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn nb_faces(&self) -> usize { self.faces.len() }
    pub fn nb_normals(&self) -> usize { self.normals.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OBJ: &str = "# simple cube face\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvn 0 0 1\nf 1//1 2//1 3//1\nf 1//1 3//1 4//1\n";

    #[test]
    fn obj_reader_parse() {
        let mut r = ObjReader::new();
        r.parse(OBJ);
        assert!(r.is_done());
        assert_eq!(r.nb_vertices(), 4);
        assert_eq!(r.nb_faces(), 2);
    }

    #[test]
    fn obj_reader_normals() {
        let mut r = ObjReader::new();
        r.parse(OBJ);
        assert_eq!(r.nb_normals(), 1);
    }

    #[test]
    fn mtl_reader_parse() {
        let mtl = "newmtl Red\nKd 1 0 0\nnewmtl Blue\nKd 0 0 1\n";
        let mut r = MtlReader::new();
        r.parse(mtl);
        assert_eq!(r.nb_materials(), 2);
        let red = r.find("Red").unwrap();
        assert!((red.diffuse[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn material_transparent() {
        let m = ObjMaterial::new("glass").with_transparency(0.5);
        assert!(m.is_transparent());
    }

    #[test]
    fn obj_rejected_faces() {
        let mut r = ObjReader::new();
        r.parse("v 0 0 0\nv 1 0 0\nf 1 2\n");
        assert_eq!(r.nb_rejected_faces, 1);
    }
}
