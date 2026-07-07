// FILE: if_select_signature_list.rs
// occt: IFSelect_SignatureList

/// Manages a list of signatures.
#[derive(Clone, Debug)]
pub struct IFSelectSignatureList {
    signatures: Vec<String>,
}

impl IFSelectSignatureList {
    /// Creates an empty SignatureList
    pub fn new() -> Self {
        Self {
            signatures: Vec::new(),
        }
    }

    /// Adds a signature to the list
    pub fn add(&mut self, signature: String) {
        self.signatures.push(signature);
    }

    /// Returns the count of signatures
    pub fn count(&self) -> usize {
        self.signatures.len()
    }

    /// Returns a signature by index (1-indexed)
    pub fn item(&self, num: usize) -> Option<&str> {
        if num >= 1 && num <= self.signatures.len() {
            Some(&self.signatures[num - 1])
        } else {
            None
        }
    }

    /// Clears all signatures
    pub fn clear(&mut self) {
        self.signatures.clear();
    }
}

impl Default for IFSelectSignatureList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = IFSelectSignatureList::new();
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_add() {
        let mut list = IFSelectSignatureList::new();
        list.add("sig1".to_string());
        list.add("sig2".to_string());
        assert_eq!(list.count(), 2);
        assert_eq!(list.item(1), Some("sig1"));
        assert_eq!(list.item(2), Some("sig2"));
    }

    #[test]
    fn test_item_out_of_range() {
        let list = IFSelectSignatureList::new();
        assert_eq!(list.item(1), None);
        assert_eq!(list.item(0), None);
    }

    #[test]
    fn test_clear() {
        let mut list = IFSelectSignatureList::new();
        list.add("sig1".to_string());
        list.add("sig2".to_string());
        assert_eq!(list.count(), 2);
        list.clear();
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_default() {
        let list = IFSelectSignatureList::default();
        assert_eq!(list.count(), 0);
    }
}
