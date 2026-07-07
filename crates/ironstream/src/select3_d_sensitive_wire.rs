// FILE: select3_d_sensitive_wire.rs
// occt: Select3D_SensitiveWire

/// A sensitive wire entity for selection of wire edges.
pub struct Select3DSensitiveWire {
    owner_id: Option<()>, // TODO: real owner type
    entities: Vec<()>,    // TODO: real Select3D_SensitiveEntity handles
    entity_indexes: Vec<i32>,
    center: (f64, f64, f64), // TODO: replace with real gp_Pnt
    bnd_box: Option<()>,    // TODO: replace with real Select3D_BndBox3d
    detected_idx: i32,
}

impl Select3DSensitiveWire {
    /// Creates a new sensitive wire.
    pub fn new(owner_id: Option<()>) -> Self {
        Select3DSensitiveWire {
            owner_id,
            entities: Vec::new(),
            entity_indexes: Vec::new(),
            center: (0.0, 0.0, 0.0),
            bnd_box: None,
            detected_idx: -1,
        }
    }

    /// Adds a sensitive entity to the wire.
    pub fn add(&mut self, _entity: Option<()>) {
        // TODO: implement when real entity type is available
        self.entity_indexes.push(self.entities.len() as i32);
        self.entities.push(());
    }

    /// Returns the number of sub-entities (edges) in the wire.
    pub fn nb_sub_elements(&self) -> usize {
        self.entities.len()
    }

    /// Returns the length of the entity vector.
    pub fn size(&self) -> usize {
        self.entities.len()
    }

    /// Returns the center of the wire.
    pub fn center_of_geometry(&self) -> (f64, f64, f64) {
        self.center
    }

    /// Sets the center of the wire.
    pub fn set_center(&mut self, center: (f64, f64, f64)) {
        self.center = center;
    }

    /// Returns the bounding box of the wire.
    pub fn bounding_box(&self) -> Option<&()> {
        self.bnd_box.as_ref()
    }

    /// Sets the bounding box of the wire.
    pub fn set_bounding_box(&mut self, bbox: Option<()>) {
        self.bnd_box = bbox;
    }

    /// Returns the owner ID.
    pub fn owner_id(&self) -> Option<&()> {
        self.owner_id.as_ref()
    }

    /// Sets the owner ID for all entities in the wire.
    pub fn set_owner_id(&mut self, owner_id: Option<()>) {
        self.owner_id = owner_id;
    }

    /// Returns the index of the last detected entity.
    pub fn last_detected_entity_index(&self) -> i32 {
        if self.detected_idx >= 0 && (self.detected_idx as usize) < self.entity_indexes.len() {
            self.entity_indexes[self.detected_idx as usize]
        } else {
            -1
        }
    }

    /// Sets the index of the last detected entity.
    pub fn set_detected_idx(&mut self, idx: i32) {
        self.detected_idx = idx;
    }

    /// Clears all entities from the wire.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.entity_indexes.clear();
        self.detected_idx = -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wire() {
        let wire = Select3DSensitiveWire::new(None);
        assert_eq!(wire.nb_sub_elements(), 0);
        assert_eq!(wire.size(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut wire = Select3DSensitiveWire::new(None);
        assert_eq!(wire.nb_sub_elements(), 0);

        wire.add(Some(()));
        assert_eq!(wire.nb_sub_elements(), 1);

        wire.add(Some(()));
        assert_eq!(wire.nb_sub_elements(), 2);
    }

    #[test]
    fn test_center_of_geometry() {
        let mut wire = Select3DSensitiveWire::new(None);
        let center = (1.5, 2.5, 3.5);
        wire.set_center(center);

        assert_eq!(wire.center_of_geometry(), center);
    }

    #[test]
    fn test_bounding_box() {
        let mut wire = Select3DSensitiveWire::new(None);
        assert!(wire.bounding_box().is_none());

        wire.set_bounding_box(Some(()));
        assert!(wire.bounding_box().is_some());
    }

    #[test]
    fn test_owner_id() {
        let mut wire = Select3DSensitiveWire::new(None);
        assert!(wire.owner_id().is_none());

        wire.set_owner_id(Some(()));
        assert!(wire.owner_id().is_some());
    }

    #[test]
    fn test_last_detected_entity_index() {
        let mut wire = Select3DSensitiveWire::new(None);
        assert_eq!(wire.last_detected_entity_index(), -1);

        wire.add(Some(()));
        wire.add(Some(()));
        wire.entity_indexes[0] = 10;

        wire.set_detected_idx(0);
        assert_eq!(wire.last_detected_entity_index(), 10);
    }

    #[test]
    fn test_clear() {
        let mut wire = Select3DSensitiveWire::new(None);
        wire.add(Some(()));
        wire.detected_idx = 0;

        wire.clear();
        assert_eq!(wire.nb_sub_elements(), 0);
        assert_eq!(wire.detected_idx, -1);
    }
}
