// FILE: crates/ironstream/src/topo_tools.rs

// occt: TopTools_MapOfShape
pub struct TopToolsMapOfShape {
    entries: std::collections::HashSet<String>,
}

impl TopToolsMapOfShape {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashSet::new(),
        }
    }

    /// Returns true if the name was newly added (was not already present).
    pub fn add(&mut self, name: &str) -> bool {
        self.entries.insert(name.to_string())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains(name)
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.entries.remove(name)
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for TopToolsMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

// occt: TopTools_ListOfShape
pub struct TopToolsListOfShape {
    shapes: std::collections::VecDeque<String>,
}

impl TopToolsListOfShape {
    pub fn new() -> Self {
        Self {
            shapes: std::collections::VecDeque::new(),
        }
    }

    pub fn append(&mut self, s: String) {
        self.shapes.push_back(s);
    }

    pub fn prepend(&mut self, s: String) {
        self.shapes.push_front(s);
    }

    pub fn first(&self) -> Option<&str> {
        self.shapes.front().map(|s| s.as_str())
    }

    pub fn last(&self) -> Option<&str> {
        self.shapes.back().map(|s| s.as_str())
    }

    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    pub fn remove_first(&mut self) -> Option<String> {
        self.shapes.pop_front()
    }
}

impl Default for TopToolsListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

// occt: TopTools_SequenceOfShape
pub struct TopToolsSequenceOfShape {
    shapes: Vec<String>,
}

impl TopToolsSequenceOfShape {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn append(&mut self, s: String) {
        self.shapes.push(s);
    }

    pub fn prepend(&mut self, s: String) {
        self.shapes.insert(0, s);
    }

    /// 1-based index access.
    pub fn value(&self, idx: usize) -> Option<&str> {
        if idx == 0 {
            return None;
        }
        self.shapes.get(idx - 1).map(|s| s.as_str())
    }

    /// 1-based index set. No-op if out of range.
    pub fn set_value(&mut self, idx: usize, s: String) {
        if idx == 0 {
            return;
        }
        if let Some(slot) = self.shapes.get_mut(idx - 1) {
            *slot = s;
        }
    }

    pub fn length(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// 1-based index removal. No-op if out of range.
    pub fn remove(&mut self, idx: usize) {
        if idx == 0 || idx > self.shapes.len() {
            return;
        }
        self.shapes.remove(idx - 1);
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    pub fn first(&self) -> Option<&str> {
        self.shapes.first().map(|s| s.as_str())
    }

    pub fn last(&self) -> Option<&str> {
        self.shapes.last().map(|s| s.as_str())
    }
}

impl Default for TopToolsSequenceOfShape {
    fn default() -> Self {
        Self::new()
    }
}

// occt: TopTools_HSequenceOfShape
pub struct TopToolsHSequenceOfShape {
    inner: TopToolsSequenceOfShape,
}

impl TopToolsHSequenceOfShape {
    pub fn new() -> Self {
        Self {
            inner: TopToolsSequenceOfShape::new(),
        }
    }

    pub fn append(&mut self, s: String) {
        self.inner.append(s);
    }

    pub fn prepend(&mut self, s: String) {
        self.inner.prepend(s);
    }

    pub fn value(&self, idx: usize) -> Option<&str> {
        self.inner.value(idx)
    }

    pub fn set_value(&mut self, idx: usize, s: String) {
        self.inner.set_value(idx, s);
    }

    pub fn length(&self) -> usize {
        self.inner.length()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn remove(&mut self, idx: usize) {
        self.inner.remove(idx);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn first(&self) -> Option<&str> {
        self.inner.first()
    }

    pub fn last(&self) -> Option<&str> {
        self.inner.last()
    }
}

impl Default for TopToolsHSequenceOfShape {
    fn default() -> Self {
        Self::new()
    }
}

// occt: TopTools_ShapeSet
pub struct TopToolsShapeSet {
    shapes: Vec<String>,
}

impl TopToolsShapeSet {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    /// Adds a shape name and returns its 1-based index.
    /// If the name already exists, returns its existing index.
    pub fn add(&mut self, s: String) -> usize {
        if let Some(pos) = self.shapes.iter().position(|x| x == &s) {
            return pos + 1;
        }
        self.shapes.push(s);
        self.shapes.len()
    }

    /// Returns the 1-based index of the shape name, or None if not found.
    pub fn find(&self, s: &str) -> Option<usize> {
        self.shapes.iter().position(|x| x == s).map(|i| i + 1)
    }

    /// Returns the shape name at the given 1-based index.
    pub fn shape(&self, idx: usize) -> Option<&str> {
        if idx == 0 {
            return None;
        }
        self.shapes.get(idx - 1).map(|s| s.as_str())
    }

    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Default for TopToolsShapeSet {
    fn default() -> Self {
        Self::new()
    }
}
