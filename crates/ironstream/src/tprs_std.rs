// FILE: tprs_std.rs
// occt: TPrsStd_AISPresentation, TPrsStd_AISViewer, TPrsStd_Driver
//       TPrsStd_DriverTable

/// Display mode for AIS presentations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AisDisplayMode {
    #[default]
    Wireframe,
    Shading,
    ShadingWithEdges,
}

/// Transparency level [0.0 = opaque, 1.0 = fully transparent].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AisTransparency(pub f64);

impl Default for AisTransparency {
    fn default() -> Self { Self(0.0) }
}

impl AisTransparency {
    pub fn new(v: f64) -> Self { Self(v.clamp(0.0, 1.0)) }
    pub fn value(&self) -> f64 { self.0 }
    pub fn is_opaque(&self) -> bool { self.0 < 1e-6 }
}

/// Color stored as RGB floats [0,1].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TprsColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for TprsColor {
    fn default() -> Self { Self { r: 1.0, g: 1.0, b: 1.0 } }
}

impl TprsColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r: r.clamp(0.0, 1.0), g: g.clamp(0.0, 1.0), b: b.clamp(0.0, 1.0) }
    }
    pub fn red()   -> Self { Self::new(1.0, 0.0, 0.0) }
    pub fn green() -> Self { Self::new(0.0, 1.0, 0.0) }
    pub fn blue()  -> Self { Self::new(0.0, 0.0, 1.0) }
    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
}

/// AIS presentation attribute stored on an OCAF label.
/// occt: TPrsStd_AISPresentation
#[derive(Clone, Debug)]
pub struct TPrsStdAISPresentation {
    pub label_id: u32,
    pub driver_guid: String,
    pub display_mode: AisDisplayMode,
    pub transparency: AisTransparency,
    pub color: Option<TprsColor>,
    pub material_index: Option<u32>,
    pub is_displayed: bool,
    pub is_selected: bool,
    pub width: Option<f64>,
}

impl TPrsStdAISPresentation {
    pub fn new(label_id: u32, driver_guid: impl Into<String>) -> Self {
        Self {
            label_id,
            driver_guid: driver_guid.into(),
            display_mode: AisDisplayMode::Shading,
            transparency: AisTransparency::default(),
            color: None,
            material_index: None,
            is_displayed: false,
            is_selected: false,
            width: None,
        }
    }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; self.is_selected = false; }
    pub fn select(&mut self) { self.is_selected = true; }
    pub fn deselect(&mut self) { self.is_selected = false; }

    pub fn set_display_mode(&mut self, mode: AisDisplayMode) { self.display_mode = mode; }
    pub fn set_transparency(&mut self, t: f64) { self.transparency = AisTransparency::new(t); }
    pub fn set_color(&mut self, c: TprsColor) { self.color = Some(c); }
    pub fn unset_color(&mut self) { self.color = None; }
    pub fn set_width(&mut self, w: f64) { self.width = Some(w.max(0.0)); }
    pub fn unset_width(&mut self) { self.width = None; }
    pub fn set_material(&mut self, idx: u32) { self.material_index = Some(idx); }

    pub fn has_own_color(&self) -> bool { self.color.is_some() }
    pub fn has_own_material(&self) -> bool { self.material_index.is_some() }
}

/// AIS viewer reference on a label.
/// occt: TPrsStd_AISViewer
#[derive(Clone, Debug, Default)]
pub struct TPrsStdAISViewer {
    pub label_id: u32,
    pub viewer_id: u32,
    pub context_id: u32,
}

impl TPrsStdAISViewer {
    pub fn new(label_id: u32, viewer_id: u32) -> Self {
        Self { label_id, viewer_id, context_id: 0 }
    }

    pub fn set_context(&mut self, ctx: u32) { self.context_id = ctx; }
    pub fn has_context(&self) -> bool { self.context_id != 0 }
}

/// Driver entry: maps a GUID to a driver name.
/// occt: TPrsStd_Driver
#[derive(Clone, Debug)]
pub struct TPrsStdDriver {
    pub guid: String,
    pub name: String,
}

impl TPrsStdDriver {
    pub fn new(guid: impl Into<String>, name: impl Into<String>) -> Self {
        Self { guid: guid.into(), name: name.into() }
    }
}

/// Driver table: registry of presentation drivers by GUID.
/// occt: TPrsStd_DriverTable
#[derive(Clone, Debug, Default)]
pub struct TPrsStdDriverTable {
    pub drivers: Vec<TPrsStdDriver>,
}

impl TPrsStdDriverTable {
    pub fn new() -> Self { Self::default() }

    pub fn add_driver(&mut self, d: TPrsStdDriver) {
        if !self.drivers.iter().any(|x| x.guid == d.guid) {
            self.drivers.push(d);
        }
    }

    pub fn find_by_guid(&self, guid: &str) -> Option<&TPrsStdDriver> {
        self.drivers.iter().find(|d| d.guid == guid)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&TPrsStdDriver> {
        self.drivers.iter().find(|d| d.name == name)
    }

    pub fn nb_drivers(&self) -> usize { self.drivers.len() }

    pub fn remove_driver(&mut self, guid: &str) -> bool {
        let before = self.drivers.len();
        self.drivers.retain(|d| d.guid != guid);
        self.drivers.len() < before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_display_erase() {
        let mut p = TPrsStdAISPresentation::new(1, "guid-abc");
        assert!(!p.is_displayed);
        p.display();
        assert!(p.is_displayed);
        p.select();
        assert!(p.is_selected);
        p.erase();
        assert!(!p.is_displayed);
        assert!(!p.is_selected);
    }

    #[test]
    fn transparency_clamp() {
        let mut p = TPrsStdAISPresentation::new(2, "guid-xyz");
        p.set_transparency(1.5);
        assert_eq!(p.transparency.value(), 1.0);
        p.set_transparency(-0.1);
        assert_eq!(p.transparency.value(), 0.0);
        assert!(p.transparency.is_opaque());
    }

    #[test]
    fn color_own() {
        let mut p = TPrsStdAISPresentation::new(3, "g");
        assert!(!p.has_own_color());
        p.set_color(TprsColor::red());
        assert!(p.has_own_color());
        p.unset_color();
        assert!(!p.has_own_color());
    }

    #[test]
    fn driver_table() {
        let mut dt = TPrsStdDriverTable::new();
        dt.add_driver(TPrsStdDriver::new("guid-1", "ShapeDriver"));
        dt.add_driver(TPrsStdDriver::new("guid-2", "PointDriver"));
        dt.add_driver(TPrsStdDriver::new("guid-1", "dup")); // duplicate
        assert_eq!(dt.nb_drivers(), 2);
        assert!(dt.find_by_guid("guid-1").is_some());
        assert!(dt.find_by_name("PointDriver").is_some());
        dt.remove_driver("guid-1");
        assert_eq!(dt.nb_drivers(), 1);
    }

    #[test]
    fn ais_viewer() {
        let mut v = TPrsStdAISViewer::new(10, 1);
        assert!(!v.has_context());
        v.set_context(42);
        assert!(v.has_context());
        assert_eq!(v.context_id, 42);
    }
}
