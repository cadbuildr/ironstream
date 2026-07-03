// FILE: de_wrapper.rs
// occt: DE_Wrapper, DE_ShapeFixParameters, DE_PluginHolder,
//       DE_ConfigurationNode

use std::collections::HashMap;

/// Shape healing parameters applied after import.
// occt: DE_ShapeFixParameters
#[derive(Clone, Debug)]
pub struct ShapeFixParameters {
    pub fix_shape: bool,
    pub fix_solid: bool,
    pub fix_shell: bool,
    pub fix_face: bool,
    pub fix_wire: bool,
    pub fix_edge: bool,
    pub fix_vertex: bool,
    pub fix_small_face: bool,
    pub fix_stitch_edges: bool,
    pub tolerance: f64,
    pub max_tolerance: f64,
    pub min_tolerance: f64,
}

impl ShapeFixParameters {
    pub fn default_healing() -> Self {
        Self {
            fix_shape: true,
            fix_solid: true,
            fix_shell: true,
            fix_face: true,
            fix_wire: true,
            fix_edge: true,
            fix_vertex: true,
            fix_small_face: false,
            fix_stitch_edges: false,
            tolerance: 1e-7,
            max_tolerance: 1e-5,
            min_tolerance: 1e-10,
        }
    }

    pub fn none() -> Self {
        Self {
            fix_shape: false, fix_solid: false, fix_shell: false,
            fix_face: false, fix_wire: false, fix_edge: false,
            fix_vertex: false, fix_small_face: false, fix_stitch_edges: false,
            tolerance: 1e-7, max_tolerance: 1e-5, min_tolerance: 1e-10,
        }
    }

    pub fn with_tolerance(mut self, tol: f64) -> Self { self.tolerance = tol; self }
    pub fn with_max_tolerance(mut self, tol: f64) -> Self { self.max_tolerance = tol; self }

    pub fn any_enabled(&self) -> bool {
        self.fix_shape || self.fix_solid || self.fix_shell || self.fix_face
            || self.fix_wire || self.fix_edge || self.fix_vertex
    }
}

impl Default for ShapeFixParameters {
    fn default() -> Self { Self::default_healing() }
}

/// Configuration node for a DE (data exchange) plugin.
// occt: DE_ConfigurationNode
#[derive(Clone, Debug, Default)]
pub struct ConfigurationNode {
    pub format: String,
    pub vendor: String,
    pub version: String,
    pub read_enabled: bool,
    pub write_enabled: bool,
    pub parameters: HashMap<String, String>,
}

impl ConfigurationNode {
    pub fn new(format: &str, vendor: &str) -> Self {
        Self {
            format: format.to_owned(),
            vendor: vendor.to_owned(),
            version: "1.0".to_owned(),
            read_enabled: true,
            write_enabled: true,
            parameters: HashMap::new(),
        }
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) {
        self.parameters.insert(key.to_owned(), value.to_owned());
    }

    pub fn get_parameter(&self, key: &str) -> Option<&str> {
        self.parameters.get(key).map(String::as_str)
    }

    pub fn is_supported_extension(&self, ext: &str) -> bool {
        self.format.eq_ignore_ascii_case(ext)
            || ext.eq_ignore_ascii_case(&self.format.to_ascii_lowercase())
    }
}

/// Plugin holder — wraps a read/write capability bundle for a format.
// occt: DE_PluginHolder
#[derive(Clone, Debug)]
pub struct PluginHolder {
    pub node: ConfigurationNode,
    pub fix_params: ShapeFixParameters,
    pub is_active: bool,
}

impl PluginHolder {
    pub fn new(node: ConfigurationNode) -> Self {
        Self { node, fix_params: ShapeFixParameters::default_healing(), is_active: true }
    }

    pub fn activate(&mut self) { self.is_active = true; }
    pub fn deactivate(&mut self) { self.is_active = false; }

