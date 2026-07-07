// FILE: iges_to_b_rep_actor.rs
// occt: IGESToBRep_Actor

/// Actor for transferring IGES entities to B-Rep entities
#[derive(Default, Clone, Debug)]
pub struct IgesToBRepActor;

impl IgesToBRepActor {
    /// Creates a new Actor
    pub fn new() -> Self {
        Self
    }

    /// Returns mode for transferring curves
    pub fn mode(&self) -> i32 {
        0
    }

    /// Sets mode for transferring curves
    pub fn set_mode(&mut self, _mode: i32) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let actor = IgesToBRepActor::new();
        assert_eq!(actor.mode(), 0);
    }

    #[test]
    fn test_set_mode() {
        let mut actor = IgesToBRepActor::new();
        actor.set_mode(1);
    }
}
