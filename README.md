
# lsb_png_steganography

(Least significant bit portable network graphic steganography)

This repo is a module for the commandline tool [`steg`](https://github.com/peterheesterman/steg) but can also be used independently

It takes paths to two pngs and hides the first in the second one, it can then reveal the hidden image.


### Usage

Add the following to the Cargo.toml in your project:

```toml
[dependencies]
lsb_png_steganography = "*"
```

and import using ```extern crate```:

```rust
extern crate lsb_png_steganography;

use lsb_png_steganography::{ hide, reveal };

fn run () {
    let payload_path = "./images/payload.png";
    let carrier_path = "./images/carrier.png";
    let output_carrier_path = "./output_carrier.png";
    let output_payload_path = "./output_payload_path.png";

    let img = hide(payload_path, carrier_path);
    img.save(output_carrier_path).unwrap();
    
    let img = reveal(output_carrier_path);
    img.save(output_payload_path).unwrap();
}
```

## Documentation

Read it. . .(coming soon)


