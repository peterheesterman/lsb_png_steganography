
use super::*;

//cargo test -- --nocapture
#[test]
fn run_hide() {
    let payload_path = "./test-images/dot.png";
    let carrier_path = "./test-images/search.png";

    hide(payload_path, carrier_path);
}

#[test]
fn run_reveal() {
    let carrier_path = "./test-images/search.png";

    reveal(carrier_path)
}
