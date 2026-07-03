// FILE: bop_algo_section_attribute.rs
// occt: BOPAlgo_SectionAttribute

/// Section attribute for Boolean operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BopAlgoSectionAttribute {
    keep_contact: bool,
    ancestor_inc: bool,
}

impl BopAlgoSectionAttribute {
    pub fn new() -> Self {
        BopAlgoSectionAttribute {
            keep_contact: false,
            ancestor_inc: false,
        }
    }

    pub fn with_flags(keep: bool, anc: bool) -> Self {
        BopAlgoSectionAttribute {
            keep_contact: keep,
            ancestor_inc: anc,
        }
    }

    pub fn keep_contact(&self) -> bool {
        self.keep_contact
    }

    pub fn set_keep_contact(&mut self, val: bool) {
        self.keep_contact = val;
    }

    pub fn ancestor_inc(&self) -> bool {
        self.ancestor_inc
    }

    pub fn set_ancestor_inc(&mut self, val: bool) {
        self.ancestor_inc = val;
    }
}

impl Default for BopAlgoSectionAttribute {
    fn default() -> Self {
        BopAlgoSectionAttribute::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let attr = BopAlgoSectionAttribute::new();
        assert_eq!(attr.keep_contact(), false);
        assert_eq!(attr.ancestor_inc(), false);
    }

    #[test]
    fn test_with_flags() {
        let attr = BopAlgoSectionAttribute::with_flags(true, false);
        assert_eq!(attr.keep_contact(), true);
        assert_eq!(attr.ancestor_inc(), false);
    }

    #[test]
    fn test_set_flags() {
        let mut attr = BopAlgoSectionAttribute::new();
        attr.set_keep_contact(true);
        attr.set_ancestor_inc(true);
        assert_eq!(attr.keep_contact(), true);
        assert_eq!(attr.ancestor_inc(), true);
    }
}
