```shell
cargo build
npm install
npm run build
```

```shell
cargo test
npm run format
```

```shell
npm run benchmark
```

commonjs: wasm esm modules is not yet ready https://github.com/WebAssembly/esm-integration/tree/main/proposals/esm-integration (stage 2 at time of writing)

future, optimize: https://lise-henry.github.io/articles/optimising_strings.html

TODO:

- screaming snake case
- try js_sys::JsString to avoid copying? https://rustwasm.github.io/wasm-bindgen/reference/types/str.html
  - right now, benchmarks are losing to change-case https://github.com/blakeembrey/change-case
- Unicode tests
- rust benchmarks https://crates.io/crates/criterion, https://doc.rust-lang.org/cargo/commands/cargo-bench.html
