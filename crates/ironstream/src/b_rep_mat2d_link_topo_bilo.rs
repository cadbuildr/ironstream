// FILE: b_rep_mat2d_link_topo_bilo.rs
// occt: BRepMAT2d_LinkTopoBilo

/// Constructs links between wire/face of explorer and BasicElts in bisecting locus.
#[derive(Debug, Clone)]
pub struct BRepMat2dLinkTopoBilo {
    current: i32,
    is_empty: bool,
}

impl BRepMat2dLinkTopoBilo {
    pub fn new() -> Self {
        BRepMat2dLinkTopoBilo {
            current: 0,
            is_empty: true,
        }
    }

    pub fn init(&mut self) {
        self.current = 0;
    }

    pub fn more(&self) -> bool {
        !self.is_empty
    }

    pub fn next(&mut self) {
        self.current += 1;
    }
}

impl Default for BRepMat2dLinkTopoBilo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_topo_bilo() {
        let link = BRepMat2dLinkTopoBilo::new();
        assert!(!link.more());
    }
}
