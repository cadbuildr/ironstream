// FILE: xde_doc.rs
//! `xde_doc` — XDE document (XCAF) framework, mirroring OpenCascade's
//! `XDE` / `XCAF` package family (`TDocStd`, `XCAFDoc`, `XCAFApp`).
//!
//! # Scope
//!
//! * [`XCAFDoc_DocumentTool`]  — document-level accessor; creates / retrieves
//!   the four standard tool attributes on a document.
//! * [`XCAFDoc_ShapeTool`]     — manages the shape hierarchy (free shapes,
//!   components, references, instances).
//! * [`XCAFDoc_ColorTool`]     — manages per-shape / per-face color assignments.
//! * [`XCAFDoc_LayerTool`]     — manages layer definitions and assigns shapes
//!   to layers.
//!
//! # OCCT correspondence
//!
//! | OCCT class               | IronStream type              |
//! |--------------------------|------------------------------|
//! | `XCAFDoc_DocumentTool`   | [`XCAFDoc_DocumentTool`]     |
//! | `XCAFDoc_ShapeTool`      | [`XCAFDoc_ShapeTool`]        |
//! | `XCAFDoc_ColorTool`      | [`XCAFDoc_ColorTool`]        |
//! | `XCAFDoc_LayerTool`      | [`XCAFDoc_LayerTool`]        |
//!
//! All in-memory; no actual file I/O is performed.  The API surface matches
//! OCCT so that callers can use the same method names as in C++.

use std::collections::HashMap;

use crate::gp::Pnt;
use crate::top_exp::TopoDS_Shape;

// ---------------------------------------------------------------------------
// Label — lightweight handle representing a node in the XCAF label tree
// ---------------------------------------------------------------------------

/// A handle identifying a node in the XCAF document label tree.
///
/// In OCCT a `TDF_Label` is a persistent node in a `TDF_Data` tree identified
/// by an entry string such as `"0:1:1"`.  Here we use a `u64` id for
/// simplicity.
// occt-ref: TDF_Label
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub u64);

impl Label {
    /// Null label — matches OCCT's `TDF_Label::IsNull()`.
    pub fn null() -> Self {
        Label(0)
    }

    /// Returns `true` when this is the null label.
    pub fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Label({})", self.0)
    }
}

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

/// An RGBA color value.
///
/// Components are in `[0.0, 1.0]`.  Alpha defaults to `1.0` (fully opaque).
// occt-ref: Quantity_Color // / Quantity_ColorRGBA
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    /// Construct an opaque RGB color.
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Construct an RGBA color.
    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Black.
    pub fn black() -> Self { Self::rgb(0.0, 0.0, 0.0) }
    /// White.
    pub fn white() -> Self { Self::rgb(1.0, 1.0, 1.0) }
    /// Red.
    pub fn red()   -> Self { Self::rgb(1.0, 0.0, 0.0) }
    /// Green.
    pub fn green() -> Self { Self::rgb(0.0, 1.0, 0.0) }
    /// Blue.
    pub fn blue()  -> Self { Self::rgb(0.0, 0.0, 1.0) }

    /// Linearly interpolate between two colors.
    pub fn lerp(self, other: Color, t: f64) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Luminance (perceived brightness) using the ITU-R BT.601 formula.
    pub fn luminance(self) -> f64 {
        0.299 * self.r + 0.587 * self.g + 0.114 * self.b
    }
}

impl Default for Color {
    fn default() -> Self { Self::white() }
}

// ---------------------------------------------------------------------------
// ColorKind — matches XCAFDoc_ColorType
// ---------------------------------------------------------------------------

/// Specifies which part of a shape the color is assigned to.
// occt: XCAFDoc_ColorType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorKind {
    /// Generic color (XCAFDoc_ColorGen).
    Gen,
    /// Surface color (XCAFDoc_ColorSurf).
    Surf,
    /// Curve color (XCAFDoc_ColorCurv).
    Curv,
}

// ---------------------------------------------------------------------------
// Internal counter — generates monotonically increasing Label ids
// ---------------------------------------------------------------------------

struct LabelAlloc {
    next: u64,
}

impl LabelAlloc {
    fn new() -> Self {
        Self { next: 1 }
    }

    fn alloc(&mut self) -> Label {
        let id = self.next;
        self.next += 1;
        Label(id)
    }
}

