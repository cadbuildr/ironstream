// FILE: step_shape_faceted_brep_and_brep_with_voids.rs
// occt: StepShape_FacetedBrepAndBrepWithVoids

//! Representation of STEP entity FacetedBrepAndBrepWithVoids

#[derive(Clone, Debug)]
pub struct FacetedBrepAndBrepWithVoids {
    name: String,
    outer: Option<String>,
    faceted_brep: Option<String>,
    brep_with_voids: Option<String>,
    voids: Vec<String>,
}

impl FacetedBrepAndBrepWithVoids {
    /// Returns a FacetedBrepAndBrepWithVoids
    pub fn new() -> Self {
        FacetedBrepAndBrepWithVoids {
            name: String::new(),
            outer: None,
            faceted_brep: None,
            brep_with_voids: None,
            voids: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init_from_faceted_brep(
        &mut self,
        name: String,
        outer: Option<String>,
        faceted_brep: Option<String>,
        brep_with_voids: Option<String>,
    ) {
        self.name = name;
        self.outer = outer;
        self.faceted_brep = faceted_brep;
        self.brep_with_voids = brep_with_voids;
        self.voids = Vec::new();
    }

    /// Initialize all fields from voids
    pub fn init_from_voids(&mut self, name: String, outer: Option<String>, voids: Vec<String>) {
        self.name = name;
        self.outer = outer;
        self.voids = voids;
        self.faceted_brep = None;
        self.brep_with_voids = None;
    }

    /// Set FacetedBrep
    pub fn set_faceted_brep(&mut self, faceted_brep: Option<String>) {
        self.faceted_brep = faceted_brep;
    }

    /// Returns FacetedBrep
    pub fn faceted_brep(&self) -> &Option<String> {
        &self.faceted_brep
    }

    /// Set BrepWithVoids
    pub fn set_brep_with_voids(&mut self, brep_with_voids: Option<String>) {
        self.brep_with_voids = brep_with_voids;
    }

    /// Returns BrepWithVoids
    pub fn brep_with_voids(&self) -> &Option<String> {
        &self.brep_with_voids
    }

    /// Set Voids
    pub fn set_voids(&mut self, voids: Vec<String>) {
        self.voids = voids;
    }

    /// Returns Voids
    pub fn voids(&self) -> &[String] {
        &self.voids
    }

    /// Returns value at index (1-based)
    pub fn voids_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.voids.len() {
            Some(&self.voids[num - 1])
        } else {
            None
        }
    }

    /// Returns number of voids
    pub fn nb_voids(&self) -> usize {
        self.voids.len()
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for FacetedBrepAndBrepWithVoids {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let brep = FacetedBrepAndBrepWithVoids::new();
        assert_eq!(brep.name(), "");
        assert_eq!(brep.nb_voids(), 0);
    }

    #[test]
    fn test_init_from_faceted_brep() {
        let mut brep = FacetedBrepAndBrepWithVoids::new();
        brep.init_from_faceted_brep(
            "Brep1".to_string(),
            Some("outer1".to_string()),
            Some("fb1".to_string()),
            Some("bwv1".to_string()),
        );
        assert_eq!(brep.name(), "Brep1");
        assert_eq!(brep.faceted_brep(), &Some("fb1".to_string()));
    }

    #[test]
    fn test_init_from_voids() {
        let mut brep = FacetedBrepAndBrepWithVoids::new();
        brep.init_from_voids(
            "Brep2".to_string(),
            Some("outer2".to_string()),
            vec!["void1".to_string(), "void2".to_string()],
        );
        assert_eq!(brep.name(), "Brep2");
        assert_eq!(brep.nb_voids(), 2);
    }

    #[test]
    fn test_voids_value() {
        let mut brep = FacetedBrepAndBrepWithVoids::new();
        brep.set_voids(vec!["v1".to_string(), "v2".to_string()]);
        assert_eq!(brep.voids_value(1), Some(&"v1".to_string()));
        assert_eq!(brep.voids_value(2), Some(&"v2".to_string()));
        assert_eq!(brep.voids_value(3), None);
    }
}
