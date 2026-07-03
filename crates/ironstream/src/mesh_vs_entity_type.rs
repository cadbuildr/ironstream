// FILE: mesh_vs_entity_type.rs
// occt: MeshVS_EntityType

/// Entity type enumeration for mesh visualization.
/// Describes the types of entities that can be visualized in a mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityType(pub i32);

impl EntityType {
    /// No entity
    pub const NONE: Self = EntityType(0x00);

    /// Node entity
    pub const NODE: Self = EntityType(0x01);

    /// 0D element
    pub const ELEMENT_0D: Self = EntityType(0x02);

    /// Link (edge) element
    pub const LINK: Self = EntityType(0x04);

    /// Face element
    pub const FACE: Self = EntityType(0x08);

    /// Volume element
    pub const VOLUME: Self = EntityType(0x10);

    /// All element types (0D, Link, Face, Volume)
    pub const ELEMENT: Self = EntityType(0x1E); // 0x02 | 0x04 | 0x08 | 0x10

    /// All entity types (Element | Node)
    pub const ALL: Self = EntityType(0x1F); // 0x1E | 0x01

    /// Creates a new EntityType from a raw i32 value
    pub fn new(value: i32) -> Self {
        EntityType(value)
    }

    /// Returns the raw i32 value
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Checks if this entity type contains the given type
    pub fn contains(&self, ty: Self) -> bool {
        (self.0 & ty.0) != 0
    }

    /// Sets the given entity type flag
    pub fn set(&mut self, ty: Self) {
        self.0 |= ty.0;
    }

    /// Clears the given entity type flag
    pub fn clear(&mut self, ty: Self) {
        self.0 &= !ty.0;
    }
}

impl std::ops::BitOr for EntityType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        EntityType(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for EntityType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        EntityType(self.0 & rhs.0)
    }
}

impl From<i32> for EntityType {
    fn from(value: i32) -> Self {
        EntityType(value)
    }
}

impl From<EntityType> for i32 {
    fn from(ty: EntityType) -> Self {
        ty.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_values() {
        assert_eq!(EntityType::NONE.value(), 0x00);
        assert_eq!(EntityType::NODE.value(), 0x01);
        assert_eq!(EntityType::ELEMENT_0D.value(), 0x02);
        assert_eq!(EntityType::LINK.value(), 0x04);
        assert_eq!(EntityType::FACE.value(), 0x08);
        assert_eq!(EntityType::VOLUME.value(), 0x10);
    }

    #[test]
    fn test_element_type_composition() {
        let element = EntityType::ELEMENT_0D | EntityType::LINK | EntityType::FACE | EntityType::VOLUME;
        assert_eq!(element.value(), EntityType::ELEMENT.value());
        assert!(element.contains(EntityType::ELEMENT_0D));
        assert!(element.contains(EntityType::LINK));
        assert!(element.contains(EntityType::FACE));
        assert!(element.contains(EntityType::VOLUME));
    }

    #[test]
    fn test_all_type_composition() {
        let all = EntityType::ELEMENT | EntityType::NODE;
        assert_eq!(all.value(), EntityType::ALL.value());
        assert!(all.contains(EntityType::NODE));
        assert!(all.contains(EntityType::ELEMENT_0D));
        assert!(all.contains(EntityType::LINK));
        assert!(all.contains(EntityType::FACE));
        assert!(all.contains(EntityType::VOLUME));
    }

    #[test]
    fn test_contains() {
        let ty = EntityType::NODE | EntityType::FACE;
        assert!(ty.contains(EntityType::NODE));
        assert!(ty.contains(EntityType::FACE));
        assert!(!ty.contains(EntityType::LINK));
    }

    #[test]
    fn test_set_clear() {
        let mut ty = EntityType::NONE;
        ty.set(EntityType::NODE);
        assert!(ty.contains(EntityType::NODE));

        ty.set(EntityType::FACE);
        assert!(ty.contains(EntityType::NODE));
        assert!(ty.contains(EntityType::FACE));

        ty.clear(EntityType::NODE);
        assert!(!ty.contains(EntityType::NODE));
        assert!(ty.contains(EntityType::FACE));
    }

    #[test]
    fn test_from_i32() {
        let ty: EntityType = 0x05i32.into();
        assert_eq!(ty.value(), 0x05);
        assert!(ty.contains(EntityType::NODE));
        assert!(ty.contains(EntityType::LINK));
    }
}
