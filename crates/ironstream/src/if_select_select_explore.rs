// FILE: if_select_select_explore.rs
// occt: IFSelect_SelectExplore

#[derive(Clone, Debug)]
pub struct IfSelectSelectExplore {
    depth: usize,
}

impl IfSelectSelectExplore {
    pub fn new(depth: usize) -> Self {
        IfSelectSelectExplore { depth }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn set_depth(&mut self, d: usize) {
        self.depth = d;
    }
}

impl Default for IfSelectSelectExplore {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let se = IfSelectSelectExplore::new(3);
        assert_eq!(se.depth(), 3);
    }

    #[test]
    fn test_set_depth() {
        let mut se = IfSelectSelectExplore::new(3);
        se.set_depth(5);
        assert_eq!(se.depth(), 5);
    }
}
