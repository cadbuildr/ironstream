// FILE: graphic3d_type_of_answer.rs
// occt: Graphic3d_TypeOfAnswer

/// The answer to AcceptDisplay queries.
///
/// AcceptDisplay determines whether it is possible to display a specified
/// structure in a specified view, and if so, whether computation is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Graphic3dTypeOfAnswer {
    /// Display is possible without computation
    Yes,
    /// Display is not possible
    No,
    /// Display is possible but requires computation of representation
    Compute,
}

impl Graphic3dTypeOfAnswer {
    /// Returns true if the answer is Yes
    pub fn is_yes(&self) -> bool {
        matches!(self, Graphic3dTypeOfAnswer::Yes)
    }

    /// Returns true if the answer is No
    pub fn is_no(&self) -> bool {
        matches!(self, Graphic3dTypeOfAnswer::No)
    }

    /// Returns true if the answer is Compute
    pub fn is_compute(&self) -> bool {
        matches!(self, Graphic3dTypeOfAnswer::Compute)
    }

    /// Returns a descriptive string for the answer
    pub fn description(&self) -> &'static str {
        match self {
            Graphic3dTypeOfAnswer::Yes => "Display is possible",
            Graphic3dTypeOfAnswer::No => "Display is not possible",
            Graphic3dTypeOfAnswer::Compute => "Display requires computation",
        }
    }
}

impl std::fmt::Display for Graphic3dTypeOfAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Graphic3dTypeOfAnswer::Yes => "Yes",
                Graphic3dTypeOfAnswer::No => "No",
                Graphic3dTypeOfAnswer::Compute => "Compute",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_yes() {
        assert!(Graphic3dTypeOfAnswer::Yes.is_yes());
        assert!(!Graphic3dTypeOfAnswer::No.is_yes());
        assert!(!Graphic3dTypeOfAnswer::Compute.is_yes());
    }

    #[test]
    fn test_is_no() {
        assert!(!Graphic3dTypeOfAnswer::Yes.is_no());
        assert!(Graphic3dTypeOfAnswer::No.is_no());
        assert!(!Graphic3dTypeOfAnswer::Compute.is_no());
    }

    #[test]
    fn test_is_compute() {
        assert!(!Graphic3dTypeOfAnswer::Yes.is_compute());
        assert!(!Graphic3dTypeOfAnswer::No.is_compute());
        assert!(Graphic3dTypeOfAnswer::Compute.is_compute());
    }

    #[test]
    fn test_description() {
        assert_eq!(
            Graphic3dTypeOfAnswer::Yes.description(),
            "Display is possible"
        );
        assert_eq!(
            Graphic3dTypeOfAnswer::No.description(),
            "Display is not possible"
        );
        assert_eq!(
            Graphic3dTypeOfAnswer::Compute.description(),
            "Display requires computation"
        );
    }

    #[test]
    fn test_display_trait() {
        assert_eq!(format!("{}", Graphic3dTypeOfAnswer::Yes), "Yes");
        assert_eq!(format!("{}", Graphic3dTypeOfAnswer::No), "No");
        assert_eq!(format!("{}", Graphic3dTypeOfAnswer::Compute), "Compute");
    }

    #[test]
    fn test_equality() {
        assert_eq!(Graphic3dTypeOfAnswer::Yes, Graphic3dTypeOfAnswer::Yes);
        assert_ne!(Graphic3dTypeOfAnswer::Yes, Graphic3dTypeOfAnswer::No);
        assert_ne!(Graphic3dTypeOfAnswer::No, Graphic3dTypeOfAnswer::Compute);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Graphic3dTypeOfAnswer::Yes);
        set.insert(Graphic3dTypeOfAnswer::No);
        set.insert(Graphic3dTypeOfAnswer::Compute);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&Graphic3dTypeOfAnswer::Yes));
        assert!(set.contains(&Graphic3dTypeOfAnswer::No));
        assert!(set.contains(&Graphic3dTypeOfAnswer::Compute));
    }

    #[test]
    fn test_copy_clone() {
        let answer = Graphic3dTypeOfAnswer::Compute;
        let copied = answer;
        let cloned = answer.clone();

        assert_eq!(answer, copied);
        assert_eq!(answer, cloned);
    }
}
