// FILE: if_select_act_func.rs
// occt: IFSelect_ActFunc

/// IFSelect_ActFunc is a function pointer type for action functions
/// that operate on an IFSelect session pilot.
/// Signature: fn(SessionPilot) -> ReturnStatus
pub type IfSelectActFunc = fn() -> i32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_type_exists() {
        let _f: IfSelectActFunc = || 0;
        assert!(true);
    }
}
