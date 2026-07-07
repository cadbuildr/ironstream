// FILE: moni_tool_value_interpret.rs
// occt: MoniTool_ValueInterpret

/// Interprets values in monitoring tools
pub trait MoniToolValueInterpret {
    fn interpret(&self, value: &str) -> bool;
}

pub struct DefaultValueInterpret;

impl MoniToolValueInterpret for DefaultValueInterpret {
    fn interpret(&self, _value: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        let interp = DefaultValueInterpret;
        assert!(interp.interpret("any"));
    }
}
