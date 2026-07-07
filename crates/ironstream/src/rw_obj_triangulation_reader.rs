// FILE: rw_obj_triangulation_reader.rs
// occt: RWObj_TriangulationReader

//! `RWObj_Reader` implementation dumping OBJ file into Poly_Triangulation,
//! faithful port of RWObj_TriangulationReader.hxx/.cxx together with the
//! inherited RWObj_Reader parsing loop it completes (v/vn/vt/f/g/s/o/
//! usemtl/mtllib statements, packed-index vertex dedup, negative index
//! resolution, quad splitting, group/object sub-mesh flushing).
//!
//! Local plumbing: gp_Pnt/Poly_Triangulation/TopoDS shapes are modeled by
//! small in-module types (a triangulated face and nested compounds);
//! the RWObj_IShapeReceiver callback is modeled by an internal binding log.
//! Polygon splitting for >4-node faces uses the fan triangulation
//! (`triangulatePolygonFan`), which is the documented fallback of
//! `triangulatePolygon` in the C++ code (the Delaunay path is a mesh-kernel
//! optimization producing an equivalent cover for convex polygons).

use std::collections::HashMap;
use std::rc::Rc;

/// strtol-like integer scan; returns (value, bytes consumed).
fn objtr_strtol_scan(input: &str) -> (i64, usize) {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() && (bytes[i] as char).is_whitespace() {
        i += 1;
    }
    let start = i;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let digits_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits_start {
        return (0, 0);
    }
    (input[start..i].parse().unwrap_or(0), i)
}

/// strtod-like float scan; returns (value, bytes consumed).
fn objtr_strtod_scan(input: &str) -> (f64, usize) {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() && (bytes[i] as char).is_whitespace() {
        i += 1;
    }
    let start = i;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut saw_digit = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
        saw_digit = true;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
            saw_digit = true;
        }
    }
    if !saw_digit {
        return (0.0, 0);
    }
    let mut end = i;
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let mut j = i + 1;
        if j < bytes.len() && (bytes[j] == b'+' || bytes[j] == b'-') {
            j += 1;
        }
        let ds = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j > ds {
            end = j;
        }
    }
    (input[start..end].parse().unwrap_or(0.0), end)
}

/// RWObj_Tools::ReadName equivalent.
fn objtr_read_name(pos: &str) -> String {
    pos.trim_end_matches('\n')
        .trim_end_matches('\r')
        .trim_matches(|c: char| c.is_whitespace())
        .to_string()
}

/// Local stand-in for `RWObj_Material` (what usemtl resolution needs).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RWObjMaterialTr {
    pub name: String,
    pub diffuse_color: [f32; 3],
    pub diffuse_texture: String,
    pub specular_texture: String,
    pub bump_texture: String,
}

/// Local stand-in for `Poly_Triangulation` built by GetTriangulation.
#[derive(Debug, PartialEq)]
pub struct PolyTriangulationTr {
    pub nodes: Vec<[f64; 3]>,
    /// UV per node when defined.
    pub uv_nodes: Option<Vec<[f32; 2]>>,
    /// Normal per node when defined.
    pub normals: Option<Vec<[f32; 3]>>,
    /// 1-based node indices.
    pub triangles: Vec<[i32; 3]>,
}

/// Local shape model (TopoDS_Face carrying a triangulation, or a compound).
#[derive(Clone, Debug)]
pub enum ObjShapeTr {
    Face(Rc<PolyTriangulationTr>),
    Compound(Vec<ObjShapeTr>),
}

impl ObjShapeTr {
    pub fn is_face(&self) -> bool {
        matches!(self, ObjShapeTr::Face(_))
    }

    pub fn nb_children(&self) -> usize {
        match self {
            ObjShapeTr::Face(_) => 0,
            ObjShapeTr::Compound(v) => v.len(),
        }
    }

    /// Count faces recursively (test helper).
    pub fn total_faces(&self) -> usize {
        match self {
            ObjShapeTr::Face(_) => 1,
            ObjShapeTr::Compound(v) => v.iter().map(|s| s.total_faces()).sum(),
        }
    }
}

