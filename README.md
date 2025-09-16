Wrapper for [chafa](https://hpjansson.org/chafa/development/) bindings. Part of [term3dv](https://github.com/uiriansan/term3dv).

The bindings are generated at build time using [bindgen](https://docs.rs/bindgen/latest/bindgen/).

# System dependencies:
- [Chafa/libchafa](https://hpjansson.org/chafa/);
- [glib2](https://docs.gtk.org/glib/);
- [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/);
- [Clang](https://rust-lang.github.io/rust-bindgen/requirements.html)

# Build:
```bash
cargo build
# ...or in release mode:
    cargo build --release
```

# Usage:
An example can be found in `examples/adaptive.rs`.
