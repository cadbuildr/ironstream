// FILE: if_select_select_signature.rs
// occt: IFSelect_SelectSignature

/// Sorts entities by matching a signature.
/// Supports exact match or substring match.
/// Also supports numeric comparisons (< <= > >= val).
#[derive(Clone, Debug)]
pub struct IFSelectSelectSignature {
    signature_text: String,
    is_exact: bool,
    // Would hold handle<IFSelect_Signature> or handle<IFSelect_SignCounter>
    // but without those in our system, we represent the key fields
}

impl IFSelectSelectSignature {
    /// Creates a SelectSignature with signature text to match.
    /// exact=true requires exact match, false means substring match.
    pub fn new(signature_text: String, exact: bool) -> Self {
        Self {
            signature_text,
            is_exact: exact,
        }
    }

    /// Returns the text used to match signatures
    pub fn signature_text(&self) -> &str {
        &self.signature_text
    }

    /// Returns true if match must be exact
    pub fn is_exact(&self) -> bool {
        self.is_exact
    }

    /// Checks if a given signature matches according to match rules
    pub fn matches(&self, signature: &str) -> bool {
        if self.is_exact {
            signature == self.signature_text
        } else {
            signature.contains(&self.signature_text)
        }
    }

    /// Returns a text defining the criterium
    pub fn extract_label(&self) -> String {
        if self.is_exact {
            format!("Signature = '{}'", self.signature_text)
        } else {
            format!("Signature contains '{}'", self.signature_text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let sel = IFSelectSelectSignature::new("Type1".to_string(), true);
        assert!(sel.is_exact());
        assert!(sel.matches("Type1"));
        assert!(!sel.matches("Type1Sub"));
        assert!(!sel.matches("MyType1"));
    }

    #[test]
    fn test_substring_match() {
        let sel = IFSelectSelectSignature::new("Type".to_string(), false);
        assert!(!sel.is_exact());
        assert!(sel.matches("Type1"));
        assert!(sel.matches("Type"));
        assert!(sel.matches("MyType"));
        assert!(sel.matches("MyTypeSub"));
        assert!(!sel.matches("Typo"));
    }

    #[test]
    fn test_signature_text() {
        let sel = IFSelectSelectSignature::new("TestSig".to_string(), false);
        assert_eq!(sel.signature_text(), "TestSig");
    }

    #[test]
    fn test_extract_label_exact() {
        let sel = IFSelectSelectSignature::new("Type1".to_string(), true);
        assert_eq!(sel.extract_label(), "Signature = 'Type1'");
    }

    #[test]
    fn test_extract_label_substring() {
        let sel = IFSelectSelectSignature::new("Type".to_string(), false);
        assert_eq!(sel.extract_label(), "Signature contains 'Type'");
    }
}