/// One record of RWObj_IShapeReceiver::BindNamedShape.
#[derive(Clone, Debug)]
pub struct BoundShapeRecTr {
    pub name: String,
    pub material_name: Option<String>,
    pub is_root_shape: bool,
    pub is_face: bool,
}

/// Active sub-mesh context (RWObj_SubMesh fields).
#[derive(Clone, Debug, Default, PartialEq)]
struct ActiveSubMeshTr {
    object: String,
    group: String,
    smooth_group: String,
    material: String,
}

/// Sub-mesh split reason (RWObj_SubMeshReason).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SubMeshReasonTr {
    NewObject,
    NewGroup,
    NewMaterial,
    NewSmoothGroup,
}

/// RWObj_Reader + RWObj_TriangulationReader state.
pub struct RWObjTriangulationReader {
    // ---- RWObj_Reader (base) ----
    obj_verts: Vec<[f64; 3]>,
    obj_verts_uv: Vec<[f32; 2]>,
    obj_norms: Vec<[f32; 3]>,
    packed_indices: HashMap<[i32; 3], i32>,
    materials: HashMap<String, RWObjMaterialTr>,
    file_comments: String,
    external_files: Vec<String>,
    active_sub_mesh: ActiveSubMeshTr,
    nb_lines: usize,
    nb_probe_nodes: usize,
    nb_probe_elems: usize,
    nb_elems_big: usize,
    to_abort: bool,
    warnings: Vec<String>,
    // ---- RWObj_TriangulationReader ----
    nodes: Vec<[f64; 3]>,
    normals: Vec<[f32; 3]>,
    nodes_uv: Vec<[f32; 2]>,
    triangles: Vec<[i32; 3]>,
    to_create_shapes: bool,
    receiver_enabled: bool,
    bound_shapes: Vec<BoundShapeRecTr>,
    result_shape: Option<ObjShapeTr>,
    last_object_shape: Option<ObjShapeTr>,
    last_group_shape: Option<ObjShapeTr>,
    last_group_name: String,
    last_face_material: String,
}

impl Default for RWObjTriangulationReader {
    fn default() -> Self {
        RWObjTriangulationReader::new()
    }
}

impl RWObjTriangulationReader {
    pub fn new() -> Self {
        RWObjTriangulationReader {
            obj_verts: Vec::new(),
            obj_verts_uv: Vec::new(),
            obj_norms: Vec::new(),
            packed_indices: HashMap::new(),
            materials: HashMap::new(),
            file_comments: String::new(),
            external_files: Vec::new(),
            active_sub_mesh: ActiveSubMeshTr::default(),
            nb_lines: 0,
            nb_probe_nodes: 0,
            nb_probe_elems: 0,
            nb_elems_big: 0,
            to_abort: false,
            warnings: Vec::new(),
            nodes: Vec::new(),
            normals: Vec::new(),
            nodes_uv: Vec::new(),
            triangles: Vec::new(),
            to_create_shapes: true,
            receiver_enabled: false,
            bound_shapes: Vec::new(),
            result_shape: None,
            last_object_shape: None,
            last_group_shape: None,
            last_group_name: String::new(),
            last_face_material: String::new(),
        }
    }

    /// SetCreateShapes.
    pub fn set_create_shapes(&mut self, to_create: bool) {
        self.to_create_shapes = to_create;
    }

    /// SetShapeReceiver (modeled: enables the binding log).
    pub fn set_shape_receiver(&mut self, enabled: bool) {
        self.receiver_enabled = enabled;
    }

    /// Bindings received so far (RWObj_IShapeReceiver log).
    pub fn bound_shapes(&self) -> &[BoundShapeRecTr] {
        &self.bound_shapes
    }

    /// Pre-registers an MTL material definition (models RWObj_MtlReader
    /// output that `readMaterialLib` would fill in the base reader).
    pub fn define_material(&mut self, material: RWObjMaterialTr) {
        self.materials.insert(material.name.clone(), material);
    }

