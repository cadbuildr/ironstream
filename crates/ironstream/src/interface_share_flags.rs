// FILE: interface_share_flags.rs
// occt: Interface_ShareFlags

/// Manages share flags for entities
pub struct InterfaceShareFlags {
    flags: Vec<bool>,
}

impl InterfaceShareFlags {
    pub fn new(size: usize) -> Self {
        InterfaceShareFlags {
            flags: vec![false; size],
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index < self.flags.len() {
            self.flags[index] = value;
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index < self.flags.len() {
            self.flags[index]
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        for flag in &mut self.flags {
            *flag = false;
        }
    }
}

impl Default for InterfaceShareFlags {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let flags = InterfaceShareFlags::new(10);
        assert!(!flags.get(0));
    }

    #[test]
    fn test_set_get() {
        let mut flags = InterfaceShareFlags::new(10);
        flags.set(5, true);
        assert!(flags.get(5));
    }

    #[test]
    fn test_clear() {
        let mut flags = InterfaceShareFlags::new(10);
        flags.set(3, true);
        flags.clear();
        assert!(!flags.get(3));
    }
}
