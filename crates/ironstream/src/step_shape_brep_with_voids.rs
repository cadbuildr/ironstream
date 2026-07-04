// FILE: step_shape_brep_with_voids.rs
// occt: StepShape_BrepWithVoids

/// Placeholder for ClosedShell
#[derive(Clone, Debug, PartialEq)]
pub struct ClosedShell {
    id: String,
}

/// Placeholder for OrientedClosedShell
#[derive(Clone, Debug, PartialEq)]
pub struct OrientedClosedShell {
    id: String,
}

/// Represents a BREP with voids (holes) in STEP
pub struct BrepWithVoids {
    name: Option<String>,
    outer: Option<ClosedShell>,
    voids: Vec<OrientedClosedShell>,
}

impl BrepWithVoids {
    /// Create a new BrepWithVoids
    pub fn new() -> Self {
        BrepWithVoids {
            name: None,
            outer: None,
            voids: Vec::new(),
        }
    }

    /// Initialize with name, outer shell, and void shells
    pub fn init(
        &mut self,
        name: String,
        outer: ClosedShell,
        voids: Vec<OrientedClosedShell>,
    ) {
        self.name = Some(name);
        self.outer = Some(outer);
        self.voids = voids;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the outer shell
    pub fn outer(&self) -> Option<&ClosedShell> {
        self.outer.as_ref()
    }

    /// Set the outer shell
    pub fn set_outer(&mut self, outer: ClosedShell) {
        self.outer = Some(outer);
    }

    /// Set the voids
    pub fn set_voids(&mut self, voids: Vec<OrientedClosedShell>) {
        self.voids = voids;
    }

    /// Get the voids
    pub fn voids(&self) -> &[OrientedClosedShell] {
        &self.voids
    }

    /// Get a void by index (1-based)
    pub fn voids_value(&self, num: usize) -> Option<&OrientedClosedShell> {
        if num > 0 && num <= self.voids.len() {
            Some(&self.voids[num - 1])
        } else {
            None
        }
    }

    /// Get the number of voids
    pub fn nb_voids(&self) -> usize {
        self.voids.len()
    }
}

impl Default for BrepWithVoids {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let brep = BrepWithVoids::new();
        assert_eq!(brep.name(), None);
        assert_eq!(brep.outer(), None);
        assert_eq!(brep.nb_voids(), 0);
    }

    #[test]
    fn test_init() {
        let mut brep = BrepWithVoids::new();
        let outer = ClosedShell {
            id: "outer".to_string(),
        };
        let void1 = OrientedClosedShell {
            id: "void1".to_string(),
        };
        let void2 = OrientedClosedShell {
            id: "void2".to_string(),
        };
        brep.init(
            "BrepWithVoids1".to_string(),
            outer.clone(),
            vec![void1.clone(), void2.clone()],
        );
        assert_eq!(brep.name(), Some("BrepWithVoids1"));
        assert_eq!(brep.outer(), Some(&outer));
        assert_eq!(brep.nb_voids(), 2);
        assert_eq!(brep.voids_value(1), Some(&void1));
        assert_eq!(brep.voids_value(2), Some(&void2));
    }

    #[test]
    fn test_set_voids() {
        let mut brep = BrepWithVoids::new();
        let void1 = OrientedClosedShell {
            id: "test1".to_string(),
        };
        brep.set_voids(vec![void1.clone()]);
        assert_eq!(brep.nb_voids(), 1);
        assert_eq!(brep.voids_value(1), Some(&void1));
    }
}
