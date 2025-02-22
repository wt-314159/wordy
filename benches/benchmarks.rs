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

fn zip_vs_for_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("zip vs for slice");

    group.bench_with_input(
        "zip",
        &("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz",
        "abcdefgjijklmnoplrstuvwxydabcdefghijklmnopqrstuvwnyzabcdefghijklmnopqrstuvwxyzabcdegghijklmnopqrstuvwxyz"),
        |b, (a, c)| b.iter(|| {
            let mut count = black_box(0);
            for (i, (x, y)) in a.chars().zip(c.chars()).enumerate() {
                if x == y {
                    count += black_box(i);
                }
            }
            black_box(count);
        })
    );

    group.bench_with_input(
        "for_slice",
        &("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz",
        "abcdefgjijklmnoplrstuvwxydabcdefghijklmnopqrstuvwnyzabcdefghijklmnopqrstuvwxyzabcdegghijklmnopqrstuvwxyz"),
        |b, (a, c)| b.iter(|| {
            let mut count = black_box(0);
            for i in 0..104 {
                if a[i..i+1] == c[i..i+1] {
                    count += black_box(i);
                }
            }
            black_box(count);
        })
    );
}

criterion_group!(benches, zip_vs_for_slice);
criterion_main!(benches);
