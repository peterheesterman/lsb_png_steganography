
pub fn promote_least_significant_bits(carrier_byte: u8) -> u8 {
    carrier_byte << 4
}

#[cfg(test)]
mod revealer_tests {
    use super::*;
    #[test]
    fn run_promote_least_significant_bits() {
        assert_eq!(promote_least_significant_bits(0b1110_0100), 0b0100_0000);
    }
}
