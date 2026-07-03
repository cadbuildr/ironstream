// FILE: mesh_vs_mesh_owner.rs
// occt: MeshVS_MeshOwner

//! Owner of a mesh entity for selection management.

#[derive(Clone, Debug)]
pub struct MeshOwner {
    pub owner_id: u32,
    pub mesh_id: u32,
    pub entity_id: u32,
    pub is_highlighted: bool,
}

impl MeshOwner {
    pub fn new(owner_id: u32, mesh_id: u32, entity_id: u32) -> Self {
        MeshOwner {
            owner_id,
            mesh_id,
            entity_id,
            is_highlighted: false,
        }
    }

    pub fn highlight(&mut self) {
        self.is_highlighted = true;
    }

    pub fn unhighlight(&mut self) {
        self.is_highlighted = false;
    }

    pub fn is_highlighted(&self) -> bool {
        self.is_highlighted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_owner_creation() {
        let owner = MeshOwner::new(1, 10, 20);
        assert_eq!(owner.owner_id, 1);
        assert_eq!(owner.mesh_id, 10);
        assert_eq!(owner.entity_id, 20);
        assert!(!owner.is_highlighted());
    }

    #[test]
    fn mesh_owner_highlight() {
        let mut owner = MeshOwner::new(1, 10, 20);
        owner.highlight();
        assert!(owner.is_highlighted());
        owner.unhighlight();
        assert!(!owner.is_highlighted());
    }
}
