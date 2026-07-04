// FILE: b_rep_to_igesb_rep_entity.rs
// occt: BRepToIGESBRep_Entity

/// Tool class for transferring BRep entities from CASCADE to IGESBRep.
pub struct Entity {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    curves: Vec<IGESEntity>,
}

pub struct Vertex;
pub struct Edge;
pub struct IGESEntity;

impl Entity {
    pub fn new() -> Self {
        Entity {
            vertices: Vec::new(),
            edges: Vec::new(),
            curves: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.curves.clear();
    }

    pub fn index_vertex(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(&mut self) -> usize {
        self.vertices.push(Vertex);
        self.vertices.len()
    }

    pub fn index_edge(&self) -> usize {
        self.edges.len()
    }

    pub fn add_edge(&mut self) -> usize {
        self.edges.push(Edge);
        self.curves.push(IGESEntity);
        self.edges.len()
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = Entity::new();
        assert_eq!(entity.index_vertex(), 0);
        assert_eq!(entity.index_edge(), 0);
    }

    #[test]
    fn test_clear() {
        let mut entity = Entity::new();
        entity.add_vertex();
        entity.clear();
        assert_eq!(entity.index_vertex(), 0);
    }
}
