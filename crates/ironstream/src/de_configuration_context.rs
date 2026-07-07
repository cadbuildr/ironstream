// FILE: de_configuration_context.rs
// occt: DE_ConfigurationContext

use std::collections::HashMap;

/// Configuration context for loading and managing resource parameters.
/// Provides convenient interface to resource files and parameter lookup with scope support.
///
/// Allows loading configuration from files or strings, and retrieving parameters
/// with type conversion (real, integer, boolean, string) with scope support.
#[derive(Clone)]
pub struct DeConfigurationContext {
    /// Internal parameters map: (fully qualified parameter name) -> value
    resource_map: HashMap<String, String>,
}

impl DeConfigurationContext {
    /// Creates an empty configuration context
    pub fn new() -> Self {
        DeConfigurationContext {
            resource_map: HashMap::new(),
        }
    }

    /// Imports configuration from file or string
    pub fn load(&mut self, config: &str) -> bool {
        // Try loading as file first, then as string
        self.load_file(config) || self.load_str(config)
    }

    /// Loads configuration from a file
    pub fn load_file(&mut self, file_path: &str) -> bool {
        match std::fs::read_to_string(file_path) {
            Ok(content) => self.load_str(&content),
            Err(_) => false,
        }
    }

    /// Loads configuration from a string
    pub fn load_str(&mut self, resource: &str) -> bool {
        for line in resource.lines() {
            let _ = self.load_line(line);
        }
        true
    }

    /// Processes a single configuration line
    fn load_line(&mut self, line: &str) -> bool {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('!') {
            return true;
        }

        // Look for key:value format
        if let Some(colon_pos) = trimmed.find(':') {
            let key = trimmed[..colon_pos].trim().to_string();
            let value = trimmed[colon_pos + 1..].trim().to_string();

            if !key.is_empty() {
                self.resource_map.insert(key, value);
                return true;
            }
        }

        false
    }

    /// Checks if a parameter is set
    pub fn is_param_set(&self, param: &str, scope: &str) -> bool {
        let full_name = self.make_name(scope, param);
        self.resource_map.contains_key(&full_name)
    }

    /// Gets a real (double) parameter value
    pub fn get_real(&self, param: &str, scope: &str) -> Option<f64> {
        let full_name = self.make_name(scope, param);
        self.resource_map
            .get(&full_name)
            .and_then(|v| v.parse::<f64>().ok())
    }

    /// Gets an integer parameter value
    pub fn get_integer(&self, param: &str, scope: &str) -> Option<i32> {
        let full_name = self.make_name(scope, param);
        self.resource_map
            .get(&full_name)
            .and_then(|v| v.parse::<i32>().ok())
    }

    /// Gets a boolean parameter value
    pub fn get_boolean(&self, param: &str, scope: &str) -> Option<bool> {
        let full_name = self.make_name(scope, param);
        self.resource_map.get(&full_name).and_then(|v| {
            match v.as_str() {
                "true" | "1" | "on" | "yes" => Some(true),
                "false" | "0" | "off" | "no" => Some(false),
                _ => None,
            }
        })
    }

    /// Gets a string parameter value
    pub fn get_string(&self, param: &str, scope: &str) -> Option<String> {
        let full_name = self.make_name(scope, param);
        self.resource_map.get(&full_name).cloned()
    }

    /// Gets a string sequence parameter value (space or comma-separated)
    pub fn get_string_seq(&self, param: &str, scope: &str) -> Vec<String> {
        self.get_string(param, scope)
            .map(|s| {
                s.split(|c: char| c.is_whitespace() || c == ',')
                    .filter(|p| !p.is_empty())
                    .map(|p| p.to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets a real value with a default
    pub fn real_val(&self, param: &str, default: f64, scope: &str) -> f64 {
        self.get_real(param, scope).unwrap_or(default)
    }

    /// Gets an integer value with a default
    pub fn integer_val(&self, param: &str, default: i32, scope: &str) -> i32 {
        self.get_integer(param, scope).unwrap_or(default)
    }

    /// Gets a boolean value with a default
    pub fn boolean_val(&self, param: &str, default: bool, scope: &str) -> bool {
        self.get_boolean(param, scope).unwrap_or(default)
    }

    /// Gets a string value with a default
    pub fn string_val(&self, param: &str, default: &str, scope: &str) -> String {
        self.get_string(param, scope)
            .unwrap_or_else(|| default.to_string())
    }

    /// Gets the internal resource map
    pub fn get_internal_map(&self) -> &HashMap<String, String> {
        &self.resource_map
    }

    /// Makes a fully qualified parameter name from scope and parameter
    fn make_name(&self, scope: &str, param: &str) -> String {
        if scope.is_empty() {
            param.to_string()
        } else {
            format!("{}.{}", scope, param)
        }
    }

    /// Adds a parameter directly
    pub fn add_parameter(&mut self, param: &str, value: &str, scope: &str) {
        let full_name = self.make_name(scope, param);
        self.resource_map.insert(full_name, value.to_string());
    }
}

impl Default for DeConfigurationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = DeConfigurationContext::new();
        assert!(ctx.resource_map.is_empty());
    }

    #[test]
    fn test_load_string() {
        let mut ctx = DeConfigurationContext::new();
        let config = "provider.OBJ.OCC.file.length.unit : 1.5\nprovider.OBJ.OCC.read.create.shapes : true";
        assert!(ctx.load_str(config));
        assert!(ctx.is_param_set("file.length.unit", "provider.OBJ.OCC"));
    }

    #[test]
    fn test_load_with_comments() {
        let mut ctx = DeConfigurationContext::new();
        let config = "!This is a comment\nprovider.OBJ.OCC.value : 42\n!Another comment";
        ctx.load_str(config);
        assert!(ctx.is_param_set("value", "provider.OBJ.OCC"));
    }

    #[test]
    fn test_get_real() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("file.length.unit", "2.5", "provider.OBJ.OCC");
        let value = ctx.get_real("file.length.unit", "provider.OBJ.OCC");
        assert_eq!(value, Some(2.5));
    }

    #[test]
    fn test_get_integer() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("read.memory.limit.mib", "512", "provider.OBJ.OCC");
        let value = ctx.get_integer("read.memory.limit.mib", "provider.OBJ.OCC");
        assert_eq!(value, Some(512));
    }

