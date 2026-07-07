// FILE: vrml_data_node.rs
// occt: VrmlData_Node
//
// Faithful port of OCCT VrmlData_Node (DataExchange/TKDEVRML/VrmlData/
// VrmlData_Node.hxx/.cxx): abstract base class for all VRML nodes.
// Provides node naming, scene membership, and virtual read/write methods
// for VRML 2.0 (ISO/IEC 14772-1:1997) parsing and serialization.

use std::cell::RefCell;
use std::rc::Rc;

/// Error status enumeration for VRML reading operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDataErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
    BraceExpected = 4,
    EndBraceExpected = 5,
    NameExpected = 6,
    SeparatorExpected = 7,
    FieldTypeExpected = 8,
    FieldValueExpected = 9,
}

/// Input buffer for VRML reading.
pub struct VrmlDataInBuffer {
    pub line_num: u32,
    pub pos: usize,
}

impl VrmlDataInBuffer {
    pub fn new() -> Self {
        VrmlDataInBuffer {
            line_num: 1,
            pos: 0,
        }
    }
}

impl Default for VrmlDataInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Reference to a Scene containing VRML nodes.
pub struct VrmlDataScene {
    pub name: String,
}

impl VrmlDataScene {
    pub fn new(name: &str) -> Self {
        VrmlDataScene {
            name: name.to_string(),
        }
    }
}

/// Abstract VRML Node base class.
/// All VRML nodes inherit from this class and must implement
/// the Read and Write methods for VRML format serialization.
pub struct VrmlDataNode {
    my_name: String,
    my_scene: Rc<RefCell<VrmlDataScene>>,
}

impl VrmlDataNode {
    /// Constructor with optional name and scene.
    pub fn new(name: Option<&str>, scene: Rc<RefCell<VrmlDataScene>>) -> Self {
        VrmlDataNode {
            my_name: name.unwrap_or("").to_string(),
            my_scene: scene,
        }
    }

    /// Query the name of this node.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name of this node.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Query the Scene containing this node.
    pub fn scene(&self) -> Rc<RefCell<VrmlDataScene>> {
        Rc::clone(&self.my_scene)
    }

    /// Virtual read method: subclasses must parse their fields from buffer.
    /// Returns VrmlDataErrorStatus indicating success or error.
    pub fn read(&mut self, _buffer: &mut VrmlDataInBuffer) -> VrmlDataErrorStatus {
        // Default implementation: subclasses override.
        VrmlDataErrorStatus::Ok
    }

    /// Virtual write method: subclasses must output their representation.
    /// Returns VrmlDataErrorStatus indicating success or error.
    pub fn write(&self, _prefix: Option<&str>) -> VrmlDataErrorStatus {
        // Default implementation: subclasses override.
        VrmlDataErrorStatus::Ok
    }

    /// Returns true if the node is in default state and should be omitted from output.
    pub fn is_default(&self) -> bool {
        true
    }

    /// Write the closing brace and newline for this node.
    pub fn write_closing(&self) -> VrmlDataErrorStatus {
        println!("}}");
        VrmlDataErrorStatus::Ok
    }

    /// Clone this node (creates a new instance with identical data).
    pub fn clone_node(&self) -> VrmlDataNode {
        VrmlDataNode {
            my_name: self.my_name.clone(),
            my_scene: Rc::clone(&self.my_scene),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_creation_with_name() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("test_scene")));
        let node = VrmlDataNode::new(Some("TestNode"), scene.clone());
        assert_eq!(node.name(), "TestNode");
    }

    #[test]
    fn node_creation_without_name() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("test_scene")));
        let node = VrmlDataNode::new(None, scene.clone());
        assert_eq!(node.name(), "");
    }

    #[test]
    fn set_node_name() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("test_scene")));
        let mut node = VrmlDataNode::new(Some("Original"), scene.clone());
        node.set_name("Updated");
        assert_eq!(node.name(), "Updated");
    }

    #[test]
    fn scene_reference() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("MyScene")));
        let node = VrmlDataNode::new(Some("Node1"), scene.clone());
        let retrieved = node.scene();
        assert_eq!(retrieved.borrow().name, "MyScene");
    }

    #[test]
    fn default_status_ok() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("test")));
        let mut node = VrmlDataNode::new(Some("N"), scene);
        let mut buffer = VrmlDataInBuffer::new();
        assert_eq!(node.read(&mut buffer), VrmlDataErrorStatus::Ok);
        assert_eq!(node.write(None), VrmlDataErrorStatus::Ok);
    }

    #[test]
    fn node_is_default() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("test")));
        let node = VrmlDataNode::new(Some("N"), scene);
        assert!(node.is_default());
    }

    #[test]
    fn clone_preserves_name() {
        let scene = Rc::new(RefCell::new(VrmlDataScene::new("test")));
        let node = VrmlDataNode::new(Some("Original"), scene);
        let cloned = node.clone_node();
        assert_eq!(cloned.name(), "Original");
    }
}
