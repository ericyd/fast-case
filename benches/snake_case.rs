use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate fast_case;
use fast_case::convert_case;

// #[bench]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("to_snake_case 'aa_bb CC_DD eeFf GgHh Ii Jj'", |b| {
        b.iter(|| convert_case("snake_case", black_box("aa_bb CC_DD eeFf GgHh Ii Jj")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
