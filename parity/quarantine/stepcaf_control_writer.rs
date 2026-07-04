// FILE: stepcaf_control_writer.rs
// occt: STEPCAFControl_Writer

/// Writer tool for translating DECAF document to STEP file
pub struct STEPCAFControl_Writer {
    color_mode: bool,
    name_mode: bool,
    layer_mode: bool,
    props_mode: bool,
    shuo_mode: bool,
    gdt_mode: bool,
    material_mode: bool,
    view_mode: bool,
}

impl STEPCAFControl_Writer {
    /// Creates a writer with an empty STEP model
    pub fn new() -> Self {
        STEPCAFControl_Writer {
            color_mode: true,
            name_mode: true,
            layer_mode: true,
            props_mode: true,
            shuo_mode: true,
            gdt_mode: true,
            material_mode: true,
            view_mode: true,
        }
    }

    /// Set ColorMode for writing Colors or not
    pub fn set_color_mode(&mut self, color_mode: bool) {
        self.color_mode = color_mode;
    }

    pub fn get_color_mode(&self) -> bool {
        self.color_mode
    }

    /// Set NameMode for writing Name or not
    pub fn set_name_mode(&mut self, name_mode: bool) {
        self.name_mode = name_mode;
    }

    pub fn get_name_mode(&self) -> bool {
        self.name_mode
    }

    /// Set LayerMode for writing Layers or not
    pub fn set_layer_mode(&mut self, layer_mode: bool) {
        self.layer_mode = layer_mode;
    }

    pub fn get_layer_mode(&self) -> bool {
        self.layer_mode
    }

    /// Set PropsMode for writing Validation properties or not
    pub fn set_props_mode(&mut self, props_mode: bool) {
        self.props_mode = props_mode;
    }

    pub fn get_props_mode(&self) -> bool {
        self.props_mode
    }

    /// Set SHUO mode for writing SHUO or not
    pub fn set_shuo_mode(&mut self, shuo_mode: bool) {
        self.shuo_mode = shuo_mode;
    }

    pub fn get_shuo_mode(&self) -> bool {
        self.shuo_mode
    }

    /// Set GDT mode for writing GDT or not
    pub fn set_gdt_mode(&mut self, gdt_mode: bool) {
        self.gdt_mode = gdt_mode;
    }

    pub fn get_gdt_mode(&self) -> bool {
        self.gdt_mode
    }

    /// Set Material mode
    pub fn set_material_mode(&mut self, material_mode: bool) {
        self.material_mode = material_mode;
    }

    pub fn get_material_mode(&self) -> bool {
        self.material_mode
    }

    /// Set View mode
    pub fn set_view_mode(&mut self, view_mode: bool) {
        self.view_mode = view_mode;
    }

    pub fn get_view_mode(&self) -> bool {
        self.view_mode
    }
}

impl Default for STEPCAFControl_Writer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_writer() {
        let writer = STEPCAFControl_Writer::new();
        assert_eq!(writer.get_color_mode(), true);
        assert_eq!(writer.get_name_mode(), true);
        assert_eq!(writer.get_layer_mode(), true);
    }

    #[test]
    fn test_set_color_mode() {
        let mut writer = STEPCAFControl_Writer::new();
        writer.set_color_mode(false);
        assert_eq!(writer.get_color_mode(), false);
    }

    #[test]
    fn test_set_multiple_modes() {
        let mut writer = STEPCAFControl_Writer::new();
        writer.set_color_mode(false);
        writer.set_gdt_mode(false);
        writer.set_material_mode(false);
        assert_eq!(writer.get_color_mode(), false);
        assert_eq!(writer.get_gdt_mode(), false);
        assert_eq!(writer.get_material_mode(), false);
    }

    #[test]
    fn test_all_modes_default_true() {
        let writer = STEPCAFControl_Writer::new();
        assert!(writer.get_color_mode());
        assert!(writer.get_name_mode());
        assert!(writer.get_layer_mode());
        assert!(writer.get_props_mode());
        assert!(writer.get_shuo_mode());
        assert!(writer.get_gdt_mode());
        assert!(writer.get_material_mode());
        assert!(writer.get_view_mode());
    }
}
