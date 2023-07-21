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

Tests are written in both Rust and JS. The Rust tests ensure the core algorithm works, and the JS tests ensure the WASM bindings work. To run the JS tests, you **must** build the entire lib first.

```shell
cargo test
# npm run build (if needed)
node --test
```

## Definition of word boundary

Word boundaries are the crux of changing cases. This lib defines word boundaries as any of the following

1. [Whitespace](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace)
2. Underscore `_` or hyphen `-`
3. An [uppercase](https://doc.rust-lang.org/std/primitive.char.html#method.is_uppercase) letter preceded by a [lowercase](https://doc.rust-lang.org/std/primitive.char.html#method.is_lowercase) letter, e.g. the word boundary in `abcDef` is between `cD`
4. An uppercase letter preceded by an uppercase letter and succeeded by a lowercase letter, e.g. the word boundary in `HTMLElement` is between `LE`

## JS Benchmarks

JS benchmarks compare this lib to [`change-case`](https://www.npmjs.com/package/change-case) which seems to be one of the most popular libs on npm for changing string casing.

```shell
npm run benchmark:js
```

## Rust Benchmarks

```shell
npm run benchmark:rs
```

Note: although the intention of this library was originally for TS/JS, the core Rust algorithm is substantially faster than most other leading case-changing crates on crates.io. Below is a comparison of several popular libraries. The notable exception is [heck](https://crates.io/crates/heck) which was either functionally equivalent in performance, or better. I did not compare functionality; there are likely cases that vary between each library. The times listed are from [criterion's command-line output](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html) which states:

> The left and right values show the lower and upper bounds of the confidence interval respectively, while the center value shows Criterion.rs' best estimate of the time taken for each iteration of the benchmarked routine.

Tests performed on

```
Darwin 22.5.0 x64
Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz × 12
```

snake_case

```
heck           time:   [832.29 ns 837.56 ns 843.26 ns]
fast_case      time:   [787.43 ns 840.54 ns 911.55 ns]
recase         time:   [9.2141 µs 9.3199 µs 9.4381 µs]
convert_case   time:   [39.593 µs 40.731 µs 42.002 µs]
change_case    time:   [59.965 µs 62.111 µs 64.238 µs]
case_switcher  time:   [868.08 µs 880.13 µs 893.50 µs]
```

camelCase

```
heck           time:   [759.27 ns 775.13 ns 800.10 ns]
fast_case      time:   [775.18 ns 787.95 ns 801.42 ns]
recase         time:   [11.876 µs 11.979 µs 12.096 µs]
convert_case   time:   [38.083 µs 38.641 µs 39.289 µs]
change_case    time:   [55.058 µs 55.354 µs 55.718 µs]
case_switcher  time:   [859.65 µs 877.71 µs 895.72 µs]
```

PascalCase

```
heck          time:   [756.82 ns 764.52 ns 774.15 ns]
fast_case     time:   [788.86 ns 810.62 ns 835.11 ns]
recase        time:   [12.278 µs 12.405 µs 12.545 µs]
convert_case  time:   [36.931 µs 37.327 µs 37.795 µs]
change_case   time:   [60.615 µs 63.079 µs 65.653 µs]
case_switcher time:   [841.91 µs 854.95 µs 869.63 µs]
```

Title Case

```
heck           time:   [837.70 ns 842.13 ns 846.74 ns]
fast_case      time:   [866.92 ns 891.07 ns 915.83 ns]
change_case    time:   [10.809 µs 10.909 µs 11.019 µs]
recase         time:   [12.342 µs 12.518 µs 12.718 µs]
convert_case   time:   [38.815 µs 39.351 µs 39.935 µs]
case_switcher  time:   [872.13 µs 893.60 µs 917.50 µs]
```

Sentence case

```
fast_case   time:   [815.95 ns 824.93 ns 833.40 ns]
recase      time:   [9.7883 µs 9.9663 µs 10.132 µs]
change_case time:   [53.678 µs 53.958 µs 54.250 µs]
```

kebab-case

```
fast_case      time:   [811.80 ns 822.13 ns 831.34 ns]
recase heck    time:   [828.29 ns 836.80 ns 846.28 ns]
recase         time:   [9.3987 µs 9.5677 µs 9.7547 µs]
convert_case   time:   [35.783 µs 36.059 µs 36.373 µs]
change_case    time:   [54.596 µs 55.238 µs 55.938 µs]
case_switcher  time:   [828.96 µs 842.41 µs 856.96 µs]
```

## Why CommonJS?

ESM modules cannot yet be used with WASM: [the proposal is in stage 2 at time of writing](https://github.com/WebAssembly/esm-integration/tree/main/proposals/esm-integration)

## TODO

- [ ] Add SCREAMING_SNAKE_CASE option
- [ ] Add tests for non-ascii unicode handling (done in snake_case, need to carry through to others)
- [ ] beat change-case in benchmark
