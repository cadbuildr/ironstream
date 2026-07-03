// FILE: step_construct.rs
// occt: STEPConstruct_Tool, STEPConstruct_Assembly, STEPConstruct_Styles

// occt: STEPConstruct_Tool
/// Base utility for accessing STEP model entities.
#[derive(Clone, Debug, Default)]
pub struct StepTool {
    pub model_name: String,
    pub entity_count: usize,
}

impl StepTool {
    pub fn new(model_name: impl Into<String>) -> Self {
        Self { model_name: model_name.into(), entity_count: 0 }
    }

    pub fn nb_entities(&self) -> usize { self.entity_count }
    pub fn is_loaded(&self) -> bool { !self.model_name.is_empty() }
}

// occt: STEPConstruct_Assembly
/// Reads or writes STEP assembly structure (NEXT_ASSEMBLY_USAGE_OCCURRENCES).
#[derive(Clone, Debug, Default)]
pub struct StepAssembly {
    /// (parent, child, reference_designation, quantity) tuples.
    pub relations: Vec<(String, String, String, u32)>,
}

impl StepAssembly {
    pub fn new() -> Self { Self::default() }

    pub fn add_relation(
        &mut self,
        parent: impl Into<String>,
        child: impl Into<String>,
        designation: impl Into<String>,
        qty: u32,
    ) {
        self.relations.push((parent.into(), child.into(), designation.into(), qty));
    }

    pub fn nb_relations(&self) -> usize { self.relations.len() }

    pub fn children_of(&self, parent: &str) -> Vec<&str> {
        self.relations.iter()
            .filter(|(p, _, _, _)| p == parent)
            .map(|(_, c, _, _)| c.as_str())
            .collect()
    }
}

// occt: STEPConstruct_Styles
/// Reads or writes STEP presentation styles (colours, line widths).
#[derive(Clone, Debug, Default)]
pub struct StepStyles {
    pub styles: Vec<StepStyle>,
}

#[derive(Clone, Debug)]
pub struct StepStyle {
    pub entity_name: String,
    pub color_rgb: [f32; 3],
    pub line_width: f32,
}

impl StepStyle {
    pub fn new(entity_name: impl Into<String>, color: [f32; 3], line_width: f32) -> Self {
        Self { entity_name: entity_name.into(), color_rgb: color, line_width }
    }
}

impl StepStyles {
    pub fn new() -> Self { Self::default() }

    pub fn add_style(&mut self, style: StepStyle) {
        self.styles.push(style);
    }

    pub fn get_style(&self, entity: &str) -> Option<&StepStyle> {
        self.styles.iter().find(|s| s.entity_name == entity)
    }

    pub fn nb_styles(&self) -> usize { self.styles.len() }
}

// occt: STEPConstruct_UnitContext
/// Unit conversion factors read from a STEP file.
#[derive(Clone, Debug)]
pub struct UnitContext {
    pub length_factor: f64,
    pub angle_factor: f64,
    pub plane_angle_factor: f64,
}

impl UnitContext {
    pub fn mm() -> Self { Self { length_factor: 1.0, angle_factor: 1.0, plane_angle_factor: 1.0 } }
    pub fn inch() -> Self { Self { length_factor: 25.4, angle_factor: 1.0, plane_angle_factor: 1.0 } }

    pub fn to_mm(&self, val: f64) -> f64 { val * self.length_factor }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_tool_loaded() {
        let t = StepTool::new("MyModel");
        assert!(t.is_loaded());
        assert_eq!(t.nb_entities(), 0);
    }

    #[test]
    fn step_assembly_children() {
        let mut asm = StepAssembly::new();
        asm.add_relation("Root", "PartA", "A", 1);
        asm.add_relation("Root", "PartB", "B", 2);
        let children = asm.children_of("Root");
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn step_styles() {
        let mut styles = StepStyles::new();
        styles.add_style(StepStyle::new("Face_1", [1.0, 0.0, 0.0], 0.5));
        assert!(styles.get_style("Face_1").is_some());
        assert!(styles.get_style("Face_2").is_none());
    }

    #[test]
    fn unit_context_inch_to_mm() {
        let uc = UnitContext::inch();
        assert!((uc.to_mm(1.0) - 25.4).abs() < 1e-10);
    }
}
