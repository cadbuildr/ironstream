// FILE: prs3d_root.rs
// occt: Prs3d_Root, Prs3d_Drawer, Prs3d_Presentation,
//       Prs3d_PresentationShadow

/// Presentation mode enumeration.
/// occt: Prs3d_DisplayMode (subset)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Prs3dDisplayMode {
    #[default]
    Wireframe,
    Shading,
    ShadingWithEdges,
    HiddenLine,
}

impl Prs3dDisplayMode {
    pub fn is_shaded(&self) -> bool { matches!(self, Self::Shading | Self::ShadingWithEdges) }
    pub fn shows_edges(&self) -> bool { matches!(self, Self::ShadingWithEdges | Self::HiddenLine) }
}

/// Presentation attribute drawer: consolidates all graphic attributes.
/// occt: Prs3d_Drawer
#[derive(Clone, Debug)]
pub struct Prs3dDrawer {
    pub drawer_id: u32,
    pub line_width: f64,
    pub point_size: f64,
    pub transparency: f32,
    pub display_mode: Prs3dDisplayMode,
    pub deduce_mode: bool,
}

impl Default for Prs3dDrawer {
    fn default() -> Self {
        Self {
            drawer_id: 0,
            line_width: 1.0,
            point_size: 5.0,
            transparency: 0.0,
            display_mode: Prs3dDisplayMode::Shading,
            deduce_mode: false,
        }
    }
}

impl Prs3dDrawer {
    pub fn new(id: u32) -> Self { Self { drawer_id: id, ..Self::default() } }

    pub fn set_line_width(&mut self, w: f64) { self.line_width = w.max(0.1); }
    pub fn set_point_size(&mut self, s: f64) { self.point_size = s.max(1.0); }
    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_display_mode(&mut self, m: Prs3dDisplayMode) { self.display_mode = m; }

    pub fn line_width(&self) -> f64 { self.line_width }
    pub fn point_size(&self) -> f64 { self.point_size }
    pub fn is_transparent(&self) -> bool { self.transparency > 0.0 }
}

/// Presentation: group of drawable primitives.
/// occt: Prs3d_Presentation
#[derive(Clone, Debug)]
pub struct Prs3dPresentation {
    pub pres_id: u32,
    pub object_id: u32,
    pub display_mode: Prs3dDisplayMode,
    pub is_visible: bool,
    pub is_highlighted: bool,
    pub layer_index: i32,
    pub primitive_count: u32,
}

impl Prs3dPresentation {
    pub fn new(pres_id: u32, obj_id: u32) -> Self {
        Self {
            pres_id,
            object_id: obj_id,
            display_mode: Prs3dDisplayMode::Shading,
            is_visible: false,
            is_highlighted: false,
            layer_index: 0,
            primitive_count: 0,
        }
    }

    pub fn display(&mut self) { self.is_visible = true; }
    pub fn erase(&mut self) { self.is_visible = false; self.is_highlighted = false; }
    pub fn highlight(&mut self) { self.is_highlighted = true; }
    pub fn unhighlight(&mut self) { self.is_highlighted = false; }

    pub fn set_display_mode(&mut self, m: Prs3dDisplayMode) { self.display_mode = m; }
    pub fn set_layer(&mut self, l: i32) { self.layer_index = l; }

    pub fn add_primitive(&mut self) { self.primitive_count += 1; }
    pub fn nb_primitives(&self) -> u32 { self.primitive_count }
}

/// Presentation shadow: lightweight overlay.
/// occt: Prs3d_PresentationShadow
#[derive(Clone, Debug)]
pub struct Prs3dPresentationShadow {
    pub shadow_id: u32,
    pub parent_pres_id: u32,
    pub override_mode: Option<Prs3dDisplayMode>,
    pub override_visibility: Option<bool>,
}

impl Prs3dPresentationShadow {
    pub fn new(shadow_id: u32, parent_id: u32) -> Self {
        Self {
            shadow_id,
            parent_pres_id: parent_id,
            override_mode: None,
            override_visibility: None,
        }
    }

    pub fn set_mode_override(&mut self, m: Prs3dDisplayMode) { self.override_mode = Some(m); }
    pub fn clear_mode_override(&mut self) { self.override_mode = None; }

    pub fn set_visibility_override(&mut self, v: bool) { self.override_visibility = Some(v); }
    pub fn clear_visibility_override(&mut self) { self.override_visibility = None; }

    pub fn has_overrides(&self) -> bool {
        self.override_mode.is_some() || self.override_visibility.is_some()
    }
}

/// Root presentation manager.
/// occt: Prs3d_Root
#[derive(Clone, Debug, Default)]
pub struct Prs3dRoot {
    pub presentations: Vec<Prs3dPresentation>,
    pub shadows: Vec<Prs3dPresentationShadow>,
    pub drawers: Vec<Prs3dDrawer>,
}

impl Prs3dRoot {
    pub fn new() -> Self { Self::default() }

    pub fn add_presentation(&mut self, p: Prs3dPresentation) { self.presentations.push(p); }
    pub fn add_shadow(&mut self, s: Prs3dPresentationShadow) { self.shadows.push(s); }
    pub fn add_drawer(&mut self, d: Prs3dDrawer) { self.drawers.push(d); }

    pub fn find_presentation(&self, id: u32) -> Option<&Prs3dPresentation> {
        self.presentations.iter().find(|p| p.pres_id == id)
    }

    pub fn find_drawer(&self, id: u32) -> Option<&Prs3dDrawer> {
        self.drawers.iter().find(|d| d.drawer_id == id)
    }

    pub fn nb_presentations(&self) -> usize { self.presentations.len() }
    pub fn nb_shadows(&self) -> usize { self.shadows.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_mode_props() {
        assert!(Prs3dDisplayMode::Shading.is_shaded());
        assert!(!Prs3dDisplayMode::Wireframe.is_shaded());
        assert!(Prs3dDisplayMode::ShadingWithEdges.shows_edges());
        assert!(!Prs3dDisplayMode::Shading.shows_edges());
    }

    #[test]
    fn drawer_properties() {
        let mut d = Prs3dDrawer::new(1);
        d.set_line_width(2.5);
        d.set_transparency(0.5);
        assert_eq!(d.line_width(), 2.5);
        assert!(d.is_transparent());
    }

    #[test]
    fn presentation_visible_highlight() {
        let mut p = Prs3dPresentation::new(1, 10);
        assert!(!p.is_visible);
        p.display();
        assert!(p.is_visible);
        p.highlight();
        assert!(p.is_highlighted);
        p.erase();
        assert!(!p.is_visible);
        assert!(!p.is_highlighted);
    }

    #[test]
    fn presentation_primitives() {
        let mut p = Prs3dPresentation::new(1, 1);
        assert_eq!(p.nb_primitives(), 0);
        p.add_primitive();
        p.add_primitive();
        assert_eq!(p.nb_primitives(), 2);
    }

    #[test]
    fn presentation_shadow() {
        let mut s = Prs3dPresentationShadow::new(1, 100);
        assert!(!s.has_overrides());
        s.set_mode_override(Prs3dDisplayMode::Wireframe);
        assert!(s.has_overrides());
        s.clear_mode_override();
        assert!(!s.has_overrides());
    }

    #[test]
    fn root_collections() {
        let mut r = Prs3dRoot::new();
        r.add_presentation(Prs3dPresentation::new(1, 10));
        r.add_drawer(Prs3dDrawer::new(1));
        assert_eq!(r.nb_presentations(), 1);
        assert!(r.find_presentation(1).is_some());
        assert!(r.find_drawer(1).is_some());
    }
}