    /// File header comments collected from leading `#` lines.
    pub fn file_comments(&self) -> &str {
        &self.file_comments
    }

    /// External file references (mtllib + textures).
    pub fn external_files(&self) -> &[String] {
        &self.external_files
    }

    pub fn nb_probe_nodes(&self) -> usize {
        self.nb_probe_nodes
    }

    pub fn nb_probe_elems(&self) -> usize {
        self.nb_probe_elems
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    // ---------------- RWObj_Reader::read ----------------

    /// RWObj_Reader::Read over an in-memory document.
    pub fn read(&mut self, content: &str) -> bool {
        self.nb_lines = 0;
        self.nb_probe_nodes = 0;
        self.nb_probe_elems = 0;
        self.nb_elems_big = 0;
        self.to_abort = false;
        self.obj_verts.clear();
        self.obj_verts_uv.clear();
        self.obj_norms.clear();
        self.packed_indices.clear();
        self.file_comments.clear();
        self.external_files.clear();
        self.active_sub_mesh = ActiveSubMeshTr::default();

        if content.is_empty() {
            return false;
        }

        let mut is_start = true;
        let lines: Vec<&str> = content.split('\n').collect();
        for line in lines {
            self.nb_lines += 1;
            if let Some(comment) = line.strip_prefix('#') {
                if is_start {
                    let c = comment.trim();
                    if !c.is_empty() {
                        if !self.file_comments.is_empty() {
                            self.file_comments.push('\n');
                        }
                        self.file_comments.push_str(c);
                    }
                }
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }
            is_start = false;

            let b = line.as_bytes();
            if b[0] == b'v' && b.len() > 1 && (b[1] == b' ' || b[1] == b'\t') {
                self.nb_probe_nodes += 1;
                self.push_vertex(&line[2..]);
            } else if b[0] == b'v' && b.len() > 2 && b[1] == b'n' && (b[2] == b' ' || b[2] == b'\t')
            {
                self.push_normal(&line[3..]);
            } else if b[0] == b'v' && b.len() > 2 && b[1] == b't' && (b[2] == b' ' || b[2] == b'\t')
            {
                self.push_texel(&line[3..]);
            } else if b[0] == b'f' && b.len() > 1 && (b[1] == b' ' || b[1] == b'\t') {
                self.nb_probe_elems += 1;
                self.push_indices(&line[2..]);
                if self.to_abort {
                    self.add_mesh_active(SubMeshReasonTr::NewObject);
                    return false;
                }
            } else if b[0] == b'g' && b.len() > 1 && (b[1] as char).is_whitespace() {
                self.push_group(&line[2..]);
            } else if b[0] == b's' && b.len() > 1 && (b[1] as char).is_whitespace() {
                self.push_smooth_group(&line[2..]);
            } else if b[0] == b'o' && b.len() > 1 && (b[1] as char).is_whitespace() {
                self.push_object(&line[2..]);
            } else if line.starts_with("mtllib") {
                let arg = if line.len() > 6 { &line[7.min(line.len())..] } else { "" };
                self.read_material_lib(arg);
            } else if line.starts_with("usemtl") {
                let arg = if line.len() > 6 { &line[7.min(line.len())..] } else { "" };
                self.push_material(arg);
            }
        }

        // Collect external texture references from materials.
        let mut texture_files: Vec<String> = Vec::new();
        for mat in self.materials.values() {
            for tex in [&mat.diffuse_texture, &mat.specular_texture, &mat.bump_texture] {
                if !tex.is_empty() && !texture_files.contains(tex) {
                    texture_files.push(tex.clone());
                }
            }
        }
        for tex in texture_files {
            if !self.external_files.contains(&tex) {
                self.external_files.push(tex);
            }
        }

        // Flush the last group.
        self.add_mesh_active(SubMeshReasonTr::NewObject);
        true
    }

    fn push_vertex(&mut self, s: &str) {
        let (x, n1) = objtr_strtod_scan(s);
        let (y, n2) = objtr_strtod_scan(&s[n1..]);
        let (z, _n3) = objtr_strtod_scan(&s[n1 + n2..]);
        self.obj_verts.push([x, y, z]);
    }

    fn push_normal(&mut self, s: &str) {
        let (x, n1) = objtr_strtod_scan(s);
        let (y, n2) = objtr_strtod_scan(&s[n1..]);
        let (z, _n3) = objtr_strtod_scan(&s[n1 + n2..]);
        self.obj_norms.push([x as f32, y as f32, z as f32]);
    }

    fn push_texel(&mut self, s: &str) {
        let (u, n1) = objtr_strtod_scan(s);
        let (v, _n2) = objtr_strtod_scan(&s[n1..]);
        self.obj_verts_uv.push([u as f32, v as f32]);
    }

    /// RWObj_Reader::pushIndices — the heart of face parsing.
    fn push_indices(&mut self, s: &str) {
        let mut pos = s;
        let mut curr_elem: Vec<i32> = Vec::new();
        loop {
            let mut tri = [-1i32; 3]; // vertex / uv / normal (0-based)
            let (v, n) = objtr_strtol_scan(pos);
            if n == 0 {
                break;
            }
            tri[0] = (v - 1) as i32;
            pos = &pos[n..];
            if pos.starts_with('/') {
                pos = &pos[1..];
                if !pos.starts_with('/') {
                    let (uv, n2) = objtr_strtol_scan(pos);
                    if n2 > 0 {
                        tri[1] = (uv - 1) as i32;
                        pos = &pos[n2..];
                    }
                }
                if pos.starts_with('/') {
                    pos = &pos[1..];
                    if !pos.is_empty() && !pos.starts_with(char::is_whitespace) {
                        let (nn, n3) = objtr_strtol_scan(pos);
                        if n3 > 0 {
                            tri[2] = (nn - 1) as i32;
                            pos = &pos[n3..];
                        }
                    }
                }
            }
            // Handle negative (relative) indices.
            if tri[0] < -1 {
                tri[0] += self.obj_verts.len() as i32 + 1;
            }
            if tri[1] < -1 {
                tri[1] += self.obj_verts_uv.len() as i32 + 1;
            }
            if tri[2] < -1 {
                tri[2] += self.obj_norms.len() as i32 + 1;
            }

            let index = if let Some(&i) = self.packed_indices.get(&tri) {
                i
            } else {
                if tri[0] < 0 || tri[0] >= self.obj_verts.len() as i32 {
                    self.to_abort = true;
                    self.warnings.push(format!(
                        "Error: invalid OBJ syntax at line {}: vertex index is out of range",
                        self.nb_lines
                    ));
                    return;
                }
                let idx = self.add_node(self.obj_verts[tri[0] as usize]);
                self.packed_indices.insert(tri, idx);
                if tri[1] >= 0 {
                    if self.obj_verts_uv.is_empty() {
                        self.warnings.push(format!(
                            "Warning: invalid OBJ syntax at line {}: UV index is specified but no UV nodes are defined",
                            self.nb_lines
                        ));
                    } else if tri[1] >= self.obj_verts_uv.len() as i32 {
                        self.warnings.push(format!(
                            "Warning: invalid OBJ syntax at line {}: UV index is out of range",
                            self.nb_lines
                        ));
                        self.set_node_uv(idx, [0.0, 0.0]);
                    } else {
                        self.set_node_uv(idx, self.obj_verts_uv[tri[1] as usize]);
                    }
                }
                if tri[2] >= 0 {
                    if self.obj_norms.is_empty() {
                        self.warnings.push(format!(
                            "Warning: invalid OBJ syntax at line {}: Normal index is specified but no Normals nodes are defined",
                            self.nb_lines
                        ));
                    } else if tri[2] >= self.obj_norms.len() as i32 {
                        self.warnings.push(format!(
                            "Warning: invalid OBJ syntax at line {}: Normal index is out of range",
                            self.nb_lines
                        ));
                        self.set_node_normal(idx, [0.0, 0.0, 1.0]);
                    } else {
                        self.set_node_normal(idx, self.obj_norms[tri[2] as usize]);
                    }
                }
                idx
            };
            curr_elem.push(index);

            if pos.is_empty() || pos.starts_with('\n') {
                break;
            }
            if !pos.starts_with(' ') && !pos.starts_with('\t') {
                pos = &pos[1..];
            }
        }

        if curr_elem.len() < 3 {
            return;
        }
        if curr_elem.len() == 3 {
            self.add_element(curr_elem[0], curr_elem[1], curr_elem[2], -1);
        } else if curr_elem.len() == 4 {
            self.add_element(curr_elem[0], curr_elem[1], curr_elem[2], curr_elem[3]);
        } else {
            let added = self.triangulate_polygon_fan(&curr_elem);
            if added < 1 {
                return;
            }
            self.nb_elems_big += 1;
        }
    }

    /// RWObj_Reader::triangulatePolygonFan.
    fn triangulate_polygon_fan(&mut self, indices: &[i32]) -> i32 {
        let n = indices.len() as i32;
        for i in 0..(n - 2) as usize {
            self.add_element(indices[0], indices[i + 1], indices[i + 2], -1);
        }
        n - 2
    }

    fn push_object(&mut self, name: &str) {
        let new_object = objtr_read_name(name);
        if self.add_mesh_active(SubMeshReasonTr::NewObject) {
            self.packed_indices.clear();
        }
        self.active_sub_mesh.object = new_object;
    }

    fn push_group(&mut self, name: &str) {
        let new_group = objtr_read_name(name);
        if self.add_mesh_active(SubMeshReasonTr::NewGroup) {
            self.packed_indices.clear();
        }
        self.active_sub_mesh.group = new_group;
    }

    fn push_smooth_group(&mut self, arg: &str) {
        let mut new_sg = objtr_read_name(arg);
        if new_sg == "off" || new_sg == "0" {
            new_sg.clear();
        }
        if self.active_sub_mesh.smooth_group == new_sg {
            // Duplicated statements are ignored (weird OBJ files workaround).
            return;
        }
        if self.add_mesh_active(SubMeshReasonTr::NewSmoothGroup) {
            self.packed_indices.clear();
        }
        self.active_sub_mesh.smooth_group = new_sg;
    }

    fn push_material(&mut self, name: &str) {
        let new_mat = objtr_read_name(name);
        if !new_mat.is_empty() && !self.materials.contains_key(&new_mat) {
            self.warnings.push(format!(
                "Warning: use of undefined OBJ material at line {}",
                self.nb_lines
            ));
            return;
        }
        if self.active_sub_mesh.material == new_mat {
            return; // ignore
        }
        if self.add_mesh_active(SubMeshReasonTr::NewMaterial) {
            self.packed_indices.clear();
        }
        self.active_sub_mesh.material = new_mat;
    }

    fn read_material_lib(&mut self, arg: &str) {
        let path = objtr_read_name(arg);
        if path.is_empty() {
            self.warnings
                .push(format!("Warning: invalid OBJ syntax at line {}", self.nb_lines));
            return;
        }
        if !self.external_files.contains(&path) {
            self.external_files.push(path);
        }
    }

    // ---------------- triangulation accumulation ----------------

    /// addNode: appends and returns the 1-based running index.
    fn add_node(&mut self, pnt: [f64; 3]) -> i32 {
        self.nodes.push(pnt);
        self.nodes.len() as i32
    }

    fn set_node_normal(&mut self, index: i32, normal: [f32; 3]) {
        let slot = (index - 1) as usize;
        if self.normals.len() <= slot {
            self.normals.resize(slot + 1, [0.0; 3]);
        }
        self.normals[slot] = normal;
    }

    fn set_node_uv(&mut self, index: i32, uv: [f32; 2]) {
        let slot = (index - 1) as usize;
        if self.nodes_uv.len() <= slot {
            self.nodes_uv.resize(slot + 1, [0.0; 2]);
        }
        self.nodes_uv[slot] = uv;
    }

    /// addElement: triangle or quad (quad split into two triangles).
    fn add_element(&mut self, n1: i32, n2: i32, n3: i32, n4: i32) {
        self.triangles.push([n1, n2, n3]);
        if n4 != -1 {
            self.triangles.push([n1, n3, n4]);
        }
    }

    /// RWObj_TriangulationReader::GetTriangulation.
    pub fn get_triangulation(&mut self) -> Option<Rc<PolyTriangulationTr>> {
        if self.triangles.is_empty() {
            return None;
        }
        let has_normals = self.nodes.len() == self.normals.len();
        let has_uv = self.nodes.len() == self.nodes_uv.len();

        let uv_nodes = if has_uv { Some(self.nodes_uv.clone()) } else { None };
        let normals = if has_normals {
            let mut nb_invalid = 0;
            let mut out = Vec::with_capacity(self.normals.len());
            for n in &self.normals {
                let mod2 = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
                if mod2 > 0.001 {
                    out.push(*n);
                } else {
                    nb_invalid += 1;
                    out.push([0.0, 0.0, 1.0]);
                }
            }
            if nb_invalid == self.nodes.len() {
                None // all invalid -> RemoveNormals
            } else {
                Some(out)
            }
        } else {
            None
        };

        Some(Rc::new(PolyTriangulationTr {
            nodes: self.nodes.clone(),
            uv_nodes,
            normals,
            triangles: self.triangles.clone(),
        }))
    }

    // ---------------- shape assembly (addMesh / addSubShape) ----------------

    fn bind_shape(&mut self, shape: &ObjShapeTr, name: &str, material: Option<&str>, root: bool) {
        if self.receiver_enabled {
            self.bound_shapes.push(BoundShapeRecTr {
                name: name.to_string(),
                material_name: material.map(|s| s.to_string()),
                is_root_shape: root,
                is_face: shape.is_face(),
            });
        }
    }

    /// RWObj_TriangulationReader::addSubShape.
    fn add_sub_shape(
        parent: &mut Option<ObjShapeTr>,
        sub_shape: Option<ObjShapeTr>,
        to_expand_compound: bool,
    ) -> bool {
        let sub = match sub_shape {
            Some(s) => s,
            None => return false,
        };
        if parent.is_none() && to_expand_compound {
            *parent = Some(sub);
            return true;
        }
        match parent.take() {
            Some(ObjShapeTr::Compound(mut v)) => {
                v.push(sub);
                *parent = Some(ObjShapeTr::Compound(v));
            }
            Some(other) => {
                *parent = Some(ObjShapeTr::Compound(vec![other, sub]));
            }
            None => {
                *parent = Some(ObjShapeTr::Compound(vec![sub]));
            }
        }
        true
    }

    /// RWObj_TriangulationReader::addMesh on the active sub-mesh.
    fn add_mesh_active(&mut self, reason: SubMeshReasonTr) -> bool {
        let mesh = self.active_sub_mesh.clone();
        self.add_mesh(&mesh, reason)
    }

    fn add_mesh(&mut self, mesh: &ActiveSubMeshTr, reason: SubMeshReasonTr) -> bool {
        if !self.to_create_shapes {
            return false;
        }

        if let Some(tris) = self.get_triangulation() {
            self.nodes.clear();
            self.nodes_uv.clear();
            self.normals.clear();
            self.triangles.clear();
            if mesh.group != self.last_group_name {
                // Flush previous group and start a new one.
                let group_shape = self.last_group_shape.clone();
                if Self::add_sub_shape(&mut self.last_object_shape, group_shape.clone(), false) {
                    let gsh = group_shape.unwrap();
                    let mat = if gsh.is_face() && !self.last_face_material.is_empty() {
                        Some(self.last_face_material.clone())
                    } else {
                        None
                    };
                    let gname = self.last_group_name.clone();
                    self.bind_shape(&gsh, &gname, mat.as_deref(), false);
                }
                self.last_group_shape = None;
                self.last_group_name = mesh.group.clone();
            }

            let new_face = ObjShapeTr::Face(tris);
            Self::add_sub_shape(&mut self.last_group_shape, Some(new_face.clone()), true);
            self.last_face_material = mesh.material.clone();
            let mat = if self.materials.contains_key(&mesh.material) {
                Some(mesh.material.clone())
            } else {
                None
            };
            self.bind_shape(&new_face, "", mat.as_deref(), false);
        }

        if reason == SubMeshReasonTr::NewObject {
            // Forced flush at the end of the object.
            let group_shape = self.last_group_shape.clone();
            if Self::add_sub_shape(&mut self.last_object_shape, group_shape.clone(), false) {
                let gsh = group_shape.unwrap();
                let mat = if gsh.is_face() && !self.last_face_material.is_empty() {
                    Some(self.last_face_material.clone())
                } else {
                    None
                };
                let gname = self.last_group_name.clone();
                self.bind_shape(&gsh, &gname, mat.as_deref(), false);
            }
            self.last_group_shape = None;
            self.last_group_name.clear();

            let object_shape = self.last_object_shape.clone();
            if Self::add_sub_shape(&mut self.result_shape, object_shape.clone(), false) {
                let osh = object_shape.unwrap();
                self.bind_shape(&osh, &mesh.object, None, true);
            }
            self.last_object_shape = None;
        }
        true
    }

    /// RWObj_TriangulationReader::ResultShape.
    pub fn result_shape(&mut self) -> Option<ObjShapeTr> {
        if !self.to_create_shapes {
            return self.get_triangulation().map(ObjShapeTr::Face);
        }
        if let Some(ObjShapeTr::Compound(children)) = &self.result_shape {
            if children.len() == 1 && self.active_sub_mesh.object.is_empty() {
                return Some(children[0].clone());
            }
        }
        self.result_shape.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRI_OBJ: &str = "# simple triangle\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";

    #[test]
    fn single_triangle_document() {
        let mut reader = RWObjTriangulationReader::new();
        assert!(reader.read(TRI_OBJ));
        assert_eq!(reader.file_comments(), "simple triangle");
        assert_eq!(reader.nb_probe_nodes(), 3);
        assert_eq!(reader.nb_probe_elems(), 1);
        let shape = reader.result_shape().expect("result shape");
        // ResultShape unwraps the root compound with a single (unnamed)
        // object child: the returned shape is that object compound,
        // which holds the single face.
        let tris = match shape {
            ObjShapeTr::Compound(children) => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    ObjShapeTr::Face(t) => t.clone(),
                    other => panic!("expected face child, got {other:?}"),
                }
            }
            other => panic!("expected object compound, got {other:?}"),
        };
        assert_eq!(tris.nodes.len(), 3);
        assert_eq!(tris.triangles, vec![[1, 2, 3]]);
        assert!(tris.normals.is_none());
        assert!(tris.uv_nodes.is_none());
    }

