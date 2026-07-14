// FILE: rust/ironstream/crates/ironstream/src/tdf_label.rs

use std::collections::HashMap;

// occt: TDF_Label // — a node in the OCAF label tree
/// A single node in the OCAF attribute/data tree.
///
/// Each entry holds a numeric `tag`, a dot-colon entry path string (e.g. `"0:1:2"`),
/// a human-readable `name`, a list of direct child tags, and a string-keyed
/// attribute map.
#[derive(Debug, Clone)]
pub struct TdfEntry {
    tag: u32,
    entry: String,
    name: String,
    children: Vec<u32>,
    attrs: HashMap<String, String>,
}

impl TdfEntry {
    /// Create a new label node with the given tag, entry path string, and name.
    pub fn new(tag: u32, entry: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            tag,
            entry: entry.into(),
            name: name.into(),
            children: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    /// Return the numeric tag that identifies this node among its siblings.
    pub fn tag(&self) -> u32 {
        self.tag
    }

    /// Return the entry path string (e.g. `"0:1:2"`).
    pub fn entry(&self) -> &str {
        &self.entry
    }

    /// Return the human-readable label name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Overwrite the human-readable label name.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    /// Return `true` when this label has no name set (mirrors `TDF_Label::IsNull`).
    pub fn is_null(&self) -> bool {
        self.name.is_empty()
    }

    /// Register `tag` as a direct child of this label.
    pub fn add_child(&mut self, tag: u32) {
        if !self.children.contains(&tag) {
            self.children.push(tag);
        }
    }

    /// Return the slice of direct child tags in insertion order.
    pub fn children(&self) -> &[u32] {
        &self.children
    }

    /// Set or overwrite a string attribute.
    pub fn set_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(key.to_owned(), value.to_owned());
    }

    /// Look up a string attribute by key.
    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|s| s.as_str())
    }

    /// Return the number of attributes stored on this label.
    pub fn nb_attrs(&self) -> usize {
        self.attrs.len()
    }
}

// ---------------------------------------------------------------------------

// occt-ref: TDF_Data // — the label tree (document root)
/// The whole OCAF label tree rooted at entry `"0"`.
///
/// Labels are stored in a flat `Vec` and looked up by their entry string.
/// Tags are allocated monotonically from `next_tag`.
#[derive(Debug, Clone)]
pub struct TdfData {
    labels: Vec<TdfEntry>,
    next_tag: u32,
}

impl TdfData {
    /// Create a new, empty data tree.  A root label with entry `"0"` is
    /// inserted automatically.
    pub fn new() -> Self {
        let root = TdfEntry::new(0, "0", "");
        Self {
            labels: vec![root],
            next_tag: 1,
        }
    }

    /// Return a reference to the root label (entry `"0"`).
    pub fn root(&self) -> &TdfEntry {
        // The root is always the first element and always present.
        &self.labels[0]
    }

    /// Create a new child label under `parent_entry` and return its entry string.
    ///
    /// The new entry string is formed by appending the allocated tag to the
    /// parent entry, e.g. `"0"` → `"0:1"`, `"0:1"` → `"0:1:2"`.
    ///
    /// Returns an empty string if `parent_entry` does not exist in the tree.
    pub fn new_label(&mut self, parent_entry: &str, name: &str) -> String {
        // Check that the parent exists.
        let parent_idx = self.labels.iter().position(|l| l.entry == parent_entry);
        let Some(parent_idx) = parent_idx else {
            return String::new();
        };

        let tag = self.next_tag;
        self.next_tag += 1;

        let new_entry = format!("{}:{}", parent_entry, tag);
        let label = TdfEntry::new(tag, new_entry.clone(), name);
        self.labels.push(label);

        // Register the child tag on the parent.
        self.labels[parent_idx].add_child(tag);

        new_entry
    }

    /// Find a label by its entry string, returning an immutable reference.
    pub fn find_label(&self, entry: &str) -> Option<&TdfEntry> {
        self.labels.iter().find(|l| l.entry == entry)
    }

    /// Find a label by its entry string, returning a mutable reference.
    pub fn find_label_mut(&mut self, entry: &str) -> Option<&mut TdfEntry> {
        self.labels.iter_mut().find(|l| l.entry == entry)
    }

    /// Return the total number of labels in the tree (including the root).
    pub fn nb_labels(&self) -> usize {
        self.labels.len()
    }

    /// Set or overwrite a string attribute on the label identified by `entry`.
    ///
    /// Returns `true` on success, `false` if the label was not found.
    pub fn set_attr(&mut self, entry: &str, key: &str, value: &str) -> bool {
        match self.find_label_mut(entry) {
            Some(label) => {
                label.set_attr(key, value);
                true
            }
            None => false,
        }
    }
}

impl Default for TdfData {
    fn default() -> Self {
        Self::new()
    }
}
