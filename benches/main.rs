// https://crates.io/crates/criterion
// https://doc.rust-lang.org/cargo/commands/cargo-bench.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate fast_case;
use convert_case::{Case, Casing};
use fast_case::*;
use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};

fn criterion_benchmark(c: &mut Criterion) {
    let test = "aa_bb CC_DD eeFf GgHh Ii Jj";

    // snake_case
    c.bench_function("snake case fast_case", |b| {
        b.iter(|| to_snake_case(black_box(test)))
    });

    c.bench_function("snake case convert_case", |b| {
        b.iter(|| black_box(test).to_case(Case::Snake))
    });

    c.bench_function("snake case change_case", |b| {
        b.iter(|| change_case::snake_case(black_box(test)))
    });

    c.bench_function("snake case case_switcher", |b| {
        b.iter(|| case_switcher::to_snake(black_box(test)))
    });

    c.bench_function("snake case recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).snake_case())
    });

    c.bench_function("snake case heck", |b| {
        b.iter(|| black_box(test).to_snake_case())
    });

    // camelCase
    c.bench_function("camel case fast_case", |b| {
        b.iter(|| to_camel_case(black_box(test)))
    });

    c.bench_function("camel case convert_case", |b| {
        b.iter(|| black_box(test).to_case(Case::Camel))
    });

    c.bench_function("camel case change_case", |b| {
        b.iter(|| change_case::camel_case(black_box(test)))
    });

    c.bench_function("camel case case_switcher", |b| {
        b.iter(|| case_switcher::to_camel(black_box(test)))
    });

    c.bench_function("camel case recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).camel_case())
    });

    c.bench_function("camel case heck", |b| {
        b.iter(|| black_box(test).to_lower_camel_case())
    });

    // PascalCase
    c.bench_function("pascal case fast_case", |b| {
        b.iter(|| to_pascal_case(black_box(test)))
    });

    c.bench_function("pascal case convert_case", |b| {
        b.iter(|| black_box(test).to_case(Case::Pascal))
    });

    c.bench_function("pascal case change_case", |b| {
        b.iter(|| change_case::pascal_case(black_box(test)))
    });

    c.bench_function("pascal case case_switcher", |b| {
        b.iter(|| case_switcher::to_pascal(black_box(test)))
    });

    c.bench_function("pascal case recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).pascal_case())
    });

    c.bench_function("pascal case heck", |b| {
        b.iter(|| black_box(test).to_upper_camel_case())
    });

    // Title Case
    c.bench_function("title case fast_case", |b| {
        b.iter(|| to_title_case(black_box(test)))
    });

    c.bench_function("title case convert_case", |b| {
        b.iter(|| black_box(test).to_case(Case::Title))
    });

    c.bench_function("title case change_case", |b| {
        b.iter(|| change_case::title_case(black_box(test)))
    });

    c.bench_function("title case case_switcher", |b| {
        b.iter(|| case_switcher::to_title(black_box(test)))
    });

    c.bench_function("title case recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).title_case())
    });

    c.bench_function("title case heck", |b| {
        b.iter(|| black_box(test).to_title_case())
    });

    // Sentence case
    c.bench_function("sentence case fast_case", |b| {
        b.iter(|| to_sentence_case(black_box(test)))
    });

    // no equivalent function in convert_case
    // no equivalent function in case_switcher
    // no equivalent function in heck

    c.bench_function("sentence case change_case", |b| {
        b.iter(|| change_case::sentence_case(black_box(test)))
    });

    c.bench_function("sentence case recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).sentence_case())
    });

    // kebab-case
    c.bench_function("kebab case fast_case", |b| {
        b.iter(|| to_kebab_case(black_box(test)))
    });

    c.bench_function("kebab case convert_case", |b| {
        b.iter(|| black_box(test).to_case(Case::Kebab))
    });

    c.bench_function("kebab case change_case", |b| {
        b.iter(|| change_case::param_case(black_box(test)))
    });

    c.bench_function("kebab case case_switcher", |b| {
        b.iter(|| case_switcher::to_kebab(black_box(test)))
    });

    c.bench_function("kebab case recase", |b| {
        b.iter(|| recase::ReCase::new(black_box(test)).kebab_case())
    });

    c.bench_function("kebab case heck", |b| {
        b.iter(|| black_box(test).to_kebab_case())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
