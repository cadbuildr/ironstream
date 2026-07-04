// FILE: if_select_graph_counter.rs
// occt: IFSelect_GraphCounter

#[derive(Clone, Debug)]
pub struct IfSelectGraphCounter {
    count: usize,
}

impl IfSelectGraphCounter {
    pub fn new() -> Self {
        IfSelectGraphCounter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

impl Default for IfSelectGraphCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let gc = IfSelectGraphCounter::new();
        assert_eq!(gc.count(), 0);
    }

    #[test]
    fn test_increment() {
        let mut gc = IfSelectGraphCounter::new();
        gc.increment();
        assert_eq!(gc.count(), 1);
    }
}
