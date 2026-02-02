// Utility functions and helpers

pub mod bitops {
    /// Get bit at position
    pub fn get_bit(value: u8, bit: u8) -> bool {
        (value & (1 << bit)) != 0
    }
    
    /// Set bit at position
    pub fn set_bit(value: u8, bit: u8, set: bool) -> u8 {
        if set {
            value | (1 << bit)
        } else {
            value & !(1 << bit)
        }
    }
}

pub mod cpu {
    /// Check if adding two values would cause a carry
    pub fn would_carry(a: u8, b: u8) -> bool {
        (a as u16 + b as u16) > 0xFF
    }
    
    /// Check if subtracting would cause a borrow
    pub fn would_borrow(a: u8, b: u8) -> bool {
        a < b
    }
}
