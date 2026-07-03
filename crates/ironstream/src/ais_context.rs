// FILE: src/ais_context.rs

// occt: AIS_DisplayMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisDisplayMode {
    WireFrame = 0,
    Shaded = 1,
}

// occt: AIS_SelectionModesConcern
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisSelectionMode {
    All = 0,
    Vertex = 1,
    Edge = 2,
    Wire = 3,
    Face = 4,
    Shell = 5,
    Solid = 6,
}

// occt: displayed object record
#[derive(Clone, Debug)]
pub struct AisDisplayedObject {
    label: String,
    display_mode: AisDisplayMode,
    is_selected: bool,
    is_highlighted: bool,
    layer: i32,
}

impl AisDisplayedObject {
    pub fn new(label: &str, display_mode: AisDisplayMode) -> Self {
        AisDisplayedObject {
            label: label.to_string(),
            display_mode,
            is_selected: false,
            is_highlighted: false,
            layer: 0,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn display_mode(&self) -> AisDisplayMode {
        self.display_mode
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn is_highlighted(&self) -> bool {
        self.is_highlighted
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }

    pub fn set_layer(&mut self, layer: i32) {
        self.layer = layer;
    }
}

// occt: AIS_InteractiveContext stub
pub struct AisInteractiveContext {
    objects: Vec<AisDisplayedObject>,
    selected_labels: Vec<String>,
    highlight_color: [f32; 4],
}

impl AisInteractiveContext {
    pub fn new() -> Self {
        AisInteractiveContext {
            objects: Vec::new(),
            selected_labels: Vec::new(),
            highlight_color: [1.0, 1.0, 0.0, 1.0],
        }
    }

    /// Display an object with the given label and mode; returns its index.
    pub fn display(&mut self, label: &str, mode: AisDisplayMode) -> usize {
        // If already present, update mode and return existing index.
        if let Some(idx) = self.objects.iter().position(|o| o.label == label) {
            self.objects[idx].display_mode = mode;
            return idx;
        }
        let idx = self.objects.len();
        self.objects.push(AisDisplayedObject::new(label, mode));
        idx
    }

    /// Remove an object by label; returns true if it was present.
    pub fn remove(&mut self, label: &str) -> bool {
        if let Some(idx) = self.objects.iter().position(|o| o.label == label) {
            self.objects.remove(idx);
            self.selected_labels.retain(|l| l != label);
            true
        } else {
            false
        }
    }

    pub fn nb_displayed(&self) -> usize {
        self.objects.len()
    }

    pub fn object(&self, i: usize) -> &AisDisplayedObject {
        &self.objects[i]
    }

    pub fn select(&mut self, label: &str) {
        if let Some(obj) = self.objects.iter_mut().find(|o| o.label == label) {
            obj.is_selected = true;
            obj.is_highlighted = true;
        }
        if !self.selected_labels.iter().any(|l| l == label) {
            self.selected_labels.push(label.to_string());
        }
    }

    pub fn deselect_all(&mut self) {
        for obj in &mut self.objects {
            obj.is_selected = false;
            obj.is_highlighted = false;
        }
        self.selected_labels.clear();
    }

    pub fn nb_selected(&self) -> usize {
        self.selected_labels.len()
    }

    pub fn selected(&self) -> &[String] {
        &self.selected_labels
    }

    pub fn set_highlight_color(&mut self, color: [f32; 4]) {
        self.highlight_color = color;
    }

    pub fn highlight_color(&self) -> [f32; 4] {
        self.highlight_color
    }
}

impl Default for AisInteractiveContext {
    fn default() -> Self {
        Self::new()
    }
}
