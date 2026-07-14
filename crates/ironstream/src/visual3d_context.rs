// FILE: src/visual3d_context.rs

// occt-note: display mode — Visual3d_TypeOfSurfaceDetail / V3d_TypeOfVisualization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visual3dContextMode {
    WireFrame,
    Shaded,
    HiddenLine,
    FaceBoundary,
}

// occt-note: highlight style — Graphic3d_HighlightStyle
#[derive(Clone, Debug)]
pub struct Visual3dHighlightStyle {
    color: [f32; 4],
    is_transparent: bool,
    line_width: f32,
}

impl Visual3dHighlightStyle {
    /// Create a new highlight style with the given color; not transparent, line_width = 2.0.
    pub fn new(color: [f32; 4]) -> Self {
        Visual3dHighlightStyle {
            color,
            is_transparent: false,
            line_width: 2.0,
        }
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn is_transparent(&self) -> bool {
        self.is_transparent
    }

    pub fn set_transparent(&mut self, value: bool) {
        self.is_transparent = value;
    }

    pub fn line_width(&self) -> f32 {
        self.line_width
    }
}

// occt-note: displayed structure record — Visual3d_StructureManager entry
#[derive(Clone, Debug)]
pub struct Visual3dDisplayed {
    structure_id: usize,
    mode: Visual3dContextMode,
    is_highlighted: bool,
    is_selected: bool,
    layer: i32,
}

impl Visual3dDisplayed {
    pub fn new(structure_id: usize, mode: Visual3dContextMode) -> Self {
        Visual3dDisplayed {
            structure_id,
            mode,
            is_highlighted: false,
            is_selected: false,
            layer: 0,
        }
    }

    pub fn structure_id(&self) -> usize {
        self.structure_id
    }

    pub fn mode(&self) -> Visual3dContextMode {
        self.mode
    }

    pub fn is_highlighted(&self) -> bool {
        self.is_highlighted
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn highlight(&mut self) {
        self.is_highlighted = true;
    }

    pub fn unhighlight(&mut self) {
        self.is_highlighted = false;
    }

    pub fn select(&mut self) {
        self.is_selected = true;
    }

    pub fn deselect(&mut self) {
        self.is_selected = false;
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }
}

// occt-note: Visual3d_View context stub — manages the set of displayed structures
pub struct Visual3dContext {
    displayed: Vec<Visual3dDisplayed>,
    background_color: [f32; 3],
    highlight_style: Visual3dHighlightStyle,
}

impl Visual3dContext {
    pub fn new() -> Self {
        Visual3dContext {
            displayed: Vec::new(),
            background_color: [0.0, 0.0, 0.0],
            highlight_style: Visual3dHighlightStyle::new([1.0, 1.0, 0.0, 1.0]),
        }
    }

    /// Add a structure with the given id and mode; if already present, updates mode and returns
    /// its index. Returns the index in the internal list.
    pub fn display(&mut self, id: usize, mode: Visual3dContextMode) -> usize {
        if let Some(idx) = self.displayed.iter().position(|d| d.structure_id == id) {
            self.displayed[idx].mode = mode;
            return idx;
        }
        let idx = self.displayed.len();
        self.displayed.push(Visual3dDisplayed::new(id, mode));
        idx
    }

    /// Remove the structure with the given id; returns true if it was present.
    pub fn remove(&mut self, id: usize) -> bool {
        if let Some(idx) = self.displayed.iter().position(|d| d.structure_id == id) {
            self.displayed.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn nb_displayed(&self) -> usize {
        self.displayed.len()
    }

    /// Highlight the structure with the given id (no-op if not displayed).
    pub fn highlight(&mut self, id: usize) {
        if let Some(d) = self.displayed.iter_mut().find(|d| d.structure_id == id) {
            d.highlight();
        }
    }

    /// Mark the structure with the given id as selected (no-op if not displayed).
    pub fn select(&mut self, id: usize) {
        if let Some(d) = self.displayed.iter_mut().find(|d| d.structure_id == id) {
            d.select();
        }
    }

    /// Remove all displayed structures.
    pub fn clear(&mut self) {
        self.displayed.clear();
    }

    pub fn background_color(&self) -> [f32; 3] {
        self.background_color
    }

    pub fn set_background_color(&mut self, color: [f32; 3]) {
        self.background_color = color;
    }
}

impl Default for Visual3dContext {
    fn default() -> Self {
        Self::new()
    }
}
