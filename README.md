# 

create lib (differs to an executable .. see rust doc)
```
$ cargo new --lib hello-wasm
```
build for web (direct use)
```
$ wasm-pack build --target web
```
build for package (npm)
```
$ wasm-pack build --target bundler
```
cd into pkg and link it globally to consume it as "official" npm package without the need to actually release it to npm

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
https://stackoverflow.com/a/68769192/10323879
https://stackoverflow.com/questions/43667176/what-files-in-a-cargo-project-should-be-in-my-gitignore