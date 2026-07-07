// FILE: select3_d_sensitive_entity.rs
// occt: Select3D_SensitiveEntity

/// Abstract base for 3D sensitive entities used in selection/picking.
/// Represents a selectable object in 3D space with owner information and sensitivity settings.
pub struct Select3DSensitiveEntity {
    owner_id: Option<()>, // TODO: real owner type when available
    sensitivity_factor: i32,
    transform_persistence: Option<()>, // TODO: real transform type
    flipper: Option<()>,                // TODO: real flipper type
}

impl Select3DSensitiveEntity {
    /// Returns the owner ID of this sensitive entity.
    pub fn owner_id(&self) -> Option<&()> {
        self.owner_id.as_ref()
    }

    /// Sets the owner of the entity.
    pub fn set(&mut self, owner_id: Option<()>) {
        self.owner_id = owner_id;
    }

    /// Returns the sensitivity factor for this entity.
    /// Higher values make selection easier for small entities.
    pub fn sensitivity_factor(&self) -> i32 {
        self.sensitivity_factor
    }

    /// Sets the sensitivity factor. Must be non-negative.
    pub fn set_sensitivity_factor(&mut self, factor: i32) {
        assert!(factor >= 0, "Sensitivity factor must be non-negative");
        self.sensitivity_factor = factor;
    }

    /// Returns the transform persistence handle if set.
    pub fn transform_persistence(&self) -> Option<&()> {
        self.transform_persistence.as_ref()
    }

    /// Sets the transform persistence.
    pub fn set_transform_persistence(&mut self, trsf_pers: Option<()>) {
        self.transform_persistence = trsf_pers;
    }

    /// Returns the flipper handle if set.
    pub fn flipper(&self) -> Option<&()> {
        self.flipper.as_ref()
    }

    /// Returns true if entity has an initial location set.
    pub fn has_init_location(&self) -> bool {
        false
    }

    /// Clears all resources and resets the entity.
    pub fn clear(&mut self) {
        self.owner_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensitivity_factor() {
        let mut entity = Select3DSensitiveEntity {
            owner_id: None,
            sensitivity_factor: 0,
            transform_persistence: None,
            flipper: None,
        };

        assert_eq!(entity.sensitivity_factor(), 0);

        entity.set_sensitivity_factor(5);
        assert_eq!(entity.sensitivity_factor(), 5);
    }

    #[test]
    fn test_sensitivity_factor_negative_panic() {
        let mut entity = Select3DSensitiveEntity {
            owner_id: None,
            sensitivity_factor: 0,
            transform_persistence: None,
            flipper: None,
        };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            entity.set_sensitivity_factor(-1);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_clear() {
        let mut entity = Select3DSensitiveEntity {
            owner_id: Some(()),
            sensitivity_factor: 5,
            transform_persistence: None,
            flipper: None,
        };

        entity.clear();
        assert!(entity.owner_id().is_none());
    }
}
