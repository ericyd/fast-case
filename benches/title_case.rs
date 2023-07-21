// https://crates.io/crates/criterion
// https://doc.rust-lang.org/cargo/commands/cargo-bench.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate fast_case;
use convert_case::{Case, Casing};
use fast_case::*;
use heck::{ToTitleCase};

fn criterion_benchmark(c: &mut Criterion) {
    let test = "aa_bb CC_DD eeFf GgHh Ii Jj";

    c.bench_function("fast_case", |b| {
        b.iter(|| to_title_case(black_box(test)))
    });

    c.bench_function("convert_case", |b| {
        b.iter(|| black_box(test).to_case(Case::Title))
    });

    c.bench_function("change_case", |b| {
        b.iter(|| change_case::title_case(black_box(test)))
    });

    c.bench_function("case_switcher", |b| {
        b.iter(|| case_switcher::to_title(black_box(test)))
    });

    c.bench_function("recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).title_case())
    });

    c.bench_function("heck", |b| {
        b.iter(|| black_box(test).to_title_case())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
