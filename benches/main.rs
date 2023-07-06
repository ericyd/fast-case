// https://crates.io/crates/criterion
// https://doc.rust-lang.org/cargo/commands/cargo-bench.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate fast_case;
use fast_case::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("snake case", |b| {
        b.iter(|| to_snake_case(black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });

    c.bench_function("camel case", |b| {
        b.iter(|| to_camel_case(black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });

    c.bench_function("pascal case", |b| {
        b.iter(|| to_pascal_case(black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });

    c.bench_function("title case", |b| {
        b.iter(|| to_title_case(black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });

    c.bench_function("sentence case", |b| {
        b.iter(|| to_sentence_case(black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });

    c.bench_function("kebab case", |b| {
        b.iter(|| to_kebab_case(black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
