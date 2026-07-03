// FILE: extrema_ext_el_c.rs
// occt: Extrema_ExtElC

pub struct ExtremaExtElC {
    done: bool,
    nb_extrema: i32,
}

impl ExtremaExtElC {
    pub fn new() -> Self { ExtremaExtElC { done: false, nb_extrema: 0 } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_extrema(&self) -> i32 { self.nb_extrema }
}
impl Default for ExtremaExtElC {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert_eq!(ExtremaExtElC::new().nb_extrema(), 0); }
}
