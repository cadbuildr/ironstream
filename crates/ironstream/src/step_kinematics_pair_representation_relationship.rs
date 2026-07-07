// FILE: step_kinematics_pair_representation_relationship.rs
// occt: StepKinematics_PairRepresentationRelationship

pub struct PairRepresentationRelationship {
    representation_relationship_with_transformation:
        Option<Box<dyn std::any::Any>>,
}

impl PairRepresentationRelationship {
    pub fn new() -> Self {
        PairRepresentationRelationship {
            representation_relationship_with_transformation: None,
        }
    }

    pub fn init(&mut self, rrwt: Option<Box<dyn std::any::Any>>) {
        self.representation_relationship_with_transformation = rrwt;
    }

    pub fn representation_relationship_with_transformation(
        &self,
    ) -> &Option<Box<dyn std::any::Any>> {
        &self.representation_relationship_with_transformation
    }

    pub fn set_representation_relationship_with_transformation(
        &mut self,
        rrwt: Option<Box<dyn std::any::Any>>,
    ) {
        self.representation_relationship_with_transformation = rrwt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_representation_relationship_creation() {
        let prr = PairRepresentationRelationship::new();
        assert!(
            prr.representation_relationship_with_transformation()
                .is_none()
        );
    }

    #[test]
    fn test_init() {
        let mut prr = PairRepresentationRelationship::new();
        prr.init(None);
        assert!(
            prr.representation_relationship_with_transformation()
                .is_none()
        );
    }
}