    pub fn can_read(&self) -> bool { self.is_active && self.node.read_enabled }
    pub fn can_write(&self) -> bool { self.is_active && self.node.write_enabled }

    pub fn format(&self) -> &str { &self.node.format }
}

/// DE wrapper — manages a collection of format plugins.
// occt: DE_Wrapper
#[derive(Clone, Debug, Default)]
pub struct DeWrapper {
    pub plugins: Vec<PluginHolder>,
    pub active_format: Option<String>,
}

impl DeWrapper {
    pub fn new() -> Self { Self::default() }

    pub fn register_plugin(&mut self, plugin: PluginHolder) {
        self.plugins.push(plugin);
    }

    pub fn find_plugin(&self, format: &str) -> Option<&PluginHolder> {
        self.plugins.iter().find(|p| p.format().eq_ignore_ascii_case(format))
    }

    pub fn find_plugin_mut(&mut self, format: &str) -> Option<&mut PluginHolder> {
        self.plugins.iter_mut().find(|p| p.format().eq_ignore_ascii_case(format))
    }

    pub fn can_read(&self, format: &str) -> bool {
        self.find_plugin(format).map(|p| p.can_read()).unwrap_or(false)
    }

    pub fn can_write(&self, format: &str) -> bool {
        self.find_plugin(format).map(|p| p.can_write()).unwrap_or(false)
    }

    pub fn nb_plugins(&self) -> usize { self.plugins.len() }
    pub fn nb_active(&self) -> usize { self.plugins.iter().filter(|p| p.is_active).count() }

    pub fn all_formats(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.format()).collect()
    }

    /// Set fix params for a specific format.
    pub fn set_fix_params(&mut self, format: &str, params: ShapeFixParameters) -> bool {
        if let Some(p) = self.find_plugin_mut(format) {
            p.fix_params = params;
            true
        } else {
            false
        }
    }
}

/// Pre-configured wrappers for common formats.
impl DeWrapper {
    pub fn with_step() -> Self {
        let mut w = Self::new();
        let node = ConfigurationNode::new("STEP", "OCC");
        w.register_plugin(PluginHolder::new(node));
        w
    }

    pub fn with_iges() -> Self {
        let mut w = Self::new();
        let node = ConfigurationNode::new("IGES", "OCC");
        w.register_plugin(PluginHolder::new(node));
        w
    }

    pub fn with_all_formats() -> Self {
        let mut w = Self::new();
        for fmt in &["STEP", "IGES", "BREP", "OBJ", "GLTF", "STL"] {
            let node = ConfigurationNode::new(fmt, "OCC");
            w.register_plugin(PluginHolder::new(node));
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_fix_params() {
        let p = ShapeFixParameters::default_healing();
        assert!(p.any_enabled());
        let none = ShapeFixParameters::none();
        assert!(!none.any_enabled());
    }

    #[test]
    fn configuration_node() {
        let mut node = ConfigurationNode::new("STEP", "OCC");
        node.set_parameter("read.products.mode", "1");
        assert_eq!(node.get_parameter("read.products.mode"), Some("1"));
        assert!(node.is_supported_extension("step"));
    }

    #[test]
    fn de_wrapper_step() {
        let w = DeWrapper::with_step();
        assert_eq!(w.nb_plugins(), 1);
        assert!(w.can_read("STEP"));
        assert!(w.can_write("STEP"));
        assert!(!w.can_read("IGES"));
    }

    #[test]
    fn de_wrapper_all_formats() {
        let w = DeWrapper::with_all_formats();
        assert_eq!(w.nb_plugins(), 6);
        assert!(w.can_read("GLTF"));
    }

    #[test]
    fn plugin_deactivate() {
        let mut w = DeWrapper::with_step();
        w.find_plugin_mut("STEP").unwrap().deactivate();
        assert!(!w.can_read("STEP"));
        assert_eq!(w.nb_active(), 0);
    }
}
