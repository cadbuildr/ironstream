// FILE: interface_stat.rs
// occt: Interface_STAT

/// Statistical information
pub struct InterfaceStat {
    name: String,
    value: i32,
}

impl InterfaceStat {
    pub fn new(name: &str, value: i32) -> Self {
        InterfaceStat {
            name: name.to_string(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

impl Default for InterfaceStat {
    fn default() -> Self {
        InterfaceStat {
            name: String::new(),
            value: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let stat = InterfaceStat::new("count", 42);
        assert_eq!(stat.name(), "count");
        assert_eq!(stat.value(), 42);
    }

    #[test]
    fn test_set_value() {
        let mut stat = InterfaceStat::new("test", 0);
        stat.set_value(100);
        assert_eq!(stat.value(), 100);
    }
}
