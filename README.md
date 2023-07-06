# Fast case

A case-changing library written in Rust and exposed as a npm module via WebAssembly.

## Disclaimer

This lib currently underperforms [`change-case`](https://www.npmjs.com/package/change-case) by a significant margin. It is currently more of an experiment in exposing a Rust algorithm as a Node.js library via WASM compilation. [This guide](https://rybicki.io/blog/2023/06/27/rust-crate-into-typescript-library.html) was immensely helpful. Until the algorithm is optimized to a point where it is at least equal to change-case, I will not bother publishing it.

## Setup

Install [rust](https://www.rust-lang.org/) and [node.js](https://nodejs.org/en)

Then configure git hooks and install dependencies

```shell
git config core.hooksPath .git-hooks
npm ci
```

## Build

```shell
npm run build
```

## Test

```shell
cargo test
```

## Benchmark

JS benchmarks compare this lib to [`change-case`](https://www.npmjs.com/package/change-case) which seems to be one of the most popular libs on npm for changing string casing.

```shell
npm run benchmark:js
```

Rust benchmarks do not compare to other implementations, but they can track changes over time to the core algorithm

```shell
npm run benchmark:rs
```

## Why CommonJS?

ESM modules cannot yet be used with WASM: [the proposal is in stage 2 at time of writing](https://github.com/WebAssembly/esm-integration/tree/main/proposals/esm-integration)

## TODO

- [ ] [Optimize string processing](https://lise-henry.github.io/articles/optimising_strings.html)
  - try [`js_sys::JsString`](https://rustwasm.github.io/wasm-bindgen/reference/types/str.html) to use references instead of copying
- [ ] Add SCREAMING_SNAKE_CASE option
- [ ] Add tests for non-ascii unicode handling
