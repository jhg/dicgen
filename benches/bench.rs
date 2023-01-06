use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dicgen::DictionaryGenerator;

pub fn criterion_benchmark(c: &mut Criterion) {
    let alphabet = black_box("0123456789");
    c.bench_function("gen 00000 99999", |b| b.iter(|| {
        let generator = DictionaryGenerator::new(alphabet, black_box("00000"), black_box("99999"));
        for item in generator {
            black_box(item);
        }
    }));
    c.bench_function("gen 00000 000000", |b| b.iter(|| {
        let generator = DictionaryGenerator::new(alphabet, black_box("00000"), black_box("000000"));
        for item in generator {
            black_box(item);
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);