// FILE: graphic3d_group.rs
// occt: Graphic3d_Group, Graphic3d_ArrayOfPrimitives
// occt-ref: Graphic3d_Structure
//       Graphic3d_ArrayOfTriangles, Graphic3d_ArrayOfLines, Graphic3d_ArrayOfPoints

/// Primitive array type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
}

/// A vertex with position, normal, color, and UV.
#[derive(Clone, Debug)]
pub struct Vertex3d {
    pub xyz: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex3d {
    pub fn new(xyz: [f32; 3]) -> Self {
        Self { xyz, normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], uv: [0.0, 0.0] }
    }

    pub fn with_normal(mut self, n: [f32; 3]) -> Self { self.normal = n; self }
    pub fn with_color(mut self, c: [f32; 4]) -> Self { self.color = c; self }
    pub fn with_uv(mut self, uv: [f32; 2]) -> Self { self.uv = uv; self }
}

// occt: Graphic3d_ArrayOfPrimitives
#[derive(Clone, Debug)]
pub struct ArrayOfPrimitives {
    pub prim_type: PrimitiveType,
    pub vertices: Vec<Vertex3d>,
    pub indices: Vec<u32>,
    pub has_normals: bool,
    pub has_colors: bool,
    pub has_uvs: bool,
}

impl ArrayOfPrimitives {
    pub fn new(prim_type: PrimitiveType, max_verts: usize) -> Self {
        Self {
            prim_type,
            vertices: Vec::with_capacity(max_verts),
            indices: Vec::new(),
            has_normals: false,
            has_colors: false,
            has_uvs: false,
        }
    }

    pub fn with_normals(mut self) -> Self { self.has_normals = true; self }
    pub fn with_colors(mut self) -> Self { self.has_colors = true; self }
    pub fn with_uvs(mut self) -> Self { self.has_uvs = true; self }

    pub fn add_vertex(&mut self, v: Vertex3d) {
        self.vertices.push(v);
    }

    pub fn add_index(&mut self, i: u32) {
        self.indices.push(i);
    }

    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.extend_from_slice(&[a, b, c]);
    }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn nb_indices(&self) -> usize { self.indices.len() }

    pub fn nb_primitives(&self) -> usize {
        match self.prim_type {
            PrimitiveType::Points => self.vertices.len(),
            PrimitiveType::Lines => self.indices.len() / 2,
            PrimitiveType::Triangles => self.indices.len() / 3,
            PrimitiveType::Quads => self.indices.len() / 4,
            PrimitiveType::LineStrip => self.indices.len().saturating_sub(1),
            PrimitiveType::TriangleStrip | PrimitiveType::TriangleFan =>
                self.indices.len().saturating_sub(2),
        }
    }

    pub fn is_indexed(&self) -> bool { !self.indices.is_empty() }
    pub fn is_empty(&self) -> bool { self.vertices.is_empty() }

    pub fn bounding_box(&self) -> ([f32; 3], [f32; 3]) {
        if self.vertices.is_empty() {
            return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }
        let mut mn = [f32::INFINITY; 3];
        let mut mx = [f32::NEG_INFINITY; 3];
        for v in &self.vertices {
            for i in 0..3 {
                mn[i] = mn[i].min(v.xyz[i]);
                mx[i] = mx[i].max(v.xyz[i]);
            }
        }
        (mn, mx)
    }
}

// occt: Graphic3d_ArrayOfTriangles
pub struct ArrayOfTriangles(pub ArrayOfPrimitives);

impl ArrayOfTriangles {
    pub fn new(max_verts: usize, has_normals: bool, has_uvs: bool) -> Self {
        let mut a = ArrayOfPrimitives::new(PrimitiveType::Triangles, max_verts);
        if has_normals { a = a.with_normals(); }
        if has_uvs { a = a.with_uvs(); }
        Self(a)
    }

    pub fn add_vertex(&mut self, xyz: [f32; 3]) { self.0.add_vertex(Vertex3d::new(xyz)); }
    pub fn add_vertex_with_normal(&mut self, xyz: [f32; 3], n: [f32; 3]) {
        self.0.add_vertex(Vertex3d::new(xyz).with_normal(n));
    }
    pub fn add_edge(&mut self, i: u32) { self.0.add_index(i); }
    pub fn nb_triangles(&self) -> usize { self.0.nb_primitives() }
}

// occt-ref: Graphic3d_ArrayOfLines
pub struct ArrayOfLines(pub ArrayOfPrimitives);

impl ArrayOfLines {
    pub fn new(max_verts: usize) -> Self {
        Self(ArrayOfPrimitives::new(PrimitiveType::Lines, max_verts))
    }

    pub fn add_vertex(&mut self, xyz: [f32; 3]) { self.0.add_vertex(Vertex3d::new(xyz)); }
    pub fn add_edge(&mut self, i: u32) { self.0.add_index(i); }
    pub fn nb_segments(&self) -> usize { self.0.nb_primitives() }
}

// occt: Graphic3d_ArrayOfPoints
pub struct ArrayOfPoints(pub ArrayOfPrimitives);

impl ArrayOfPoints {
    pub fn new(max_verts: usize) -> Self {
        Self(ArrayOfPrimitives::new(PrimitiveType::Points, max_verts))
    }
    pub fn add_vertex(&mut self, xyz: [f32; 3]) { self.0.add_vertex(Vertex3d::new(xyz)); }
    pub fn add_vertex_with_color(&mut self, xyz: [f32; 3], c: [f32; 4]) {
        self.0.add_vertex(Vertex3d::new(xyz).with_color(c));
    }
    pub fn nb_points(&self) -> usize { self.0.nb_vertices() }
}

