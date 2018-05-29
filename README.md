# rust-hello-world

>Example app for the Epsilon operating system, written in Rust

## Getting started

```bash
git clone https://github.com/chocol4te/rust-hello-world.git apps/rust-hello-word
```

* Add `RUST_LIB_DIR = apps/rust-hello-world/target/thumbv7em-none-eabihf/release` to `./Makefile`

* Add `-L$(RUST_LIB_DIR) -l:librust_hello_world.a` to LD build rules (line 57 of `./Makefile`)

* Add `rust-hello-world` to `EPSILON_APPS ?=` in `./build/config.mak`

* `make` and then `make epsilon_flash` to build and upload to calculator
