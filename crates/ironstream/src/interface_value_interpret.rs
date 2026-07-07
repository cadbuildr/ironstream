// FILE: interface_value_interpret.rs
// occt: Interface_ValueInterpret

/// Interprets a value
pub trait InterfaceValueInterpret {
    fn interpret(&self, value: &str) -> bool;
}

pub struct DefaultValueInterpret;

impl InterfaceValueInterpret for DefaultValueInterpret {
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
