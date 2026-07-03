// FILE: rust/ironstream/crates/ironstream/src/xcaf_material.rs

// occt: XCAFDoc_Material — entry storing a named material with optional description, density, and density unit
#[derive(Clone, Debug)]
pub struct XcafMaterial {
    label: String,
    name: String,
    description: String,
    density: f64,
    density_unit: String,
}

impl XcafMaterial {
    /// Create a new material entry with the given label and name.
    /// Description defaults to empty, density to 0.0, density_unit to empty.
    pub fn new(label: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: name.into(),
            description: String::new(),
            density: 0.0,
            density_unit: String::new(),
        }
    }

    /// Return the label string for this material entry.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the material name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the material description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set the material description.
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_owned();
    }

    /// Return the material density.
    pub fn density(&self) -> f64 {
        self.density
    }

    /// Set the material density.
    pub fn set_density(&mut self, density: f64) {
        self.density = density;
    }

    /// Return the density unit string (e.g. "kg/m^3").
    pub fn density_unit(&self) -> &str {
        &self.density_unit
    }

    /// Set the density unit string.
    pub fn set_density_unit(&mut self, unit: &str) {
        self.density_unit = unit.to_owned();
    }
}

// occt: XCAFDoc_MaterialTool — manages a collection of material entries and their associations to shapes
pub struct XcafMaterialTool {
    materials: Vec<XcafMaterial>,
    /// Maps shape_label -> index into `materials`
    shape_materials: Vec<(String, usize)>,
}

impl XcafMaterialTool {
    /// Create an empty material tool.
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            shape_materials: Vec::new(),
        }
    }

    /// Add a material and return its index.
    pub fn add_material(&mut self, m: XcafMaterial) -> usize {
        let idx = self.materials.len();
        self.materials.push(m);
        idx
    }

    /// Find the index of the first material whose name matches exactly.
    pub fn find_material(&self, name: &str) -> Option<usize> {
        self.materials.iter().position(|m| m.name == name)
    }

    /// Retrieve a material by index.
    pub fn get_material(&self, idx: usize) -> Option<&XcafMaterial> {
        self.materials.get(idx)
    }

    /// Return the total number of materials.
    pub fn nb_materials(&self) -> usize {
        self.materials.len()
    }

    /// Associate a shape (by label) with a material index.
    /// If the shape already has an association it is replaced.
    pub fn set_material_for_shape(&mut self, shape_label: &str, mat_idx: usize) {
        for entry in &mut self.shape_materials {
            if entry.0 == shape_label {
                entry.1 = mat_idx;
                return;
            }
        }
        self.shape_materials.push((shape_label.to_owned(), mat_idx));
    }

    /// Return the material index associated with the given shape label, if any.
    pub fn material_for_shape(&self, shape_label: &str) -> Option<usize> {
        self.shape_materials
            .iter()
            .find(|e| e.0 == shape_label)
            .map(|e| e.1)
    }
}

impl Default for XcafMaterialTool {
    fn default() -> Self {
        Self::new()
    }
}
