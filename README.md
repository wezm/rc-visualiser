RC Visualiser
=============

Display a visual representation of the sticks of an RC transmitter on your
screen. Intended for demonstrating things with transmitter connected via USB or
receiver.

<img src="rc-visualiser.gif" alt="Screenshot of the RC Visualiser UI">

Features
--------

* Display the state of two gimabls
* Cross-platform (*nix, macOS, Windows)

Building
--------

Dependencies:

* [Rust compiler][install-rust]
* [sdl2](https://libsdl.org/) development library (`sdl2` package on Arch Linux)
* [cairo](https://www.cairographics.org/) development library (`cairo` package on Arch Linux)

```
cargo build --release --locked
```

Running
-------

After building the binary will be at `target/release/rc-visualiser`.

When run it will look for a `config.toml` file in the current directory. There
is a sample included in this repo, adjust to your liking.

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/rc-visualiser/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/rc-visualiser/blob/master/LICENSE-MIT))

at your option.

[install-rust]: https://www.rust-lang.org/learn/get-started

