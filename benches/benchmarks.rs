use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use wordy::Tiles;

macro_rules! tiles {
    (X) => {
        Tiles::Grey
    };
    (Y) => {
        Tiles::Yellow
    };
    (G) => {
        Tiles::Green
    };
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt) => {
        [
            tiles!($t1),
            tiles!($t2),
            tiles!($t3),
            tiles!($t4),
            tiles!($t5),
        ]
    };
}

fn match_vs_compute(c: &mut Criterion) {
    let mut group = c.benchmark_group("match vs compute");

    group.bench_with_input(
        "compute", 
        &("abcde", "fghij", tiles!(G G G G G)),
    |b, (g, a, r)| b.iter(|| Tiles::compute(g, a) == *black_box(r)));

    group.bench_with_input(
        "match_best_case", 
        &("abcde", "fghij", tiles!(G G G G G)),
    |b, (g, a, r)| b.iter(|| Tiles::matches(g, a, r)));

    group.bench_with_input(
        "match_worst_case", 
        &("abcde", "fghij", tiles!(X X X X X)),
    |b, (g, a, r)| b.iter(|| Tiles::matches(g, a, r)));
}

criterion_group!(benches, match_vs_compute);
criterion_main!(benches);
