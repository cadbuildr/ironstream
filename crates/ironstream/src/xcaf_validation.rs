// FILE: rust/ironstream/crates/ironstream/src/xcaf_validation.rs

// occt-ref: XCAFDoc_VisMaterial // — PBR physically-based rendering material with base color,
// emissive factor, metallic/roughness workflow, and alpha cut-off.
#[derive(Clone, Debug)]
pub struct VisMaterial {
    pub name: String,
    pub base_color: [f64; 4],
    pub emissive_factor: [f64; 3],
    pub metallic: f64,
    pub roughness: f64,
    pub alpha_cut_off: f64,
}

impl VisMaterial {
    /// Create a new, undefined material with the given name.
    /// All numeric fields are initialised to sentinel values that indicate
    /// the material has not yet been configured (metallic = -1.0).
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            base_color: [1.0, 1.0, 1.0, 1.0],
            emissive_factor: [0.0, 0.0, 0.0],
            metallic: -1.0,
            roughness: 1.0,
            alpha_cut_off: 0.5,
        }
    }

    /// Set the PBR metallic and roughness factors.
    /// Both values are clamped to [0.0, 1.0].
    pub fn set_pbr(&mut self, metallic: f64, roughness: f64) {
        self.metallic = metallic.clamp(0.0, 1.0);
        self.roughness = roughness.clamp(0.0, 1.0);
    }

    /// Set the base colour as linear RGBA, each component in [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.base_color = [r, g, b, a];
    }

    /// Return `true` when the material has been explicitly configured
    /// (i.e. `set_pbr` has been called at least once).
    pub fn is_defined(&self) -> bool {
        self.metallic >= 0.0
    }
}

// occt: XCAFDoc_VisMaterialTool // — manages a collection of VisMaterial entries
// and provides lookup by name.
pub struct VisMaterialTool {
    pub materials: Vec<VisMaterial>,
}

impl VisMaterialTool {
    /// Create an empty material tool.
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }

    /// Append a material to the collection.
    pub fn add_material(&mut self, m: VisMaterial) {
        self.materials.push(m);
    }

    /// Find the first material whose name matches exactly, returning a shared
    /// reference, or `None` when no match is found.
    pub fn find_material(&self, name: &str) -> Option<&VisMaterial> {
        self.materials.iter().find(|m| m.name == name)
    }

    /// Return the total number of materials currently stored.
    pub fn material_count(&self) -> usize {
        self.materials.len()
    }
}

impl Default for VisMaterialTool {
    fn default() -> Self {
        Self::new()
    }
}
