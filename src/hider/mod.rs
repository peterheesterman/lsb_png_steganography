
pub fn capture_most_significant_bits(carrier_byte: u8, payload_byte: u8) -> u8 {
      let carrier_msbs = carrier_byte & 0b1111_0000;
      let payload_msbs = payload_byte >> 4;

      carrier_msbs + payload_msbs
}

#[cfg(test)]
mod hider_tests {
  use super::*;
  #[test]
  fn run_capture_most_significant_bits() {
      assert_eq!(capture_most_significant_bits(0b1110_0000, 0b1001_1111), 0b1110_1001);
  }
}