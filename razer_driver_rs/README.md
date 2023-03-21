# Razer Driver for Rust

A native rust library to communicate with Razer USB devices directly without any of the Razer software.

## Work in Progress

This project is missing many many features. However the framework is there to easily add anything missing.

Contributions are very welcome.

## Example

```rust
use razer_driver_rs::*;

let devices = scan_for_devices()?;
let keyboard = devices.keyboards.get(0).unwrap();
let brightness = keyboard.get_brightness()?;
println!("brightness {}", brightness);
keyboard.set_brightness(90)?;
```

## Credits

This library is very heavily inspired by:

- [OpenRazer](https://github.com/openrazer/openrazer)
- [OpenRGB](https://gitlab.com/CalcProgrammer1/OpenRGB)
