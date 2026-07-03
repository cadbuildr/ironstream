// FILE: aspect_gen_id.rs
// occt: Aspect_GenId

/// Generator of unique identifiers.
pub struct AspectGenId {
    current: i32,
    free_ids: Vec<i32>,
}

impl AspectGenId {
    /// Create identifier generator starting from 1.
    pub fn new() -> Self {
        AspectGenId {
            current: 1,
            free_ids: Vec::new(),
        }
    }

    /// Get next unique identifier.
    pub fn next(&mut self) -> i32 {
        if let Some(id) = self.free_ids.pop() {
            id
        } else {
            let id = self.current;
            self.current += 1;
            id
        }
    }

    /// Free an identifier for reuse.
    pub fn free(&mut self, id: i32) {
        if id < self.current {
            self.free_ids.push(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gen = AspectGenId::new();
        assert_eq!(gen.current, 1);
    }

    #[test]
    fn test_next() {
        let mut gen = AspectGenId::new();
        assert_eq!(gen.next(), 1);
        assert_eq!(gen.next(), 2);
        assert_eq!(gen.next(), 3);
    }

    #[test]
    fn test_free_reuse() {
        let mut gen = AspectGenId::new();
        let id1 = gen.next();
        let id2 = gen.next();
        gen.free(id1);
        assert_eq!(gen.next(), id1);
        assert_eq!(gen.next(), 3);
    }
}
