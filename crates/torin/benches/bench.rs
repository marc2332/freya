pub mod support;

use criterion::BatchSize;
use hotpath::criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};

use crate::support::{
    BenchmarkConfig,
    BenchmarkInput,
};

fn criterion_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("benchmarks");

    for config in BenchmarkConfig::all() {
        group.significance_level(0.05).sample_size(config.sample);
        group.bench_function(config.name(), |bencher| {
            bencher.iter_batched_ref(
                || config.prepare(),
                BenchmarkInput::run,
                BatchSize::PerIteration,
            );
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
