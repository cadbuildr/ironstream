// FILE: top_ope_b_rep_build_face_builder.rs
// occt: TopOpeBRepBuild_FaceBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildFaceBuilder {
    wires: Vec<i32>,
    current_index: usize,
}

impl TopOpeBRepBuildFaceBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildFaceBuilder {
            wires: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_wire(&mut self, wire_id: i32) {
        self.wires.push(wire_id);
    }

    pub fn init_wires(&mut self) {
        self.current_index = 0;
    }

    pub fn more_wires(&self) -> bool {
        self.current_index < self.wires.len()
    }

    pub fn next_wire(&mut self) {
        self.current_index += 1;
    }

    pub fn wire(&self) -> Option<i32> {
        if self.current_index < self.wires.len() {
            Some(self.wires[self.current_index])
        } else {
            None
        }
    }

    pub fn num_wires(&self) -> usize {
        self.wires.len()
    }
}

impl Default for TopOpeBRepBuildFaceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_builder_new() {
        let fb = TopOpeBRepBuildFaceBuilder::new();
        assert_eq!(fb.num_wires(), 0);
    }

    #[test]
    fn test_face_builder_add_wire() {
        let mut fb = TopOpeBRepBuildFaceBuilder::new();
        fb.add_wire(1);
        fb.add_wire(2);
        assert_eq!(fb.num_wires(), 2);
    }

    #[test]
    fn test_face_builder_iteration() {
        let mut fb = TopOpeBRepBuildFaceBuilder::new();
        fb.add_wire(10);
        fb.add_wire(20);

        fb.init_wires();
        assert!(fb.more_wires());
        assert_eq!(fb.wire(), Some(10));

        fb.next_wire();
        assert!(fb.more_wires());
        assert_eq!(fb.wire(), Some(20));

        fb.next_wire();
        assert!(!fb.more_wires());
    }
}
