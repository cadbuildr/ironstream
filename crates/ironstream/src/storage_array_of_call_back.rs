// FILE: storage_array_of_call_back.rs
// occt: Storage_ArrayOfCallBack

/// Storage_ArrayOfCallBack: a 1-based array of Storage_CallBack handles.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// OCCT Array1 uses 1-based indexing (Lower/Upper bounds).
#[derive(Debug, Clone)]
pub struct Storage_ArrayOfCallBack {
    lower: i32,
    upper: i32,
    data: Vec<u64>,
}

impl Storage_ArrayOfCallBack {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        Self {
            lower,
            upper,
            data: vec![0; size],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn len(&self) -> i32 {
        self.upper - self.lower + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }

    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize]
    }

    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize] = value;
    }

    pub fn fill(&mut self, value: u64) {
        for elem in &mut self.data {
            *elem = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_bounds() {
        let arr = Storage_ArrayOfCallBack::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.len(), 10);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_array_at_and_set() {
        let mut arr = Storage_ArrayOfCallBack::new(1, 5);
        arr.set(2, 42);
        assert_eq!(arr.at(2), 42);
    }

    #[test]
    fn test_array_fill() {
        let mut arr = Storage_ArrayOfCallBack::new(1, 3);
        arr.fill(99);
        for i in 1..=3 {
            assert_eq!(arr.at(i), 99);
        }
    }
}
