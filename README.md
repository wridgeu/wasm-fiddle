# Taking a peek at WASM & Rust

Simple demo repository where I went through the [MDN article](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) on Rust to WASM compilation.

## A few steps

Quick reminder to myself regarding a few important steps.

### Creating a library.

This differs to an executable. More info [here](https://doc.rust-lang.org/reference/linkage.html), or [here](https://doc.rust-lang.org/reference/linkage.html#:~:text=%2D%2Dcrate%2Dtype%3Dcdylib%2C%20%23%5Bcrate_type%20%3D%20%22cdylib%22%5D%20%2D%20A%20dynamic%20system%20library%20will%20be%20produced.%20This%20is%20used%20when%20compiling%20a%20dynamic%20library%20to%20be%20loaded%20from%20another%20language.%20This%20output%20type%20will%20create%20*.so%20files%20on%20Linux%2C%20*.dylib%20files%20on%20macOS%2C%20and%20*.dll%20files%20on%20Windows.).

```console
cargo new --lib hello-wasm
```

### Build for Web use (direct)

```console
wasm-pack build --target web
```

### Build for NPM package use

The resulting package folder (pkg) can then be linked as globally available package via `npm link`.

```console
wasm-pack build --target bundler
```

or directly `cd` into the corresponding _consumption_ packages and execute:

```console
npm run compile:rust
```

---

## Sources

- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
- Issue when trying to `cargo install wasm-pack`
  - https://stackoverflow.com/a/68769192/10323879
  - https://rustwasm.github.io/wasm-pack/installer/
- https://stackoverflow.com/questions/43667176/what-files-in-a-cargo-project-should-be-in-my-gitignore
