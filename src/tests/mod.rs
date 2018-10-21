
use super::*;

#[test]
fn hide_large_payload() {
    let payload_path = "./src/tests/images/arrow.png";
    let carrier_path = "./src/tests/images/search.png";

    let img = hide(payload_path, carrier_path);
    assert_eq!(img.dimensions(), (128, 128));
    //let output_path = "./output_path.png";
    //img.save(output_path).unwrap();
}

#[test]
fn hide_in_large_carrier() {
    let payload_path = "./src/tests/images/search.png";
    let carrier_path = "./src/tests/images/arrow.png";

    let img = hide(payload_path, carrier_path);
    assert_eq!(img.dimensions(), (340, 148));
}


#[test]
fn hide_same_size() {
    let payload_path = "./src/tests/images/dot.png";
    let carrier_path = "./src/tests/images/search.png";

    let img = hide(payload_path, carrier_path);
    assert_eq!(img.dimensions(), (128, 128));
}

#[test]
fn reveal_small() {
    let carrier_path = "./src/tests/images/hidden_small.png";
    
    let img = reveal(carrier_path);
    assert_eq!(img.dimensions(), (128, 128));
}

#[test]
fn reveal_large() {
    let carrier_path = "./src/tests/images/hidden_large.png";
    let img = reveal(carrier_path);
    assert_eq!(img.dimensions(), (340, 148));
}
