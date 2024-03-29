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

JS benchmarks compare this lib to [`change-case`](https://www.npmjs.com/package/change-case) which seems to be one of the most popular libs on npm for changing string casing. Sadly, it is apparent that the overhead of copying the string to WASM is not worth the performance gains of the Rust algorithm, at least for relatively small strings. Even for moderately long strings, the performance of `fast-case` is almost always worse than `change-case`. For short strings, the difference is even more substantial.

```shell
npm run benchmark:js
```

```
==========================
  Long string comparison
==========================

Platform info:
==============
   Darwin 22.5.0 x64
   Node.JS: 18.12.0
   V8: 10.2.154.15-node.12
   Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz × 12

Suite: snake_case
✔ fastCase             295,615 rps
✔ changeCase           315,662 rps

   fastCase          -6.35%         (295,615rps)   (avg: 3μs)
   changeCase            0%         (315,662rps)   (avg: 3μs)
------------------------------------------------------------------------

Suite: camelCase
✔ fastCase             306,827 rps
✔ changeCase           315,142 rps

   fastCase          -2.64%         (306,827rps)   (avg: 3μs)
   changeCase            0%         (315,142rps)   (avg: 3μs)
------------------------------------------------------------------------

Suite: Sentence case
✔ fastCase             310,235 rps
✔ changeCase           373,011 rps

   fastCase         -16.83%         (310,235rps)   (avg: 3μs)
   changeCase            0%         (373,011rps)   (avg: 2μs)
------------------------------------------------------------------------

Suite: Title Case
✔ fastCase             308,133 rps
✔ changeCase           254,785 rps

   fastCase              0%         (308,133rps)   (avg: 3μs)
   changeCase       -17.31%         (254,785rps)   (avg: 3μs)
------------------------------------------------------------------------

Suite: PascalCase
✔ fastCase             310,261 rps
✔ changeCase           319,515 rps

   fastCase           -2.9%         (310,261rps)   (avg: 3μs)
   changeCase            0%         (319,515rps)   (avg: 3μs)
------------------------------------------------------------------------

Suite: kebab-case
✔ fastCase             313,911 rps
✔ changeCase           388,195 rps

   fastCase         -19.14%         (313,911rps)   (avg: 3μs)
   changeCase            0%         (388,195rps)   (avg: 2μs)
------------------------------------------------------------------------

Suite: SCREAMING_SNAKE_CASE
✔ fastCase             316,369 rps
✔ changeCase           359,744 rps

   fastCase         -12.06%         (316,369rps)   (avg: 3μs)
   changeCase            0%         (359,744rps)   (avg: 2μs)
------------------------------------------------------------------------

===========================
  Short string comparison
===========================

Platform info:
==============
   Darwin 22.5.0 x64
   Node.JS: 18.12.0
   V8: 10.2.154.15-node.12
   Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz × 12

Suite: snake_case
✔ fastCase             442,144 rps
✔ changeCase           652,746 rps

   fastCase         -32.26%         (442,144rps)   (avg: 2μs)
   changeCase            0%         (652,746rps)   (avg: 1μs)
------------------------------------------------------------------------

Suite: camelCase
✔ fastCase             476,177 rps
✔ changeCase           740,295 rps

   fastCase         -35.68%         (476,177rps)   (avg: 2μs)
   changeCase            0%         (740,295rps)   (avg: 1μs)
------------------------------------------------------------------------

Suite: Sentence case
✔ fastCase             457,466 rps
✔ changeCase           658,887 rps

   fastCase         -30.57%         (457,466rps)   (avg: 2μs)
   changeCase            0%         (658,887rps)   (avg: 1μs)
------------------------------------------------------------------------

Suite: Title Case
✔ fastCase             445,018 rps
✔ changeCase           621,155 rps

   fastCase         -28.36%         (445,018rps)   (avg: 2μs)
   changeCase            0%         (621,155rps)   (avg: 1μs)
------------------------------------------------------------------------

Suite: PascalCase
✔ fastCase             439,313 rps
✔ changeCase           588,201 rps

   fastCase         -25.31%         (439,313rps)   (avg: 2μs)
   changeCase            0%         (588,201rps)   (avg: 1μs)
------------------------------------------------------------------------

Suite: kebab-case
✔ fastCase             458,029 rps
✔ changeCase           779,859 rps

   fastCase         -41.27%         (458,029rps)   (avg: 2μs)
   changeCase            0%         (779,859rps)   (avg: 1μs)
------------------------------------------------------------------------

Suite: SCREAMING_SNAKE_CASE
✔ fastCase             472,326 rps
✔ changeCase           783,018 rps

   fastCase         -39.68%         (472,326rps)   (avg: 2μs)
   changeCase            0%         (783,018rps)   (avg: 1μs)
------------------------------------------------------------------------
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

`camelCase`

```
heck                    time:   [773.01 ns 781.47 ns 791.56 ns]
fast_case               time:   [784.32 ns 823.50 ns 884.32 ns]
recase                  time:   [12.056 µs 12.283 µs 12.519 µs]
convert_case            time:   [47.795 µs 51.206 µs 54.798 µs]
change_case             time:   [56.198 µs 56.602 µs 57.031 µs]
case_switcher           time:   [902.02 µs 923.14 µs 946.39 µs]
```

`kebab-case`

```
fast_case               time:   [769.56 ns 787.96 ns 808.37 ns]
heck                    time:   [878.14 ns 895.94 ns 914.69 ns]
recase                  time:   [9.1931 µs 9.3237 µs 9.4633 µs]
convert_case            time:   [36.573 µs 36.998 µs 37.589 µs]
change_case             time:   [55.955 µs 56.418 µs 56.967 µs]
case_switcher           time:   [846.57 µs 859.85 µs 876.16 µs]
```

`PascalCase`

```
fast_case               time:   [741.56 ns 753.87 ns 767.54 ns]
heck                    time:   [755.94 ns 774.52 ns 794.09 ns]
recase                  time:   [11.898 µs 11.918 µs 11.940 µs]
convert_case            time:   [40.108 µs 41.093 µs 42.120 µs]
change_case             time:   [57.911 µs 59.295 µs 60.913 µs]
case_switcher           time:   [903.85 µs 923.28 µs 943.71 µs]
```

`SCREAMING_SNAKE_CASE`

_no equivalent function in case_switcher_

```
fast_case               time:   [773.34 ns 788.22 ns 804.36 ns]
heck                    time:   [823.64 ns 835.03 ns 848.53 ns]
recase                  time:   [9.8614 µs 9.9668 µs 10.106 µs]
convert_case            time:   [38.043 µs 38.899 µs 39.817 µs]
change_case             time:   [60.540 µs 62.494 µs 64.515 µs]
```

`Sentence case`

_no equivalent function in convert_case, case_switcher, or heck_

```
fast_case               time:   [813.26 ns 830.14 ns 847.61 ns]
recase                  time:   [10.508 µs 10.907 µs 11.402 µs]
change_case             time:   [56.309 µs 57.212 µs 58.312 µs]
```

`snake_case`

```
fast_case               time:   [812.83 ns 828.26 ns 845.00 ns]
heck                    time:   [837.82 ns 851.40 ns 867.38 ns]
recase                  time:   [8.9598 µs 9.0850 µs 9.2377 µs]
convert_case            time:   [38.483 µs 39.160 µs 39.819 µs]
change_case             time:   [56.825 µs 57.843 µs 59.071 µs]
case_switcher           time:   [859.34 µs 882.49 µs 906.87 µs]
```

`Title Case`

```
fast_case               time:   [774.48 ns 781.43 ns 789.39 ns]
heck                    time:   [894.64 ns 910.24 ns 927.27 ns]
change_case             time:   [11.228 µs 11.471 µs 11.734 µs]
recase                  time:   [11.735 µs 11.812 µs 11.894 µs]
convert_case            time:   [36.617 µs 36.804 µs 37.042 µs]
case_switcher           time:   [911.66 µs 935.57 µs 961.28 µs]
```

## Why CommonJS?

ESM modules cannot yet be used with WASM: [the proposal is in stage 2 at time of writing](https://github.com/WebAssembly/esm-integration/tree/main/proposals/esm-integration)

## TODO

- [ ] beat change-case in benchmark
