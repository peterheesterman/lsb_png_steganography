
use super::hide;

#[test]
fn run_hide() {
    let payload_path = "./test-images/dot.png";
    let carrier_path = "./test-images/search.png";

    hide(payload_path, carrier_path);
}