    #[test]
    fn quad_is_split_into_two_triangles() {
        let mut reader = RWObjTriangulationReader::new();
        reader.set_create_shapes(false);
        assert!(reader.read("v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\n"));
        let face = reader.result_shape().expect("face");
        let tris = match face {
            ObjShapeTr::Face(t) => t,
            _ => unreachable!(),
        };
        assert_eq!(tris.triangles, vec![[1, 2, 3], [1, 3, 4]]);
    }

    #[test]
    fn packed_index_dedup_shares_nodes() {
        // Two triangles sharing an edge: 4 unique v//-combinations.
        let mut reader = RWObjTriangulationReader::new();
        reader.set_create_shapes(false);
        assert!(reader.read("v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3\nf 1 3 4\n"));
        let tris = match reader.result_shape().unwrap() {
            ObjShapeTr::Face(t) => t,
            _ => unreachable!(),
        };
        assert_eq!(tris.nodes.len(), 4, "shared corners are not duplicated");
        assert_eq!(tris.triangles.len(), 2);
    }

    #[test]
    fn negative_indices_resolve_relative() {
        let mut reader = RWObjTriangulationReader::new();
        reader.set_create_shapes(false);
        assert!(reader.read("v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n"));
        let tris = match reader.result_shape().unwrap() {
            ObjShapeTr::Face(t) => t,
            _ => unreachable!(),
        };
        assert_eq!(tris.triangles, vec![[1, 2, 3]]);
        assert_eq!(tris.nodes[2], [0.0, 1.0, 0.0]);
    }

