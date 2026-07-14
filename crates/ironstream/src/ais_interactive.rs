// FILE: ais_interactive.rs
// occt-ref: AIS_InteractiveObject, AIS_Shape, AIS_ColoredShape

/// AIS display mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisDisplayMode {
    Wireframe,
    Shading,
}

impl Default for AisDisplayMode {
    fn default() -> Self { Self::Shading }
}

/// AIS detection priority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AisPriority(pub i32);

impl AisPriority {
    pub const DEFAULT: Self = Self(5);
    pub const HIGH: Self = Self(10);
    pub fn value(self) -> i32 { self.0 }
}

impl Default for AisPriority {
    fn default() -> Self { Self::DEFAULT }
}

// occt-ref: AIS_InteractiveObject // — base interactive object
#[derive(Clone, Debug)]
pub struct AisInteractiveObject {
    pub object_id: u32,
    pub display_mode: AisDisplayMode,
    pub current_display_mode: i32,
    pub is_displayed: bool,
    pub is_selected: bool,
    pub priority: AisPriority,
    pub owner: u32,
    pub color: [f32; 3],
    pub transparency: f32,
    pub is_infinite: bool,
}

impl AisInteractiveObject {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            display_mode: AisDisplayMode::Shading,
            current_display_mode: 1,
            is_displayed: false,
            is_selected: false,
            priority: AisPriority::DEFAULT,
            owner: 0,
            color: [1.0, 1.0, 1.0],
            transparency: 0.0,
            is_infinite: false,
        }
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; }
    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_display_mode(&mut self, m: AisDisplayMode) { self.display_mode = m; }
    pub fn set_infinite_state(&mut self, v: bool) { self.is_infinite = v; }
    pub fn color(&self) -> [f32; 3] { self.color }
    pub fn transparency(&self) -> f32 { self.transparency }
    pub fn is_transparent(&self) -> bool { self.transparency > 1e-3 }
    pub fn display_mode(&self) -> AisDisplayMode { self.display_mode }
    pub fn is_displayed(&self) -> bool { self.is_displayed }

    pub fn mark_displayed(&mut self) { self.is_displayed = true; }
    pub fn mark_hidden(&mut self) { self.is_displayed = false; }
}

// occt-ref: AIS_Shape // — wraps a BRep shape for AIS display
#[derive(Clone, Debug)]
pub struct AisShape {
    pub base: AisInteractiveObject,
    pub shape_id: u32,
    pub owncolor: Option<[f32; 3]>,
    pub ownwidth: Option<f32>,
    pub selection_modes: Vec<i32>,
}

impl AisShape {
    pub fn new(shape_id: u32) -> Self {
        let mut base = AisInteractiveObject::new(shape_id + 100000);
        base.color = [0.7, 0.7, 0.7];
        Self {
            base,
            shape_id,
            owncolor: None,
            ownwidth: None,
            selection_modes: vec![0],
        }
    }

    pub fn set_shape(&mut self, shape_id: u32) { self.shape_id = shape_id; }
    pub fn shape(&self) -> u32 { self.shape_id }

    pub fn set_own_color(&mut self, r: f32, g: f32, b: f32) {
        self.owncolor = Some([r, g, b]);
        self.base.set_color(r, g, b);
    }

    pub fn set_own_width(&mut self, w: f32) { self.ownwidth = Some(w); }
    pub fn has_own_color(&self) -> bool { self.owncolor.is_some() }
    pub fn has_own_width(&self) -> bool { self.ownwidth.is_some() }
    pub fn add_selection_mode(&mut self, mode: i32) { self.selection_modes.push(mode); }
    pub fn selection_mode(&self, i: usize) -> Option<i32> { self.selection_modes.get(i).copied() }
}

// occt-ref: AIS_ColoredShape // — shape with per-face color support
#[derive(Clone, Debug)]
pub struct AisColoredShape {
    pub base: AisShape,
    pub face_colors: std::collections::HashMap<u32, [f32; 3]>,
    pub edge_colors: std::collections::HashMap<u32, [f32; 3]>,
}

impl AisColoredShape {
    pub fn new(shape_id: u32) -> Self {
        Self {
            base: AisShape::new(shape_id),
            face_colors: std::collections::HashMap::new(),
            edge_colors: std::collections::HashMap::new(),
        }
    }

    pub fn set_custom_color(&mut self, sub_shape_id: u32, r: f32, g: f32, b: f32, is_face: bool) {
        if is_face {
            self.face_colors.insert(sub_shape_id, [r, g, b]);
        } else {
            self.edge_colors.insert(sub_shape_id, [r, g, b]);
        }
    }

    pub fn face_color(&self, face_id: u32) -> Option<[f32; 3]> {
        self.face_colors.get(&face_id).copied()
    }

    pub fn nb_custom_colors(&self) -> usize { self.face_colors.len() + self.edge_colors.len() }
    pub fn clear_colors(&mut self) { self.face_colors.clear(); self.edge_colors.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ais_object_color_transparency() {
        let mut obj = AisInteractiveObject::new(1);
        obj.set_color(1.0, 0.0, 0.0);
        obj.set_transparency(0.5);
        assert_eq!(obj.color(), [1.0, 0.0, 0.0]);
        assert!(obj.is_transparent());
    }

    #[test]
    fn ais_object_display_state() {
        let mut obj = AisInteractiveObject::new(2);
        assert!(!obj.is_displayed());
        obj.mark_displayed();
        assert!(obj.is_displayed());
        obj.mark_hidden();
        assert!(!obj.is_displayed());
    }

    #[test]
    fn ais_shape_own_color() {
        let mut s = AisShape::new(100);
        assert!(!s.has_own_color());
        s.set_own_color(0.0, 1.0, 0.0);
        assert!(s.has_own_color());
        assert_eq!(s.base.color(), [0.0, 1.0, 0.0]);
    }

    #[test]
    fn ais_shape_selection_modes() {
        let mut s = AisShape::new(200);
        s.add_selection_mode(4);
        assert_eq!(s.selection_mode(0), Some(0)); // default mode
        assert_eq!(s.selection_mode(1), Some(4));
    }

    #[test]
    fn ais_colored_shape_face_color() {
        let mut cs = AisColoredShape::new(300);
        cs.set_custom_color(1, 1.0, 0.0, 0.0, true);
        cs.set_custom_color(2, 0.0, 1.0, 0.0, false);
        assert_eq!(cs.face_color(1), Some([1.0, 0.0, 0.0]));
        assert_eq!(cs.nb_custom_colors(), 2);
        cs.clear_colors();
        assert_eq!(cs.nb_custom_colors(), 0);
    }

    #[test]
    fn ais_priority_ordering() {
        assert!(AisPriority::HIGH > AisPriority::DEFAULT);
        assert_eq!(AisPriority::DEFAULT.value(), 5);
    }
}
