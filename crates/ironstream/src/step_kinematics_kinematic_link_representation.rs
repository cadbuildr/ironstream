// FILE: step_kinematics_kinematic_link_representation.rs
// occt: StepKinematics_KinematicLinkRepresentation

pub struct KinematicLinkRepresentation {
    represented_link: Option<Box<dyn std::any::Any>>,
}

impl KinematicLinkRepresentation {
    pub fn new() -> Self {
        KinematicLinkRepresentation {
            represented_link: None,
        }
    }

    pub fn init(&mut self, represented_link: Option<Box<dyn std::any::Any>>) {
        self.represented_link = represented_link;
    }

    pub fn represented_link(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.represented_link
    }

    pub fn set_represented_link(&mut self, link: Option<Box<dyn std::any::Any>>) {
        self.represented_link = link;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_link_representation_creation() {
        let repr = KinematicLinkRepresentation::new();
        assert!(repr.represented_link().is_none());
    }

    #[test]
    fn test_init() {
        let mut repr = KinematicLinkRepresentation::new();
        repr.init(None);
        assert!(repr.represented_link().is_none());
    }
}