    #[test]
    fn vertex_index_out_of_range_aborts() {
        let mut reader = RWObjTriangulationReader::new();
        assert!(!reader.read("v 0 0 0\nf 1 2 3\n"));
        assert!(reader
            .warnings()
            .iter()
            .any(|w| w.contains("vertex index is out of range")));
    }

    #[test]
    fn normals_and_uv_transferred() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 0 1\nvn 0 0 1\nf 1/1/1 2/2/1 3/3/1\n";
        let mut reader = RWObjTriangulationReader::new();
        reader.set_create_shapes(false);
        assert!(reader.read(obj));
        let tris = match reader.result_shape().unwrap() {
            ObjShapeTr::Face(t) => t,
            _ => unreachable!(),
        };
        let uv = tris.uv_nodes.as_ref().expect("uv");
        assert_eq!(uv[1], [1.0, 0.0]);
        let normals = tris.normals.as_ref().expect("normals");
        assert_eq!(normals[0], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn polygon_fan_split_counts_big_elements() {
        let obj = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0.5 1.5 0\nv 0 1 0\nf 1 2 3 4 5\n";
        let mut reader = RWObjTriangulationReader::new();
        reader.set_create_shapes(false);
        assert!(reader.read(obj));
        let tris = match reader.result_shape().unwrap() {
            ObjShapeTr::Face(t) => t,
            _ => unreachable!(),
        };
        // 5-gon -> 3 fan triangles.
        assert_eq!(tris.triangles, vec![[1, 2, 3], [1, 3, 4], [1, 4, 5]]);
        assert_eq!(reader.nb_elems_big, 1);
    }

    #[test]
    fn groups_become_separate_faces_in_compound() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 0 0 1\n\
                   g left\nf 1 2 3\ng right\nf 1 2 4\n";
        let mut reader = RWObjTriangulationReader::new();
        assert!(reader.read(obj));
        let shape = reader.result_shape().expect("shape");
        // Result: single object compound holding two group faces.
        assert_eq!(shape.total_faces(), 2);
    }

    #[test]
    fn usemtl_undefined_material_warns_and_ignores() {
        let mut reader = RWObjTriangulationReader::new();
        assert!(reader.read("v 0 0 0\nv 1 0 0\nv 0 1 0\nusemtl ghost\nf 1 2 3\n"));
        assert!(reader.warnings().iter().any(|w| w.contains("undefined OBJ material")));
    }

    #[test]
    fn shape_receiver_gets_bindings() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\no part\nf 1 2 3\n";
        let mut reader = RWObjTriangulationReader::new();
        reader.set_shape_receiver(true);
        reader.define_material(RWObjMaterialTr {
            name: "steel".into(),
            ..RWObjMaterialTr::default()
        });
        assert!(reader.read(obj));
        let roots: Vec<_> = reader
            .bound_shapes()
            .iter()
            .filter(|b| b.is_root_shape)
            .collect();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].name, "part");
    }

    #[test]
    fn mtllib_recorded_as_external_file() {
        let mut reader = RWObjTriangulationReader::new();
        assert!(reader.read("mtllib scene.mtl\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n"));
        assert!(reader.external_files().contains(&"scene.mtl".to_string()));
    }
}
