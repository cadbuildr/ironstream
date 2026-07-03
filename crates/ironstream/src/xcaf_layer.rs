// FILE: rust/ironstream/crates/ironstream/src/xcaf_layer.rs

// occt: XCAFDoc_Layer — a single named layer entry with visibility state
#[derive(Clone, Debug)]
pub struct XcafLayer {
    id: usize,
    name: String,
    is_visible: bool,
}

impl XcafLayer {
    /// Create a new layer with the given id and name; visible by default.
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            is_visible: true,
        }
    }

    /// Return the unique id of this layer.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Return the name of this layer.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return whether this layer is currently visible.
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// Set the visibility of this layer.
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    /// Rename this layer.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

// occt: XCAFDoc_LayerTool — manages a collection of named layers and the
// association between shape labels and layer ids
pub struct XcafLayerTool {
    layers: Vec<XcafLayer>,
    next_id: usize,
    /// Maps shape_label -> layer_id
    shape_layers: Vec<(String, usize)>,
}

impl XcafLayerTool {
    /// Create an empty layer tool.
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            next_id: 0,
            shape_layers: Vec::new(),
        }
    }

    /// Add a new layer with the given name and return its id.
    pub fn add_layer(&mut self, name: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.layers.push(XcafLayer::new(id, name));
        id
    }

    /// Find the id of the first layer whose name matches exactly.
    pub fn find_layer(&self, name: &str) -> Option<usize> {
        self.layers.iter().find(|l| l.name == name).map(|l| l.id)
    }

    /// Retrieve a reference to a layer by its id.
    pub fn get_layer(&self, id: usize) -> Option<&XcafLayer> {
        self.layers.iter().find(|l| l.id == id)
    }

    /// Return the total number of layers.
    pub fn nb_layers(&self) -> usize {
        self.layers.len()
    }

    /// Remove the layer with the given id. Returns true if the layer existed.
    pub fn remove_layer(&mut self, id: usize) -> bool {
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            self.layers.remove(pos);
            // also remove all shape associations for this layer
            self.shape_layers.retain(|(_label, lid)| *lid != id);
            true
        } else {
            false
        }
    }

    /// Associate a shape label with a layer id, replacing any prior association
    /// for the same label.
    pub fn set_layer_for_shape(&mut self, shape_label: &str, layer_id: usize) {
        if let Some(entry) = self.shape_layers.iter_mut().find(|(lbl, _)| lbl == shape_label) {
            entry.1 = layer_id;
        } else {
            self.shape_layers.push((shape_label.to_owned(), layer_id));
        }
    }

    /// Return the layer id associated with the given shape label, if any.
    pub fn layer_for_shape(&self, shape_label: &str) -> Option<usize> {
        self.shape_layers
            .iter()
            .find(|(lbl, _)| lbl == shape_label)
            .map(|(_, id)| *id)
    }

    /// Return all shape labels that are associated with the given layer id.
    pub fn shapes_in_layer(&self, layer_id: usize) -> Vec<String> {
        self.shape_layers
            .iter()
            .filter(|(_, lid)| *lid == layer_id)
            .map(|(lbl, _)| lbl.clone())
            .collect()
    }
}

impl Default for XcafLayerTool {
    fn default() -> Self {
        Self::new()
    }
}
