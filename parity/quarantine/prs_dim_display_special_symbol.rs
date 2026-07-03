// FILE: prs_dim_display_special_symbol.rs
// occt: PrsDim_DisplaySpecialSymbol

/// Enumeration for PrsDim_DisplaySpecialSymbol.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_DisplaySpecialSymbol {
    PrsDim_DisplaySpecialSymbol_No = 0,
    PrsDim_DisplaySpecialSymbol_Before = 1,
    PrsDim_DisplaySpecialSymbol_After = 2,
}

impl PrsDim_DisplaySpecialSymbol {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_DisplaySpecialSymbol::PrsDim_DisplaySpecialSymbol_No),
            1 => Some(PrsDim_DisplaySpecialSymbol::PrsDim_DisplaySpecialSymbol_Before),
            2 => Some(PrsDim_DisplaySpecialSymbol::PrsDim_DisplaySpecialSymbol_After),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_display_special_symbol_sanity() {
        let v = PrsDim_DisplaySpecialSymbol::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}