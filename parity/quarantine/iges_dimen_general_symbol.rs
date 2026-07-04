// FILE: iges_dimen_general_symbol.rs
// occt: IGESDimen_GeneralSymbol

/// Defines GeneralSymbol, Type <228> Form <0-1>
/// in package IGESDimen
pub struct IgesDimen_GeneralSymbol {
    symbol: i32,
    placement: (f64, f64),
    scale: f64,
}

impl IgesDimen_GeneralSymbol {
    pub fn new() -> Self {
        IgesDimen_GeneralSymbol {
            symbol: 0,
            placement: (0.0, 0.0),
            scale: 1.0,
        }
    }

    pub fn init(&mut self, symbol: i32, placement: (f64, f64), scale: f64) {
        self.symbol = symbol;
        self.placement = placement;
        self.scale = scale;
    }

    pub fn symbol(&self) -> i32 {
        self.symbol
    }

    pub fn placement(&self) -> (f64, f64) {
        self.placement
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl Default for IgesDimen_GeneralSymbol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_symbol_creation() {
        let sym = IgesDimen_GeneralSymbol::new();
        assert_eq!(sym.scale(), 1.0);
    }
}
