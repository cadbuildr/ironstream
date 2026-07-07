// FILE: iges_data_param_reader.rs
// occt: IGESData_ParamReader

//! Parameter reader for parsing IGES entity parameters.

#[derive(Clone, Debug)]
pub struct ParamReader {
    params: Vec<String>,
    current: usize,
}

impl ParamReader {
    pub fn new(params: Vec<String>) -> Self {
        ParamReader {
            params,
            current: 0,
        }
    }

    pub fn param_count(&self) -> usize {
        self.params.len()
    }

    pub fn current_param(&self) -> usize {
        self.current
    }

    pub fn read_param(&mut self) -> Option<String> {
        if self.current < self.params.len() {
            let result = Some(self.params[self.current].clone());
            self.current += 1;
            result
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub fn at_end(&self) -> bool {
        self.current >= self.params.len()
    }
}

impl Default for ParamReader {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let params = vec!["p1".to_string(), "p2".to_string()];
        let reader = ParamReader::new(params);
        assert_eq!(reader.param_count(), 2);
    }

    #[test]
    fn test_read_param() {
        let params = vec!["p1".to_string(), "p2".to_string()];
        let mut reader = ParamReader::new(params);

        assert_eq!(reader.read_param(), Some("p1".to_string()));
        assert_eq!(reader.read_param(), Some("p2".to_string()));
        assert_eq!(reader.read_param(), None);
    }

    #[test]
    fn test_reset() {
        let params = vec!["p1".to_string(), "p2".to_string()];
        let mut reader = ParamReader::new(params);

        reader.read_param();
        reader.read_param();
        assert!(reader.at_end());

        reader.reset();
        assert_eq!(reader.current_param(), 0);
        assert!(!reader.at_end());
    }
}
