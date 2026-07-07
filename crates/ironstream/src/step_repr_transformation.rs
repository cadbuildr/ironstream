// FILE: step_repr_transformation.rs
// occt: StepRepr_Transformation

/// Placeholder for ItemDefinedTransformation
#[derive(Clone, Debug, PartialEq)]
pub struct ItemDefinedTransformation {
    id: String,
}

/// Placeholder for FunctionallyDefinedTransformation
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionallyDefinedTransformation {
    id: String,
}

/// SELECT type that can contain various transformation types
#[derive(Clone, Debug, PartialEq)]
pub enum Transformation {
    ItemDefinedTransformation(ItemDefinedTransformation),
    FunctionallyDefinedTransformation(FunctionallyDefinedTransformation),
}

impl Transformation {
    /// Create a new transformation
    pub fn new() -> Self {
        Transformation::ItemDefinedTransformation(ItemDefinedTransformation {
            id: String::new(),
        })
    }

    /// Get the case number
    pub fn case_num(&self) -> i32 {
        match self {
            Transformation::ItemDefinedTransformation(_) => 1,
            Transformation::FunctionallyDefinedTransformation(_) => 2,
        }
    }

    /// Get as ItemDefinedTransformation if applicable
    pub fn item_defined_transformation(&self) -> Option<&ItemDefinedTransformation> {
        match self {
            Transformation::ItemDefinedTransformation(t) => Some(t),
            _ => None,
        }
    }

    /// Get as FunctionallyDefinedTransformation if applicable
    pub fn functionally_defined_transformation(&self) -> Option<&FunctionallyDefinedTransformation> {
        match self {
            Transformation::FunctionallyDefinedTransformation(t) => Some(t),
            _ => None,
        }
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_defined_transformation() {
        let trans = ItemDefinedTransformation {
            id: "idt1".to_string(),
        };
        let sel = Transformation::ItemDefinedTransformation(trans.clone());
        assert_eq!(sel.case_num(), 1);
        assert_eq!(sel.item_defined_transformation(), Some(&trans));
        assert_eq!(sel.functionally_defined_transformation(), None);
    }

    #[test]
    fn test_functionally_defined_transformation() {
        let trans = FunctionallyDefinedTransformation {
            id: "fdt1".to_string(),
        };
        let sel = Transformation::FunctionallyDefinedTransformation(trans.clone());
        assert_eq!(sel.case_num(), 2);
        assert_eq!(sel.functionally_defined_transformation(), Some(&trans));
        assert_eq!(sel.item_defined_transformation(), None);
    }

    #[test]
    fn test_default_is_item_defined() {
        let trans = Transformation::default();
        assert_eq!(trans.case_num(), 1);
        assert!(trans.item_defined_transformation().is_some());
    }
}
