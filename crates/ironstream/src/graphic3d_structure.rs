// FILE: rust/ironstream/crates/ironstream/src/graphic3d_structure.rs

// occt: Graphic3d_GroupAspect
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dGroupType {
    PrimitivesAspect,
    TextAspect,
    MarkerAspect,
    LineAspect,
}

// occt: Graphic3d_ArrayOfPrimitives
pub struct Graphic3dPrimitive {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    pub is_indexed: bool,
}

impl Graphic3dPrimitive {
    pub fn new() -> Self {
        Graphic3dPrimitive {
            vertices: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
            is_indexed: false,
        }
    }

    pub fn add_vertex(&mut self, v: [f32; 3]) {
        self.vertices.push(v);
    }

    pub fn add_normal(&mut self, n: [f32; 3]) {
        self.normals.push(n);
    }

    pub fn add_color(&mut self, c: [f32; 4]) {
        self.colors.push(c);
    }

    pub fn add_index(&mut self, i: u32) {
        self.indices.push(i);
        self.is_indexed = true;
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn nb_indices(&self) -> usize {
        self.indices.len()
    }

    pub fn is_indexed(&self) -> bool {
        self.is_indexed
    }

    pub fn vertex(&self, i: usize) -> [f32; 3] {
        self.vertices[i]
    }
}

impl Default for Graphic3dPrimitive {
    fn default() -> Self {
        Self::new()
    }
}

// occt: Graphic3d_Group
pub struct Graphic3dGroup {
    pub group_type: Graphic3dGroupType,
    pub primitives: Vec<Graphic3dPrimitive>,
    pub is_empty: bool,
}

impl Graphic3dGroup {
    pub fn new(group_type: Graphic3dGroupType) -> Self {
        Graphic3dGroup {
            group_type,
            primitives: Vec::new(),
            is_empty: true,
        }
    }

    pub fn add_primitive(&mut self, p: Graphic3dPrimitive) {
        self.primitives.push(p);
        self.is_empty = false;
    }

    pub fn nb_primitives(&self) -> usize {
        self.primitives.len()
    }

    pub fn group_type(&self) -> Graphic3dGroupType {
        self.group_type
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn clear(&mut self) {
        self.primitives.clear();
        self.is_empty = true;
    }
}

// occt: Graphic3d_Structure
pub struct Graphic3dStructure {
    pub id: usize,
    pub groups: Vec<Graphic3dGroup>,
    pub is_visible: bool,
    pub priority: i32,
    pub transform: [[f32; 4]; 4],
}

impl Graphic3dStructure {
    pub fn new(id: usize) -> Self {
        Graphic3dStructure {
            id,
            groups: Vec::new(),
            is_visible: true,
            priority: 0,
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn new_group(&mut self, group_type: Graphic3dGroupType) -> usize {
        let index = self.groups.len();
        self.groups.push(Graphic3dGroup::new(group_type));
        index
    }

    pub fn group(&self, i: usize) -> &Graphic3dGroup {
        &self.groups[i]
    }

    pub fn nb_groups(&self) -> usize {
        self.groups.len()
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
}
