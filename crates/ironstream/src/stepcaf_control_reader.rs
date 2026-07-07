// FILE: stepcaf_control_reader.rs
// occt: STEPCAFControl_Reader

/// Reader tool for translating STEP CAF (Computer-Aided-Design Functional model) files
pub struct STEPCAFControl_Reader {
    color_mode: bool,
    name_mode: bool,
    layer_mode: bool,
    props_mode: bool,
    meta_mode: bool,
    product_meta_mode: bool,
    shuo_mode: bool,
    gdt_mode: bool,
    mat_mode: bool,
    view_mode: bool,
}

impl STEPCAFControl_Reader {
    /// Creates a reader with an empty STEP model and sets modes to true
    pub fn new() -> Self {
        STEPCAFControl_Reader {
            color_mode: true,
            name_mode: true,
            layer_mode: true,
            props_mode: true,
            meta_mode: true,
            product_meta_mode: true,
            shuo_mode: true,
            gdt_mode: true,
            mat_mode: true,
            view_mode: true,
        }
    }

    /// Set ColorMode for reading Colors or not
    pub fn set_color_mode(&mut self, colormode: bool) {
        self.color_mode = colormode;
    }

    pub fn get_color_mode(&self) -> bool {
        self.color_mode
    }

    /// Set NameMode for reading Name or not
    pub fn set_name_mode(&mut self, namemode: bool) {
        self.name_mode = namemode;
    }

    pub fn get_name_mode(&self) -> bool {
        self.name_mode
    }

    /// Set LayerMode for reading Layers or not
    pub fn set_layer_mode(&mut self, layermode: bool) {
        self.layer_mode = layermode;
    }

    pub fn get_layer_mode(&self) -> bool {
        self.layer_mode
    }

    /// Set PropsMode for reading Validation properties or not
    pub fn set_props_mode(&mut self, propsmode: bool) {
        self.props_mode = propsmode;
    }

    pub fn get_props_mode(&self) -> bool {
        self.props_mode
    }

    /// Set MetaMode for reading Metadata or not
    pub fn set_meta_mode(&mut self, meta_mode: bool) {
        self.meta_mode = meta_mode;
    }

    pub fn get_meta_mode(&self) -> bool {
        self.meta_mode
    }

    /// Set MetaMode for reading Product Metadata or not
    pub fn set_product_meta_mode(&mut self, product_meta_mode: bool) {
        self.product_meta_mode = product_meta_mode;
    }

    pub fn get_product_meta_mode(&self) -> bool {
        self.product_meta_mode
    }

    /// Set SHUO mode for reading SHUO or not
    pub fn set_shuo_mode(&mut self, shuomode: bool) {
        self.shuo_mode = shuomode;
    }

    pub fn get_shuo_mode(&self) -> bool {
        self.shuo_mode
    }

    /// Set GDT mode for reading GDT or not
    pub fn set_gdt_mode(&mut self, gdtmode: bool) {
        self.gdt_mode = gdtmode;
    }

    pub fn get_gdt_mode(&self) -> bool {
        self.gdt_mode
    }

    /// Set Material mode
    pub fn set_mat_mode(&mut self, matmode: bool) {
        self.mat_mode = matmode;
    }

    pub fn get_mat_mode(&self) -> bool {
        self.mat_mode
    }

    /// Set View mode
    pub fn set_view_mode(&mut self, viewmode: bool) {
        self.view_mode = viewmode;
    }

    /// Get View mode
    pub fn get_view_mode(&self) -> bool {
        self.view_mode
    }
}

impl Default for STEPCAFControl_Reader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_reader() {
        let reader = STEPCAFControl_Reader::new();
        assert_eq!(reader.get_color_mode(), true);
        assert_eq!(reader.get_name_mode(), true);
        assert_eq!(reader.get_layer_mode(), true);
    }

    #[test]
    fn test_set_color_mode() {
        let mut reader = STEPCAFControl_Reader::new();
        reader.set_color_mode(false);
        assert_eq!(reader.get_color_mode(), false);
    }

    #[test]
    fn test_set_multiple_modes() {
        let mut reader = STEPCAFControl_Reader::new();
        reader.set_color_mode(false);
        reader.set_name_mode(false);
        reader.set_gdt_mode(false);
        assert_eq!(reader.get_color_mode(), false);
        assert_eq!(reader.get_name_mode(), false);
        assert_eq!(reader.get_gdt_mode(), false);
    }

    #[test]
    fn test_view_mode() {
        let mut reader = STEPCAFControl_Reader::new();
        reader.set_view_mode(false);
        assert_eq!(reader.get_view_mode(), false);
    }
}