// occt: Graphic3d_Group
#[derive(Clone, Debug, Default)]
pub struct Graphic3dGroup {
    pub group_id: u32,
    pub arrays: Vec<ArrayOfPrimitives>,
    pub is_closed: bool,
    pub min_max_valid: bool,
    pub line_color: [f32; 4],
    pub fill_color: [f32; 4],
}

impl Graphic3dGroup {
    pub fn new(id: u32) -> Self {
        Self {
            group_id: id,
            line_color: [1.0, 1.0, 1.0, 1.0],
            fill_color: [0.8, 0.8, 0.8, 1.0],
            ..Default::default()
        }
    }

    pub fn add_primitive_array(&mut self, arr: ArrayOfPrimitives) {
        self.arrays.push(arr);
    }

    pub fn set_closed(&mut self, closed: bool) { self.is_closed = closed; }

    pub fn nb_arrays(&self) -> usize { self.arrays.len() }

    pub fn total_vertices(&self) -> usize { self.arrays.iter().map(|a| a.nb_vertices()).sum() }

    pub fn total_primitives(&self) -> usize { self.arrays.iter().map(|a| a.nb_primitives()).sum() }

    pub fn is_empty(&self) -> bool { self.arrays.is_empty() || self.total_vertices() == 0 }

    pub fn bounding_box(&self) -> ([f32; 3], [f32; 3]) {
        if self.arrays.is_empty() { return ([0.0; 3], [0.0; 3]); }
        let mut mn = [f32::INFINITY; 3];
        let mut mx = [f32::NEG_INFINITY; 3];
        for arr in &self.arrays {
            let (amin, amax) = arr.bounding_box();
            for i in 0..3 {
                mn[i] = mn[i].min(amin[i]);
                mx[i] = mx[i].max(amax[i]);
            }
        }
        (mn, mx)
    }

    pub fn clear(&mut self) { self.arrays.clear(); }
}

// occt-ref: Graphic3d_Structure
#[derive(Clone, Debug, Default)]
pub struct Graphic3dStructure {
    pub struct_id: u32,
    pub groups: Vec<Graphic3dGroup>,
    pub is_visible: bool,
    pub highlight_color: Option<[f32; 4]>,
    pub transformation: [[f32; 4]; 4],
}

impl Graphic3dStructure {
    pub fn new(id: u32) -> Self {
        let transform = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self { struct_id: id, is_visible: true, transformation: transform, ..Default::default() }
    }

    pub fn new_group(&mut self) -> u32 {
        let id = self.groups.len() as u32;
        self.groups.push(Graphic3dGroup::new(id));
        id
    }

    pub fn group(&self, id: u32) -> Option<&Graphic3dGroup> {
        self.groups.get(id as usize)
    }

    pub fn group_mut(&mut self, id: u32) -> Option<&mut Graphic3dGroup> {
        self.groups.get_mut(id as usize)
    }

    pub fn nb_groups(&self) -> usize { self.groups.len() }
    pub fn is_empty(&self) -> bool { self.groups.iter().all(|g| g.is_empty()) }

    pub fn display(&mut self) { self.is_visible = true; }
    pub fn erase(&mut self) { self.is_visible = false; }

    pub fn highlight(&mut self, color: [f32; 4]) { self.highlight_color = Some(color); }
    pub fn unhighlight(&mut self) { self.highlight_color = None; }
    pub fn is_highlighted(&self) -> bool { self.highlight_color.is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_of_triangles() {
        let mut arr = ArrayOfTriangles::new(4, true, false);
        arr.add_vertex_with_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        arr.add_vertex_with_normal([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        arr.add_vertex_with_normal([0.5, 1.0, 0.0], [0.0, 0.0, 1.0]);
        arr.add_edge(0); arr.add_edge(1); arr.add_edge(2);
        assert_eq!(arr.nb_triangles(), 1);
    }

    #[test]
    fn group_primitives() {
        let mut grp = Graphic3dGroup::new(0);
        let mut arr = ArrayOfPrimitives::new(PrimitiveType::Triangles, 3);
        arr.add_vertex(Vertex3d::new([0.0, 0.0, 0.0]));
        arr.add_vertex(Vertex3d::new([1.0, 0.0, 0.0]));
        arr.add_vertex(Vertex3d::new([0.5, 1.0, 0.0]));
        arr.add_triangle(0, 1, 2);
        grp.add_primitive_array(arr);
        assert_eq!(grp.total_vertices(), 3);
        assert_eq!(grp.total_primitives(), 1);
        assert!(!grp.is_empty());
    }

    #[test]
    fn structure_groups() {
        let mut st = Graphic3dStructure::new(1);
        let gid = st.new_group();
        assert_eq!(st.nb_groups(), 1);
        st.highlight([1.0, 1.0, 0.0, 1.0]);
        assert!(st.is_highlighted());
        st.erase();
        assert!(!st.is_visible);
    }

    #[test]
    fn bounding_box() {
        let mut arr = ArrayOfPrimitives::new(PrimitiveType::Points, 2);
        arr.add_vertex(Vertex3d::new([0.0, 0.0, 0.0]));
        arr.add_vertex(Vertex3d::new([2.0, 3.0, 4.0]));
        let (mn, mx) = arr.bounding_box();
        assert!((mn[0] - 0.0).abs() < 1e-6);
        assert!((mx[1] - 3.0).abs() < 1e-6);
    }
}
