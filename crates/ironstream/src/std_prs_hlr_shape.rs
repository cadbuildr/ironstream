// FILE: std_prs_hlr_shape.rs
// occt: StdPrs_HLRShape

/// Hidden Line Removal shape presenter.
#[derive(Clone, Debug)]
pub struct HlrShape {
    pub shape_id: u32,
    pub edge_count: usize,
    pub visible_edge_count: usize,
    pub hidden_edge_count: usize,
}

impl HlrShape {
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            edge_count: 0,
            visible_edge_count: 0,
            hidden_edge_count: 0,
        }
    }

    pub fn add_edge(&mut self, is_visible: bool) {
        self.edge_count += 1;
        if is_visible {
            self.visible_edge_count += 1;
        } else {
            self.hidden_edge_count += 1;
        }
    }

    pub fn nb_edges(&self) -> usize {
        self.edge_count
    }

    pub fn nb_visible_edges(&self) -> usize {
        self.visible_edge_count
    }

    pub fn nb_hidden_edges(&self) -> usize {
        self.hidden_edge_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_shape() {
        let shape = HlrShape::new(1);
        assert_eq!(shape.shape_id, 1);
        assert_eq!(shape.nb_edges(), 0);
    }

    #[test]
    fn add_edges() {
        let mut shape = HlrShape::new(1);
        shape.add_edge(true);
        shape.add_edge(true);
        shape.add_edge(false);
        shape.add_edge(false);
        assert_eq!(shape.nb_edges(), 4);
        assert_eq!(shape.nb_visible_edges(), 2);
        assert_eq!(shape.nb_hidden_edges(), 2);
    }
}
