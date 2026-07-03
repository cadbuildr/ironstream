// FILE: hlrb_rep_poly_hlr_to_shape.rs
// occt: HLRBRep_PolyHLRToShape

pub struct HlrbrépPolyHLRToShape {
    visible_edges: Vec<([f64; 3], [f64; 3])>,
    hidden_edges: Vec<([f64; 3], [f64; 3])>,
}

impl HlrbrépPolyHLRToShape {
    pub fn new() -> Self {
        HlrbrépPolyHLRToShape {
            visible_edges: Vec::new(),
            hidden_edges: Vec::new(),
        }
    }

    pub fn add_visible_edge(&mut self, start: [f64; 3], end: [f64; 3]) {
        self.visible_edges.push((start, end));
    }

    pub fn add_hidden_edge(&mut self, start: [f64; 3], end: [f64; 3]) {
        self.hidden_edges.push((start, end));
    }

    pub fn nb_visible(&self) -> usize { self.visible_edges.len() }
    pub fn nb_hidden(&self) -> usize { self.hidden_edges.len() }
}

impl Default for HlrbrépPolyHLRToShape {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let p = HlrbrépPolyHLRToShape::new();
        assert_eq!(p.nb_visible(), 0);
    }
}
