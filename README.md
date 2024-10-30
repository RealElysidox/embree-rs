# embree3-arm

[![Crates.io](https://img.shields.io/crates/v/embree.svg)](https://crates.io/crates/embree)
![Build Status](https://github.com/Twinklebear/embree-rs/workflows/CI/badge.svg)

Rust bindings to [Embree](http://embree.github.io/). Forked from [embree-rs](https://github.com/Twinklebear/embree-rs), and relinked by dynamical lib generated from [Embree](http://embree.github.io/) on MacOS with arm architecture. Only apply for using the `embree-rs` in v0.3.8 edition on MacOS arm.  

# Usage

When trying to `extern crate embree3_arm as embree`, follow these steps:

1. add this `build.rs` in the workspace of the rust program:

```rust
use std::env;
use std::path::PathBuf;

fn main() {
    println!("{:?}", env::var("EMBREE_DIR"));
    if let Ok(e) = env::var("EMBREE_DIR") {
        let mut embree_dir = PathBuf::from(e);
        embree_dir.push("lib");
        println!("cargo:rustc-link-search=native={}", embree_dir.display());
        println!("cargo:rerun-if-env-changed=EMBREE_DIR");
    }
    println!("cargo:rustc-link-lib=embree3");
}
```

2. Set the workspace to the following env variable:

```bash
export EMBREE_DIR=`pwd`
```

3. Download compressed [libembree](embree-3.13.4.arm.macosx.zip) from [github release page](https://github.com/RealElysidox/embree-rs/releases/tag/lib), and move the `lib` folder to the root dir of rust program.

![](images/image1.png)

4. `cargo build && cargo run`, then copy `libembree3.3.13.4.dylib`, `libembree3.3.dylib` and `libembree3.dylib` to `./target/debug` or `./target/release`

5. try `cargo run` again.