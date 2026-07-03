// FILE: top_ope_b_rep_ds_surface_explorer.rs
// occt: TopOpeBRepDS_SurfaceExplorer

/// Explorer for surfaces in a data structure
#[derive(Debug, Clone)]
pub struct SurfaceExplorer {
    /// Start index
    start_index: usize,
    /// End index
    end_index: usize,
    /// Current index
    current_index: usize,
}

impl SurfaceExplorer {
    /// Create new explorer
    pub fn new(start: usize, end: usize) -> Self {
        SurfaceExplorer {
            start_index: start,
            end_index: end,
            current_index: start,
        }
    }

    /// Check if there are more elements
    pub fn more(&self) -> bool {
        self.current_index < self.end_index
    }

    /// Move to next element
    pub fn next(&mut self) {
        if self.current_index < self.end_index {
            self.current_index += 1;
        }
    }

    /// Get current index
    pub fn current(&self) -> usize {
        self.current_index
    }

    /// Reset explorer
    pub fn reset(&mut self) {
        self.current_index = self.start_index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_explorer_new() {
        let exp = SurfaceExplorer::new(0, 5);
        assert!(exp.more());
        assert_eq!(exp.current(), 0);
    }

    #[test]
    fn test_surface_explorer_iteration() {
        let mut exp = SurfaceExplorer::new(1, 2);
        assert_eq!(exp.current(), 1);
        exp.next();
        assert!(!exp.more());
    }
}
