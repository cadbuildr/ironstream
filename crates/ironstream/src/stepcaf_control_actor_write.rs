// FILE: stepcaf_control_actor_write.rs
// occt: STEPCAFControl_ActorWrite

/// Actor for writing STEP CAF documents
pub struct STEPCAFControl_ActorWrite {
    write_shape_mode: bool,
    write_assembly_mode: bool,
}

impl STEPCAFControl_ActorWrite {
    /// Creates a new ActorWrite instance
    pub fn new() -> Self {
        STEPCAFControl_ActorWrite {
            write_shape_mode: true,
            write_assembly_mode: true,
        }
    }

    /// Sets whether to write shapes
    pub fn set_write_shape_mode(&mut self, value: bool) {
        self.write_shape_mode = value;
    }

    /// Returns whether shapes are being written
    pub fn write_shape_mode(&self) -> bool {
        self.write_shape_mode
    }

    /// Sets whether to write assembly structures
    pub fn set_write_assembly_mode(&mut self, value: bool) {
        self.write_assembly_mode = value;
    }

    /// Returns whether assembly structures are being written
    pub fn write_assembly_mode(&self) -> bool {
        self.write_assembly_mode
    }

    /// Executes the write action
    pub fn act(&self) -> bool {
        self.write_shape_mode && self.write_assembly_mode
    }
}

impl Default for STEPCAFControl_ActorWrite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let actor = STEPCAFControl_ActorWrite::new();
        assert!(actor.write_shape_mode());
        assert!(actor.write_assembly_mode());
    }

    #[test]
    fn test_set_write_shape_mode() {
        let mut actor = STEPCAFControl_ActorWrite::new();
        actor.set_write_shape_mode(false);
        assert!(!actor.write_shape_mode());
    }

    #[test]
    fn test_set_write_assembly_mode() {
        let mut actor = STEPCAFControl_ActorWrite::new();
        actor.set_write_assembly_mode(false);
        assert!(!actor.write_assembly_mode());
    }

    #[test]
    fn test_act() {
        let mut actor = STEPCAFControl_ActorWrite::new();
        assert!(actor.act());
        actor.set_write_shape_mode(false);
        assert!(!actor.act());
    }

    #[test]
    fn test_default_trait() {
        let actor = STEPCAFControl_ActorWrite::default();
        assert!(actor.act());
    }
}
