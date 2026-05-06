// Benchmark for large input processing
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smsdao::get_fees;

fn benchmark_fee_calculation(c: &mut Criterion) {
    c.bench_function("fee calculation", |b| {
        b.iter(|| {
            let amount = black_box(1_000_000_000);
            get_fees(amount)
        });
    });
}

criterion_group!(benches, benchmark_fee_calculation);
criterion_main!(benches);