// ---------------------------------------------------------------------------
// XCAFDoc_ShapeTool
// ---------------------------------------------------------------------------

/// Manages the shape hierarchy within an XCAF document.
///
/// Shapes are stored under a "shapes" root label.  Free shapes are top-level
/// children; components are shapes referenced by an assembly.
///
/// # OCCT correspondence
///
/// | OCCT method                         | Rust method                             |
/// |-------------------------------------|-----------------------------------------|
/// | `AddShape(shape)`                   | [`add_shape`]                           |
/// | `FindShape(shape, label)`           | [`find_shape`]                          |
/// | `GetShape(label, shape)`            | [`get_shape`]                           |
/// | `GetFreeShapes(labels)`             | [`get_free_shapes`]                     |
/// | `IsShape(label)`                    | [`is_shape`]                            |
/// | `IsAssembly(label)`                 | [`is_assembly`]                         |
/// | `AddComponent(assembly, comp, loc)` | [`add_component`]                       |
/// | `GetComponents(label, labels)`      | [`get_components`]                      |
/// | `NbShapes()`                        | [`nb_shapes`]                           |
/// | `RemoveShape(label)`                | [`remove_shape`]                        |
// occt: XCAFDoc_ShapeTool
pub struct XCAFDoc_ShapeTool {
    alloc: LabelAlloc,
    /// label → stored shape
    shapes: HashMap<Label, TopoDS_Shape>,
    /// label → human-readable name
    names: HashMap<Label, String>,
    /// label → list of component labels (for assemblies)
    components: HashMap<Label, Vec<Label>>,
    /// label → optional location (as a Pnt translation offset)
    locations: HashMap<Label, Pnt>,
}

