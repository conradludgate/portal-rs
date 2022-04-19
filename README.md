# portal-rs

A quick tech demo to make a Portal-esque game using Bevy in Rust.

Inspiration:
* <https://www.youtube.com/watch?v=cWpFZbjtSQg> - A portal demo made in unity.
    Makes use of render-camera-to-texture + frustrum culling in order to capture the
    view through a portal with the correct perspective
* <https://bevyengine.org/news/bevy-0-7/#render-to-texture> - New in Bevy 0.7: Render to Texture support

## Build

(Currently tested on Linux only with `rustc 1.62.0-nightly (311e2683e 2022-04-18)`)

Clone the repo:

```
git clone https://github.com/conradludgate/portal-rs
cd portal-rs
```

Using [rustup](https://rustup.rs/), install the latest nightly toolchain:

```sh
rustup toolchain install nightly # install the nightly toolchain
rustup override set nightly      # set it as default for the current project
```

Now, build:

```sh
# this can take a while the first time...
cargo build --release
```

You will find the executable in `target/release/portal`.

If you want to run it straight away, use:

```sh
cargo run --release
```
