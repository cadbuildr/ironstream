// FILE: moni_tool_stat.rs
// occt: MoniTool_Stat

/// Statistical data
pub struct MoniToolStat {
    name: String,
    count: i32,
    sum: f64,
}

impl MoniToolStat {
    pub fn new(name: &str) -> Self {
        MoniToolStat {
            name: name.to_string(),
            count: 0,
            sum: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_value(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn average(&self) -> f64 {
        if self.count > 0 {
            self.sum / self.count as f64
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.sum = 0.0;
    }
}

impl Default for MoniToolStat {
    fn default() -> Self {
        MoniToolStat {
            name: String::new(),
            count: 0,
            sum: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let stat = MoniToolStat::new("test");
        assert_eq!(stat.name(), "test");
        assert_eq!(stat.count(), 0);
    }

    #[test]
    fn test_add_value() {
        let mut stat = MoniToolStat::new("test");
        stat.add_value(10.0);
        stat.add_value(20.0);
        assert_eq!(stat.count(), 2);
        assert!((stat.average() - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_reset() {
        let mut stat = MoniToolStat::new("test");
        stat.add_value(5.0);
        stat.reset();
        assert_eq!(stat.count(), 0);
    }
}