impl XCAFDoc_ShapeTool {
    /// Create an empty shape tool.
    pub fn new() -> Self {
        Self {
            alloc: LabelAlloc::new(),
            shapes: HashMap::new(),
            names: HashMap::new(),
            components: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    /// Add a shape and return its label.  If the same shape (by pointer
    /// equality / id) is already stored the existing label is returned.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::AddShape`.
    pub fn add_shape(&mut self, shape: TopoDS_Shape) -> Label {
        // Check whether an identical shape is already stored.
        if let Some(label) = self.find_shape(&shape) {
            return label;
        }
        let lbl = self.alloc.alloc();
        self.shapes.insert(lbl, shape);
        lbl
    }

    /// Add a shape with an explicit name.
    pub fn add_named_shape(&mut self, shape: TopoDS_Shape, name: impl Into<String>) -> Label {
        let lbl = self.add_shape(shape);
        self.names.insert(lbl, name.into());
        lbl
    }

    /// Find the label for a shape, or `None` if not stored.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::FindShape`.
    pub fn find_shape(&self, shape: &TopoDS_Shape) -> Option<Label> {
        for (lbl, s) in &self.shapes {
            if shapes_eq(s, shape) {
                return Some(*lbl);
            }
        }
        None
    }

    /// Retrieve the shape stored at `label`, or `None`.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::GetShape`.
    pub fn get_shape(&self, label: Label) -> Option<&TopoDS_Shape> {
        self.shapes.get(&label)
    }

    /// `true` when `label` refers to a stored shape.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::IsShape`.
    pub fn is_shape(&self, label: Label) -> bool {
        self.shapes.contains_key(&label)
    }

    /// `true` when `label` refers to an assembly (has at least one component).
    ///
    /// Mirrors `XCAFDoc_ShapeTool::IsAssembly`.
    pub fn is_assembly(&self, label: Label) -> bool {
        self.components.get(&label).map_or(false, |c| !c.is_empty())
    }

    /// Number of shapes currently stored.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::NbShapes`.
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    /// Returns the labels of all free (top-level) shapes.
    ///
    /// Free shapes are those that are not themselves components of another
    /// assembly.  Mirrors `XCAFDoc_ShapeTool::GetFreeShapes`.
    pub fn get_free_shapes(&self) -> Vec<Label> {
        // Collect all labels that appear as a component of some assembly.
        let mut referenced: std::collections::HashSet<Label> = std::collections::HashSet::new();
        for comps in self.components.values() {
            for lbl in comps {
                referenced.insert(*lbl);
            }
        }
        self.shapes
            .keys()
            .filter(|lbl| !referenced.contains(lbl))
            .copied()
            .collect()
    }

    /// Add `component_label` as a component of the assembly at
    /// `assembly_label`, with an optional translation offset `location`.
    ///
    /// If `assembly_label` does not yet have a shape entry one is *not*
    /// created automatically — the caller should add a placeholder shape first.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::AddComponent`.
    pub fn add_component(
        &mut self,
        assembly_label: Label,
        component_label: Label,
        location: Option<Pnt>,
    ) {
        self.components
            .entry(assembly_label)
            .or_default()
            .push(component_label);
        if let Some(loc) = location {
            self.locations.insert(component_label, loc);
        }
    }

    /// Return the component labels of the assembly at `label`.
    ///
    /// Mirrors `XCAFDoc_ShapeTool::GetComponents`.
    pub fn get_components(&self, label: Label) -> Vec<Label> {
        self.components
            .get(&label)
            .cloned()
            .unwrap_or_default()
    }

    /// Return the location (translation offset) associated with `label`,
    /// or the origin if none was set.
    pub fn get_location(&self, label: Label) -> Pnt {
        self.locations
            .get(&label)
            .copied()
            .unwrap_or(Pnt::origin())
    }

    /// Return the name associated with `label`, if any.
    pub fn get_name(&self, label: Label) -> Option<&str> {
        self.names.get(&label).map(|s| s.as_str())
    }

    /// Remove the shape at `label` (and its component list).
    ///
    /// Mirrors `XCAFDoc_ShapeTool::RemoveShape`.
    pub fn remove_shape(&mut self, label: Label) -> bool {
        let removed = self.shapes.remove(&label).is_some();
        self.components.remove(&label);
        self.names.remove(&label);
        self.locations.remove(&label);
        removed
    }
}

impl Default for XCAFDoc_ShapeTool {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// XCAFDoc_ColorTool
// ---------------------------------------------------------------------------

/// Manages per-shape and per-face color assignments within an XCAF document.
///
/// Colors are identified by a [`Label`].  Multiple kinds of color can be
/// assigned to a single shape label (surface, curve, generic).
///
/// # OCCT correspondence
///
/// | OCCT method                           | Rust method                     |
/// |---------------------------------------|---------------------------------|
/// | `AddColor(color)`                     | [`add_color`]                   |
/// | `FindColor(color, label)`             | [`find_color`]                  |
/// | `GetColor(label, kind, color)`        | [`get_color`]                   |
/// | `SetColor(label, color, kind)`        | [`set_color`]                   |
/// | `UnSetColor(label, kind)`             | [`unset_color`]                 |
/// | `IsSet(label, kind)`                  | [`is_set`]                      |
/// | `GetColors(labels)`                   | [`get_colors`]                  |
/// | `NbColors()`                          | [`nb_colors`]                   |
// occt: XCAFDoc_ColorTool
pub struct XCAFDoc_ColorTool {
    alloc: LabelAlloc,
    /// color label → Color value
    color_defs: HashMap<Label, Color>,
    /// (shape label, ColorKind) → color label
    assignments: HashMap<(Label, ColorKind), Label>,
}

impl XCAFDoc_ColorTool {
    /// Create an empty color tool.
    pub fn new() -> Self {
        Self {
            alloc: LabelAlloc::new(),
            color_defs: HashMap::new(),
            assignments: HashMap::new(),
        }
    }

    /// Register a color value and return its label.  If an identical color is
    /// already registered the existing label is returned.
    ///
    /// Mirrors `XCAFDoc_ColorTool::AddColor`.
    pub fn add_color(&mut self, color: Color) -> Label {
        if let Some(lbl) = self.find_color(&color) {
            return lbl;
        }
        let lbl = self.alloc.alloc();
        self.color_defs.insert(lbl, color);
        lbl
    }

    /// Find the label for an already-registered color, or `None`.
    ///
    /// Mirrors `XCAFDoc_ColorTool::FindColor`.
    pub fn find_color(&self, color: &Color) -> Option<Label> {
        for (lbl, c) in &self.color_defs {
            if colors_eq(c, color) {
                return Some(*lbl);
            }
        }
        None
    }

    /// Return the color value stored at `color_label`, or `None`.
    pub fn get_color_value(&self, color_label: Label) -> Option<Color> {
        self.color_defs.get(&color_label).copied()
    }

    /// Number of distinct registered colors.
    ///
    /// Mirrors `XCAFDoc_ColorTool::NbColors`.
    pub fn nb_colors(&self) -> usize {
        self.color_defs.len()
    }

    /// Return the labels of all registered colors.
    ///
    /// Mirrors `XCAFDoc_ColorTool::GetColors`.
    pub fn get_colors(&self) -> Vec<Label> {
        let mut labels: Vec<Label> = self.color_defs.keys().copied().collect();
        labels.sort_by_key(|l| l.0);
        labels
    }

    /// Assign `color` to the shape identified by `shape_label` with the given
    /// `kind`.  If `color` is not yet registered it is registered first.
    ///
    /// Mirrors `XCAFDoc_ColorTool::SetColor`.
    pub fn set_color(&mut self, shape_label: Label, color: Color, kind: ColorKind) {
        let color_lbl = self.add_color(color);
        self.assignments.insert((shape_label, kind), color_lbl);
    }

    /// Get the color assigned to `shape_label` for `kind`.
    ///
    /// Returns `None` when no color has been assigned.
    ///
    /// Mirrors `XCAFDoc_ColorTool::GetColor`.
    pub fn get_color(&self, shape_label: Label, kind: ColorKind) -> Option<Color> {
        let color_lbl = self.assignments.get(&(shape_label, kind))?;
        self.color_defs.get(color_lbl).copied()
    }

    /// Remove the color assignment for `shape_label` / `kind`.
    ///
    /// Mirrors `XCAFDoc_ColorTool::UnSetColor`.
    pub fn unset_color(&mut self, shape_label: Label, kind: ColorKind) {
        self.assignments.remove(&(shape_label, kind));
    }

    /// `true` when a color of the given `kind` is assigned to `shape_label`.
    ///
    /// Mirrors `XCAFDoc_ColorTool::IsSet`.
    pub fn is_set(&self, shape_label: Label, kind: ColorKind) -> bool {
        self.assignments.contains_key(&(shape_label, kind))
    }

    /// Return all (shape_label, kind, color) triples currently assigned.
    pub fn all_assignments(&self) -> Vec<(Label, ColorKind, Color)> {
        let mut out = Vec::new();
        for ((shape_lbl, kind), color_lbl) in &self.assignments {
            if let Some(color) = self.color_defs.get(color_lbl) {
                out.push((*shape_lbl, *kind, *color));
            }
        }
        out
    }
}

impl Default for XCAFDoc_ColorTool {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// XCAFDoc_LayerTool
// ---------------------------------------------------------------------------

/// Manages layer definitions and assigns shapes to layers within an XCAF
/// document.
///
/// # OCCT correspondence
///
/// | OCCT method                        | Rust method                      |
/// |------------------------------------|----------------------------------|
/// | `AddLayer(name)`                   | [`add_layer`]                    |
/// | `FindLayer(name, label)`           | [`find_layer`]                   |
/// | `GetLayer(label, name)`            | [`get_layer_name`]               |
/// | `GetLayers(labels)`                | [`get_layers`]                   |
/// | `NbLayers()`                       | [`nb_layers`]                    |
/// | `SetLayer(shape_label, lyr_label)` | [`set_layer`]                    |
/// | `GetLayers(shape_label, labels)`   | [`get_shape_layers`]             |
/// | `UnSetLayer(shape_label, lyr)`     | [`unset_layer`]                  |
/// | `IsSet(shape_label, lyr_label)`    | [`is_set`]                       |
/// | `RemoveLayer(label)`               | [`remove_layer`]                 |
/// | `IsVisible(label)`                 | [`is_visible`]                   |
/// | `SetVisibility(label, flag)`       | [`set_visibility`]               |
// occt: XCAFDoc_LayerTool
pub struct XCAFDoc_LayerTool {
    alloc: LabelAlloc,
    /// layer label → layer name
    layers: HashMap<Label, String>,
    /// layer label → visibility
    visibility: HashMap<Label, bool>,
    /// shape label → set of layer labels
    shape_layers: HashMap<Label, Vec<Label>>,
}

impl XCAFDoc_LayerTool {
    /// Create an empty layer tool.
    pub fn new() -> Self {
        Self {
            alloc: LabelAlloc::new(),
            layers: HashMap::new(),
            visibility: HashMap::new(),
            shape_layers: HashMap::new(),
        }
    }

    /// Add a layer with the given name and return its label.  If a layer with
    /// that name already exists its label is returned.
    ///
    /// Mirrors `XCAFDoc_LayerTool::AddLayer`.
    pub fn add_layer(&mut self, name: impl Into<String>) -> Label {
        let name = name.into();
        if let Some(lbl) = self.find_layer(&name) {
            return lbl;
        }
        let lbl = self.alloc.alloc();
        self.layers.insert(lbl, name);
        self.visibility.insert(lbl, true);
        lbl
    }

    /// Find the label for a layer by name, or `None`.
    ///
    /// Mirrors `XCAFDoc_LayerTool::FindLayer`.
    pub fn find_layer(&self, name: &str) -> Option<Label> {
        for (lbl, n) in &self.layers {
            if n == name {
                return Some(*lbl);
            }
        }
        None
    }

    /// Return the name of the layer at `label`, or `None`.
    ///
    /// Mirrors `XCAFDoc_LayerTool::GetLayer` (name form).
    pub fn get_layer_name(&self, label: Label) -> Option<&str> {
        self.layers.get(&label).map(|s| s.as_str())
    }

    /// Number of defined layers.
    ///
    /// Mirrors `XCAFDoc_LayerTool::NbLayers`.
    pub fn nb_layers(&self) -> usize {
        self.layers.len()
    }

    /// Return labels of all defined layers, sorted by id.
    ///
    /// Mirrors `XCAFDoc_LayerTool::GetLayers`.
    pub fn get_layers(&self) -> Vec<Label> {
        let mut labels: Vec<Label> = self.layers.keys().copied().collect();
        labels.sort_by_key(|l| l.0);
        labels
    }

    /// Assign `shape_label` to `layer_label`.  Duplicate assignments are
    /// silently ignored.
    ///
    /// Mirrors `XCAFDoc_LayerTool::SetLayer`.
    pub fn set_layer(&mut self, shape_label: Label, layer_label: Label) {
        let v = self.shape_layers.entry(shape_label).or_default();
        if !v.contains(&layer_label) {
            v.push(layer_label);
        }
    }

    /// Remove `shape_label` from `layer_label`.
    ///
    /// Mirrors `XCAFDoc_LayerTool::UnSetLayer`.
    pub fn unset_layer(&mut self, shape_label: Label, layer_label: Label) {
        if let Some(v) = self.shape_layers.get_mut(&shape_label) {
            v.retain(|l| *l != layer_label);
        }
    }

    /// Return the layer labels assigned to `shape_label`.
    ///
    /// Mirrors `XCAFDoc_LayerTool::GetLayers(shape_label, ...)`.
    pub fn get_shape_layers(&self, shape_label: Label) -> Vec<Label> {
        self.shape_layers
            .get(&shape_label)
            .cloned()
            .unwrap_or_default()
    }

    /// `true` when `shape_label` is assigned to `layer_label`.
    ///
    /// Mirrors `XCAFDoc_LayerTool::IsSet`.
    pub fn is_set(&self, shape_label: Label, layer_label: Label) -> bool {
        self.shape_layers
            .get(&shape_label)
            .map_or(false, |v| v.contains(&layer_label))
    }

    /// Remove a layer and all shape assignments to it.
    ///
    /// Mirrors `XCAFDoc_LayerTool::RemoveLayer`.
    pub fn remove_layer(&mut self, label: Label) -> bool {
        let removed = self.layers.remove(&label).is_some();
        self.visibility.remove(&label);
        // Remove the layer from every shape's assignment list.
        for v in self.shape_layers.values_mut() {
            v.retain(|l| *l != label);
        }
        removed
    }

    /// `true` when the layer is visible (default after creation).
    ///
    /// Mirrors `XCAFDoc_LayerTool::IsVisible`.
    pub fn is_visible(&self, label: Label) -> bool {
        self.visibility.get(&label).copied().unwrap_or(true)
    }

    /// Set the visibility flag for `label`.
    ///
    /// Mirrors `XCAFDoc_LayerTool::SetVisibility`.
    pub fn set_visibility(&mut self, label: Label, visible: bool) {
        self.visibility.insert(label, visible);
    }

    /// Return all shapes assigned to `layer_label`.
    pub fn shapes_on_layer(&self, layer_label: Label) -> Vec<Label> {
        self.shape_layers
            .iter()
            .filter(|(_, layers)| layers.contains(&layer_label))
            .map(|(shape_lbl, _)| *shape_lbl)
            .collect()
    }
}

impl Default for XCAFDoc_LayerTool {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// XCAFDoc_DocumentTool
// ---------------------------------------------------------------------------

/// The central XCAF document: owns the four standard tools (shape, color,
/// layer) and provides static-style accessors that mirror OCCT's
/// `XCAFDoc_DocumentTool`.
///
/// In OCCT the tools are stored as `TDF_Attribute` sub-objects on fixed
/// labels within the `TDF_Data` tree.  Here we own them directly.
///
/// # OCCT correspondence
///
/// | OCCT method                      | Rust method                          |
/// |----------------------------------|--------------------------------------|
/// | `ShapeTool(doc)`                 | [`shape_tool`] / [`shape_tool_mut`]  |
/// | `ColorTool(doc)`                 | [`color_tool`] / [`color_tool_mut`]  |
/// | `LayerTool(doc)`                 | [`layer_tool`] / [`layer_tool_mut`]  |
/// | `DocLabel(doc)`                  | [`doc_label`]                        |
/// | `IsXCAFDocument(doc)`            | always `true` for this type          |
// occt: XCAFDoc_DocumentTool
pub struct XCAFDoc_DocumentTool {
    shape_tool: XCAFDoc_ShapeTool,
    color_tool: XCAFDoc_ColorTool,
    layer_tool: XCAFDoc_LayerTool,
    /// Human-readable document title.
    title: String,
    /// Internal document label (root).
    doc_label: Label,
}

impl XCAFDoc_DocumentTool {
    /// Create a new, empty XCAF document.
    pub fn new() -> Self {
        Self {
            shape_tool: XCAFDoc_ShapeTool::new(),
            color_tool: XCAFDoc_ColorTool::new(),
            layer_tool: XCAFDoc_LayerTool::new(),
            title: String::new(),
            doc_label: Label(1),
        }
    }

    /// Create with a document title.
    pub fn with_title(title: impl Into<String>) -> Self {
        let mut d = Self::new();
        d.title = title.into();
        d
    }

    // --- Title ---------------------------------------------------------------

    /// Return the document title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the document title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    // --- Doc label -----------------------------------------------------------

    /// The root label of this document.
    ///
    /// Mirrors `XCAFDoc_DocumentTool::DocLabel`.
    pub fn doc_label(&self) -> Label {
        self.doc_label
    }

    // --- Tool accessors -------------------------------------------------------

    /// Shared reference to the shape tool.
    ///
    /// Mirrors `XCAFDoc_DocumentTool::ShapeTool(doc)`.
    pub fn shape_tool(&self) -> &XCAFDoc_ShapeTool {
        &self.shape_tool
    }

    /// Mutable reference to the shape tool.
    pub fn shape_tool_mut(&mut self) -> &mut XCAFDoc_ShapeTool {
        &mut self.shape_tool
    }

    /// Shared reference to the color tool.
    ///
    /// Mirrors `XCAFDoc_DocumentTool::ColorTool(doc)`.
    pub fn color_tool(&self) -> &XCAFDoc_ColorTool {
        &self.color_tool
    }

    /// Mutable reference to the color tool.
    pub fn color_tool_mut(&mut self) -> &mut XCAFDoc_ColorTool {
        &mut self.color_tool
    }

    /// Shared reference to the layer tool.
    ///
    /// Mirrors `XCAFDoc_DocumentTool::LayerTool(doc)`.
    pub fn layer_tool(&self) -> &XCAFDoc_LayerTool {
        &self.layer_tool
    }

    /// Mutable reference to the layer tool.
    pub fn layer_tool_mut(&mut self) -> &mut XCAFDoc_LayerTool {
        &mut self.layer_tool
    }

    // --- Convenience wrappers ------------------------------------------------

    /// Add a shape and return its label.  Delegates to `ShapeTool`.
    pub fn add_shape(&mut self, shape: TopoDS_Shape) -> Label {
        self.shape_tool.add_shape(shape)
    }

    /// Assign a color to a shape.  Delegates to `ColorTool`.
    pub fn set_color(&mut self, shape_label: Label, color: Color, kind: ColorKind) {
        self.color_tool.set_color(shape_label, color, kind);
    }

    /// Assign a shape to a named layer (creating the layer if needed).
    pub fn assign_layer(&mut self, shape_label: Label, layer_name: &str) -> Label {
        let lyr = self.layer_tool.add_layer(layer_name);
        self.layer_tool.set_layer(shape_label, lyr);
        lyr
    }

    /// Return a summary string suitable for diagnostics.
    pub fn summary(&self) -> String {
        format!(
            "XCAFDocument '{}': {} shapes, {} colors, {} layers",
            self.title,
            self.shape_tool.nb_shapes(),
            self.color_tool.nb_colors(),
            self.layer_tool.nb_layers(),
        )
    }
}

impl Default for XCAFDoc_DocumentTool {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Compare two shapes by their discriminant + id.  We use the id field common
/// to every TopoDS variant as a surrogate for pointer/handle equality.
fn shapes_eq(a: &TopoDS_Shape, b: &TopoDS_Shape) -> bool {
    use TopoDS_Shape::*;
    match (a, b) {
        (Vertex(x), Vertex(y))       => x.id == y.id,
        (Edge(x), Edge(y))           => x.id == y.id,
        (Wire(x), Wire(y))           => x.id == y.id,
        (Face(x), Face(y))           => x.id == y.id,
        (Shell(x), Shell(y))         => x.id == y.id,
        (Solid(x), Solid(y))         => x.id == y.id,
        (CompSolid(x), CompSolid(y)) => x.id == y.id,
        (Compound(x), Compound(y))   => x.id == y.id,
        _                            => false,
    }
}

/// Compare two colors within a small tolerance.
fn colors_eq(a: &Color, b: &Color) -> bool {
    const EPS: f64 = 1e-9;
    (a.r - b.r).abs() < EPS
        && (a.g - b.g).abs() < EPS
        && (a.b - b.b).abs() < EPS
        && (a.a - b.a).abs() < EPS
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::top_exp::{TopoDS_Vertex, TopoDS_Edge, TopoDS_Wire, TopoDS_Face, TopoDS_Solid, TopoDS_Shell};

    // --- helpers ---

    fn make_vertex(id: u64) -> TopoDS_Shape {
        TopoDS_Shape::Vertex(TopoDS_Vertex { id, x: id as f64, y: 0.0, z: 0.0 })
    }

    fn make_edge(id: u64) -> TopoDS_Shape {
        TopoDS_Shape::Edge(TopoDS_Edge { id, vertices: vec![] })
    }

    // --- Label ----------------------------------------------------------------

    #[test]
    fn label_null_is_zero() {
        let null = Label::null();
        assert!(null.is_null());
        assert_eq!(null.0, 0);
    }

    #[test]
    fn label_display_format() {
        let l = Label(42);
        assert_eq!(l.to_string(), "Label(42)");
    }

    // --- Color ----------------------------------------------------------------

    #[test]
    fn color_luminance_values() {
        let white = Color::white();
        // Luminance of white should be ~1.0
        assert!((white.luminance() - 1.0).abs() < 1e-9);
        let black = Color::black();
        assert!((black.luminance()).abs() < 1e-9);
    }

    #[test]
    fn color_lerp_midpoint() {
        let black = Color::black();
        let white = Color::white();
        let mid = black.lerp(white, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-9);
        assert!((mid.g - 0.5).abs() < 1e-9);
        assert!((mid.b - 0.5).abs() < 1e-9);
    }

    // --- XCAFDoc_ShapeTool ----------------------------------------------------

    #[test]
    fn shape_tool_add_and_count() {
        let mut st = XCAFDoc_ShapeTool::new();
        let v = make_vertex(1);
        let lbl = st.add_shape(v);
        assert!(!lbl.is_null());
        assert_eq!(st.nb_shapes(), 1);
    }

    #[test]
    fn shape_tool_add_same_shape_returns_same_label() {
        let mut st = XCAFDoc_ShapeTool::new();
        let v = make_vertex(5);
        let l1 = st.add_shape(v.clone());
        let l2 = st.add_shape(v);
        assert_eq!(l1, l2);
        assert_eq!(st.nb_shapes(), 1);
    }

    #[test]
    fn shape_tool_is_shape_and_find() {
        let mut st = XCAFDoc_ShapeTool::new();
        let e = make_edge(10);
        let lbl = st.add_shape(e.clone());
        assert!(st.is_shape(lbl));
        assert_eq!(st.find_shape(&e), Some(lbl));
    }

    #[test]
    fn shape_tool_is_assembly() {
        let mut st = XCAFDoc_ShapeTool::new();
        let asm = make_edge(99);
        let comp = make_vertex(100);
        let asm_lbl = st.add_shape(asm);
        let comp_lbl = st.add_shape(comp);
        assert!(!st.is_assembly(asm_lbl));
        st.add_component(asm_lbl, comp_lbl, None);
        assert!(st.is_assembly(asm_lbl));
        let comps = st.get_components(asm_lbl);
        assert_eq!(comps, vec![comp_lbl]);
    }

    #[test]
    fn shape_tool_free_shapes() {
        let mut st = XCAFDoc_ShapeTool::new();
        let asm = make_edge(1);
        let comp = make_vertex(2);
        let asm_lbl = st.add_shape(asm);
        let comp_lbl = st.add_shape(comp);
        st.add_component(asm_lbl, comp_lbl, None);
        let free = st.get_free_shapes();
        assert!(free.contains(&asm_lbl));
        assert!(!free.contains(&comp_lbl));
    }

    #[test]
    fn shape_tool_remove_shape() {
        let mut st = XCAFDoc_ShapeTool::new();
        let v = make_vertex(7);
        let lbl = st.add_shape(v);
        assert!(st.remove_shape(lbl));
        assert!(!st.is_shape(lbl));
        assert!(!st.remove_shape(lbl)); // second remove returns false
    }

    // --- XCAFDoc_ColorTool ----------------------------------------------------

    #[test]
    fn color_tool_add_and_dedup() {
        let mut ct = XCAFDoc_ColorTool::new();
        let red = Color::red();
        let l1 = ct.add_color(red);
        let l2 = ct.add_color(red);
        assert_eq!(l1, l2);
        assert_eq!(ct.nb_colors(), 1);
    }

    #[test]
    fn color_tool_set_get_unset() {
        let mut ct = XCAFDoc_ColorTool::new();
        let shape_lbl = Label(1);
        ct.set_color(shape_lbl, Color::blue(), ColorKind::Surf);
        assert!(ct.is_set(shape_lbl, ColorKind::Surf));
        let c = ct.get_color(shape_lbl, ColorKind::Surf).unwrap();
        assert!((c.b - 1.0).abs() < 1e-9);
        ct.unset_color(shape_lbl, ColorKind::Surf);
        assert!(!ct.is_set(shape_lbl, ColorKind::Surf));
    }

    // --- XCAFDoc_LayerTool ----------------------------------------------------

    #[test]
    fn layer_tool_add_and_dedup() {
        let mut lt = XCAFDoc_LayerTool::new();
        let l1 = lt.add_layer("Geometry");
        let l2 = lt.add_layer("Geometry");
        assert_eq!(l1, l2);
        assert_eq!(lt.nb_layers(), 1);
    }

    #[test]
    fn layer_tool_set_visibility() {
        let mut lt = XCAFDoc_LayerTool::new();
        let lyr = lt.add_layer("Hidden");
        assert!(lt.is_visible(lyr));
        lt.set_visibility(lyr, false);
        assert!(!lt.is_visible(lyr));
    }

    #[test]
    fn layer_tool_assign_and_query() {
        let mut lt = XCAFDoc_LayerTool::new();
        let lyr = lt.add_layer("L1");
        let shape_lbl = Label(55);
        assert!(!lt.is_set(shape_lbl, lyr));
        lt.set_layer(shape_lbl, lyr);
        assert!(lt.is_set(shape_lbl, lyr));
        assert_eq!(lt.get_shape_layers(shape_lbl), vec![lyr]);
        lt.unset_layer(shape_lbl, lyr);
        assert!(!lt.is_set(shape_lbl, lyr));
    }
}
