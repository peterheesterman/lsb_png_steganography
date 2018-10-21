
pub fn capture_most_significant_bits(carrier_byte: u8, payload_byte: u8) -> u8 {
    let carrier_msbs = carrier_byte & 0b1111_0000;
    let payload_msbs = payload_byte >> 4;

    carrier_msbs + payload_msbs
}

pub fn get_limits_for_image(payload_dimensions: (u32, u32), carrier_dimensions: (u32, u32)) -> (u32, u32) {
    let (payload_x_limit, payload_y_limit) = payload_dimensions;
    let (carrier_x_limit, carrier_y_limit) = carrier_dimensions;
    
    let x_limit = if payload_x_limit < carrier_x_limit {
        payload_x_limit
    } else {
        carrier_x_limit
    };

    let y_limit = if payload_y_limit < carrier_y_limit {
        payload_y_limit
    } else {
        carrier_y_limit
    };

    (x_limit, y_limit)
}

#[cfg(test)]
mod hider_tests {
    use super::*;
    #[test]
    fn run_capture_most_significant_bits() {
        assert_eq!(capture_most_significant_bits(0b1110_0000, 0b1001_1111), 0b1110_1001);
    }

    #[test]
    fn run_get_limits_for_image() {
        assert_eq!(get_limits_for_image((12, 12), (18, 6)), (12, 6));
    }
}
