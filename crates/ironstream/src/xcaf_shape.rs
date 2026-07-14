// FILE: rust/ironstream/crates/ironstream/src/xcaf_shape.rs

// occt-note: XDE shape entry — mirrors the per-label metadata stored by XCAFDoc_ShapeTool
#[derive(Clone, Debug)]
pub struct XcafShapeEntry {
    label: String,
    name: String,
    is_free: bool,
    is_simple: bool,
    is_assembly: bool,
    is_component: bool,
    is_compound: bool,
    nb_components: usize,
}

impl XcafShapeEntry {
    /// Create a new shape entry with the given label and name.
    /// All boolean flags default to false and nb_components to 0.
    pub fn new(label: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: name.into(),
            is_free: false,
            is_simple: false,
            is_assembly: false,
            is_component: false,
            is_compound: false,
            nb_components: 0,
        }
    }

    /// Return the label string for this entry.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the human-readable name for this entry.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return true if this shape is a free shape (not referenced by any assembly).
    pub fn is_free(&self) -> bool {
        self.is_free
    }

    /// Return true if this shape is a simple (non-compound) shape.
    pub fn is_simple(&self) -> bool {
        self.is_simple
    }

    /// Return true if this shape is an assembly.
    pub fn is_assembly(&self) -> bool {
        self.is_assembly
    }

    /// Return true if this shape is a component of an assembly.
    pub fn is_component(&self) -> bool {
        self.is_component
    }

    /// Return true if this shape is a compound.
    pub fn is_compound(&self) -> bool {
        self.is_compound
    }

    /// Return the number of direct components this assembly contains.
    pub fn nb_components(&self) -> usize {
        self.nb_components
    }

    /// Mark this entry as a free shape.
    pub fn mark_as_free(&mut self) {
        self.is_free = true;
    }

    /// Mark this entry as an assembly.
    pub fn mark_as_assembly(&mut self) {
        self.is_assembly = true;
    }

    /// Set the component count for this assembly entry.
    pub fn set_nb_components(&mut self, n: usize) {
        self.nb_components = n;
    }
}

// occt-ref: XCAFDoc_ShapeTool // — shape label management; owns a flat list of shape entries
pub struct XcafShapeTool {
    entries: Vec<XcafShapeEntry>,
}

impl XcafShapeTool {
    /// Create an empty shape tool.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Append an entry and return its index.
    pub fn add_shape(&mut self, entry: XcafShapeEntry) -> usize {
        let idx = self.entries.len();
        self.entries.push(entry);
        idx
    }

    /// Find the first entry whose label matches. Returns its index if found.
    pub fn find_shape(&self, label: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.label == label)
    }

    /// Return a reference to the entry at the given index, or None if out of bounds.
    pub fn get_shape(&self, idx: usize) -> Option<&XcafShapeEntry> {
        self.entries.get(idx)
    }

    /// Return the total number of shape entries.
    pub fn nb_shapes(&self) -> usize {
        self.entries.len()
    }

    /// Return the indices of all entries that are marked as free shapes.
    pub fn free_shapes(&self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if e.is_free { Some(i) } else { None })
            .collect()
    }

    /// Return true if the entry at the given index is a free shape.
    /// Returns false when the index is out of bounds.
    pub fn is_free(&self, idx: usize) -> bool {
        self.entries.get(idx).map_or(false, |e| e.is_free)
    }

    /// Remove the entry at the given index (shifting subsequent entries).
    /// Returns true on success, false if the index is out of bounds.
    pub fn remove_shape(&mut self, idx: usize) -> bool {
        if idx < self.entries.len() {
            self.entries.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Default for XcafShapeTool {
    fn default() -> Self {
        Self::new()
    }
}
