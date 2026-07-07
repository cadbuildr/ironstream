// FILE: step_shape_block.rs
// occt: StepShape_Block

/// Placeholder for Axis2Placement3d
#[derive(Clone, Debug, PartialEq)]
pub struct Axis2Placement3d {
    origin: [f64; 3],
}

/// Represents a block (rectangular solid) in STEP
pub struct Block {
    name: Option<String>,
    position: Option<Axis2Placement3d>,
    x: f64,
    y: f64,
    z: f64,
}

impl Block {
    /// Create a new Block
    pub fn new() -> Self {
        Block {
            name: None,
            position: None,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Initialize block with name, position, and dimensions
    pub fn init(
        &mut self,
        name: String,
        position: Axis2Placement3d,
        x: f64,
        y: f64,
        z: f64,
    ) {
        self.name = Some(name);
        self.position = Some(position);
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Set the position
    pub fn set_position(&mut self, position: Axis2Placement3d) {
        self.position = Some(position);
    }

    /// Get the position
    pub fn position(&self) -> Option<&Axis2Placement3d> {
        self.position.as_ref()
    }

    /// Set the X dimension
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Get the X dimension
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Set the Y dimension
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Get the Y dimension
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Set the Z dimension
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    /// Get the Z dimension
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let block = Block::new();
        assert_eq!(block.name(), None);
        assert_eq!(block.position(), None);
        assert_eq!(block.x(), 0.0);
        assert_eq!(block.y(), 0.0);
        assert_eq!(block.z(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut block = Block::new();
        let pos = Axis2Placement3d {
            origin: [0.0, 0.0, 0.0],
        };
        block.init("Block1".to_string(), pos.clone(), 10.0, 20.0, 30.0);
        assert_eq!(block.name(), Some("Block1"));
        assert_eq!(block.position(), Some(&pos));
        assert_eq!(block.x(), 10.0);
        assert_eq!(block.y(), 20.0);
        assert_eq!(block.z(), 30.0);
    }

    #[test]
    fn test_set_dimensions() {
        let mut block = Block::new();
        block.set_x(5.0);
        block.set_y(6.0);
        block.set_z(7.0);
        assert_eq!(block.x(), 5.0);
        assert_eq!(block.y(), 6.0);
        assert_eq!(block.z(), 7.0);
    }
}