    #[test]
    fn test_get_boolean_true() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("read.create.shapes", "true", "provider.OBJ.OCC");
        assert_eq!(ctx.get_boolean("read.create.shapes", "provider.OBJ.OCC"), Some(true));

        ctx.add_parameter("test1", "1", "");
        assert_eq!(ctx.get_boolean("test1", ""), Some(true));

        ctx.add_parameter("test2", "on", "");
        assert_eq!(ctx.get_boolean("test2", ""), Some(true));
    }

    #[test]
    fn test_get_boolean_false() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("read.create.shapes", "false", "provider.OBJ.OCC");
        assert_eq!(ctx.get_boolean("read.create.shapes", "provider.OBJ.OCC"), Some(false));

        ctx.add_parameter("test1", "0", "");
        assert_eq!(ctx.get_boolean("test1", ""), Some(false));

        ctx.add_parameter("test2", "off", "");
        assert_eq!(ctx.get_boolean("test2", ""), Some(false));
    }

    #[test]
    fn test_get_string() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("read.root.prefix", "/some/path", "provider.OBJ.OCC");
        let value = ctx.get_string("read.root.prefix", "provider.OBJ.OCC");
        assert_eq!(value, Some("/some/path".to_string()));
    }

    #[test]
    fn test_get_string_seq() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("values", "obj vrml ply", "");
        let seq = ctx.get_string_seq("values", "");
        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0], "obj");
        assert_eq!(seq[1], "vrml");
        assert_eq!(seq[2], "ply");
    }

    #[test]
    fn test_real_val_with_default() {
        let mut ctx = DeConfigurationContext::new();
        let value = ctx.real_val("nonexistent", 3.14, "");
        assert_eq!(value, 3.14);

        ctx.add_parameter("exists", "2.71", "");
        let value = ctx.real_val("exists", 3.14, "");
        assert_eq!(value, 2.71);
    }

    #[test]
    fn test_integer_val_with_default() {
        let ctx = DeConfigurationContext::new();
        let value = ctx.integer_val("nonexistent", 42, "");
        assert_eq!(value, 42);
    }

    #[test]
    fn test_boolean_val_with_default() {
        let ctx = DeConfigurationContext::new();
        let value = ctx.boolean_val("nonexistent", true, "");
        assert!(value);

        let value = ctx.boolean_val("nonexistent", false, "");
        assert!(!value);
    }

    #[test]
    fn test_string_val_with_default() {
        let ctx = DeConfigurationContext::new();
        let value = ctx.string_val("nonexistent", "default", "");
        assert_eq!(value, "default");
    }

    #[test]
    fn test_scope_handling() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("param", "value1", "scope1");
        ctx.add_parameter("param", "value2", "scope2");

        assert_eq!(ctx.get_string("param", "scope1"), Some("value1".to_string()));
        assert_eq!(ctx.get_string("param", "scope2"), Some("value2".to_string()));
    }

    #[test]
    fn test_empty_scope() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("param", "value", "");
        assert!(ctx.is_param_set("param", ""));
        assert_eq!(ctx.get_string("param", ""), Some("value".to_string()));
    }

    #[test]
    fn test_is_param_set() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("exists", "value", "scope");
        assert!(ctx.is_param_set("exists", "scope"));
        assert!(!ctx.is_param_set("missing", "scope"));
    }

    #[test]
    fn test_get_internal_map() {
        let mut ctx = DeConfigurationContext::new();
        ctx.add_parameter("key1", "value1", "");
        ctx.add_parameter("key2", "value2", "");

        let map = ctx.get_internal_map();
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("key1"));
        assert!(map.contains_key("key2"));
    }

    #[test]
    fn test_make_name() {
        let ctx = DeConfigurationContext::new();
        assert_eq!(ctx.make_name("", "param"), "param");
        assert_eq!(ctx.make_name("scope", "param"), "scope.param");
        assert_eq!(ctx.make_name("a.b.c", "param"), "a.b.c.param");
    }

    #[test]
    fn test_load_complex_config() {
        let mut ctx = DeConfigurationContext::new();
        let config = r#"
            !Configuration for providers
            provider.OBJ.OCC.file.length.unit : 1.0
            provider.OBJ.OCC.read.create.shapes : true
            provider.PLY.OCC.write.normals : false
            !Another section
            provider.STL.OCC.write.ascii : true
        "#;
        ctx.load_str(config);

        assert_eq!(ctx.get_real("file.length.unit", "provider.OBJ.OCC"), Some(1.0));
        assert_eq!(ctx.get_boolean("read.create.shapes", "provider.OBJ.OCC"), Some(true));
        assert_eq!(ctx.get_boolean("write.normals", "provider.PLY.OCC"), Some(false));
        assert_eq!(ctx.get_boolean("write.ascii", "provider.STL.OCC"), Some(true));
    }
}
