// FILE: d_data_std_tree_browser.rs
// occt: DDataStd_TreeBrowser

//! Tree browser for DDataStd attributes.

/// DDataStd_TreeBrowser: browse DDataStd tree.
#[derive(Clone, Debug)]
pub struct DDataStdTreeBrowser {
    root: String,
    expanded: bool,
}

impl DDataStdTreeBrowser {
    /// Create a new tree browser.
    pub fn new(root: &str) -> Self {
        DDataStdTreeBrowser {
            root: root.to_string(),
            expanded: false,
        }
    }

    /// Expand the tree.
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Check if expanded.
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_creation() {
        let browser = DDataStdTreeBrowser::new("root");
        assert_eq!(browser.root, "root");
        assert!(!browser.is_expanded());
    }

    #[test]
    fn test_expand() {
        let mut browser = DDataStdTreeBrowser::new("root");
        browser.expand();
        assert!(browser.is_expanded());
    }
}
