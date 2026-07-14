// FILE: de_config.rs
// occt-ref: DE_ConfigurationNode, DE_Wrapper, DE_PluginHolder, DE_ShapeFixParameters

/// Configuration parameter types.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

impl ConfigValue {
    pub fn as_bool(&self) -> Option<bool> { if let ConfigValue::Bool(v) = self { Some(*v) } else { None } }
    pub fn as_int(&self) -> Option<i64> { if let ConfigValue::Int(v) = self { Some(*v) } else { None } }
    pub fn as_float(&self) -> Option<f64> { if let ConfigValue::Float(v) = self { Some(*v) } else { None } }
    pub fn as_str(&self) -> Option<&str> { if let ConfigValue::Str(v) = self { Some(v) } else { None } }
}

// occt-ref: DE_ConfigurationNode // — a named group of DE settings
#[derive(Clone, Debug, Default)]
pub struct ConfigNode {
    pub name: String,
    pub params: std::collections::HashMap<String, ConfigValue>,
}

impl ConfigNode {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), params: Default::default() } }

    pub fn set(&mut self, key: impl Into<String>, value: ConfigValue) { self.params.insert(key.into(), value); }
    pub fn get(&self, key: &str) -> Option<&ConfigValue> { self.params.get(key) }
    pub fn get_bool(&self, key: &str) -> bool { self.get(key).and_then(|v| v.as_bool()).unwrap_or(false) }
    pub fn get_float(&self, key: &str) -> f64 { self.get(key).and_then(|v| v.as_float()).unwrap_or(0.0) }
    pub fn get_int(&self, key: &str) -> i64 { self.get(key).and_then(|v| v.as_int()).unwrap_or(0) }
    pub fn get_str<'a>(&'a self, key: &str) -> &'a str { self.get(key).and_then(|v| v.as_str()).unwrap_or("") }
    pub fn nb_params(&self) -> usize { self.params.len() }
    pub fn has(&self, key: &str) -> bool { self.params.contains_key(key) }
}

// occt-ref: DE_ShapeFixParameters
#[derive(Clone, Debug)]
pub struct ShapeFixParameters {
    pub fix_solid: bool,
    pub fix_shell: bool,
    pub fix_face: bool,
    pub fix_wire: bool,
    pub fix_edge: bool,
    pub fix_vertex: bool,
    pub max_tolerance: f64,
    pub min_tolerance: f64,
    pub surface_mode: SurfaceFixMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceFixMode {
    Off,
    On,
    Auto,
}

impl ShapeFixParameters {
    pub fn default_fix() -> Self {
        Self {
            fix_solid: true, fix_shell: true, fix_face: true,
            fix_wire: true, fix_edge: true, fix_vertex: true,
            max_tolerance: 1e-3, min_tolerance: 1e-7,
            surface_mode: SurfaceFixMode::Auto,
        }
    }

    pub fn none() -> Self {
        Self {
            fix_solid: false, fix_shell: false, fix_face: false,
            fix_wire: false, fix_edge: false, fix_vertex: false,
            max_tolerance: 1e-7, min_tolerance: 1e-10,
            surface_mode: SurfaceFixMode::Off,
        }
    }

    pub fn nb_fixes_enabled(&self) -> usize {
        [self.fix_solid, self.fix_shell, self.fix_face, self.fix_wire, self.fix_edge, self.fix_vertex]
            .iter().filter(|&&b| b).count()
    }
}

impl Default for ShapeFixParameters {
    fn default() -> Self { Self::default_fix() }
}

// occt-ref: DE_Wrapper // — single entry point for read/write configuration
#[derive(Clone, Debug, Default)]
pub struct DeWrapper {
    pub nodes: Vec<ConfigNode>,
    pub fix_params: ShapeFixParameters,
    pub is_cascade_units: bool,
    pub write_mode: WriteMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WriteMode {
    #[default]
    Default,
    Cascade,
    Export,
}

impl DeWrapper {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, node: ConfigNode) { self.nodes.push(node); }

    pub fn find_node(&self, name: &str) -> Option<&ConfigNode> { self.nodes.iter().find(|n| n.name == name) }
    pub fn find_node_mut(&mut self, name: &str) -> Option<&mut ConfigNode> { self.nodes.iter_mut().find(|n| n.name == name) }

    pub fn get_param(&self, node_name: &str, key: &str) -> Option<&ConfigValue> {
        self.find_node(node_name)?.get(key)
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_node_get_set() {
        let mut n = ConfigNode::new("STEP");
        n.set("read.precision.mode", ConfigValue::Int(1));
        n.set("read.precision.val", ConfigValue::Float(1e-4));
        n.set("write.assembly.mode", ConfigValue::Str("External".into()));
        assert_eq!(n.get_int("read.precision.mode"), 1);
        assert!((n.get_float("read.precision.val") - 1e-4).abs() < 1e-14);
        assert_eq!(n.get_str("write.assembly.mode"), "External");
    }

    #[test]
    fn shape_fix_params() {
        let p = ShapeFixParameters::default_fix();
        assert_eq!(p.nb_fixes_enabled(), 6);
        let p2 = ShapeFixParameters::none();
        assert_eq!(p2.nb_fixes_enabled(), 0);
    }

    #[test]
    fn de_wrapper_lookup() {
        let mut w = DeWrapper::new();
        let mut n = ConfigNode::new("IGES");
        n.set("read.surface.mode", ConfigValue::Bool(true));
        w.add_node(n);
        assert!(w.find_node("IGES").is_some());
        assert!(w.get_param("IGES", "read.surface.mode").and_then(|v| v.as_bool()).unwrap_or(false));
    }
}
