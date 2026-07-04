// FILE: if_select_select_signed_sharing.rs
// occt: IFSelect_SelectSignedSharing

/// Explores the sharing entities of input entities until finding
/// those matching a given signature. Can be limited to a certain level.
#[derive(Clone, Debug)]
pub struct IFSelectSelectSignedSharing {
    signature_text: String,
    is_exact: bool,
    level: i32, // 0 means any level
}

impl IFSelectSelectSignedSharing {
    /// Creates a SelectSignedSharing, defaulted for any level.
    /// level=0 means any level, otherwise limits to that depth.
    pub fn new(signature_text: String, exact: bool, level: i32) -> Self {
        Self {
            signature_text,
            is_exact: exact,
            level,
        }
    }

    /// Returns the signature text to match
    pub fn signature_text(&self) -> &str {
        &self.signature_text
    }

    /// Returns true if match must be exact
    pub fn is_exact(&self) -> bool {
        self.is_exact
    }

    /// Returns the exploration level limit (0 = any)
    pub fn level(&self) -> i32 {
        self.level
    }

    /// Checks if a signature matches according to match rules
    pub fn matches(&self, signature: &str) -> bool {
        if self.is_exact {
            signature == self.signature_text
        } else {
            signature.contains(&self.signature_text)
        }
    }

    /// Returns a text defining the criterium
    pub fn explore_label(&self) -> String {
        let base = if self.is_exact {
            format!("Signature = '{}'", self.signature_text)
        } else {
            format!("Signature contains '{}'", self.signature_text)
        };

        if self.level > 0 {
            format!("{} (level {})", base, self.level)
        } else {
            format!("{} (any level)", base)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sel = IFSelectSelectSignedSharing::new("Type1".to_string(), true, 0);
        assert_eq!(sel.signature_text(), "Type1");
        assert!(sel.is_exact());
        assert_eq!(sel.level(), 0);
    }

    #[test]
    fn test_exact_match() {
        let sel = IFSelectSelectSignedSharing::new("Type1".to_string(), true, 0);
        assert!(sel.matches("Type1"));
        assert!(!sel.matches("Type1Sub"));
    }

    #[test]
    fn test_substring_match() {
        let sel = IFSelectSelectSignedSharing::new("Type".to_string(), false, 0);
        assert!(sel.matches("Type1"));
        assert!(sel.matches("MyType"));
        assert!(!sel.matches("Typo"));
    }

    #[test]
    fn test_with_level() {
        let sel = IFSelectSelectSignedSharing::new("Sig".to_string(), true, 2);
        assert_eq!(sel.level(), 2);
    }

    #[test]
    fn test_explore_label_any_level() {
        let sel = IFSelectSelectSignedSharing::new("Type1".to_string(), true, 0);
        assert_eq!(sel.explore_label(), "Signature = 'Type1' (any level)");
    }

    #[test]
    fn test_explore_label_with_level() {
        let sel = IFSelectSelectSignedSharing::new("Type".to_string(), false, 3);
        assert_eq!(
            sel.explore_label(),
            "Signature contains 'Type' (level 3)"
        );
    }
}
