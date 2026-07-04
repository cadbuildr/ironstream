// FILE: iges_data_param_cursor.rs
// occt: IGESData_ParamCursor

//! Parameter cursor for navigating IGES entity parameters.

#[derive(Clone, Copy, Debug)]
pub struct ParamCursor {
    position: usize,
    count: usize,
}

impl ParamCursor {
    pub fn new(count: usize) -> Self {
        ParamCursor {
            position: 0,
            count,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn advance(&mut self) {
        if self.position < self.count {
            self.position += 1;
        }
    }

    pub fn at_end(&self) -> bool {
        self.position >= self.count
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}

impl Default for ParamCursor {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cursor = ParamCursor::new(5);
        assert_eq!(cursor.position(), 0);
        assert_eq!(cursor.count(), 5);
    }

    #[test]
    fn test_advance() {
        let mut cursor = ParamCursor::new(5);
        assert_eq!(cursor.position(), 0);
        cursor.advance();
        assert_eq!(cursor.position(), 1);
        cursor.advance();
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn test_at_end() {
        let mut cursor = ParamCursor::new(2);
        assert!(!cursor.at_end());
        cursor.advance();
        assert!(!cursor.at_end());
        cursor.advance();
        assert!(cursor.at_end());
    }

    #[test]
    fn test_reset() {
        let mut cursor = ParamCursor::new(5);
        cursor.advance();
        cursor.advance();
        assert_eq!(cursor.position(), 2);
        cursor.reset();
        assert_eq!(cursor.position(), 0);
    }
}
