// FILE: iges_appli_nodal_displ_and_rot.rs
// occt: IGESAppli_NodalDisplAndRot

/// Stores nodal displacement and rotation data.
#[derive(Clone, Debug)]
pub struct IgesAppliNodalDisplAndRot {
    node_id: i32,
    displacement: [f64; 3],
    rotation: [f64; 3],
}

impl IgesAppliNodalDisplAndRot {
    pub fn new() -> Self {
        Self {
            node_id: 0,
            displacement: [0.0; 3],
            rotation: [0.0; 3],
        }
    }

    pub fn init(&mut self, nid: i32, disp: [f64; 3], rot: [f64; 3]) {
        self.node_id = nid;
        self.displacement = disp;
        self.rotation = rot;
    }

    pub fn node_id(&self) -> i32 {
        self.node_id
    }

    pub fn displacement(&self) -> [f64; 3] {
        self.displacement
    }

    pub fn rotation(&self) -> [f64; 3] {
        self.rotation
    }
}

impl Default for IgesAppliNodalDisplAndRot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut data = IgesAppliNodalDisplAndRot::new();
        data.init(5, [1.0, 2.0, 3.0], [0.1, 0.2, 0.3]);

        assert_eq!(data.node_id(), 5);
        assert_eq!(data.displacement(), [1.0, 2.0, 3.0]);
        assert_eq!(data.rotation(), [0.1, 0.2, 0.3]);
    }
}
