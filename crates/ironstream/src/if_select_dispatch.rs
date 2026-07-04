// FILE: if_select_dispatch.rs
// occt: IFSelect_Dispatch

/// Base class for dispatchers
#[derive(Clone, Debug)]
pub struct IfSelectDispatch {
    parts: Vec<Vec<usize>>,
}

impl IfSelectDispatch {
    /// Creates a dispatcher
    pub fn new() -> Self {
        IfSelectDispatch { parts: vec![] }
    }

    /// Returns the number of parts
    pub fn nb_parts(&self) -> usize {
        self.parts.len()
    }

    /// Adds a part
    pub fn add_part(&mut self) {
        self.parts.push(vec![]);
    }

    /// Gets entities from a part
    pub fn part(&self, num: usize) -> Option<&Vec<usize>> {
        if num > 0 && num <= self.parts.len() {
            Some(&self.parts[num - 1])
        } else {
            None
        }
    }
}

impl Default for IfSelectDispatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let disp = IfSelectDispatch::new();
        assert_eq!(disp.nb_parts(), 0);
    }

    #[test]
    fn test_add_part() {
        let mut disp = IfSelectDispatch::new();
        disp.add_part();
        assert_eq!(disp.nb_parts(), 1);
    }
}
