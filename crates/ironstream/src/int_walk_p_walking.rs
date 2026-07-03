// FILE: int_walk_p_walking.rs
// occt: IntWalk_PWalking

use std::sync::Arc;
use crate::int_walk_the_int2_s::TheInt2S;

/// Pointer to walking computation result
pub type PWalking = Arc<TheInt2S>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwalking_type() {
        let _: fn(PWalking) = |_x| {};
    }
}
