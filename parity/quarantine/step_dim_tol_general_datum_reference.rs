// FILE: step_dim_tol_general_datum_reference.rs
// occt: StepDimTol_GeneralDatumReference

//! Representation of STEP entity GeneralDatumReference.

use std::rc::Rc;

/// Logical value for STEP data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logical {
    True,
    False,
    Unknown,
}

/// Product definition shape identifier
#[derive(Debug, Clone)]
pub struct ProductDefinitionShape {
    id: String,
}

impl ProductDefinitionShape {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Datum or common datum reference
#[derive(Debug, Clone)]
pub struct DatumOrCommonDatum {
    id: String,
}

impl DatumOrCommonDatum {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Datum reference modifier
#[derive(Debug, Clone)]
pub enum DatumReferenceModifier {
    Circularity,
    Cylindricity,
    PerpendicularTo,
    ParallelTo,
    Concentricity,
}

impl DatumReferenceModifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatumReferenceModifier::Circularity => "CIRCULARITY",
            DatumReferenceModifier::Cylindricity => "CYLINDRICITY",
            DatumReferenceModifier::PerpendicularTo => "PERPENDICULAR",
            DatumReferenceModifier::ParallelTo => "PARALLEL",
            DatumReferenceModifier::Concentricity => "CONCENTRICITY",
        }
    }
}

/// Base ShapeAspect for GeneralDatumReference
#[derive(Debug, Clone)]
pub struct ShapeAspect {
    name: Option<String>,
    description: Option<String>,
    of_shape: Option<ProductDefinitionShape>,
    product_definitional: Logical,
}

impl ShapeAspect {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            of_shape: None,
            product_definitional: Logical::Unknown,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }

    pub fn of_shape(&self) -> Option<&ProductDefinitionShape> {
        self.of_shape.as_ref()
    }

    pub fn set_of_shape(&mut self, shape: ProductDefinitionShape) {
        self.of_shape = Some(shape);
    }

    pub fn product_definitional(&self) -> Logical {
        self.product_definitional
    }

    pub fn set_product_definitional(&mut self, value: Logical) {
        self.product_definitional = value;
    }
}

impl Default for ShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

/// A GeneralDatumReference is a ShapeAspect with datum reference information
#[derive(Debug, Clone)]
pub struct StepDimTolGeneralDatumReference {
    shape_aspect: ShapeAspect,
    base: Option<DatumOrCommonDatum>,
    modifiers: Vec<DatumReferenceModifier>,
}

impl StepDimTolGeneralDatumReference {
    /// Create a new GeneralDatumReference
    pub fn new() -> Self {
        Self {
            shape_aspect: ShapeAspect::new(),
            base: None,
            modifiers: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        of_shape: ProductDefinitionShape,
        product_definitional: Logical,
        base: DatumOrCommonDatum,
        modifiers: Vec<DatumReferenceModifier>,
    ) {
        self.shape_aspect.set_name(name);
        self.shape_aspect.set_description(description);
        self.shape_aspect.set_of_shape(of_shape);
        self.shape_aspect.set_product_definitional(product_definitional);
        self.base = Some(base);
        self.modifiers = modifiers;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.shape_aspect.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.shape_aspect.set_name(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.shape_aspect.description()
    }

    /// Set the description
    pub fn set_description(&mut self, desc: String) {
        self.shape_aspect.set_description(desc);
    }

    /// Get the base datum
    pub fn base(&self) -> Option<&DatumOrCommonDatum> {
        self.base.as_ref()
    }

    /// Set the base datum
    pub fn set_base(&mut self, base: DatumOrCommonDatum) {
        self.base = Some(base);
    }

    /// Check if modifiers exist
    pub fn has_modifiers(&self) -> bool {
        !self.modifiers.is_empty()
    }

    /// Get modifiers
    pub fn modifiers(&self) -> &[DatumReferenceModifier] {
        &self.modifiers
    }

    /// Set modifiers
    pub fn set_modifiers(&mut self, modifiers: Vec<DatumReferenceModifier>) {
        self.modifiers = modifiers;
    }

    /// Get number of modifiers
    pub fn nb_modifiers(&self) -> usize {
        self.modifiers.len()
    }

    /// Get a specific modifier by index
    pub fn modifier_value(&self, index: usize) -> Option<&DatumReferenceModifier> {
        self.modifiers.get(index)
    }
}

impl Default for StepDimTolGeneralDatumReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gdr = StepDimTolGeneralDatumReference::new();
        assert_eq!(gdr.name(), None);
        assert!(!gdr.has_modifiers());
    }

    #[test]
    fn test_datum_reference_modifier() {
        assert_eq!(DatumReferenceModifier::Circularity.as_str(), "CIRCULARITY");
        assert_eq!(DatumReferenceModifier::Cylindricity.as_str(), "CYLINDRICITY");
    }

    #[test]
    fn test_init() {
        let mut gdr = StepDimTolGeneralDatumReference::new();
        let shape = ProductDefinitionShape::new("SHAPE_1".to_string());
        let base = DatumOrCommonDatum::new("DATUM_A".to_string());
        let mods = vec![DatumReferenceModifier::Circularity];
        gdr.init(
            "ref".to_string(),
            "datum reference".to_string(),
            shape,
            Logical::True,
            base,
            mods,
        );
        assert_eq!(gdr.name(), Some("ref"));
        assert!(gdr.has_modifiers());
        assert_eq!(gdr.nb_modifiers(), 1);
    }

    #[test]
    fn test_set_base() {
        let mut gdr = StepDimTolGeneralDatumReference::new();
        let base = DatumOrCommonDatum::new("DATUM_X".to_string());
        gdr.set_base(base);
        assert!(gdr.base().is_some());
    }

    #[test]
    fn test_modifiers() {
        let mut gdr = StepDimTolGeneralDatumReference::new();
        let mods = vec![
            DatumReferenceModifier::Circularity,
            DatumReferenceModifier::Concentricity,
        ];
        gdr.set_modifiers(mods);
        assert_eq!(gdr.nb_modifiers(), 2);
        assert!(gdr.modifier_value(0).is_some());
    }
}
