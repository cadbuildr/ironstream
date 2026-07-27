// FILE: xcaf_vis.rs
// occt: XCAFPrs_Style, XCAFPrs_AISObject, XCAFPrs_DocumentExplorer

use std::collections::HashMap;

/// How a shape's material is displayed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XcafDisplayMode {
    Shaded,
    Wireframe,
    ShadedWithEdges,
    Hidden,
}

impl Default for XcafDisplayMode {
    fn default() -> Self { Self::Shaded }
}

// occt: XCAFPrs_Style // — visual style (color, visibility, transparency) for a shape label
#[derive(Clone, Debug)]
pub struct XcafPrsStyle {
    pub surface_color: Option<[f32; 3]>,
    pub curve_color: Option<[f32; 3]>,
    pub is_visible: bool,
    pub transparency: f32,       // 0.0 = opaque, 1.0 = fully transparent
    pub display_mode: XcafDisplayMode,
    pub line_width: f32,
}

impl Default for XcafPrsStyle {
    fn default() -> Self {
        Self {
            surface_color: None,
            curve_color: None,
            is_visible: true,
            transparency: 0.0,
            display_mode: XcafDisplayMode::Shaded,
            line_width: 1.0,
        }
    }
}

impl XcafPrsStyle {
    pub fn new() -> Self { Self::default() }

    pub fn with_surface_color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.surface_color = Some([r, g, b]);
        self
    }

    pub fn with_curve_color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.curve_color = Some([r, g, b]);
        self
    }

    pub fn with_transparency(mut self, t: f32) -> Self {
        self.transparency = t.clamp(0.0, 1.0);
        self
    }

    pub fn hidden(mut self) -> Self {
        self.is_visible = false;
        self
    }

    pub fn is_transparent(&self) -> bool { self.transparency > 0.0 }
    pub fn is_set(&self) -> bool { self.surface_color.is_some() || self.curve_color.is_some() }

    pub fn has_surface_color(&self) -> bool { self.surface_color.is_some() }
    pub fn has_curve_color(&self) -> bool { self.curve_color.is_some() }

    pub fn unified_with(&self, parent: &XcafPrsStyle) -> XcafPrsStyle {
        XcafPrsStyle {
            surface_color: self.surface_color.or(parent.surface_color),
            curve_color: self.curve_color.or(parent.curve_color),
            is_visible: self.is_visible && parent.is_visible,
            transparency: if self.is_transparent() { self.transparency } else { parent.transparency },
            display_mode: self.display_mode,
            line_width: self.line_width,
        }
    }
}

/// An entry in the document explorer stack.
#[derive(Clone, Debug)]
pub struct ExplorerEntry {
    pub label: u32,
    pub depth: usize,
    pub style: XcafPrsStyle,
    pub instance_path: Vec<u32>,
}

// occt: XCAFPrs_DocumentExplorer // — depth-first traversal of an XCAF assembly
#[derive(Clone, Debug, Default)]
pub struct XcafDocumentExplorer {
    pub doc_id: u32,
    pub entries: Vec<ExplorerEntry>,
    pub cursor: usize,
    pub max_depth: usize,
}

impl XcafDocumentExplorer {
    pub fn new(doc_id: u32, max_depth: usize) -> Self {
        Self { doc_id, entries: Vec::new(), cursor: 0, max_depth }
    }

    pub fn init(&mut self, entries: Vec<ExplorerEntry>) {
        self.entries = entries;
        self.cursor = 0;
    }

    pub fn more(&self) -> bool { self.cursor < self.entries.len() }
    pub fn next(&mut self) { if self.cursor < self.entries.len() { self.cursor += 1; } }

    pub fn current(&self) -> Option<&ExplorerEntry> { self.entries.get(self.cursor) }
    pub fn current_depth(&self) -> usize { self.current().map(|e| e.depth).unwrap_or(0) }

    pub fn nb_roots(&self) -> usize {
        self.entries.iter().filter(|e| e.depth == 0).count()
    }

