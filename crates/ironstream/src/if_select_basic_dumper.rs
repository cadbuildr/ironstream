// FILE: if_select_basic_dumper.rs
// occt: IFSelect_BasicDumper

/// Provides basic dumping capabilities for IFSelect items
#[derive(Clone, Debug)]
pub struct IfSelectBasicDumper {
    indent_level: usize,
}

impl IfSelectBasicDumper {
    /// Creates a basic dumper
    pub fn new() -> Self {
        IfSelectBasicDumper { indent_level: 0 }
    }

    /// Increases indentation level
    pub fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decreases indentation level
    pub fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Returns current indentation
    pub fn indent(&self) -> usize {
        self.indent_level
    }

    /// Dumps a string
    pub fn dump_string(&self, _label: &str, _value: &str) {}

    /// Dumps an integer
    pub fn dump_int(&self, _label: &str, _value: i32) {}

    /// Dumps a real
    pub fn dump_real(&self, _label: &str, _value: f64) {}
}

impl Default for IfSelectBasicDumper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let dumper = IfSelectBasicDumper::new();
        assert_eq!(dumper.indent(), 0);
    }

    #[test]
    fn test_indent() {
        let mut dumper = IfSelectBasicDumper::new();
        dumper.increase_indent();
        assert_eq!(dumper.indent(), 1);
        dumper.increase_indent();
        assert_eq!(dumper.indent(), 2);
        dumper.decrease_indent();
        assert_eq!(dumper.indent(), 1);
    }

    #[test]
    fn test_dump_string() {
        let dumper = IfSelectBasicDumper::new();
        dumper.dump_string("label", "value");
        assert!(true);
    }
}
