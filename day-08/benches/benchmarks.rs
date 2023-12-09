use solutions::{get_input, part_one, part_two};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = get_input(true);
    c.benchmark_group("day-08")
        .bench_function("part one", |b| b.iter(|| part_one(black_box(&input))));
    c.benchmark_group("day-08")
        .bench_function("part two", |b| b.iter(|| part_two(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
