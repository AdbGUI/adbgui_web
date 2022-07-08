# ADBGUI Web

This project is to manage and visualize all logcats for all device using the [adbgui desktop application](https://github.com/AdbGUI/adbgui_desktop)

## Usage

To use this web page, you only need install [server](https://github.com/AdbGUI/adbgui_server), this automatic download and use this web page

## Development

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- make

### Setup

If you don't already have it installed, it's time to install [Rust](https://www.rust-lang.org/tools/install).
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```sh
make clean install setup
```

> If you dont have make, can try:
```
rustup target add wasm32-unknown-unknown &&
cargo install trunk wasm-bindgen-cli &&
git config --local core.hooksPath
```

That's it, we're done!

### Running

Even if you don't have the server running you can test certain functions of the site by following the instructions below
Only run this command, and this open the browser where you have the "hot reloading":
```
make run
```

If you dont have make, can run:
```
trunk serve
```

### Release

```
make clean build
```

If you dont have make, try:

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.
