// https://crates.io/crates/criterion
// https://doc.rust-lang.org/cargo/commands/cargo-bench.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate fast_case;
use fast_case::*;

// no equivalent function in convert_case
// no equivalent function in case_switcher
// no equivalent function in heck
fn criterion_benchmark(c: &mut Criterion) {
    let test = "aa_bb CC_DD eeFf GgHh Ii Jj";

    c.bench_function("fast_case", |b| {
        b.iter(|| to_sentence_case(black_box(test)))
    });

    c.bench_function("change_case", |b| {
        b.iter(|| change_case::sentence_case(black_box(test)))
    });

    c.bench_function("recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).sentence_case())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
