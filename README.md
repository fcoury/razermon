# Razer Monitor

Add a Razer device battery monitoring gadget to your macOS menubar.

![Screenshot of the Menubar Gadget](https://user-images.githubusercontent.com/1371/195428733-6c96bdc0-74da-4d35-9c14-20ef28ad20a7.png)

## Current Status

This tool works for my needs, which is to monitor my Razer Viper
Ultimate Wireless battery usage and proactively warn me when it's about
to run out.

The whole idea behind the app is to avoid having my mouse battery
completely flat at the end of the day, without any way to predict when
it would happen.

I would love if someone else tested this with other devices, and that's
the reason I am providing a download for arm64 (m1) macOS, even tho the
app is by no means mature.

## Thanks

Special thanks to:

- [@semicoleon](https://users.rust-lang.org/u/semicoleon/summary) and 
[@afetisov](https://users.rust-lang.org/u/afetisov) over at the Rust 
Forum for the 
[help on the initial design process](https://users.rust-lang.org/t/improvements-and-safety-of-rust-ffi-code-to-access-razer-products-on-macos/82357)
for the [razermacos-rs](https://github.com/fcoury/razermacos-rs) library 
- @1kc for the original [librazermacos](https://github.com/1kc/librazermacos)
