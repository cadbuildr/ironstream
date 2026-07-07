// FILE: t_naming_identifier.rs
// occt: TNaming_Identifier

use std::collections::VecDeque;

/// Name types for topological identification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNamingNameType {
    Unknown = 0,
    Unchanged = 1,
    New = 2,
    Deleted = 3,
    Modified = 4,
}

impl TNamingNameType {
    pub fn to_string(&self) -> &'static str {
        match self {
            TNamingNameType::Unknown => "UNKNOWN",
            TNamingNameType::Unchanged => "UNCHANGED",
            TNamingNameType::New => "NEW",
            TNamingNameType::Deleted => "DELETED",
            TNamingNameType::Modified => "MODIFIED",
        }
    }
}

/// Placeholder for a topological shape.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TopodsShape {
    id: u64,
}

impl TopodsShape {
    pub fn new() -> Self {
        TopodsShape { id: 0 }
    }

    pub fn with_id(id: u64) -> Self {
        TopodsShape { id }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// A named shape attribute (simplified placeholder).
#[derive(Clone, Debug)]
pub struct TNamingNamedShape {
    label: String,
    shape: TopodsShape,
}

impl TNamingNamedShape {
    pub fn new(label: String, shape: TopodsShape) -> Self {
        TNamingNamedShape { label, shape }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn shape(&self) -> &TopodsShape {
        &self.shape
    }
}

/// A localizer for topological shape identification.
/// Mirrors OCCT's TNaming_Localizer (simplified).
#[derive(Clone, Debug, Default)]
pub struct TNamingLocalizer {
    // Simplified: would normally track shape evolution
}

impl TNamingLocalizer {
    pub fn new() -> Self {
        TNamingLocalizer {}
    }

    pub fn init(&mut self, _context: &str, _transaction: i32) {
        // Placeholder: would initialize localizer with context
    }
}

/// An identifier for topological shapes.
/// Mirrors OCCT's TNaming_Identifier.
#[derive(Clone, Debug)]
pub struct TNamingIdentifier {
    tdf_label: String,
    shape: TopodsShape,
    done: bool,
    is_feature: bool,
    name_type: TNamingNameType,
    feature: Option<TNamingNamedShape>,
    primitive_args: VecDeque<TNamingNamedShape>,
    shape_args: VecDeque<TopodsShape>,
    ns_context: Option<TNamingNamedShape>,
}

impl TNamingIdentifier {
    /// Creates an identifier for a shape given a context shape.
    pub fn new(label: &str, shape: TopodsShape, _context: &TopodsShape, _geom: bool) -> Self {
        let mut identifier = TNamingIdentifier {
            tdf_label: label.to_string(),
            shape,
            done: false,
            is_feature: false,
            name_type: TNamingNameType::Unknown,
            feature: None,
            primitive_args: VecDeque::new(),
            shape_args: VecDeque::new(),
            ns_context: None,
        };

        identifier.init(_context);
        identifier
    }

    /// Creates an identifier for a shape given a named shape context.
    pub fn new_with_named_shape(
        label: &str,
        shape: TopodsShape,
        context_ns: &TNamingNamedShape,
        _geom: bool,
    ) -> Self {
        TNamingIdentifier {
            tdf_label: label.to_string(),
            shape,
            done: false,
            is_feature: false,
            name_type: TNamingNameType::Unknown,
            feature: None,
            primitive_args: VecDeque::new(),
            shape_args: VecDeque::new(),
            ns_context: Some(context_ns.clone()),
        }
    }

    /// Initializes the identifier with a context shape.
    /// Mirrors OCCT TNaming_Identifier::Init: with no named shape found for
    /// the shape, falls back to AncestorIdentification.
    fn init(&mut self, context: &TopodsShape) {
        let mut localizer = TNamingLocalizer::new();
        localizer.init(&self.tdf_label.clone(), 0);
        // No named-shape registry in this simplified model: NS is null,
        // so ancestor identification is attempted.
        self.ancestor_identification_impl(context);
    }

    /// Mirrors OCCT TNaming_Identifier::AncestorIdentification: with a null
    /// context the type is UNKNOWN and identification fails; with no ancestor
    /// features found, identification also fails (myDone = false).
    fn ancestor_identification_impl(&mut self, context: &TopodsShape) {
        if context.is_null() {
            self.name_type = TNamingNameType::Unknown;
            self.done = false;
            return;
        }
        // Localizer.FindFeaturesInAncestors finds nothing in this simplified
        // model, so AncInFeature is empty and identification is not done.
        self.done = false;
    }

    /// Returns whether identification was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the name type of the identified shape.
    pub fn name_type(&self) -> TNamingNameType {
        self.name_type
    }

    /// Returns whether the identified shape is a feature.
    pub fn is_feature(&self) -> bool {
        self.is_feature
    }

    /// Returns the feature named shape if available.
    pub fn feature(&self) -> Option<&TNamingNamedShape> {
        self.feature.as_ref()
    }

    /// Initializes iteration over arguments.
    pub fn init_args(&mut self) {
        // Placeholder: would reset argument iterators
    }

    /// Returns whether there are more arguments.
    pub fn more_args(&self) -> bool {
        !self.primitive_args.is_empty() || !self.shape_args.is_empty()
    }

    /// Advances to the next argument.
    pub fn next_arg(&mut self) {
        if !self.primitive_args.is_empty() {
            self.primitive_args.pop_front();
        } else if !self.shape_args.is_empty() {
            self.shape_args.pop_front();
        }
    }

    /// Returns whether the current argument is a feature.
    pub fn arg_is_feature(&self) -> bool {
        !self.primitive_args.is_empty()
    }

    /// Returns the feature argument if available.
    pub fn feature_arg(&self) -> Option<&TNamingNamedShape> {
        self.primitive_args.front()
    }

    /// Returns the shape argument if available.
    pub fn shape_arg(&self) -> Option<&TopodsShape> {
        self.shape_args.front()
    }

    /// Returns the shape context.
    pub fn shape_context(&self) -> TopodsShape {
        self.shape.clone()
    }

    /// Returns the named shape of generation if available.
    pub fn named_shape_of_generation(&self) -> Option<&TNamingNamedShape> {
        self.feature.as_ref()
    }

    /// Performs ancestor identification using a localizer.
    pub fn ancestor_identification(&mut self, _localizer: &mut TNamingLocalizer, _context: &TopodsShape) {
        // Placeholder: would trace shape ancestry
        self.done = true;
    }

    /// Performs primitive identification using a localizer.
    pub fn primitive_identification(&mut self, _localizer: &mut TNamingLocalizer, _ns: &TNamingNamedShape) {
        // Placeholder: would identify as a primitive shape
        self.done = true;
        self.name_type = TNamingNameType::New;
    }

    /// Performs generated identification using a localizer.
    pub fn generated_identification(&mut self, _localizer: &mut TNamingLocalizer, _ns: &TNamingNamedShape) {
        // Placeholder: would identify as a generated shape
        self.done = true;
        self.name_type = TNamingNameType::New;
    }

    /// Performs general identification using a localizer.
    pub fn identification(&mut self, _localizer: &mut TNamingLocalizer, _ns: &TNamingNamedShape) {
        // Placeholder: would identify shape using general method
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_type_to_string() {
        assert_eq!(TNamingNameType::Unknown.to_string(), "UNKNOWN");
        assert_eq!(TNamingNameType::New.to_string(), "NEW");
        assert_eq!(TNamingNameType::Deleted.to_string(), "DELETED");
        assert_eq!(TNamingNameType::Modified.to_string(), "MODIFIED");
    }

    #[test]
    fn test_topods_shape() {
        let shape = TopodsShape::with_id(42);
        assert!(!shape.is_null());
        assert_eq!(shape.id, 42);

        let null = TopodsShape::new();
        assert!(null.is_null());
    }

    #[test]
    fn test_named_shape() {
        let shape = TopodsShape::with_id(10);
        let ns = TNamingNamedShape::new("label1".to_string(), shape.clone());
        assert_eq!(ns.label(), "label1");
        assert_eq!(ns.shape(), &shape);
    }

    #[test]
    fn test_localizer() {
        let mut localizer = TNamingLocalizer::new();
        localizer.init("context", 1);
        // Should not panic
    }

    #[test]
    fn test_identifier_creation() {
        let shape = TopodsShape::with_id(5);
        let context = TopodsShape::with_id(1);
        let id = TNamingIdentifier::new("label", shape.clone(), &context, false);
        assert_eq!(id.tdf_label, "label");
        assert_eq!(id.shape, shape);
    }

    #[test]
    fn test_identifier_is_done() {
        let shape = TopodsShape::with_id(3);
        let context = TopodsShape::with_id(1);
        let id = TNamingIdentifier::new("label", shape, &context, false);
        // OCCT: AncestorIdentification finds no ancestor features without a
        // naming registry, so identification is not done after construction.
        assert!(!id.is_done());
    }

    #[test]
    fn test_identifier_name_type() {
        let shape = TopodsShape::with_id(2);
        let context = TopodsShape::with_id(1);
        let id = TNamingIdentifier::new("label", shape, &context, false);
        assert_eq!(id.name_type(), TNamingNameType::Unknown);
    }

    #[test]
    fn test_identifier_is_feature() {
        let shape = TopodsShape::with_id(7);
        let context = TopodsShape::with_id(1);
        let id = TNamingIdentifier::new("label", shape, &context, false);
        assert!(!id.is_feature());
    }

    #[test]
    fn test_identifier_args() {
        let shape = TopodsShape::with_id(4);
        let context = TopodsShape::with_id(1);
        let mut id = TNamingIdentifier::new("label", shape, &context, false);
        id.init_args();
        assert!(!id.more_args());
    }

    #[test]
    fn test_identifier_ancestor_identification() {
        let shape = TopodsShape::with_id(6);
        let context = TopodsShape::with_id(1);
        let mut id = TNamingIdentifier::new("label", shape, &context, false);
        let mut localizer = TNamingLocalizer::new();
        id.ancestor_identification(&mut localizer, &context);
        assert!(id.is_done());
    }

    #[test]
    fn test_identifier_primitive_identification() {
        let shape = TopodsShape::with_id(8);
        let context = TopodsShape::with_id(1);
        let mut id = TNamingIdentifier::new("label", shape, &context, false);
        let ns = TNamingNamedShape::new("ns_label".to_string(), TopodsShape::with_id(100));
        let mut localizer = TNamingLocalizer::new();
        id.primitive_identification(&mut localizer, &ns);
        assert!(id.is_done());
        assert_eq!(id.name_type(), TNamingNameType::New);
    }

    #[test]
    fn test_identifier_with_named_shape_context() {
        let shape = TopodsShape::with_id(9);
        let ns = TNamingNamedShape::new("context_ns".to_string(), TopodsShape::with_id(50));
        let id = TNamingIdentifier::new_with_named_shape("label", shape, &ns, false);
        assert_eq!(id.tdf_label, "label");
        assert!(id.ns_context.is_some());
    }

    #[test]
    fn test_identifier_shape_context() {
        let shape = TopodsShape::with_id(11);
        let context = TopodsShape::with_id(1);
        let id = TNamingIdentifier::new("label", shape.clone(), &context, false);
        assert_eq!(id.shape_context(), shape);
    }
}
