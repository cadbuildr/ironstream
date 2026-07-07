// FILE: interface_value_satisfies.rs
// occt: Interface_ValueSatisfies

/// Determines if a value satisfies a condition
pub trait InterfaceValueSatisfies {
    fn satisfies(&self, value: &str) -> bool;
}

pub struct DefaultValueSatisfies;

impl InterfaceValueSatisfies for DefaultValueSatisfies {
    fn satisfies(&self, _value: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfies() {
        let satisfier = DefaultValueSatisfies;
        assert!(satisfier.satisfies("any"));
    }
}
