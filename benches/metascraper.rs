use criterion::{criterion_group, criterion_main, Criterion};
use metascraper::MetaScraper;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| {
            let input = include_str!("../src/test.html");
            let metascraper = MetaScraper::parse(input).unwrap();
            let _metadata = metascraper.metadata();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
