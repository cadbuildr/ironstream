// FILE: int_walk_the_int2_s.rs
// occt: IntWalk_TheInt2S

/// 2-surface intersection computation class
#[derive(Clone, Debug)]
pub struct TheInt2S {
    is_done: bool,
}

impl TheInt2S {
    /// Create new computation
    pub fn new() -> Self {
        TheInt2S { is_done: false }
    }

    /// Check if computation is done
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Set done flag
    pub fn set_done(&mut self, done: bool) {
        self.is_done = done;
    }
}

impl Default for TheInt2S {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inter = TheInt2S::new();
        assert!(!inter.is_done());
    }

    #[test]
    fn test_done() {
        let mut inter = TheInt2S::new();
        inter.set_done(true);
        assert!(inter.is_done());
    }
}
