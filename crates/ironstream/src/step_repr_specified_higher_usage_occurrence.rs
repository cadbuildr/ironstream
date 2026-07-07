// FILE: step_repr_specified_higher_usage_occurrence.rs
// occt: StepRepr_SpecifiedHigherUsageOccurrence

/// Placeholder for AssemblyComponentUsage
#[derive(Clone, Debug, PartialEq)]
pub struct AssemblyComponentUsage {
    id: String,
}

/// Placeholder for NextAssemblyUsageOccurrence
#[derive(Clone, Debug, PartialEq)]
pub struct NextAssemblyUsageOccurrence {
    id: String,
}

/// Placeholder for ProductDefinition
#[derive(Clone, Debug, PartialEq)]
pub struct ProductDefinition {
    id: String,
}

/// Represents a specified higher usage occurrence in an assembly (STEP).
pub struct SpecifiedHigherUsageOccurrence {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    reference_designator: Option<String>,
    upper_usage: Option<AssemblyComponentUsage>,
    next_usage: Option<NextAssemblyUsageOccurrence>,
}

impl SpecifiedHigherUsageOccurrence {
    /// Create a new SpecifiedHigherUsageOccurrence
    pub fn new() -> Self {
        SpecifiedHigherUsageOccurrence {
            id: None,
            name: None,
            description: None,
            reference_designator: None,
            upper_usage: None,
            next_usage: None,
        }
    }

    /// Initialize with all fields
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        reference_designator: Option<String>,
        upper_usage: AssemblyComponentUsage,
        next_usage: NextAssemblyUsageOccurrence,
    ) {
        self.id = Some(id);
        self.name = Some(name);
        if has_description {
            self.description = description;
        }
        self.reference_designator = reference_designator;
        self.upper_usage = Some(upper_usage);
        self.next_usage = Some(next_usage);
    }

    /// Get the upper usage
    pub fn upper_usage(&self) -> Option<&AssemblyComponentUsage> {
        self.upper_usage.as_ref()
    }

    /// Set the upper usage
    pub fn set_upper_usage(&mut self, upper_usage: AssemblyComponentUsage) {
        self.upper_usage = Some(upper_usage);
    }

    /// Get the next usage
    pub fn next_usage(&self) -> Option<&NextAssemblyUsageOccurrence> {
        self.next_usage.as_ref()
    }

    /// Set the next usage
    pub fn set_next_usage(&mut self, next_usage: NextAssemblyUsageOccurrence) {
        self.next_usage = Some(next_usage);
    }

    /// Get the id
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set the id
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

impl Default for SpecifiedHigherUsageOccurrence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let shu = SpecifiedHigherUsageOccurrence::new();
        assert_eq!(shu.id(), None);
        assert_eq!(shu.name(), None);
        assert_eq!(shu.upper_usage(), None);
        assert_eq!(shu.next_usage(), None);
    }

    #[test]
    fn test_init() {
        let mut shu = SpecifiedHigherUsageOccurrence::new();
        let upper = AssemblyComponentUsage {
            id: "upper1".to_string(),
        };
        let next = NextAssemblyUsageOccurrence {
            id: "next1".to_string(),
        };
        shu.init(
            "id1".to_string(),
            "name1".to_string(),
            true,
            Some("desc".to_string()),
            Some("ref_des".to_string()),
            upper.clone(),
            next.clone(),
        );
        assert_eq!(shu.id(), Some("id1"));
        assert_eq!(shu.name(), Some("name1"));
        assert_eq!(shu.description(), Some("desc"));
        assert_eq!(shu.upper_usage(), Some(&upper));
        assert_eq!(shu.next_usage(), Some(&next));
    }

    #[test]
    fn test_set_and_get_usage() {
        let mut shu = SpecifiedHigherUsageOccurrence::new();
        let upper = AssemblyComponentUsage {
            id: "upper_test".to_string(),
        };
        let next = NextAssemblyUsageOccurrence {
            id: "next_test".to_string(),
        };
        shu.set_upper_usage(upper.clone());
        shu.set_next_usage(next.clone());
        assert_eq!(shu.upper_usage(), Some(&upper));
        assert_eq!(shu.next_usage(), Some(&next));
    }
}