    pub fn collect_leaves(&self) -> Vec<u32> {
        // A leaf is an entry whose label doesn't appear as anyone's instance_path parent
        let all_labels: std::collections::HashSet<u32> = self.entries.iter().map(|e| e.label).collect();
        self.entries.iter()
            .filter(|e| !self.entries.iter().any(|o| o.instance_path.contains(&e.label) && o.depth > e.depth))
            .map(|e| e.label)
            .collect()
    }
}

// occt: XCAFPrs_AISObject // — AIS presentation built from an XCAF shape label + style
#[derive(Clone, Debug)]
pub struct XcafPrsAISObject {
    pub label: u32,
    pub doc_id: u32,
    pub style: XcafPrsStyle,
    pub display_mode: XcafDisplayMode,
    pub is_displayed: bool,
    pub sub_styles: HashMap<u32, XcafPrsStyle>,
}

impl XcafPrsAISObject {
    pub fn new(label: u32, doc_id: u32) -> Self {
        Self {
            label,
            doc_id,
            style: XcafPrsStyle::default(),
            display_mode: XcafDisplayMode::Shaded,
            is_displayed: false,
            sub_styles: HashMap::new(),
        }
    }

    pub fn set_style(&mut self, style: XcafPrsStyle) { self.style = style; }
    pub fn set_sub_style(&mut self, sub_label: u32, style: XcafPrsStyle) {
        self.sub_styles.insert(sub_label, style);
    }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }

    pub fn effective_style(&self, sub_label: u32) -> XcafPrsStyle {
        if let Some(sub) = self.sub_styles.get(&sub_label) {
            sub.unified_with(&self.style)
        } else {
            self.style.clone()
        }
    }

    pub fn is_visible(&self) -> bool { self.is_displayed && self.style.is_visible }
    pub fn nb_sub_styles(&self) -> usize { self.sub_styles.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xcaf_prs_style_builder() {
        let s = XcafPrsStyle::new()
            .with_surface_color(1.0, 0.0, 0.0)
            .with_transparency(0.5);
        assert!(s.has_surface_color());
        assert!(s.is_transparent());
        assert_eq!(s.surface_color.unwrap(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn xcaf_prs_style_unified() {
        let parent = XcafPrsStyle::new().with_surface_color(0.5, 0.5, 0.5);
        let child = XcafPrsStyle::new();
        let unified = child.unified_with(&parent);
        assert_eq!(unified.surface_color, parent.surface_color);
    }

    #[test]
    fn xcaf_prs_style_hidden() {
        let s = XcafPrsStyle::new().hidden();
        assert!(!s.is_visible);
        assert!(!s.is_set()); // no color set
    }

    #[test]
    fn xcaf_ais_object_visibility() {
        let mut obj = XcafPrsAISObject::new(42, 1);
        assert!(!obj.is_visible());
        obj.display();
        assert!(obj.is_visible());
        obj.erase();
        assert!(!obj.is_visible());
    }

    #[test]
    fn xcaf_ais_object_sub_style() {
        let mut obj = XcafPrsAISObject::new(1, 1);
        obj.set_style(XcafPrsStyle::new().with_surface_color(1.0, 1.0, 1.0));
        let sub = XcafPrsStyle::new().with_surface_color(0.0, 0.0, 1.0);
        obj.set_sub_style(99, sub);
        let eff = obj.effective_style(99);
        assert_eq!(eff.surface_color.unwrap(), [0.0, 0.0, 1.0]);
        // Sub-label not overriding falls back to parent style
        let fallback = obj.effective_style(100);
        assert_eq!(fallback.surface_color.unwrap(), [1.0, 1.0, 1.0]);
    }

    #[test]
    fn xcaf_explorer_traversal() {
        let mut exp = XcafDocumentExplorer::new(1, 10);
        exp.init(vec![
            ExplorerEntry { label: 10, depth: 0, style: Default::default(), instance_path: vec![] },
            ExplorerEntry { label: 20, depth: 1, style: Default::default(), instance_path: vec![10] },
        ]);
        assert!(exp.more());
        assert_eq!(exp.current().unwrap().label, 10);
        exp.next();
        assert_eq!(exp.current_depth(), 1);
        assert_eq!(exp.nb_roots(), 1);
    }
}
