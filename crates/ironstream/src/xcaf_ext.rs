// FILE: xcaf_ext.rs
// occt: XCAFDoc_Color, XCAFDoc_Layer, XCAFDoc_Material, XCAFDoc_DocumentTool

/// Color in XCAF document.
/// occt: XCAFDoc_Color
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XcafColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl XcafColor {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r: r.clamp(0.0,1.0), g: g.clamp(0.0,1.0), b: b.clamp(0.0,1.0), a: 1.0 }
    }

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r: r.clamp(0.0,1.0), g: g.clamp(0.0,1.0), b: b.clamp(0.0,1.0), a: a.clamp(0.0,1.0) }
    }

    pub fn to_array(&self) -> [f64; 4] { [self.r, self.g, self.b, self.a] }
    pub fn is_opaque(&self) -> bool { self.a >= 1.0 - 1e-6 }
}

/// Layer entry in XCAF document.
/// occt: XCAFDoc_Layer
#[derive(Clone, Debug)]
pub struct XcafLayer {
    pub layer_id: u32,
    pub name: String,
    pub is_visible: bool,
}

impl XcafLayer {
    pub fn new(layer_id: u32, name: &str) -> Self {
        Self { layer_id, name: name.to_string(), is_visible: true }
    }
}

/// Material in XCAF document.
/// occt: XCAFDoc_Material
#[derive(Clone, Debug)]
pub struct XcafMaterial {
    pub material_id: u32,
    pub name: String,
    pub description: String,
    pub density: f64,
    pub density_name: String,
}

impl XcafMaterial {
    pub fn new(material_id: u32, name: &str) -> Self {
        Self {
            material_id,
            name: name.to_string(),
            description: String::new(),
            density: 0.0,
            density_name: String::from("kg/m3"),
        }
    }

    pub fn set_density(&mut self, d: f64, unit: &str) {
        self.density = d.max(0.0);
        self.density_name = unit.to_string();
    }
}

/// Dimension entry in XCAF document.
/// occt: XCAFDoc_Dimension
#[derive(Clone, Debug)]
pub struct XcafDimension {
    pub dim_id: u32,
    pub value: f64,
    pub description: String,
    pub qualifier: String,
}

impl XcafDimension {
    pub fn new(dim_id: u32, value: f64) -> Self {
        Self { dim_id, value, description: String::new(), qualifier: String::new() }
    }
}

/// XCAF Document tool: manages color/layer/material tables.
/// occt: XCAFDoc_DocumentTool
#[derive(Clone, Debug, Default)]
pub struct XcafDocumentTool {
    pub colors: Vec<XcafColor>,
    pub layers: Vec<XcafLayer>,
    pub materials: Vec<XcafMaterial>,
    pub shape_colors: Vec<(u32, usize)>,  // (shape_id, color_index)
    pub shape_layers: Vec<(u32, u32)>,    // (shape_id, layer_id)
    pub shape_materials: Vec<(u32, u32)>, // (shape_id, material_id)
}

impl XcafDocumentTool {
    pub fn new() -> Self { Self::default() }

    pub fn add_color(&mut self, c: XcafColor) -> usize {
        // dedup
        if let Some(pos) = self.colors.iter().position(|x| *x == c) { return pos; }
        self.colors.push(c);
        self.colors.len() - 1
    }

    pub fn add_layer(&mut self, layer: XcafLayer) {
        if !self.layers.iter().any(|l| l.layer_id == layer.layer_id) {
            self.layers.push(layer);
        }
    }

    pub fn add_material(&mut self, mat: XcafMaterial) {
        if !self.materials.iter().any(|m| m.material_id == mat.material_id) {
            self.materials.push(mat);
        }
    }

    pub fn set_color(&mut self, shape_id: u32, color: XcafColor) {
        let idx = self.add_color(color);
        if let Some(entry) = self.shape_colors.iter_mut().find(|(s,_)| *s == shape_id) {
            entry.1 = idx;
        } else {
            self.shape_colors.push((shape_id, idx));
        }
    }

    pub fn get_color(&self, shape_id: u32) -> Option<XcafColor> {
        self.shape_colors.iter().find(|(s,_)| *s == shape_id)
            .and_then(|(_,idx)| self.colors.get(*idx).copied())
    }

    pub fn set_layer(&mut self, shape_id: u32, layer_id: u32) {
        if let Some(entry) = self.shape_layers.iter_mut().find(|(s,_)| *s == shape_id) {
            entry.1 = layer_id;
        } else {
            self.shape_layers.push((shape_id, layer_id));
        }
    }

    pub fn get_layer(&self, shape_id: u32) -> Option<u32> {
        self.shape_layers.iter().find(|(s,_)| *s == shape_id).map(|(_,l)| *l)
    }

    pub fn nb_colors(&self) -> usize { self.colors.len() }
    pub fn nb_layers(&self) -> usize { self.layers.len() }
    pub fn nb_materials(&self) -> usize { self.materials.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xcaf_color_clamp() {
        let c = XcafColor::rgb(2.0, -1.0, 0.5);
        assert!((c.r - 1.0).abs() < 1e-10);
        assert!((c.g - 0.0).abs() < 1e-10);
        assert!((c.b - 0.5).abs() < 1e-10);
        assert!(c.is_opaque());
    }

    #[test]
    fn xcaf_document_add_color_dedup() {
        let mut doc = XcafDocumentTool::new();
        let c = XcafColor::rgb(1.0, 0.0, 0.0);
        let i1 = doc.add_color(c);
        let i2 = doc.add_color(c);
        assert_eq!(i1, i2);
        assert_eq!(doc.nb_colors(), 1);
    }

    #[test]
    fn xcaf_set_get_color() {
        let mut doc = XcafDocumentTool::new();
        doc.set_color(10, XcafColor::rgb(0.0, 1.0, 0.0));
        let c = doc.get_color(10).unwrap();
        assert!((c.g - 1.0).abs() < 1e-10);
        assert!(doc.get_color(99).is_none());
    }

    #[test]
    fn xcaf_layer_material() {
        let mut doc = XcafDocumentTool::new();
        doc.add_layer(XcafLayer::new(1, "Layer0"));
        doc.add_layer(XcafLayer::new(1, "Layer0")); // dup
        assert_eq!(doc.nb_layers(), 1);
        let mut mat = XcafMaterial::new(1, "Steel");
        mat.set_density(7800.0, "kg/m3");
        doc.add_material(mat);
        assert_eq!(doc.nb_materials(), 1);
    }

    #[test]
    fn xcaf_shape_layer() {
        let mut doc = XcafDocumentTool::new();
        doc.add_layer(XcafLayer::new(5, "Walls"));
        doc.set_layer(100, 5);
        assert_eq!(doc.get_layer(100), Some(5));
        doc.set_layer(100, 6); // override
        assert_eq!(doc.get_layer(100), Some(6));
    }
}
