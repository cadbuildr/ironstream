// FILE: contap_the_iw_line_of_the_i_walking.rs
// occt: Contap_TheIWLineOfTheIWalking

pub struct ContapTheIWLineOfTheIWalking;

impl ContapTheIWLineOfTheIWalking {
    pub fn new() -> Self {
        ContapTheIWLineOfTheIWalking
    }
}

impl Default for ContapTheIWLineOfTheIWalking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapTheIWLineOfTheIWalking;

    #[test]
    fn test_new() {
        let _ = ContapTheIWLineOfTheIWalking::new();
    }
}
