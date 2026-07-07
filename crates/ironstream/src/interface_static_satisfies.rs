// FILE: interface_static_satisfies.rs
// occt: Interface_StaticSatisfies

/// Determines if a static value satisfies a condition
pub trait InterfaceStaticSatisfies {
    fn satisfies(&self, value: &str) -> bool;
}

pub struct DefaultStaticSatisfies;

impl InterfaceStaticSatisfies for DefaultStaticSatisfies {
    fn satisfies(&self, _value: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfies() {
        let satisfier = DefaultStaticSatisfies;
        assert!(satisfier.satisfies("any"));
    }
}
